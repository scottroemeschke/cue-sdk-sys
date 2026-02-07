use std::env;

const WINDOWS_LIB_PATH_VAR: &str = "CUE_SDK_LIB_FILES_PATH";
const MAC_OS_FRAMEWORK_PATH: &str = "CUE_SDK_FRAMEWORK_PATH";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed={}", WINDOWS_LIB_PATH_VAR);
    println!("cargo:rerun-if-env-changed={}", MAC_OS_FRAMEWORK_PATH);

    if cfg!(target_os = "windows") {
        let libs_path = env::var(WINDOWS_LIB_PATH_VAR).unwrap_or_else(|_| {
            panic!(
                "Environment variable {} is not set. Set it to the iCUE SDK lib directory (containing iCUESDK.x64_2019.lib).",
                WINDOWS_LIB_PATH_VAR
            )
        });

        println!("cargo:rustc-link-search={}", libs_path);
        println!("cargo:rustc-link-lib=iCUESDK.x64_2019");
    } else if cfg!(target_os = "macos") {
        let framework_path = env::var(MAC_OS_FRAMEWORK_PATH).unwrap_or_else(|_| {
            panic!(
                "Environment variable {} is not set. Set it to the directory containing iCUESDK.framework.",
                MAC_OS_FRAMEWORK_PATH
            )
        });

        println!("cargo:rustc-link-search=framework={}/", framework_path);
        println!("cargo:rustc-link-lib=framework=iCUESDK");
    }
}
