#!/bin/sh

set -e

#JNI_LIBS=~/code/github/covercash2/theremin/HearLib/libs/
JNI_LIBS=/home/chrash/code/github/covercash2/theremin/HearLib/build/rustJniLibs/android

cd ~/code/github/covercash2/libhear/

cargo ndk \
      --target arm64-v8a \
      --target armeabi-v7a \
      --target x86 \
      --target x86_64 \
      --output-dir $JNI_LIBS \
      build --release
