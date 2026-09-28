use rust_iso20022_benchmarks::{WORKLOADS, allocation::CountingAllocator, run_workload};

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator::new();

fn main() {
    let iterations = std::env::var("ISO20022_BENCH_ITERATIONS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(100);
    let Some(workload) = std::env::args().nth(1) else {
        eprintln!("usage: allocations <workload>");
        std::process::exit(2);
    };
    if !WORKLOADS.contains(&workload.as_str()) {
        eprintln!("unknown workload");
        std::process::exit(2);
    }

    run_workload(&workload, 1).unwrap_or_else(|error| panic!("{workload}: {error}"));
    ALLOCATOR.reset();
    run_workload(&workload, iterations).unwrap_or_else(|error| panic!("{workload}: {error}"));
    let snapshot = ALLOCATOR.snapshot();
    println!(
        "{{\"workload\":\"{workload}\",\"iterations\":{iterations},\"metric\":\"requested_allocator_bytes\",\"allocation_calls\":{},\"allocated_bytes\":{},\"deallocated_bytes\":{},\"live_requested_bytes\":{},\"includes_one_internal_warmup\":true}}",
        snapshot.calls,
        snapshot.allocated_bytes,
        snapshot.deallocated_bytes,
        snapshot.live_requested_bytes,
    );
}
