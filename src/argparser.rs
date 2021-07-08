use clap::{App, Arg, SubCommand};

pub const CONFIG_ARG: &str = "config";


pub mod account {
    pub const ACCOUNT_COMMAND: &str = "account-create";
    pub const PATH_ARG: &str = "path";
}

pub fn build() -> App<'static, 'static> {
    App::new("MoneyRaven")
        .version("1.0")
        .about("MoneyRaven is an accounting utility designed for personal finances")
        .arg(
            Arg::with_name(CONFIG_ARG)
                .short("c")
                .long("config")
                .help("Allows to specify a custom config file"),
        )
        .subcommand(
            SubCommand::with_name(account::ACCOUNT_COMMAND)
                .arg(Arg::with_name(account::PATH_ARG).help("Specifies the path to create the database")),
        )
}
