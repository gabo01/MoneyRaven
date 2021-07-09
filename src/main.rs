mod argparser;
mod config;
mod platform;
mod commands;

use crate::commands::Commands;

fn main() {
    let options = argparser::build().get_matches();
    let mut config = config::build(options.value_of(argparser::CONFIG_ARG));
    let command: Commands = options.subcommand().into();
    command.run(&mut config);
}
