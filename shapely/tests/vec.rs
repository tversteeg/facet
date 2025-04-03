use std::fmt::{Debug, Display};

use owo_colors::OwoColorize;
use shapely::{Peek, Shapely};

#[test]
fn test_spez() {
    struct Wrap<T>(T);

    trait ViaString {
        fn foo(&self);
    }
    impl ViaString for &&Wrap<String> {
        fn foo(&self) {
            println!("String: {}", self.0);
        }
    }

    trait ViaDisplay {
        fn foo(&self);
    }
    impl<T: Display> ViaDisplay for &Wrap<T> {
        fn foo(&self) {
            println!("Display: {}", self.0);
        }
    }

    trait ViaDebug {
        fn foo(&self);
    }
    impl<T: Debug> ViaDebug for Wrap<T> {
        fn foo(&self) {
            println!("Debug: {:?}", self.0);
        }
    }

    // Test method calls
    (&&&Wrap(String::from("hi"))).foo();
    (&&Wrap(3)).foo();
    Wrap(['a', 'b']).foo();
}

#[test]
fn test_spez_const() {
    struct Wrap<T>(T);

    trait ViaString {
        const VALUE: &'static str;

        fn value(&self) -> &'static str {
            Self::VALUE
        }
    }
    impl ViaString for &&Wrap<String> {
        const VALUE: &'static str = "String";

        fn value(&self) -> &'static str {
            Self::VALUE
        }
    }

    trait ViaDisplay {
        const VALUE: &'static str;

        fn value(&self) -> &'static str {
            Self::VALUE
        }
    }
    impl<T: Display> ViaDisplay for &Wrap<T> {
        const VALUE: &'static str = "Display";
    }

    trait ViaDebug {
        const VALUE: &'static str;

        fn value(&self) -> &'static str {
            Self::VALUE
        }
    }

    impl<T: Debug> ViaDebug for Wrap<T> {
        const VALUE: &'static str = "Debug";
    }

    // Test method calls
    eprintln!("{}", (&&&Wrap(String::from("hi"))).value());
    eprintln!("{}", (&&Wrap(3)).value());
    eprintln!("{}", Wrap(['a', 'b']).value());
}

#[test]
fn vec_can_be_debug_or_not() {
    eprintln!();
    eprintln!("{}", "=================== i32".yellow());
    let v: i32 = 42;
    let peek = Peek::new(&v);
    println!("Integer Peek: {}", format!("{peek:#?}").green());

    eprintln!();
    eprintln!("{}", "=================== Vec<i32>".yellow());
    let v: Vec<i32> = vec![1, 2, 3];
    let peek = Peek::new(&v);
    println!("Vector Peek: {}", format!("{peek:#?}").blue());

    #[derive(Shapely)]
    struct NotDebug {
        blah: i32,
    }
    let peek = Peek::new(&NotDebug { blah: 42 });
    println!("NotDebug Peek: {}", format!("{peek:#?}").red());

    #[derive(Shapely, Debug)]
    struct YesDebug {
        blah: i32,
    }
    let peek = Peek::new(&YesDebug { blah: 42 });
    println!("YesDebug Peek: {}", format!("{peek:#?}").green());
}
