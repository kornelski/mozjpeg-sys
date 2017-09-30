fn main() {
    let num_jobs = std::env::var("NUM_JOBS").expect("NUM_JOBS");
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");

    let mut cmd = std::process::Command::new("make");
    cmd.arg("-j").arg(num_jobs);

    match cmd.status() {
        Ok(_) => {},
        Err(e) => {
            panic!("Failed to launch make: {}", e);
        },
    }
    println!("cargo:rustc-flags=-L {}", out_dir);
}
