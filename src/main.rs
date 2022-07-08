#[macro_use]
mod args;

use args::DoArgs;
use clap::Parser;
use std::process::Command;

fn main() {
   let args = DoArgs::parse();
   match args.command.as_str() {
      "push" | "Push" | "PUSH" => {
         let mut command = Command::new("git");
         command.args(["add", "."]);
         command.output().unwrap();

         let mut command2 = Command::new("git");
         command2.args(["commit", "-m", &args.message]);
         command2.output().unwrap();
         
         let mut command3 = Command::new("git");
         command3.args(["push"]);
         command3.output().unwrap();


      },
      _ => println!("unknown command"),
   }
}
