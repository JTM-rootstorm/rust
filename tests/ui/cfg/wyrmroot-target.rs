//@ add-minicore
//@ check-pass
//@ compile-flags: --target x86_64-unknown-wyrmroot
//@ needs-llvm-components: x86
//@ ignore-backends: gcc

#![feature(no_core)]
#![crate_type = "rlib"]
#![no_core]

extern crate minicore;

#[cfg(not(target_os = "wyrmroot"))]
compile_error!("the Wyrmroot target must report target_os=wyrmroot");

#[cfg(unix)]
compile_error!("the Wyrmroot target must not claim cfg(unix)");

#[cfg(target_family = "unix")]
compile_error!("the Wyrmroot target must not claim target_family=unix");

// The SysV x86-64 psABI baseline: x87, FXSAVE, SSE and SSE2, nothing wider.
#[cfg(not(all(
    target_feature = "x87",
    target_feature = "fxsr",
    target_feature = "sse",
    target_feature = "sse2"
)))]
compile_error!("the Wyrmroot target must advertise the x86-64 baseline features");

#[cfg(any(target_feature = "sse3", target_feature = "avx"))]
compile_error!("the Wyrmroot target must not go beyond or below the x86-64 baseline");
