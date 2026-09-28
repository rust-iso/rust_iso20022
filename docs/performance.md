# Performance measurement policy

Performance claims require recorded measurements from comparable runners. The
benchmark harness covers pacs.008 parse, serialize, detect, and L2 validate;
camt.053 parse; and catalogue lookup. Every workload performs a correctness
smoke operation before its timed loop.

## Comparable runs

Compare results only when all of the following match:

- repository commit/tree state and fixture digests;
- Rust compiler and Cargo versions;
- target triple, OS version, CPU architecture/model, and logical CPU count;
- build profile, feature set, codegen units, debug information, and job count;
- benchmark iteration count and workload name;
- power/virtualization/container environment where known.

The harness reports elapsed nanoseconds and nanoseconds per iteration. These
are latency measurements, not memory measurements. Linux and macOS use
`getrusage(RUSAGE_SELF).ru_maxrss`, with the platform-specific units converted
to bytes and labelled `peak_rss_bytes`. The measurement script launches each
workload in a fresh process so one workload's high-water mark does not carry
into the next. Unsupported platforms report no memory value.

The allocation subprocess installs a counting wrapper over the system
allocator. It reports allocation/reallocation calls, cumulative requested
bytes, deallocated requested bytes, and remaining requested bytes. These are
allocator requests, not physical memory, RSS, retained-capacity analysis, or a
heap-profiler attribution. Each observation declares that `run_workload`
includes one internal warmup operation in addition to the requested iterations.

## Build-cost measurements

`scripts/measure-build-baseline.sh` uses new temporary target directories for
cold default and selected-feature builds. It never runs `cargo clean` against a
developer target directory. It also records normal dependency counts and CLI
and WASM artifact sizes when their required targets are available.

Compile time, dependency count, binary size, WASM size, latency, and memory are
distinct dimensions. A change is reviewed against the same dimension and a
comparable runner; improvement in one does not imply improvement in another.

## Regression review

This project does not enforce a universal percentage threshold from a single
developer machine. A material change should be rerun at least three times on
the same runner, with medians compared and noise/outliers retained in evidence.
Regressions require an implementation explanation or an explicit acceptance
decision. Optimizations are not merged solely from intuition or one warm-cache
sample.
