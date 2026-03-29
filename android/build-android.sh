#!/bin/bash
# Build whyyoulying Android APK
# Requires: cargo-ndk, Android SDK (API 35), NDK
# Install: cargo install cargo-ndk
# Usage: cd android && ./build-android.sh

set -e
cd "$(dirname "$0")/.."

echo "Building Rust .so for Android targets..."
cargo ndk -t arm64-v8a -t x86_64 \
    -o android/app/src/main/jniLibs \
    build --release --lib

echo "Renaming .so to match System.loadLibrary name..."
for abi in arm64-v8a x86_64; do
    dir="android/app/src/main/jniLibs/$abi"
    if [ -f "$dir/libwhyyoulying.so" ]; then
        mv "$dir/libwhyyoulying.so" "$dir/libwhyyoulying_android.so"
    fi
done

echo "Building APK..."
cd android
./gradlew assembleRelease

echo "APK: android/app/build/outputs/apk/release/app-release-unsigned.apk"
ls -la app/build/outputs/apk/release/app-release-unsigned.apk 2>/dev/null || echo "(build with ./gradlew assembleRelease)"
