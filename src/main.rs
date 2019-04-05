#[macro_use]
extern crate clap;
extern crate postgres;
extern crate rpassword;

use clap::{App, Arg};

mod db;
mod commands;

use db::*;
use commands::*;

fn main() {
    let matches = App::new("conn-cli")
            .version(crate_version!())
            .author(crate_authors!())
            .about("CLI Database tool for Postgres")
            .arg(Arg::with_name("host")
                    .short("h")
                    .long("host")
                    .help("Database host address (default localhost)")
                    .takes_value(true))
            .arg(Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .help("Database host port (default 5432)")
                    .takes_value(true))
            .arg(Arg::with_name("database")
                    .short("d")
                    .long("db")
                    .help("Database name")
                    .takes_value(true))
            .arg(Arg::with_name("username")
                    .short("u")
                    .long("user")
                    .help("Database Username")
                    .takes_value(true))
            .arg(Arg::with_name("prompt")
                    .long("prompt")
                    .help("Always prompt for password"))
            .get_matches();

    let host = matches.value_of("host").unwrap_or("localhost");
    let port: u16 = matches.value_of("port").map(|it| it.parse().expect("Port must be valid number")).unwrap_or(5432);
    let database = matches.value_of("database").unwrap_or("postgres");
    let username = matches.value_of("username").unwrap_or("postgres");
    let password = if matches.is_present("prompt") {
        rpassword::read_password_from_tty(Some("Password: ")).unwrap()
    } else {
        unimplemented!();
    };

    let config = Config::new(host, port, database, username, &password);
    run_repl(create_connection(&config));
}

/*fn command() {
    let editor = env::var("EDITOR").unwrap_or("vi".to_string());
    let fname = env::args().nth(1).expect("Need to provide a filename as arg");

    println!("Opening the file in an editor...");
    prompt();
    let c = Command::new(editor)
            .arg(&fname)
            .spawn()
            .unwrap();
    c.wait_with_output().unwrap();

    println!("Viewing the file in less...");
    prompt();
    let c = Command::new("less")
            .arg(&fname)
            .spawn()
            .unwrap();
    c.wait_with_output().unwrap();

    println!("Viewing arbitrary data in less...");
    prompt();
    let mut less = Command::new("less")
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();

    {
        let less_in = less.stdin.as_mut().unwrap();
        less_in.write_all(b"hello world").unwrap();
    }

    less.wait_with_output().unwrap();

    println!("Exiting...");
}*/
