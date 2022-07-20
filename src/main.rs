use std::process::Command;


fn main() {
    // let args: Vec<_> = env::args().collect();
    let day = "01b";
    let cmd = Command::new("cargo")
        .args(&["run", "--bin", day])
        .output()
        .unwrap();
    let output = String::from_utf8(cmd.stdout).unwrap();
    println!("Day {}:\n{}", day, output);
}
