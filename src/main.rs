#[macro_use]
mod args;
mod git;

use args::DoArgs;
use clap::Parser;
use git::git_cli;

fn main() {
    let args = DoArgs::parse();
    git_cli(args);
}
