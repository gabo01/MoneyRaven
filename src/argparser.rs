use clap::{App, AppSettings, Arg, SubCommand};

pub const CONFIG_ARG: &str = "config";

pub const DB_CREATE_COMMAND: &str = "create-db";
pub const DB_DELETE_COMMAND: &str = "delete-db";
pub const PATH_ARG: &str = "path";

pub fn build() -> App<'static, 'static> {
    App::new("MoneyRaven")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version("1.0")
        .about("MoneyRaven is an accounting utility designed for personal finances")
        .arg(
            Arg::with_name(CONFIG_ARG)
                .short("c")
                .long("config")
                .help("Allows to specify a custom config file"),
        )
        .subcommand(
            SubCommand::with_name(DB_CREATE_COMMAND)
                .arg(Arg::with_name(PATH_ARG).help("Specifies the path to create the database")),
        )
        .subcommand(SubCommand::with_name(DB_DELETE_COMMAND))
}
