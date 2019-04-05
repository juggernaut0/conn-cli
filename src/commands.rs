use std::io::{stdin, stdout, Write};

use crate::db::Connection;

pub fn run_repl(conn: Connection) {
    loop {
    	let cmd = prompt(">>> ");
    	match cmd.as_ref() {
    		"tables" => print_tables(&conn),
    		"exit" => break,
    		_ => {
    			println!("Unknown command: {}", cmd)
    		}
    	}
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    stdout().flush().unwrap();
    let mut val = String::new();
    stdin().read_line(&mut val).unwrap();
    val.trim().to_string()
}

fn print_tables(conn: &Connection) {
	for tablename in conn.get_all_tables() {
    	println!("{}", tablename);
    }
}