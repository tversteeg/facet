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
fn debug_or_not() {
    eprintln!("{}", "=================== i32".yellow());
    let v: i32 = 42;
    let peek = Peek::new(&v);
    eprintln!("{}", format!("{peek:?}").green());

    eprintln!("{}", "=================== Vec<i32>".yellow());
    let v: Vec<i32> = vec![1, 2, 3];
    let peek = Peek::new(&v);
    eprintln!("{}", format!("{peek:?}").blue());

    eprintln!("{}", "=================== StructDebugNo".yellow());
    #[derive(Shapely)]
    struct StructDebugNo {
        blah: i32,
    }
    let peek = Peek::new(&StructDebugNo { blah: 42 });
    eprintln!("{}", format!("{peek:?}").red());

    eprintln!("{}", "=================== StructDebugYes".yellow());
    #[derive(Shapely, Debug)]
    struct StructDebugYes {
        blah: i32,
    }
    let peek = Peek::new(&StructDebugYes { blah: 42 });
    eprintln!("{}", format!("{peek:?}").green());

    eprintln!("{}", "=================== TupleStructDebugNo".yellow());
    #[derive(Shapely)]
    struct TupleStructDebugNo(i32, String);
    let tuple_struct_no = TupleStructDebugNo(42, "Hello".to_string());
    let peek_no = Peek::new(&tuple_struct_no);
    eprintln!("{}", format!("{peek_no:?}").red());

    eprintln!("{}", "=================== TupleStructDebugYes".yellow());
    #[derive(Shapely, Debug)]
    struct TupleStructDebugYes(i32, String);
    let tuple_struct_yes = TupleStructDebugYes(42, "Hello".to_string());
    let peek_yes = Peek::new(&tuple_struct_yes);
    eprintln!("{}", format!("{peek_yes:?}").green());

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
    // println!("Enum Peek (Variant1): {}", format!("{peek1:?}").green());
    // println!("Enum Peek (Variant2): {}", format!("{peek2:?}").green());
    // println!("Enum Peek (Variant3): {}", format!("{peek3:?}").green());
}

#[test]
fn eq_or_not() {
    eprintln!("{}", "=================== i32 (PartialEq)".yellow());
    let v1: i32 = 42;
    let v2: i32 = 42;
    let peek1 = Peek::new(&v1);
    let peek2 = Peek::new(&v2);
    println!(
        "i32 Equality: {}",
        format!("{:?}", peek1.as_value().eq(&peek2.as_value())).green()
    );

    eprintln!("{}", "=================== StructEqNo".yellow());
    #[derive(Shapely)]
    struct StructEqNo {
        blah: i32,
    }
    let s1 = StructEqNo { blah: 42 };
    let s2 = StructEqNo { blah: 42 };
    let peek1 = Peek::new(&s1);
    let peek2 = Peek::new(&s2);
    println!(
        "StructEqNo Equality: {}",
        format!("{:?}", peek1.as_value().eq(&peek2.as_value())).red()
    );

    eprintln!("{}", "=================== StructEqYes".yellow());
    #[derive(Shapely, PartialEq)]
    struct StructEqYes {
        blah: i32,
    }
    let s1 = StructEqYes { blah: 42 };
    let s2 = StructEqYes { blah: 42 };
    let peek1 = Peek::new(&s1);
    let peek2 = Peek::new(&s2);
    println!(
        "StructEqYes Equality: {}",
        format!("{:?}", peek1.as_value().eq(&peek2.as_value())).green()
    );
}

#[test]
fn cmp_or_not() {
    eprintln!("{}", "=================== i32 (Ord)".yellow());
    let v1: i32 = 42;
    let v2: i32 = 24;
    let peek1 = Peek::new(&v1);
    let peek2 = Peek::new(&v2);
    println!(
        "i32 Comparison: {}",
        format!("{:?}", peek1.as_value().cmp(&peek2.as_value())).green()
    );

    eprintln!("{}", "=================== StructOrdNo".yellow());
    #[derive(Shapely)]
    struct StructOrdNo {
        blah: i32,
    }
    let s1 = StructOrdNo { blah: 42 };
    let s2 = StructOrdNo { blah: 24 };
    let peek1 = Peek::new(&s1);
    let peek2 = Peek::new(&s2);
    println!(
        "StructOrdNo Comparison: {}",
        format!("{:?}", peek1.as_value().cmp(&peek2.as_value())).red()
    );

    eprintln!("{}", "=================== StructOrdYes".yellow());
    #[derive(Shapely, PartialOrd, Ord, PartialEq, Eq)]
    struct StructOrdYes {
        blah: i32,
    }
    let s1 = StructOrdYes { blah: 42 };
    let s2 = StructOrdYes { blah: 24 };
    let peek1 = Peek::new(&s1);
    let peek2 = Peek::new(&s2);
    println!(
        "StructOrdYes Comparison: {}",
        format!("{:?}", peek1.as_value().cmp(&peek2.as_value())).green()
    );
}
