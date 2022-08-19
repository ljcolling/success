use std::process::{Command, Stdio};


fn main() {

    let res = Command::new("sh")
        // .arg("-c")
        .args(std::env::args().skip(1))
        .stdout(Stdio::inherit()).output().unwrap();

    let status =  res.status.code();  
    
    match status {
        Some(1) => println!("👎"),
        Some(0) => println!("👍"),
        Some(v) => println!("{}", v),
        _ => println!("🚨")
    }

}
