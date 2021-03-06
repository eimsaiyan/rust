// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # The Rust Standard Library
//!
//! The Rust Standard Library provides the essential runtime
//! functionality for building portable Rust software.
//! It is linked to all Rust crates by default.
//!
//! ## Intrinsic types and operations
//!
//! The [`ptr`](ptr/index.html) and [`mem`](mem/index.html)
//! modules deal with unsafe pointers and memory manipulation.
//! [`marker`](marker/index.html) defines the special built-in traits,
//! and [`raw`](raw/index.html) the runtime representation of Rust types.
//! These are some of the lowest-level building blocks in Rust.
//!
//! ## Math on primitive types and math traits
//!
//! Although basic operations on primitive types are implemented
//! directly by the compiler, the standard library additionally
//! defines many common operations through traits defined in
//! mod [`num`](num/index.html).
//!
//! ## Pervasive types
//!
//! The [`option`](option/index.html) and [`result`](result/index.html)
//! modules define optional and error-handling types, `Option` and `Result`.
//! [`iter`](iter/index.html) defines Rust's iterator protocol
//! along with a wide variety of iterators.
//! [`Cell` and `RefCell`](cell/index.html) are for creating types that
//! manage their own mutability.
//!
//! ## Vectors, slices and strings
//!
//! The common container type, `Vec`, a growable vector backed by an array,
//! lives in the [`vec`](vec/index.html) module. Contiguous, unsized regions
//! of memory, `[T]`, commonly called "slices", and their borrowed versions,
//! `&[T]`, commonly called "borrowed slices", are built-in types for which the
//! [`slice`](slice/index.html) module defines many methods.
//!
//! `&str`, a UTF-8 string, is a built-in type, and the standard library
//! defines methods for it on a variety of traits in the
//! [`str`](str/index.html) module. Rust strings are immutable;
//! use the `String` type defined in [`string`](string/index.html)
//! for a mutable string builder.
//!
//! For converting to strings use the [`format!`](fmt/index.html)
//! macro, and for converting from strings use the
//! [`FromStr`](str/trait.FromStr.html) trait.
//!
//! ## Platform abstractions
//!
//! Besides basic data types, the standard library is largely concerned
//! with abstracting over differences in common platforms, most notably
//! Windows and Unix derivatives. The [`os`](os/index.html) module
//! provides a number of basic functions for interacting with the
//! operating environment, including program arguments, environment
//! variables, and directory navigation. The [`path`](path/index.html)
//! module encapsulates the platform-specific rules for dealing
//! with file paths.
//!
//! `std` also includes the [`ffi`](ffi/index.html) module for interoperating
//! with the C language.
//!
//! ## Concurrency, I/O, and the runtime
//!
//! The [`thread`](thread/index.html) module contains Rust's threading abstractions.
//! [`sync`](sync/index.html) contains further, primitive, shared memory types,
//! including [`atomic`](sync/atomic/index.html), and [`mpsc`](sync/mpsc/index.html),
//! which contains the channel types for message passing.
//!
//! Common types of I/O, including files, TCP, UDP, pipes, Unix domain sockets, and
//! process spawning, are defined in the [`io`](io/index.html) module.
//!
//! Rust's I/O and concurrency depends on a small runtime interface
//! that lives, along with its support code, in mod [`rt`](rt/index.html).
//! While a notable part of the standard library's architecture, this
//! module is not intended for public use.
//!
//! ## The Rust prelude and macros
//!
//! Finally, the [`prelude`](prelude/index.html) defines a
//! common set of traits, types, and functions that are made available
//! to all code by default. [`macros`](macros/index.html) contains
//! all the standard macros, such as `assert!`, `panic!`, `println!`,
//! and `format!`, also available to all Rust code.
// Do not remove on snapshot creation. Needed for bootstrap. (Issue #22364)
#![cfg_attr(stage0, feature(custom_attribute))]
#![crate_name = "std"]
#![stable(feature = "rust1", since = "1.0.0")]
#![staged_api]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![doc(html_logo_url = "http://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "http://www.rust-lang.org/favicon.ico",
       html_root_url = "http://doc.rust-lang.org/nightly/",
       html_playground_url = "http://play.rust-lang.org/")]
#![doc(test(no_crate_inject, attr(deny(warnings))))]
#![doc(test(attr(allow(dead_code, deprecated, unused_variables, unused_mut))))]

#![feature(alloc)]
#![feature(box_syntax)]
#![feature(collections)]
#![feature(core)]
#![feature(lang_items)]
#![feature(libc)]
#![feature(linkage, thread_local, asm)]
#![feature(optin_builtin_traits)]
#![feature(rand)]
#![feature(staged_api)]
#![feature(unboxed_closures)]
#![feature(unicode)]
#![feature(unsafe_destructor)]
#![feature(unsafe_no_drop_flag, filling_drop)]
#![feature(macro_reexport)]
#![feature(unique)]
#![feature(allow_internal_unstable)]
#![feature(str_char)]
#![feature(into_cow)]
#![feature(std_misc)]
#![feature(slice_patterns)]
#![feature(debug_builders)]
#![cfg_attr(test, feature(test, rustc_private, std_misc))]

// Don't link to std. We are std.
#![feature(no_std)]
#![no_std]

#![allow(trivial_casts)]
#![deny(missing_docs)]

#[cfg(test)] extern crate test;
#[cfg(test)] #[macro_use] extern crate log;

#[macro_use]
#[macro_reexport(assert, assert_eq, debug_assert, debug_assert_eq,
    unreachable, unimplemented, write, writeln)]
extern crate core;

#[macro_use]
#[macro_reexport(vec, format)]
extern crate collections as core_collections;

#[allow(deprecated)] extern crate rand as core_rand;
extern crate alloc;
extern crate unicode;
extern crate libc;

#[macro_use] #[no_link] extern crate rustc_bitflags;

// Make std testable by not duplicating lang items. See #2912
#[cfg(test)] extern crate std as realstd;
#[cfg(test)] pub use realstd::marker;
#[cfg(test)] pub use realstd::ops;
#[cfg(test)] pub use realstd::cmp;
#[cfg(test)] pub use realstd::boxed;


// NB: These reexports are in the order they should be listed in rustdoc

pub use core::any;
pub use core::cell;
pub use core::clone;
#[cfg(not(test))] pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::hash;
pub use core::intrinsics;
pub use core::iter;
#[cfg(not(test))] pub use core::marker;
pub use core::mem;
#[cfg(not(test))] pub use core::ops;
pub use core::ptr;
pub use core::raw;
pub use core::simd;
pub use core::result;
pub use core::option;
pub mod error;

#[cfg(not(test))] pub use alloc::boxed;
pub use alloc::rc;

pub use core_collections::borrow;
pub use core_collections::fmt;
pub use core_collections::slice;
pub use core_collections::str;
pub use core_collections::string;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core_collections::vec;

pub use unicode::char;

/* Exported macros */

#[macro_use]
mod macros;

mod rtdeps;

/* The Prelude. */

pub mod prelude;


/* Primitive types */

// NB: slice and str are primitive types too, but their module docs + primitive doc pages
// are inlined from the public re-exports of core_collections::{slice, str} above.

#[path = "num/float_macros.rs"]
#[macro_use]
mod float_macros;

#[path = "num/int_macros.rs"]
#[macro_use]
mod int_macros;

#[path = "num/uint_macros.rs"]
#[macro_use]
mod uint_macros;

#[path = "num/isize.rs"]  pub mod isize;
#[path = "num/i8.rs"]   pub mod i8;
#[path = "num/i16.rs"]  pub mod i16;
#[path = "num/i32.rs"]  pub mod i32;
#[path = "num/i64.rs"]  pub mod i64;

#[path = "num/usize.rs"] pub mod usize;
#[path = "num/u8.rs"]   pub mod u8;
#[path = "num/u16.rs"]  pub mod u16;
#[path = "num/u32.rs"]  pub mod u32;
#[path = "num/u64.rs"]  pub mod u64;

#[path = "num/f32.rs"]   pub mod f32;
#[path = "num/f64.rs"]   pub mod f64;

pub mod ascii;

pub mod thunk;

/* Common traits */

pub mod num;

/* Runtime and platform support */

#[macro_use]
pub mod thread;

pub mod collections;
pub mod dynamic_lib;
pub mod env;
pub mod ffi;
pub mod fs;
pub mod io;
pub mod net;
pub mod os;
pub mod path;
pub mod process;
pub mod sync;
pub mod time;

#[macro_use]
#[path = "sys/common/mod.rs"] mod sys_common;

#[cfg(unix)]
#[path = "sys/unix/mod.rs"] mod sys;
#[cfg(windows)]
#[path = "sys/windows/mod.rs"] mod sys;

pub mod rt;
mod panicking;
mod rand;

// Some external utilities of the standard library rely on randomness (aka
// rustc_back::TempDir and tests) and need a way to get at the OS rng we've got
// here. This module is not at all intended for stabilization as-is, however,
// but it may be stabilized long-term. As a result we're exposing a hidden,
// unstable module so we can get our build working.
#[doc(hidden)]
#[unstable(feature = "rand")]
pub mod __rand {
    pub use rand::{thread_rng, ThreadRng, Rng};
}

// Modules that exist purely to document + host impl docs for primitive types

mod array;
mod bool;
mod unit;
mod tuple;

// A curious inner-module that's not exported that contains the binding
// 'std' so that macro-expanded references to std::error and such
// can be resolved within libstd.
#[doc(hidden)]
mod std {
    pub use sync; // used for select!()
    pub use error; // used for try!()
    pub use fmt; // used for any formatting strings
    pub use option; // used for bitflags!{}
    pub use rt; // used for panic!()
    pub use vec; // used for vec![]
    pub use cell; // used for tls!
    pub use thread; // used for thread_local!
    pub use marker;  // used for tls!
    pub use ops; // used for bitflags!

    // The test runner calls ::std::env::args() but really wants realstd
    #[cfg(test)] pub use realstd::env as env;
    // The test runner requires std::slice::Vector, so re-export std::slice just for it.
    //
    // It is also used in vec![]
    pub use slice;

    pub use boxed; // used for vec![]
}
