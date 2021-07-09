use clap::ArgMatches;
use std::path::PathBuf;

use crate::argparser;
use crate::platform::{self, get_default_path};
use crate::config::AppConfig;

pub enum Commands {
    CreateAccount(PathBuf),
}

impl Commands {
    pub fn run(self, config: &mut AppConfig) {
        match self {
            Commands::CreateAccount(path) => {
                //TODO: Create the database
                config.set_db_path(path);
                if let Err(_err) = config.save() {
                    eprintln!("Unable to save the new configuration file");
                }
            }
        }
    }
}

impl From<(&str, Option<&ArgMatches<'_>>)> for Commands {
    fn from(tuple: (&str, Option<&ArgMatches<'_>>)) -> Self {
        let command_string = tuple.0;
        let options = tuple.1;
        match (command_string, options) {
            (argparser::account::ACCOUNT_COMMAND, Some(opts)) => {
                let input_val = opts.value_of(argparser::account::PATH_ARG);
                let db_path = match get_default_path(input_val, platform::DB_PATH) {
                    Ok(path) => path,
                    Err(err) => {
                        eprintln!("{}", err);
                        std::process::exit(1);
                    }
                };
                Commands::CreateAccount(db_path)
            }
            (_, _) => unreachable!(),
        }
    }
}
