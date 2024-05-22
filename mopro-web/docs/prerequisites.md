---
sidebar_position: 1
---

# Prerequisites

Depending on what platforms and adapters you use, there are several prerequisites to install before getting started.

-   General
    -   [Rust](https://www.rust-lang.org/learn/get-started)
-   Circom
    -   [circom](https://docs.circom.io/)
    -   [snarkjs](https://github.com/iden3/snarkjs)
-   iOS
    -   [Xcode](https://developer.apple.com/xcode/)
    -   [CocoaPods](https://cocoapods.org/)
-   Android
    -   [Android Studio](https://developer.android.com/studio)
    -   Also see configuration below

## Android configuration

Some additional configuration is required for Android.

First, install the latest SDK. In Android Studio, go to `SDK Manager > SDK Tools` and install `NDK (Side by Side)` (see [Android Developer site](https://developer.android.com/studio/projects/install-ndk#default-version)).

After that, set the following environment variables:

```sh
# Export `$ANDROID_HOME` and change `{USER_NAME}` to your username
export ANDROID_HOME="/Users/{USER_NAME}/Library/Android/sdk"

# Locate which NDK version you have
ls $ANDROID_HOME/ndk # => 26.1.10909125

# Set it to your `NDK_PATH` environment variable
NDK_PATH=$ANDROID_HOME/ndk/26.1.10909125
```

(Reference: [Running Rust on Android with UniFFI](https://sal.dev/android/intro-rust-android-uniffi/)).