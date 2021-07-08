use std::env;
use std::fmt::{self, Display};
use std::path::PathBuf;

pub const CONFIG_PATH: &str = "MoneyRaven/config.toml";
pub const DB_PATH: &str = "MoneyRaven/account.db";

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

pub fn get_default_path(path: Option<&str>, local: &str) -> Result<PathBuf, FetchError> {
    Ok(path
        .map(|path| PathBuf::from(path))
        .unwrap_or(PathBuf::from(default_data_path()?).join(local)))
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
