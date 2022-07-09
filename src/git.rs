use crate::args::DoArgs;
use std::process::Command;

pub fn git_cli(args: DoArgs) {
    match args.command.as_str() {
        "pall" | "Pall" | "PALL" => {
            Command::new("git").args(["add", "."]).output().unwrap();

            Command::new("git")
                .args(["commit", "-m", &args.message])
                .output()
                .unwrap();

            Command::new("git").args(["push"]).output().unwrap();

            println!("Added everything");
            println!("Committed with message: {}", args.message);
            println!("Pushed to remote");
        }
        "pec" | "Pec" | "PEC" => {
            Command::new("git").arg("pull").output().unwrap();

            let output = Command::new("git")
                .args(["branch", "-a"])
                .output()
                .unwrap()
                .stdout;

            let branches = String::from_utf8_lossy(&output);
            for branch in branches.split("\n") {
                if branch.to_lowercase().contains(&args.message.to_lowercase()) {
                    let mut checkout = Command::new("git");
                    println!("Checking out branch: {}", branch);
                    checkout.args(["checkout", branch]);
                    checkout.output().unwrap();
                    return;
                }
            }
        }

        _ => println!("unknown command"),
    }
}
