pub fn logger(msg: String) {
    use std::fs::OpenOptions;
    use std::io::Write;

    // Open the file in write mode (create if not exists).
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("runtime.log")
        .unwrap();

    writeln!(file, "{:?}", msg).unwrap();
}
