use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct DoArgs {
    // The command to execute!
    pub command: String,

    // The message!
    pub message: String,
}
