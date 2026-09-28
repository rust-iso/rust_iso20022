//! Platform-qualified process memory observations.
//!
//! Linux and macOS expose process high-water RSS through `getrusage`; Linux
//! reports KiB while macOS reports bytes, so conversion is platform-specific.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryObservation {
    pub platform: &'static str,
    pub metric: &'static str,
    pub bytes: Option<u64>,
    pub source: &'static str,
}

pub fn observe() -> MemoryObservation {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let mut usage = std::mem::MaybeUninit::<libc::rusage>::zeroed();
        // SAFETY: `usage` points to writable storage for exactly one
        // `libc::rusage`; `RUSAGE_SELF` requires no additional lifetime.
        let status = unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) };
        let bytes = (status == 0).then(|| {
            // SAFETY: getrusage initialized the structure when it returned 0.
            let maximum = unsafe { usage.assume_init() }.ru_maxrss as u64;
            #[cfg(target_os = "linux")]
            {
                maximum.saturating_mul(1024)
            }
            #[cfg(target_os = "macos")]
            {
                maximum
            }
        });
        return MemoryObservation {
            platform: std::env::consts::OS,
            metric: "peak_rss_bytes",
            bytes,
            source: "getrusage(RUSAGE_SELF).ru_maxrss",
        };
    }

    #[allow(unreachable_code)]
    MemoryObservation {
        platform: std::env::consts::OS,
        metric: "unsupported",
        bytes: None,
        source: "no platform memory probe",
    }
}
