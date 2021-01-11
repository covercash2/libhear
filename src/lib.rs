#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use jni::{
    objects::{JClass, JObject, JString},
    sys::jstring,
    JNIEnv,
};

mod audio;

// no mangle preserves the function name so that JNI/NDK can see it
#[no_mangle]
pub unsafe extern "C" fn Java_dev_covercash_hearlib_Hear_hello(
    env: JNIEnv,
    _: JClass,
    input: JString,
) -> jstring {
    let input: String = env
        .get_string(input)
        .expect("unable to get Java string")
        .into();

    let output = env
        .new_string(format!("Hello {}", input))
        .expect("unable to make new string");
    output.into_inner()
}

// #[no_mangle]
// pub unsafe extern fn Java_dev_covercash_hearlib_Hear_playTest(_: JNIEnv, _: JObject) {
//     audio::create_playback_stream()
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
