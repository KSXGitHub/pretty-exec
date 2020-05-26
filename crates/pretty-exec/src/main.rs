fn main() {
    let status = match pretty_exec::main() {
        Ok(status) => status,
        Err(error) => {
            eprintln!("ERROR: {}", error);
            1
        }
    };

    std::process::exit(status);
}
