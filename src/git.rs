use crate::args::DoArgs;
use std::process::Command;

pub fn git_cli(args: DoArgs) {
    match args.command.as_str() {
        "pall" | "PAll" | "PALL" => {
            let mut command = Command::new("git");
            command.args(["add", "."]);
            command.output().unwrap();

            let mut command2 = Command::new("git");
            command2.args(["commit", "-m", &args.message]);
            command2.output().unwrap();

            let mut command3 = Command::new("git");
            command3.args(["push"]);
            command3.output().unwrap();

            println!("Added everything");
            println!("Committed with message: {}", args.message);
            println!("Pushed to remote");
        }
        "pull" | "Pull" | "PULL" => {
            let mut command4 = Command::new("git");
            command4.args(["pull"]);
            command4.output().unwrap();
            println!("Pulled from remote");
        }
        "status" | "Status" | "STATUS" => {
            let mut command5 = Command::new("git");
            command5.args(["status"]);
            println!(
                "{:?}",
                format!("{:?}", String::from_utf8(command5.output().unwrap().stdout))
            );
        }
        "log" | "Log" | "LOG" => {
            let mut command6 = Command::new("git");
            command6.args(["log"]);
            command6.output().unwrap();
            println!("Log");
        }
        "clone" | "Clone" | "CLONE" => {
            let mut command7 = Command::new("git");
            command7.args(["clone", &args.message]);
            command7.output().unwrap();
            println!("Cloned");
        }
        "init" | "Init" | "INIT" => {
            let mut command8 = Command::new("git");
            command8.args(["init"]);
            command8.output().unwrap();
            println!("Initialized repo with GIT");
        }
        "add" | "Add" | "ADD" => {
            let mut command9 = Command::new("git");
            command9.args(["add", "."]);
            command9.output().unwrap();
            println!("Added everything");
        }
        "commit" | "Commit" | "COMMIT" => {
            let mut command10 = Command::new("git");
            command10.args(["commit", "-m", &args.message]);
            command10.output().unwrap();
            println!("Committed with message: {}", args.message);
        }
        "push" | "Push" | "PUSH" => {
            let mut command11 = Command::new("git");
            command11.args(["push"]);
            command11.output().unwrap();
            println!("Pushed to remote");
        }
        "branch" | "Branch" | "BRANCH" => {
            let mut command12 = Command::new("git");
            command12.args(["checkout", "-b", &args.message]);
            command12.output().unwrap();
            println!("Checked out branch: {}", args.message);
        }

        _ => println!("unknown command"),
    }
}
