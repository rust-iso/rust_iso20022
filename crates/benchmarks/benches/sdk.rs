use rust_iso20022_benchmarks::{WORKLOADS, memory, run_workload};

fn main() {
    let iterations = std::env::var("ISO20022_BENCH_ITERATIONS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(1_000);

    for workload in WORKLOADS {
        let timing = run_workload(workload, iterations)
            .unwrap_or_else(|error| panic!("benchmark smoke failed for {workload}: {error}"));
        let observation = memory::observe();
        let bytes = observation
            .bytes
            .map_or_else(|| "null".to_owned(), |value| value.to_string());
        println!(
            "{{\"workload\":\"{workload}\",\"iterations\":{},\"elapsed_ns\":{},\"ns_per_iteration\":{:.3},\"memory_metric\":\"{}\",\"memory_bytes\":{bytes},\"memory_source\":\"{}\"}}",
            timing.iterations,
            timing.elapsed.as_nanos(),
            timing.nanoseconds_per_iteration(),
            observation.metric,
            observation.source,
        );
    }
}
