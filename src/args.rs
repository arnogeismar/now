use clap:: {
  Args,
  Parser,
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct DoArgs {
  // The command to execute!
  pub command: String,
  // The subcommand to execute!
  pub message: String,
}