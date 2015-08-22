fn main() {
    let num_jobs = std::env::var("NUM_JOBS").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let mut cmd = std::process::Command::new("make");
    cmd.arg("-j").arg(num_jobs);

    if !cmd.status().unwrap().success() {
        panic!("Script failed");
    }

    println!("cargo:rustc-flags=-L {}", out_dir);
}
