use clap::ArgMatches;
use std::path::PathBuf;

use crate::argparser;
use crate::platform::{self, get_default_path};

pub enum Commands {
    CreateAccount(PathBuf),
}

impl Commands {
    pub fn from_cli(command_string: &str, options: Option<&ArgMatches>) -> Commands {
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

impl From<(&str, Option<&ArgMatches<'_>>)> for Commands {
    fn from(tuple: (&str, Option<&ArgMatches<'_>>)) -> Self {
        Self::from_cli(tuple.0, tuple.1)
    }
}
