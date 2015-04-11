
fn main() {
    if !std::process::Command::new("./src/build.sh")
        .status().unwrap().success() {
        panic!("Script failed");
    }
}
