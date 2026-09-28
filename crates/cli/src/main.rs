fn main() {
    let code = rust_iso20022_cli::runner::run(std::env::args().skip(1));
    if code != rust_iso20022_cli::ExitCode::Success {
        std::process::exit(code as i32);
    }
}
