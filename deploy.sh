#!/bin/sh

set -e

JNI_LIBS=~/code/github/covercash2/theremin/HearLib/libs/

cd ~/code/github/covercash2/libhear/

cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release

rm -rf $JNI_LIBS
mkdir $JNI_LIBS
mkdir $JNI_LIBS/arm64-v8a
mkdir $JNI_LIBS/armeabi-v7a
mkdir $JNI_LIBS/x86

cp target/aarch64-linux-android/release/libhear.so $JNI_LIBS/arm64-v8a/libhear.so
cp target/armv7-linux-androideabi/release/libhear.so $JNI_LIBS/armeabi-v7a/libhear.so
cp target/i686-linux-android/release/libhear.so $JNI_LIBS/x86/libhear.so
