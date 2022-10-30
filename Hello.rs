
use std::process::Command;
use std::str;

const TMP:&str = "/tmp/";

fn pack_config()
{
   let output = {
      Command::new("tar")
               .arg("-czf")
               .arg(TMP)
               .output()
               .expect("Failed to pack config")
   };

   println!("Pack output: {}", str::from_utf8(&output.stdout).unwrap())
}

fn main()
{
   println!("Rust says Hello to TutorialsPoint !!");

   let output = {
      Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
   };

   let hello = str::from_utf8(&output.stdout).unwrap();

   println!("Output: {} {}", hello, TMP);
}
