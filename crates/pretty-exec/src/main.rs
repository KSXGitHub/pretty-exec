fn main() {
    let status = match pretty_exec_lib::main().and_then(|status| {
        status
            .code()
            .ok_or_else(|| "Failed to get status code".to_owned())
    }) {
        Ok(status) => status,
        Err(error) => {
            eprintln!("ERROR: {}", error);
            1
        }
    };

    std::process::exit(status);
}
