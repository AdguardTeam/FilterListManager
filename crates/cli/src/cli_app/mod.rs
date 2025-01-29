mod fill_database;
mod test;

use crate::cli_app::test::test;
use adguard_flm::FilterListType;
use clap::builder::PossibleValuesParser;
use clap::{arg, value_parser, Arg, ArgAction, Command};
use std::path::PathBuf;

const FILL_STANDARD_DATABASE: &str = "standard";
const FILL_DNS_DATABASE: &str = "dns";

/// Setup CLI commands
fn cli_setup() -> Command {
    Command::new("agfl")
        .about("Adguard filter lists cli utility")
        .subcommand_required(true)
        .subcommand(
            Command::new("migrate")
                .about("Run migrations from specified folder")
                .arg(arg!(<DB_PATH> "Database path").value_parser(value_parser!(PathBuf)))
                .arg(
                    arg!(<MIGRATIONS_PATH> "Migrations folder path")
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("fill_database")
                .about("Fill database with filters indexes")
                .args([
                    Arg::new("DB_FOLDER_PATH")
                        .long("database")
                        .short('d')
                        .action(ArgAction::Set)
                        .help("Folder where the database file will be stored")
                        .value_parser(value_parser!(PathBuf))
                        .required(true),
                    Arg::new("INDEX_URL")
                        .long("index_url")
                        .short('i')
                        .action(ArgAction::Set)
                        .help("Main index URL")
                        .required(true),
                    Arg::new("I18N_URL")
                        .long("index_locales_url")
                        .short('l')
                        .action(ArgAction::Set)
                        .help("Index locales URL")
                        .required(true),
                    Arg::new("DB_TYPE")
                        .long("db-type")
                        .short('t')
                        .action(ArgAction::Set)
                        .help("Database type")
                        .value_parser(PossibleValuesParser::new([
                            FILL_STANDARD_DATABASE,
                            FILL_DNS_DATABASE,
                        ]))
                        .default_value(FILL_STANDARD_DATABASE),
                ]),
        )
        .subcommand(Command::new("test"))
}

/// Entrypoint of a CLI application
pub(super) fn run_app() {
    let matches = cli_setup().get_matches();

    match matches.subcommand() {
        Some(("fill_database", sub_matches)) => {
            let db_path = sub_matches.get_one::<PathBuf>("DB_FOLDER_PATH").unwrap();
            let index_url = sub_matches.get_one::<String>("INDEX_URL").unwrap();
            let index_i18n_url = sub_matches.get_one::<String>("I18N_URL").unwrap();
            let filter_list_type_str = sub_matches.get_one::<String>("DB_TYPE").unwrap().as_str();

            let filter_list_type: FilterListType = match filter_list_type_str {
                FILL_STANDARD_DATABASE => FilterListType::STANDARD,
                FILL_DNS_DATABASE => FilterListType::DNS,
                _ => unimplemented!(),
            };

            fill_database::entry(db_path, index_url, index_i18n_url, filter_list_type);
        }

        Some(("test", _)) => {
            test();
        }

        _ => unreachable!(),
    }
}
