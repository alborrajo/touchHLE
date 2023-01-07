//! The Core Foundation framework.
//!
//! In Apple's implementation, this is a layer independent of, or below,
//! Foundation, and there is "Toll-Free Bridging" that lets some Foundation
//! types be used as if they were the corresponding Core Foundation types and
//! vice-versa. But in this implementation we will cheat and implement things
//! backwards (Core Foundation on top of Foundation) where we can get away with
//! it.
//!
//! Useful resources:
//! - Apple's [Core Foundation Design Concepts](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFDesignConcepts/CFDesignConcepts.html)
//! - Apple's [Memory Management Programming Guide for Core Foundation](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/CFMemoryMgmt.html)

pub mod cf_bundle;
pub mod cf_type;
pub mod cf_url;

pub use cf_type::{CFRelease, CFRetain, CFTypeRef};

pub type CFIndex = i32;