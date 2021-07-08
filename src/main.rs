use clap::ArgMatches;

mod argparser;
mod config;
mod platform;
mod commands;

use crate::config::AppConfig;
use crate::commands::Commands;

fn main() {
    let options = argparser::build().get_matches();
    let config = config::build(options.value_of(argparser::CONFIG_ARG));
    run_command(options, config);
}

fn run_command(options: ArgMatches, mut config: AppConfig) {
    let command: Commands = options.subcommand().into();
    match command {
        Commands::CreateAccount(path) => {
            //TODO: Create the database
            config.set_db_path(path);
            if let Err(_err) = config.save() {
                eprintln!("Unable to save the new configuration file");
            }
        }
    }
}
