#[allow(unstable)]
fn main() {
    if !std::io::Command::new("./src/build.sh")
        .stdout(::std::io::process::InheritFd(1))
        .stderr(::std::io::process::InheritFd(2))
        .status().unwrap().success() {
        panic!("Script failed");
    }
}
