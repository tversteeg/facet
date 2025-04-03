use std::fmt::{Debug, Display};

use owo_colors::OwoColorize;
use shapely::{Peek, Shapely};

#[test]
fn test_spez1() {
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
fn test_spez2() {
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

    eprintln!();
    eprintln!("{}", "=================== StructDebugNo".yellow());
    #[derive(Shapely)]
    struct StructDebugNo {
        blah: i32,
    }
    let peek = Peek::new(&StructDebugNo { blah: 42 });
    println!("StructDebugNo Peek: {}", format!("{peek:#?}").red());

    eprintln!();
    eprintln!("{}", "=================== StructDebugYes".yellow());
    #[derive(Shapely, Debug)]
    struct StructDebugYes {
        blah: i32,
    }
    let peek = Peek::new(&StructDebugYes { blah: 42 });
    println!("StructDebugYes Peek: {}", format!("{peek:#?}").green());

    eprintln!();
    eprintln!("{}", "=================== TupleStructDebugNo".yellow());
    #[derive(Shapely)]
    struct TupleStructDebugNo(i32, String);
    let tuple_struct_no = TupleStructDebugNo(42, "Hello".to_string());
    let peek_no = Peek::new(&tuple_struct_no);
    println!("TupleStructDebugNo Peek: {}", format!("{peek_no:#?}").red());

    eprintln!();
    eprintln!("{}", "=================== TupleStructDebugYes".yellow());
    #[derive(Shapely, Debug)]
    struct TupleStructDebugYes(i32, String);
    let tuple_struct_yes = TupleStructDebugYes(42, "Hello".to_string());
    let peek_yes = Peek::new(&tuple_struct_yes);
    println!(
        "TupleStructDebugYes Peek: {}",
        format!("{peek_yes:#?}").green()
    );

    // eprintln!();
    // eprintln!("{}", "=================== Enum".yellow());
    // #[derive(Shapely, Debug)]
    // enum MyEnum {
    //     Variant1,
    //     Variant2(i32),
    //     Variant3 { field: String },
    // }
    // let peek1 = Peek::new(&MyEnum::Variant1);
    // let peek2 = Peek::new(&MyEnum::Variant2(42));
    // let peek3 = Peek::new(&MyEnum::Variant3 {
    //     field: "Hello".to_string(),
    // });
    // println!("Enum Peek (Variant1): {}", format!("{peek1:#?}").green());
    // println!("Enum Peek (Variant2): {}", format!("{peek2:#?}").green());
    // println!("Enum Peek (Variant3): {}", format!("{peek3:#?}").green());
}
