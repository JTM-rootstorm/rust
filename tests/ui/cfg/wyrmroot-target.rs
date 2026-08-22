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
