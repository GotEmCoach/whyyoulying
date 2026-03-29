//! JNI bridge for Android. Exposes demo report + SBOM to Java/WebView.

#[cfg(target_os = "android")]
mod jni_impl {
    use std::ffi::CString;
    use std::os::raw::c_char;

    // Minimal JNI types — no external crate needed
    #[repr(C)]
    pub struct JNINativeInterface {
        _reserved: [*const (); 167],
        new_string_utf: unsafe extern "C" fn(*mut *const JNINativeInterface, *const c_char) -> *mut (),
    }

    type JNIEnv = *mut *const JNINativeInterface;
    type JClass = *mut ();
    type JString = *mut ();

    unsafe fn make_jstring(env: JNIEnv, s: &str) -> JString {
        let cs = CString::new(s).unwrap_or_default();
        ((**env).new_string_utf)(env, cs.as_ptr())
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_org_cochranblock_whyyoulying_MainActivity_runDemoHtml(
        env: JNIEnv, _class: JClass,
    ) -> JString {
        let (ds, alerts) = crate::demo::run_demo();
        let html = crate::demo::format_html(&ds, &alerts);
        make_jstring(env, &html)
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_org_cochranblock_whyyoulying_MainActivity_runDemoJson(
        env: JNIEnv, _class: JClass,
    ) -> JString {
        let (_, alerts) = crate::demo::run_demo();
        let json = serde_json::to_string_pretty(&alerts).unwrap_or_default();
        make_jstring(env, &json)
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_org_cochranblock_whyyoulying_MainActivity_getSbom(
        env: JNIEnv, _class: JClass,
    ) -> JString {
        let sbom = format!(
            "SPDXVersion: SPDX-2.3\nPackageName: whyyoulying\nPackageVersion: {}\nPackageLicenseConcluded: Unlicense\n",
            env!("CARGO_PKG_VERSION")
        );
        make_jstring(env, &sbom)
    }
}
