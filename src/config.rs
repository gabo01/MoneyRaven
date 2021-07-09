use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::fs;
use std::io;
use std::path::PathBuf;
use toml;

use crate::platform::{self, get_default_path};

pub fn build(path: Option<&str>) -> AppConfig {
    let config_path = match get_default_path(path, platform::CONFIG_PATH) {
        Ok(path) => path,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
    match AppConfig::from_file(config_path) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

pub struct AppConfig {
    filepath: PathBuf,
    config: Config,
}

impl AppConfig {
    fn from_file(filepath: PathBuf) -> Result<Self, BuildError> {
        let config = match fs::read_to_string(&filepath) {
            Ok(contents) => toml::from_str(&contents)?,
            Err(err) if err.kind() == io::ErrorKind::NotFound => Config::default(),
            Err(err) => {
                return Err(BuildError::IOError {
                    file: filepath.display().to_string(),
                    error: err,
                })
            }
        };
        Ok(Self { filepath, config })
    }

    pub fn save(&self) -> Result<(), SaveError> {
        Ok(fs::write(
            &self.filepath,
            toml::to_string_pretty(&self.config)?,
        )?)
    }

    pub fn set_db_path<P: Into<PathBuf>>(&mut self, db_path: P) {
        self.config.dbpath = Some(db_path.into())
    }
}

#[derive(Serialize, Deserialize, Default)]
struct Config {
    dbpath: Option<PathBuf>,
}

pub enum BuildError {
    IOError { file: String, error: io::Error },
    ParseError(toml::de::Error),
}

impl From<toml::de::Error> for BuildError {
    fn from(parse_error: toml::de::Error) -> Self {
        Self::ParseError(parse_error)
    }
}

impl Display for BuildError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let message = match self {
            Self::IOError { file, error: _ } => format!(
                "The application was unable to read the contents of {}",
                file
            ),
            Self::ParseError(_) => {
                format!("The application was unable to parse the contents of the config file")
            }
        };
        write!(fmt, "{}", message)
    }
}

pub enum SaveError {
    IOError(io::Error),
    ParseError(toml::ser::Error),
}

impl From<io::Error> for SaveError {
    fn from(io_error: io::Error) -> Self {
        Self::IOError(io_error)
    }
}

impl From<toml::ser::Error> for SaveError {
    fn from(parse_error: toml::ser::Error) -> Self {
        Self::ParseError(parse_error)
    }
}

impl Display for SaveError {
    fn fmt(&self, _fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        todo!()
    }
}
