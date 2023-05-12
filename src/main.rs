mod cli;
mod commands;
mod constants;
mod helper;
mod registry;

fn main() {
  let cli = cli::parse();
  cli.subcmd.call();
}
