//! JNI bridge for Android. Exposes demo report + SBOM to Java/WebView.
//! Build: cargo ndk -t arm64-v8a -t x86_64 -o android/app/src/main/jniLibs build --release --lib

#[cfg(target_os = "android")]
mod jni_impl {
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_void};

    // JNI types (minimal, no jni crate dependency)
    type JNIEnv = *mut c_void;
    type JClass = *mut c_void;
    type JString = *mut c_void;

    extern "C" {
        fn NewStringUTF(env: JNIEnv, s: *const c_char) -> JString;
    }

    fn to_jstring(env: JNIEnv, s: &str) -> JString {
        let cs = CString::new(s).unwrap_or_default();
        unsafe {
            let new_string_utf = *(*(env as *const *const [*const c_void; 800])).get_unchecked(167)
                as unsafe extern "C" fn(JNIEnv, *const c_char) -> JString;
            new_string_utf(env, cs.as_ptr())
        }
    }

    #[no_mangle]
    pub extern "C" fn Java_org_cochranblock_whyyoulying_MainActivity_runDemoHtml(
        env: JNIEnv, _class: JClass,
    ) -> JString {
        let (ds, alerts) = crate::demo::run_demo();
        let html = crate::demo::format_html(&ds, &alerts);
        to_jstring(env, &html)
    }

    #[no_mangle]
    pub extern "C" fn Java_org_cochranblock_whyyoulying_MainActivity_runDemoJson(
        env: JNIEnv, _class: JClass,
    ) -> JString {
        let (_, alerts) = crate::demo::run_demo();
        let json = serde_json::to_string_pretty(&alerts).unwrap_or_default();
        to_jstring(env, &json)
    }

    #[no_mangle]
    pub extern "C" fn Java_org_cochranblock_whyyoulying_MainActivity_getSbom(
        env: JNIEnv, _class: JClass,
    ) -> JString {
        let sbom = format!(
            "SPDXVersion: SPDX-2.3\nPackageName: whyyoulying\nPackageVersion: {}\nPackageLicenseConcluded: Unlicense\n",
            env!("CARGO_PKG_VERSION")
        );
        to_jstring(env, &sbom)
    }
}
