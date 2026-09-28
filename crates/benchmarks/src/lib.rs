//! Shared, dependency-free benchmark workloads and platform-qualified memory probes.

pub mod allocation;
pub mod memory;

use std::hint::black_box;
use std::time::{Duration, Instant};

pub const PACS008: &str =
    include_str!("../../../fixtures/iso/valid/pacs.008.001.08-cross-field.xml");
pub const CAMT053: &str = include_str!("../../../fixtures/migration/mt940/expected.xml");
pub const WORKLOADS: &[&str] = &[
    "pacs008_parse",
    "pacs008_serialize",
    "pacs008_detect",
    "pacs008_validate",
    "camt053_parse",
    "catalogue_lookup",
];

#[derive(Debug, Clone, Copy)]
pub struct Timing {
    pub iterations: u64,
    pub elapsed: Duration,
}

impl Timing {
    pub fn nanoseconds_per_iteration(self) -> f64 {
        self.elapsed.as_nanos() as f64 / self.iterations as f64
    }
}

/// Smoke-check and execute one named workload for a fixed iteration count.
pub fn run_workload(name: &str, iterations: u64) -> Result<Timing, String> {
    if iterations == 0 {
        return Err("iterations must be positive".to_owned());
    }
    let timing = match name {
        "pacs008_parse" => timed(iterations, || {
            black_box(
                rust_iso20022::parse(black_box(PACS008)).expect("validated pacs.008 fixture"),
            );
        }),
        "pacs008_serialize" => {
            let parsed = rust_iso20022::parse(PACS008).map_err(|error| error.to_string())?;
            parsed.to_xml().map_err(|error| error.to_string())?;
            timed(iterations, || {
                black_box(parsed.to_xml().expect("serialize verified pacs.008"));
            })
        }
        "pacs008_detect" => {
            rust_iso20022::detect_message(PACS008, rust_iso20022::ParseLimits::DEFAULT)
                .map_err(|error| error.to_string())?;
            timed(iterations, || {
                black_box(
                    rust_iso20022::detect_message(
                        black_box(PACS008),
                        rust_iso20022::ParseLimits::DEFAULT,
                    )
                    .expect("detect verified pacs.008"),
                );
            })
        }
        "pacs008_validate" => {
            let parsed = rust_iso20022::parse(PACS008).map_err(|error| error.to_string())?;
            rust_iso20022::validation::bindings::validate_parsed_message(&parsed)
                .map_err(|error| error.to_string())?;
            timed(iterations, || {
                black_box(
                    rust_iso20022::validation::bindings::validate_parsed_message(&parsed)
                        .expect("validate verified pacs.008"),
                );
            })
        }
        "camt053_parse" => timed(iterations, || {
            black_box(
                rust_iso20022::parse(black_box(CAMT053)).expect("validated camt.053 fixture"),
            );
        }),
        "catalogue_lookup" => {
            rust_iso20022::catalogue::lookup_descriptor("pacs.008.001.08")
                .ok_or_else(|| "catalogue fixture missing".to_owned())?;
            timed(iterations, || {
                black_box(
                    rust_iso20022::catalogue::lookup_descriptor(black_box("pacs.008.001.08"))
                        .expect("catalogue fixture"),
                );
            })
        }
        _ => return Err(format!("unknown workload: {name}")),
    };
    Ok(timing)
}

fn timed(mut iterations: u64, mut operation: impl FnMut()) -> Timing {
    operation();
    let requested = iterations;
    let started = Instant::now();
    while iterations > 0 {
        operation();
        iterations -= 1;
    }
    Timing {
        iterations: requested,
        elapsed: started.elapsed(),
    }
}
