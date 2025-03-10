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
            use shapely::{Innards, MapField, MapInnards, Shaps, StructManipulator};

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
                innards: Innards::Map(MapInnards {
                    fields: &[NAME_FIELD, AGE_FIELD],
                    open_ended: false,
                    slots: &StructManipulator {
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
