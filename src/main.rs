#[macro_use]
extern crate clap;
extern crate rpassword;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("conn-cli")
            .version(crate_version!())
            .author(crate_authors!())
            .about("CLI Database tool for Postgres")
            .arg(Arg::with_name("conn")
                    .short("c")
                    .long("conn")
                    .help("Connection String")
                    .takes_value(true))
            .arg(Arg::with_name("prompt")
                    .short("p")
                    .help("Always prompt for password"))
            .subcommand(SubCommand::with_name("test")
                    .about("does testing things"))
            .get_matches();

    let conn_str = matches.value_of("conn").unwrap_or("some default");
    println!("Conn str: {}", conn_str);

    if matches.is_present("prompt") {
        let pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
        println!("Password is {}", pass);
    }

    if let Some(test_matches) = matches.subcommand_matches("test") {
        println!("test command was called");
    }
}
