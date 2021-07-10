use clap::ArgMatches;
use std::path::PathBuf;

use crate::argparser;
use crate::config::AppConfig;
use crate::platform::{self, resolve_path};

pub enum Commands {
    CreateAccount(PathBuf),
}

impl Commands {
    pub fn run(self, config: &mut AppConfig) {
        match self {
            Commands::CreateAccount(path) => {
                let mut db = match ravenlib::Database::open_or_create(&path) {
                    Ok(db) => db,
                    Err(_err) => {
                        eprintln!("Unable to create the database");
                        return;
                    }
                };
                config.set_db_path(path);
                if let Err(err) = config.save() {
                    eprintln!(
                        "Unable to save the new configuration file. The error found was {}",
                        err
                    );
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
                let db_path = resolve_path(input_val, platform::DB_PATH);
                Commands::CreateAccount(db_path)
            }
            (_, _) => unreachable!(),
        }
    }
}
