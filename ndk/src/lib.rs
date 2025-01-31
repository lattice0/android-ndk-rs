//! # Android NDK
//!
//! Bindings to the Android NDK.
//!
//! Currently has bindings:
//!  * `InputEvent`, `KeyEvent`, and `MotionEvent`, in the `event` module
//!  * `Looper`, in the `looper` module
//!  * `InputQueue`, in the `input_queue` module
//!  * `AssetManager`, `AssetDir`, and `Asset`, in the `asset` module
//!  * `NativeActivity`, in the `native_activity` module
//!  * `Configuration`, in the `configuration` module
#![cfg_attr(
    feature = "native_app_glue",
    doc = "  * `native_app_glue`'s `AndroidApp`, in the `android_app` module"
)]
#![warn(missing_debug_implementations, trivial_casts)]

pub mod asset;
pub mod audio;
pub mod bitmap;
pub mod configuration;
pub mod event;
pub mod hardware_buffer;
pub mod hardware_buffer_format;
pub mod input_queue;
pub mod looper;
pub mod media;
pub mod native_activity;
pub mod native_window;
pub mod posix;
pub mod surface_texture;
pub mod trace;
