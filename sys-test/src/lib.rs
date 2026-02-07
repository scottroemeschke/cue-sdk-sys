#[cfg(test)]
mod tests {
    #[test]
    pub fn sanity_test() {
        println!("starting sanity test");
        unsafe {
            let mut details = std::mem::zeroed::<cue_sdk_sys::CorsairSessionDetails>();
            let err = cue_sdk_sys::CorsairGetSessionDetails(&mut details);
            // CE_NotConnected (1) is expected when iCUE is not running.
            // CE_Success (0) means iCUE is running and we got session details.
            assert!(
                err == cue_sdk_sys::CorsairError_CE_Success
                    || err == cue_sdk_sys::CorsairError_CE_NotConnected,
                "CorsairGetSessionDetails returned unexpected error: {}",
                err,
            );

            // clientVersion is always populated, even without a connection
            let cv = details.clientVersion;
            assert_eq!(
                cv.major, 4,
                "Expected SDK major version 4, got {}",
                cv.major
            );
        }
    }
}
