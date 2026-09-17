//! Linux-specific definitions.

#![stable(feature = "raw_ext", since = "1.1.0")]
#![doc(cfg(target_os = "linux"))]

pub mod fs;
// `net` (AF_VSOCK) and `process` (pidfd, CommandExt::create_pidfd/pidfd) are genuinely
// Linux-kernel-specific features OxideBSD doesn't have -- unlike `fs`/`raw` (MetadataExt, raw
// type aliases), which are just musl-ABI-shaped struct/field definitions we do have real
// equivalents for. Scoped out for oxidebsd rather than stubbed/implemented, matching this fork's
// "thin unix-target addition" scope -- revisit if oxbsduserland ever needs either.
#[cfg(target_os = "linux")]
pub mod net;
#[cfg(target_os = "linux")]
pub mod process;
pub mod raw;
