fn main() {
    let status = match pretty_exec_lib::main() {
        Ok(status) => status,
        Err(error) => {
            eprintln!("ERROR: {}", error);
            1
        }
    };

    std::process::exit(status);
}
