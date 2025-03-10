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
        fn shape() -> shapely::Shape {
            use shapely::Innards;
            static SCHEMA: shapely::Shape = shapely::Shape {
                name: "TestStruct",
                size: std::mem::size_of::<TestStruct>(),
                align: std::mem::align_of::<TestStruct>(),
                innards: Innards::Struct {
                    fields: shapely::struct_fields!(TestStruct, (name, age)),
                },
                display: None,
                debug: Some(|addr: *const u8, f: &mut std::fmt::Formatter| {
                    std::fmt::Debug::fmt(unsafe { &*(addr as *const TestStruct) }, f)
                }),
                set_to_default: None,
            };
            SCHEMA
        }
    }

    let json = r#"{"name": "Alice", "age": 30}"#;

    let mut test_struct = TestStruct::shape_uninit();
    let result = from_json(&mut test_struct, json);

    result.unwrap();
    let built_struct = test_struct.build::<TestStruct>();

    assert_eq!(
        built_struct,
        TestStruct {
            name: "Alice".to_string(),
            age: 30
        }
    );
}
