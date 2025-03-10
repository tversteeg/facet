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
pub enum JsonParseError {
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

impl std::fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonParseError::ExpectedOpeningQuote => write!(f, "Expected opening quote for string"),
            JsonParseError::UnterminatedString => write!(f, "Unterminated string"),
            JsonParseError::InvalidEscapeSequence(ch) => {
                write!(f, "Invalid escape sequence: \\{}", ch)
            }
            JsonParseError::IncompleteUnicodeEscape => {
                write!(f, "Incomplete Unicode escape sequence")
            }
            JsonParseError::InvalidUnicodeEscape => write!(f, "Invalid Unicode escape sequence"),
            JsonParseError::ExpectedNumber => write!(f, "Expected a number"),
            JsonParseError::ExpectedOpeningBrace => write!(f, "Expected opening brace for object"),
            JsonParseError::ExpectedColon => write!(f, "Expected ':' after object key"),
            JsonParseError::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
            JsonParseError::InvalidValue => write!(f, "Invalid value"),
            JsonParseError::ExpectedClosingBrace => write!(f, "Expected closing brace for object"),
            JsonParseError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for JsonParseError {}

pub fn from_json(target: *mut u8, schema: Shape, json: &str) -> Result<(), JsonParseError> {
    use shapely::{MapShape, Scalar, ShapeKind};

    trace!("Starting JSON deserialization");
    let mut parser = JsonParser::new(json);

    fn deserialize_value(
        parser: &mut JsonParser,
        target: *mut u8,
        schema: &Shape,
    ) -> Result<(), JsonParseError> {
        trace!("Deserializing value with schema:\n{:?}", schema);
        match &schema.shape {
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
                        return Err(JsonParseError::Custom(format!(
                            "Unsupported scalar type: {:?}",
                            scalar
                        )));
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
                        unsafe {
                            manipulator.set_field_raw(target, *field, &mut |field_ptr| {
                                deserialize_value(parser, field_ptr, &field_schema).unwrap();
                            });
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
                return Err(JsonParseError::Custom(format!(
                    "Unsupported shape: {:?}",
                    schema.shape
                )));
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

    fn parse_string(&mut self) -> Result<String, JsonParseError> {
        self.skip_whitespace();
        if self.position >= self.input.len() || self.input.as_bytes()[self.position] != b'"' {
            return Err(JsonParseError::ExpectedOpeningQuote);
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
                            return Err(JsonParseError::IncompleteUnicodeEscape);
                        }
                        let hex = &self.input[self.position..self.position + 4];
                        self.position += 4;
                        if let Ok(code) = u16::from_str_radix(hex, 16) {
                            result.push(char::from_u32(code as u32).unwrap_or('\u{FFFD}'));
                        } else {
                            return Err(JsonParseError::InvalidUnicodeEscape);
                        }
                    }
                    _ => return Err(JsonParseError::InvalidEscapeSequence(ch as char)),
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

        Err(JsonParseError::UnterminatedString)
    }

    fn parse_u64(&mut self) -> Result<u64, JsonParseError> {
        self.skip_whitespace();
        let start = self.position;
        while self.position < self.input.len()
            && self.input.as_bytes()[self.position].is_ascii_digit()
        {
            self.position += 1;
        }
        if start == self.position {
            return Err(JsonParseError::ExpectedNumber);
        }
        let num_str = &self.input[start..self.position];
        num_str
            .parse::<u64>()
            .map_err(|_| JsonParseError::ExpectedNumber)
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            match self.input.as_bytes()[self.position] {
                b' ' | b'\t' | b'\n' | b'\r' => self.position += 1,
                _ => break,
            }
        }
    }

    fn expect_object_start(&mut self) -> Result<(), JsonParseError> {
        self.skip_whitespace();
        if self.position >= self.input.len() || self.input.as_bytes()[self.position] != b'{' {
            return Err(JsonParseError::ExpectedOpeningBrace);
        }
        self.position += 1;
        Ok(())
    }

    fn parse_object_key(&mut self) -> Result<Option<String>, JsonParseError> {
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
                    Err(JsonParseError::ExpectedColon)
                }
            }
            b'}' => Ok(None),
            _ => Err(JsonParseError::InvalidValue),
        }
    }

    fn skip_value(&mut self) -> Result<(), JsonParseError> {
        self.skip_whitespace();
        if self.position >= self.input.len() {
            return Err(JsonParseError::UnexpectedEndOfInput);
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
                    Err(JsonParseError::InvalidValue)
                }
            }
            b'f' => {
                if self.input[self.position..].starts_with("false") {
                    self.position += 5;
                    Ok(())
                } else {
                    Err(JsonParseError::InvalidValue)
                }
            }
            b'n' => {
                if self.input[self.position..].starts_with("null") {
                    self.position += 4;
                    Ok(())
                } else {
                    Err(JsonParseError::InvalidValue)
                }
            }
            _ => Err(JsonParseError::InvalidValue),
        }
    }

    fn skip_object(&mut self) -> Result<(), JsonParseError> {
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

    fn skip_array(&mut self) -> Result<(), JsonParseError> {
        self.skip_whitespace();
        if self.position >= self.input.len() || self.input.as_bytes()[self.position] != b'[' {
            return Err(JsonParseError::InvalidValue);
        }
        self.position += 1;
        let mut depth = 1;
        while depth > 0 {
            self.skip_whitespace();
            if self.position >= self.input.len() {
                return Err(JsonParseError::UnexpectedEndOfInput);
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

    fn expect_object_end(&mut self) -> Result<(), JsonParseError> {
        self.skip_whitespace();
        if self.position >= self.input.len() || self.input.as_bytes()[self.position] != b'}' {
            return Err(JsonParseError::ExpectedClosingBrace);
        }
        self.position += 1;
        Ok(())
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

        assert!(result.is_ok());
        assert_eq!(
            test_struct,
            TestStruct {
                name: "Alice".to_string(),
                age: 30
            }
        );
    }
}
