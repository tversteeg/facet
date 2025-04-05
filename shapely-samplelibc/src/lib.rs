unsafe extern "C" {
    pub unsafe fn get_library_message() -> *const std::ffi::c_char;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let msg = unsafe { get_library_message() };
        let c_str = unsafe { std::ffi::CStr::from_ptr(msg) };
        let rust_str = c_str.to_string_lossy();
        println!("{}", rust_str);

        assert_eq!(rust_str, "IAMA C lib AMA")
    }
}
