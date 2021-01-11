#![cfg(target_os="android")]
#![allow(non_snake_case)]

use std::ffi::{CString, CStr};
use jni::{
    objects::{JObject, JString},
    sys::jstring,
    JNIEnv
};

mod audio;

// no mangle preserves the function name so that JNI/NDK can see it
#[no_mangle]
pub unsafe extern fn Java_dev_covercash_hearlib_NativeInterface_hello(env: JNIEnv, _: JObject, j_recipient: JString) -> jstring {
    let recipient = CString::from(
	CStr::from_ptr(
	    env.get_string(j_recipient).expect("unable to get j_recipient").as_ptr()
	)
    );

    let output = env.new_string(format!("Hello {}", recipient.to_str().expect("unable to get string from CString"))).expect("unable to make new string");
    output.into_inner()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
