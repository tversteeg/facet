use shapely::Shapely;

unsafe extern "C" {
    pub unsafe fn get_library_message() -> *const std::ffi::c_char;
    pub unsafe fn get_foo() -> *const Foo;
}

#[derive(Shapely)]
#[repr(C)]
pub struct Foo {
    pub x: i64,
    pub bar: Bar,
    pub y: i64,
}

#[derive(Shapely)]
#[repr(C)]
pub struct Bar {
    pub a: i32,
    pub b: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message() {
        let msg = unsafe { get_library_message() };
        let c_str = unsafe { std::ffi::CStr::from_ptr(msg) };
        let rust_str = c_str.to_string_lossy();
        println!("{}", rust_str);

        assert_eq!(rust_str, "IAMA C lib AMA")
    }

    #[test]
    fn foo() {
        let foo = unsafe { get_foo() };
        let bar = unsafe { &(*foo).bar };
        let x = unsafe { (*foo).x };
        let y = unsafe { (*foo).y };

        println!("Foo: x={}, bar.a={}, bar.b={}, y={}", x, bar.a, bar.b, y);
    }
}
