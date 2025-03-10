use shapely::Shape;

#[cfg(feature = "log")]
use log::{error, trace, warn};

#[cfg(not(feature = "log"))]
macro_rules! error {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "log"))]
macro_rules! trace {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "log"))]
macro_rules! warn {
    ($($arg:tt)*) => {};
}

#[derive(Debug)]
pub struct JsonParseError {
    pub kind: JsonParseErrorKind,
    pub position: usize,
}

#[derive(Debug)]
pub enum JsonParseErrorKind {
    ExpectedOpeningQuote,
    UnterminatedString,
    InvalidEscapeSequence(char),
    IncompleteUnicodeEscape,
    InvalidUnicodeEscape,
    ExpectedNumber,
    ExpectedOpeningBrace,
    ExpectedColon,
    UnexpectedEndOfInput,
    InvalidValue,
    ExpectedClosingBrace,
    Custom(String),
}

impl JsonParseError {
    pub fn new(kind: JsonParseErrorKind, position: usize) -> Self {
        JsonParseError { kind, position }
    }
}

#[derive(Debug)]
pub struct JsonParseErrorWithContext<'input> {
    pub error: JsonParseError,
    pub input: &'input str,
}

impl<'a> JsonParseErrorWithContext<'a> {
    pub fn strip_context(self) -> JsonParseError {
        self.error
    }
}

impl std::fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match &self.kind {
            JsonParseErrorKind::ExpectedOpeningQuote => "Expected opening quote for string",
            JsonParseErrorKind::UnterminatedString => "Unterminated string",
            JsonParseErrorKind::InvalidEscapeSequence(ch) => {
                return write!(f, "Invalid escape sequence: \\{}", ch)
            }
            JsonParseErrorKind::IncompleteUnicodeEscape => "Incomplete Unicode escape sequence",
            JsonParseErrorKind::InvalidUnicodeEscape => "Invalid Unicode escape sequence",
            JsonParseErrorKind::ExpectedNumber => "Expected a number",
            JsonParseErrorKind::ExpectedOpeningBrace => "Expected opening brace for object",
            JsonParseErrorKind::ExpectedColon => "Expected ':' after object key",
            JsonParseErrorKind::UnexpectedEndOfInput => "Unexpected end of input",
            JsonParseErrorKind::InvalidValue => "Invalid value",
            JsonParseErrorKind::ExpectedClosingBrace => "Expected closing brace for object",
            JsonParseErrorKind::Custom(msg) => msg,
        };

        write!(f, "{} at position {}", error_message, self.position)
    }
}

impl<'a> std::fmt::Display for JsonParseErrorWithContext<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let context_start = self.error.position.saturating_sub(20);
        let context_end = (self.error.position + 20).min(self.input.len());
        let context = &self.input[context_start..context_end];
        let arrow_position = self.error.position - context_start;

        write!(f, "{}\n", self.error)?;
        write!(f, "\x1b[36m{}\x1b[0m\n", context)?;
        write!(f, "{}\x1b[31m^\x1b[0m", " ".repeat(arrow_position))
    }
}

impl std::error::Error for JsonParseError {}

pub fn from_json<'input>(
    target: *mut u8,
    schema: Shape,
    json: &'input str,
) -> Result<(), JsonParseErrorWithContext<'input>> {
    use shapely::{MapShape, Scalar, ShapeKind};

    trace!("Starting JSON deserialization");
    let mut parser = JsonParser::new(json);

    fn deserialize_value<'input>(
        parser: &'input mut JsonParser,
        target: *mut u8,
        shape: &Shape,
    ) -> Result<(), JsonParseErrorWithContext<'input>> {
        trace!("Deserializing value with schema:\n{:?}", schema);
        match &shape.shape {
            ShapeKind::Scalar(scalar) => {
                match scalar {
                    Scalar::String => {
                        trace!("Deserializing String");
                        let s = parser.parse_string()?;
                        trace!("Deserialized String: {}", s);
                        unsafe {
                            *(target as *mut String) = s;
                        }
                    }
                    Scalar::U64 => {
                        trace!("Deserializing U64");
                        let n = parser.parse_u64()?;
                        unsafe {
                            *(target as *mut u64) = n;
                        }
                        trace!("Deserialized U64: {}", n);
                    }
                    // Add other scalar types as needed
                    _ => {
                        warn!("Unsupported scalar type: {:?}", scalar);
                        return Err(parser.make_error(JsonParseErrorKind::Custom(format!(
                            "Unsupported scalar type: {:?}",
                            scalar
                        ))));
                    }
                }
            }
            ShapeKind::Map(MapShape {
                fields,
                manipulator,
                ..
            }) => {
                trace!("Deserializing Map");
                parser.expect_object_start()?;
                while let Some(key) = parser.parse_object_key()? {
                    trace!("Processing map key: {}", key);
                    if let Some(field) = fields.iter().find(|f| f.name == key) {
                        let field_schema = (field.schema)();
                        trace!("Deserializing field: {}", field.name);
                        let mut field_error = None;
                        unsafe {
                            manipulator.set_field_raw(target, *field, &mut |field_ptr| {
                                if let Err(err) =
                                    deserialize_value(parser, field_ptr, &field_schema)
                                {
                                    field_error = Some(err);
                                }
                            });
                        }
                        if let Some(err) = field_error {
                            return Err(err);
                        }
                    } else {
                        warn!("Unknown field: {}, skipping", key);
                        parser.skip_value()?;
                    }
                }
                parser.expect_object_end()?;
                trace!("Finished deserializing Map");
            }
            // Add support for other shapes (Array, Transparent) as needed
            _ => {
                error!("Unsupported shape: {:?}", schema.shape);
                return Err(parser.make_error(JsonParseErrorKind::Custom(format!(
                    "Unsupported shape: {:?}",
                    shape.shape
                ))));
            }
        }
        Ok(())
    }

    let result = deserialize_value(&mut parser, target, &schema);
    if result.is_ok() {
        trace!("JSON deserialization completed successfully");
    } else {
        error!("JSON deserialization failed: {:?}", result);
    }
    result
}

struct JsonParser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> JsonParser<'a> {
    fn new(input: &'a str) -> Self {
        JsonParser { input, position: 0 }
    }

    fn make_error(&self, kind: JsonParseErrorKind) -> JsonParseErrorWithContext<'a> {
        JsonParseErrorWithContext {
            error: JsonParseError::new(kind, self.position),
            input: self.input,
        }
    }

    fn parse_string(&mut self) -> Result<String, JsonParseErrorWithContext<'a>> {
        self.skip_whitespace();
        if self.position >= self.input.len() || self.input.as_bytes()[self.position] != b'"' {
            return Err(self.make_error(JsonParseErrorKind::ExpectedOpeningQuote));
        }
        self.position += 1;

        let mut result = String::new();
        let mut escaped = false;

        while self.position < self.input.len() {
            let ch = self.input.as_bytes()[self.position];
            self.position += 1;

            if escaped {
                match ch {
                    b'"' | b'\\' | b'/' => result.push(ch as char),
                    b'b' => result.push('\x08'),
                    b'f' => result.push('\x0C'),
                    b'n' => result.push('\n'),
                    b'r' => result.push('\r'),
                    b't' => result.push('\t'),
                    b'u' => {
                        // Parse 4-digit hex code
                        if self.position + 4 > self.input.len() {
                            return Err(
                                self.make_error(JsonParseErrorKind::IncompleteUnicodeEscape)
                            );
                        }
                        let hex = &self.input[self.position..self.position + 4];
                        self.position += 4;
                        if let Ok(code) = u16::from_str_radix(hex, 16) {
                            result.push(char::from_u32(code as u32).unwrap_or('\u{FFFD}'));
                        } else {
                            return Err(self.make_error(JsonParseErrorKind::InvalidUnicodeEscape));
                        }
                    }
                    _ => {
                        return Err(
                            self.make_error(JsonParseErrorKind::InvalidEscapeSequence(ch as char))
                        )
                    }
                }
                escaped = false;
            } else if ch == b'\\' {
                escaped = true;
            } else if ch == b'"' {
                return Ok(result);
            } else {
                result.push(ch as char);
            }
        }

        Err(self.make_error(JsonParseErrorKind::UnterminatedString))
    }

    fn parse_u64(&mut self) -> Result<u64, JsonParseErrorWithContext<'a>> {
        self.skip_whitespace();
        let start = self.position;
        while self.position < self.input.len()
            && self.input.as_bytes()[self.position].is_ascii_digit()
        {
            self.position += 1;
        }
        if start == self.position {
            return Err(self.make_error(JsonParseErrorKind::ExpectedNumber));
        }
        let num_str = &self.input[start..self.position];
        num_str
            .parse::<u64>()
            .map_err(|_| self.make_error(JsonParseErrorKind::ExpectedNumber))
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            match self.input.as_bytes()[self.position] {
                b' ' | b'\t' | b'\n' | b'\r' => self.position += 1,
                _ => break,
            }
        }
    }

    fn expect_object_start(&mut self) -> Result<(), JsonParseErrorWithContext<'a>> {
        self.skip_whitespace();
        if self.position >= self.input.len() || self.input.as_bytes()[self.position] != b'{' {
            return Err(self.make_error(JsonParseErrorKind::ExpectedOpeningBrace));
        }
        self.position += 1;
        Ok(())
    }

    fn parse_object_key(&mut self) -> Result<Option<String>, JsonParseErrorWithContext<'a>> {
        self.skip_whitespace();
        if self.position >= self.input.len() {
            return Ok(None);
        }
        match self.input.as_bytes()[self.position] {
            b'"' => {
                let key = self.parse_string()?;
                self.skip_whitespace();
                if self.position < self.input.len() && self.input.as_bytes()[self.position] == b':'
                {
                    self.position += 1;
                    Ok(Some(key))
                } else {
                    Err(self.make_error(JsonParseErrorKind::ExpectedColon))
                }
            }
            b'}' => Ok(None),
            _ => Err(self.make_error(JsonParseErrorKind::InvalidValue)),
        }
    }

    fn skip_value(&mut self) -> Result<(), JsonParseErrorWithContext<'a>> {
        self.skip_whitespace();
        if self.position >= self.input.len() {
            return Err(self.make_error(JsonParseErrorKind::UnexpectedEndOfInput));
        }
        match self.input.as_bytes()[self.position] {
            b'{' => self.skip_object(),
            b'[' => self.skip_array(),
            b'"' => {
                self.parse_string()?;
                Ok(())
            }
            b'0'..=b'9' | b'-' => {
                while self.position < self.input.len() {
                    match self.input.as_bytes()[self.position] {
                        b'0'..=b'9' | b'.' | b'e' | b'E' | b'+' | b'-' => self.position += 1,
                        _ => break,
                    }
                }
                Ok(())
            }
            b't' => {
                if self.input[self.position..].starts_with("true") {
                    self.position += 4;
                    Ok(())
                } else {
                    Err(self.make_error(JsonParseErrorKind::InvalidValue))
                }
            }
            b'f' => {
                if self.input[self.position..].starts_with("false") {
                    self.position += 5;
                    Ok(())
                } else {
                    Err(self.make_error(JsonParseErrorKind::InvalidValue))
                }
            }
            b'n' => {
                if self.input[self.position..].starts_with("null") {
                    self.position += 4;
                    Ok(())
                } else {
                    Err(self.make_error(JsonParseErrorKind::InvalidValue))
                }
            }
            _ => Err(self.make_error(JsonParseErrorKind::InvalidValue)),
        }
    }

    fn skip_object(&mut self) -> Result<(), JsonParseErrorWithContext<'a>> {
        self.expect_object_start()?;
        let mut depth = 1;
        while depth > 0 {
            match self.parse_object_key()? {
                Some(_) => {
                    self.skip_value()?;
                }
                None => {
                    depth -= 1;
                    if depth > 0 {
                        self.expect_object_start()?;
                        depth += 1;
                    }
                }
            }
        }
        Ok(())
    }

    fn skip_array(&mut self) -> Result<(), JsonParseErrorWithContext<'a>> {
        self.skip_whitespace();
        if self.position >= self.input.len() || self.input.as_bytes()[self.position] != b'[' {
            return Err(self.make_error(JsonParseErrorKind::InvalidValue));
        }
        self.position += 1;
        let mut depth = 1;
        while depth > 0 {
            self.skip_whitespace();
            if self.position >= self.input.len() {
                return Err(self.make_error(JsonParseErrorKind::UnexpectedEndOfInput));
            }
            match self.input.as_bytes()[self.position] {
                b']' => {
                    depth -= 1;
                    self.position += 1;
                }
                b'[' => {
                    depth += 1;
                    self.position += 1;
                }
                b',' => {
                    self.position += 1;
                }
                _ => {
                    self.skip_value()?;
                }
            }
        }
        Ok(())
    }

    fn expect_object_end(&mut self) -> Result<(), JsonParseErrorWithContext<'a>> {
        self.skip_whitespace();
        if self.position >= self.input.len() || self.input.as_bytes()[self.position] != b'}' {
            return Err(self.make_error(JsonParseErrorKind::ExpectedClosingBrace));
        }
        self.position += 1;
        Ok(())
    }

    fn make_error(&self, kind: JsonParseErrorKind) -> JsonParseErrorWithContext<'a> {
        JsonParseErrorWithContext {
            error: JsonParseError::new(kind, self.position),
            input: self.input,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use log::{Metadata, Record};
    use shapely::Shapely;

    struct SimpleLogger;

    impl log::Log for SimpleLogger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            true
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                let level_color = match record.level() {
                    log::Level::Error => "\x1b[31m", // Red
                    log::Level::Warn => "\x1b[33m",  // Yellow
                    log::Level::Info => "\x1b[32m",  // Green
                    log::Level::Debug => "\x1b[36m", // Cyan
                    log::Level::Trace => "\x1b[35m", // Magenta
                };
                let target_color = "\x1b[34m"; // Blue for target
                let args_color = "\x1b[37m"; // White for args
                eprintln!(
                    "{}{}\x1b[0m {}{}:\x1b[0m {}{}\x1b[0m",
                    target_color,
                    record.target(),
                    level_color,
                    record.level(),
                    args_color,
                    record.args()
                );
            }
        }

        fn flush(&self) {}
    }

    #[test]
    fn test_from_json() {
        log::set_logger(&SimpleLogger).unwrap();
        log::set_max_level(log::LevelFilter::Trace);

        #[derive(Debug, PartialEq)]
        struct TestStruct {
            name: String,
            age: u64,
        }

        impl Shapely for TestStruct {
            fn shape() -> Shape {
                use shapely::{MapField, MapShape, Shape, ShapeKind, StructManipulator};

                static NAME_FIELD: MapField = MapField {
                    name: "name",
                    schema: <String as Shapely>::shape,
                };
                static AGE_FIELD: MapField = MapField {
                    name: "age",
                    schema: <u64 as Shapely>::shape,
                };
                static SCHEMA: Shape = Shape {
                    name: "TestStruct",
                    size: std::mem::size_of::<TestStruct>(),
                    align: std::mem::align_of::<TestStruct>(),
                    shape: ShapeKind::Map(MapShape {
                        fields: &[NAME_FIELD, AGE_FIELD],
                        open_ended: false,
                        manipulator: &StructManipulator {
                            fields: &[
                                (NAME_FIELD, std::mem::offset_of!(TestStruct, name)),
                                (AGE_FIELD, std::mem::offset_of!(TestStruct, age)),
                            ],
                        },
                    }),
                    display: None,
                    debug: None,
                    set_to_default: None,
                };
                SCHEMA
            }
        }

        let json = r#"{"name": "Alice", "age": 30}"#;
        let mut test_struct = TestStruct {
            name: String::new(),
            age: 0,
        };

        let result = from_json(
            &mut test_struct as *mut TestStruct as *mut u8,
            TestStruct::shape(),
            json,
        );

        result.unwrap();
        assert_eq!(
            test_struct,
            TestStruct {
                name: "Alice".to_string(),
                age: 30
            }
        );
    }
}
