use std::env;
use std::fmt::{self, Display};
use std::path::{Path, PathBuf};

pub const CONFIG_PATH: &str = "MoneyRaven/config.toml";
pub const DB_PATH: &str = "MoneyRaven/db";

pub fn resolve_path<P: AsRef<Path>>(user_value: Option<&str>, default_relative_path: P) -> PathBuf {
    user_value.map(PathBuf::from).unwrap_or({
        match get_default_path(default_relative_path) {
            Ok(path) => path,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }
    })
}

pub fn default_data_path() -> Result<String, FetchError> {
    match env::consts::OS {
        "windows" => match env::var("APPDATA") {
            Ok(val) => Ok(val),
            Err(e) => Err(FetchError::EnvError {
                key: "APPDATA".to_owned(),
                error: e,
            }),
        },
        _ => Err(FetchError::UnsuportedOS),
    }
}

fn get_default_path<P: AsRef<Path>>(default_relative_path: P) -> Result<PathBuf, FetchError> {
    Ok(PathBuf::from(default_data_path()?).join(default_relative_path))
}

pub enum FetchError {
    UnsuportedOS,
    EnvError { key: String, error: env::VarError },
}

impl Display for FetchError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let message = match self {
            FetchError::UnsuportedOS => {
                "The current operating system is not supported by the application".to_owned()
            }
            FetchError::EnvError { key, error } => format!(
                "The application was unable to fetch the value of {} because of {}",
                key, error
            ),
        };
        write!(fmt, "{}", message)
    }
}
