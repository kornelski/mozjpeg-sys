#[allow(unstable)]
fn main() {
    if !std::old_io::Command::new("./src/build.sh")
        .stdout(::std::old_io::process::InheritFd(1))
        .stderr(::std::old_io::process::InheritFd(2))
        .status().unwrap().success() {
        panic!("Script failed");
    }
}
