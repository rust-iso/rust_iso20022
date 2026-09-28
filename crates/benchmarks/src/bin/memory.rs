use rust_iso20022_benchmarks::{WORKLOADS, memory, run_workload};

fn main() {
    let iterations = std::env::var("ISO20022_BENCH_ITERATIONS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(100);
    let requested = std::env::args().nth(1);
    let workloads: Vec<&str> = match requested.as_deref() {
        Some(name) if WORKLOADS.contains(&name) => vec![
            WORKLOADS
                .iter()
                .copied()
                .find(|candidate| *candidate == name)
                .unwrap(),
        ],
        Some(_) => {
            eprintln!("unknown workload");
            std::process::exit(2);
        }
        None => WORKLOADS.to_vec(),
    };

    for workload in workloads {
        run_workload(workload, iterations).unwrap_or_else(|error| panic!("{workload}: {error}"));
        let observation = memory::observe();
        let bytes = observation
            .bytes
            .map_or_else(|| "null".to_owned(), |value| value.to_string());
        println!(
            "{{\"workload\":\"{workload}\",\"platform\":\"{}\",\"metric\":\"{}\",\"bytes\":{bytes},\"source\":\"{}\"}}",
            observation.platform, observation.metric, observation.source
        );
    }
}
