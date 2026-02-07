#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use cue_sdk_sys::*;

    /// Shared state for the session-state callback to record into.
    struct CallbackState {
        states: Vec<CorsairSessionState>,
    }

    /// FFI trampoline: the SDK calls this on session state changes.
    /// `context` is a `*mut Mutex<CallbackState>`.
    unsafe extern "C" fn on_state_changed(
        context: *mut core::ffi::c_void,
        event_data: *const CorsairSessionStateChanged,
    ) {
        let mutex = unsafe { &*(context as *const Mutex<CallbackState>) };
        let state = unsafe { (*event_data).state };
        if let Ok(mut guard) = mutex.lock() {
            guard.states.push(state);
        }
    }

    #[test]
    fn sdk_smoke_test() {
        let shared = Mutex::new(CallbackState { states: Vec::new() });
        let context = &shared as *const Mutex<CallbackState> as *mut core::ffi::c_void;

        // Step 1: CorsairConnect — proves native lib loads, FFI symbols resolve,
        // and callback registration works.
        let err = unsafe { CorsairConnect(Some(on_state_changed), context) };
        assert_eq!(
            err, CorsairError_CE_Success,
            "CorsairConnect returned error: {err}",
        );

        // Step 2: CorsairGetSessionDetails — proves struct layout matches native SDK.
        let mut details = unsafe { std::mem::zeroed::<CorsairSessionDetails>() };
        let err = unsafe { CorsairGetSessionDetails(&mut details) };
        assert_eq!(
            err, CorsairError_CE_Success,
            "CorsairGetSessionDetails returned error: {err}",
        );
        assert_eq!(
            details.clientVersion.major, 4,
            "Expected SDK major version 4, got {}",
            details.clientVersion.major,
        );
        // No iCUE running, so serverVersion should be zeroed.
        assert_eq!(details.serverVersion.major, 0);
        assert_eq!(details.serverVersion.minor, 0);
        assert_eq!(details.serverVersion.patch, 0);

        // Step 3: Wait for the async connection attempt to time out, then verify
        // the callback trampoline fired across the FFI boundary.
        std::thread::sleep(std::time::Duration::from_millis(1500));
        {
            let guard = shared.lock().unwrap();
            assert!(
                !guard.states.is_empty(),
                "Session state callback never fired",
            );
            // We expect CSS_Connecting and then CSS_Timeout or CSS_ConnectionRefused.
            assert!(
                guard.states.contains(&CorsairSessionState_CSS_Connecting),
                "Expected CSS_Connecting in callback states, got: {:?}",
                guard.states,
            );
            let has_terminal = guard.states.contains(&CorsairSessionState_CSS_Timeout)
                || guard
                    .states
                    .contains(&CorsairSessionState_CSS_ConnectionRefused);
            assert!(
                has_terminal,
                "Expected CSS_Timeout or CSS_ConnectionRefused in callback states, got: {:?}",
                guard.states,
            );
        }

        // Step 4: CorsairGetDevices — proves device query API works and fails
        // gracefully when not connected.
        let filter = CorsairDeviceFilter {
            deviceTypeMask: CorsairDeviceType_CDT_All as core::ffi::c_int,
        };
        let mut devices = [unsafe { std::mem::zeroed::<CorsairDeviceInfo>() }; 1];
        let mut size: core::ffi::c_int = 0;
        let err = unsafe {
            CorsairGetDevices(
                &filter,
                devices.len() as core::ffi::c_int,
                devices.as_mut_ptr(),
                &mut size,
            )
        };
        assert_eq!(
            err, CorsairError_CE_NotConnected,
            "CorsairGetDevices should return CE_NotConnected, got: {err}",
        );

        // Step 5: CorsairDisconnect — cleanup.
        let err = unsafe { CorsairDisconnect() };
        assert_eq!(
            err, CorsairError_CE_Success,
            "CorsairDisconnect returned error: {err}",
        );
    }

    /// Diagnostic test for macOS SIGBUS (issue #12).
    ///
    /// Performs a connect → disconnect → reconnect → disconnect cycle using the
    /// **same** callback context pointer for both sessions. The context (`shared`)
    /// lives on the stack for the entire test — it is never freed between cycles.
    ///
    /// - If this test SIGBUS-es → the SDK itself can't handle reconnect on macOS
    ///   (upstream bug).
    /// - If this test passes → the SIGBUS in cue-sdk-rust is caused by freeing the
    ///   callback context (`Pin<Box<Sender>>`) before the SDK's background thread
    ///   is done with it (context lifetime issue).
    #[test]
    fn reconnect_sigbus_diagnostic() {
        let shared = Mutex::new(CallbackState { states: Vec::new() });
        let context = &shared as *const Mutex<CallbackState> as *mut core::ffi::c_void;

        // ---- Session 1: connect, wait for timeout, disconnect ----

        eprintln!("[reconnect diag] session 1: connect");
        let err = unsafe { CorsairConnect(Some(on_state_changed), context) };
        assert_eq!(err, CorsairError_CE_Success, "session 1 connect: {err}");

        eprintln!("[reconnect diag] session 1: waiting for timeout");
        std::thread::sleep(std::time::Duration::from_millis(1500));
        {
            let guard = shared.lock().unwrap();
            eprintln!(
                "[reconnect diag] session 1: callback states = {:?}",
                guard.states
            );
        }

        eprintln!("[reconnect diag] session 1: disconnect");
        let err = unsafe { CorsairDisconnect() };
        assert_eq!(err, CorsairError_CE_Success, "session 1 disconnect: {err}");

        // Verify CSS_Closed was delivered synchronously per SDK docs.
        {
            let guard = shared.lock().unwrap();
            assert!(
                guard.states.contains(&CorsairSessionState_CSS_Closed),
                "Expected CSS_Closed after disconnect, got: {:?}",
                guard.states,
            );
            eprintln!(
                "[reconnect diag] session 1: final states = {:?}",
                guard.states
            );
        }

        // Clear state for session 2.
        shared.lock().unwrap().states.clear();

        // ---- Session 2: reconnect, wait for timeout, disconnect ----

        eprintln!("[reconnect diag] session 2: connect (reconnect)");
        let err = unsafe { CorsairConnect(Some(on_state_changed), context) };
        assert_eq!(err, CorsairError_CE_Success, "session 2 connect: {err}");

        eprintln!("[reconnect diag] session 2: details");
        let mut details = unsafe { std::mem::zeroed::<CorsairSessionDetails>() };
        let err = unsafe { CorsairGetSessionDetails(&mut details) };
        assert_eq!(err, CorsairError_CE_Success, "session 2 get details: {err}",);
        assert_eq!(details.clientVersion.major, 4);

        eprintln!("[reconnect diag] session 2: waiting for timeout");
        std::thread::sleep(std::time::Duration::from_millis(1500));
        {
            let guard = shared.lock().unwrap();
            eprintln!(
                "[reconnect diag] session 2: callback states = {:?}",
                guard.states
            );
        }

        eprintln!("[reconnect diag] session 2: disconnect");
        let err = unsafe { CorsairDisconnect() };
        assert_eq!(err, CorsairError_CE_Success, "session 2 disconnect: {err}");

        {
            let guard = shared.lock().unwrap();
            eprintln!(
                "[reconnect diag] session 2: final states = {:?}",
                guard.states
            );
        }

        eprintln!("[reconnect diag] all steps passed — context still alive on stack");
        // `shared` is dropped here at end of test. If the process crashes
        // after this point, the SDK's background thread is still accessing
        // the context during process teardown.
    }
}
