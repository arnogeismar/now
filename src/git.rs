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
            let mut command = Command::new("git");
            command.args(["pull"]);
            command.output().unwrap();
            println!("Pulled from remote");
        }
        "status" | "Status" | "STATUS" => {
            let mut command = Command::new("git");
            command.args(["status"]);
            command.output().unwrap();
            println!("Status");
        }
        "log" | "Log" | "LOG" => {
            let mut command = Command::new("git");
            command.args(["log"]);
            command.output().unwrap();
            println!("Log");
        }
        "clone" | "Clone" | "CLONE" => {
            let mut command = Command::new("git");
            command.args(["clone", &args.message]);
            command.output().unwrap();
            println!("Cloned");
        }
        "init" | "Init" | "INIT" => {
            let mut command = Command::new("git");
            command.args(["init"]);
            command.output().unwrap();
            println!("Initialized repo with GIT");
        }
        "add" | "Add" | "ADD" => {
            let mut command = Command::new("git");
            command.args(["add", "."]);
            command.output().unwrap();
            println!("Added everything");
        }
        "commit" | "Commit" | "COMMIT" => {
            let mut command = Command::new("git");
            command.args(["commit", "-m", &args.message]);
            command.output().unwrap();
            println!("Committed with message: {}", args.message);
        }
        "push" | "Push" | "PUSH" => {
            let mut command = Command::new("git");
            command.args(["push"]);
            command.output().unwrap();
            println!("Pushed to remote");
        }
        "branch" | "Branch" | "BRANCH" => {
            let mut command = Command::new("git");
            command.args(["checkout", "-b", &args.message]);
            command.output().unwrap();
            println!("Checked out branch: {}", args.message);
        }

        _ => println!("unknown command"),
    }
}
