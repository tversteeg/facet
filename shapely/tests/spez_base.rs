use std::fmt::{Debug, Display};


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
