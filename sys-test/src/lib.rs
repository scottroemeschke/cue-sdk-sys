#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    #[test]
    pub fn sanity_test() {
        println!("starting sanity test");
        unsafe {
            let handshake = cue_sdk_sys::CorsairPerformProtocolHandshake();
            let c_str = CStr::from_ptr(handshake.sdkVersion);
            let rust_string = c_str.to_str().unwrap();
            assert_eq!(rust_string, "3.0.361");
        }
    }
}
