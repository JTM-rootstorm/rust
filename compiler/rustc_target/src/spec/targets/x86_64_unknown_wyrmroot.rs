// Native x86-64 target for Wyrmroot primordial userspace.
//
// Wyrmroot uses the SysV x86-64 ABI and static ELF executables. LLVM does not
// need OS-specific lowering for that restricted early-userspace model, so the
// generic `none-elf` LLVM triple is used only as the code-generation backend;
// the Rust target identity and all `target_os` cfgs remain Wyrmroot-specific.

use crate::spec::{
    Arch, Cc, LinkerFlavor, Lld, Os, PanicStrategy, RelocModel, RustcAbi, StackProbeType, Target,
    TargetMetadata, TargetOptions,
};

pub(crate) fn target() -> Target {
    Target {
        llvm_target: "x86_64-unknown-none-elf".into(),
        metadata: TargetMetadata {
            description: Some("x86_64 Wyrmroot static ELF".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 64,
        data_layout:
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128".into(),
        arch: Arch::X86_64,
        options: TargetOptions {
            os: Os::Wyrmroot,
            cpu: "x86-64".into(),
            rustc_abi: Some(RustcAbi::Softfloat),
            features:
                "-x87,-fxsr,-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-avx,-avx2,+soft-float"
                    .into(),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("rust-lld".into()),
            dynamic_linking: false,
            max_atomic_width: Some(64),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            eh_frame_header: false,
            stack_probes: StackProbeType::Inline,
            ..Default::default()
        },
    }
}
