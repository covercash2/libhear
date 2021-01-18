#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use log::*;

use jni::{
    objects::{JClass, JString},
    sys::jstring,
    JNIEnv,
};

//mod audio;
//mod mycpal;
mod myoboe;

#[ndk_glue::main(backtrace = "on", logger(level = "debug", tag = "libhear"))]
fn main() {
    info!("run main");
    //mycpal::init().expect("unable to run audio")
    myoboe::create_playback_stream()
}

// no mangle preserves the function name so that JNI/NDK can see it
#[no_mangle]
pub extern "C" fn Java_dev_covercash_hearlib_Hear_hello(
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
// pub extern "C" fn __cxa_pure_virtual() {
//     error!("unexpected C++ runtime error");
//     loop {}
// }

// // GNU C++ personality routine, Version 0.                                      
// #[no_mangle]
// extern "C" fn  __gxx_personality_v0() -> usize {
//     return 127;
// }

// #[no_mangle]
// pub extern "system" fn Java_dev_covercash_hearlib_Hear_playTest(
//     _: JNIEnv,
//     _: JClass,
// ) -> jstring {
//     audio::create_playback_stream();

//     todo!()
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
