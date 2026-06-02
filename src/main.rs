fn main() {
    if let Err(error) = coding_flow_v0::run() {
        eprintln!("cflow error: {error}");
        std::process::exit(1);
    }
}
