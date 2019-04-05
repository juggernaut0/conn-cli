use postgres::TlsMode;

pub struct Config {
    host: String,
    port: u16,
    database: String,
    username: String,
    password: String
}

impl Config {
    pub fn new(host: &str, port: u16, database: &str, username: &str, password: &str) -> Config {
        Config { 
            host: host.to_string(), 
            port, 
            database: database.to_string(), 
            username: username.to_string(), 
            password: password.to_string()
        }
    }
}

pub fn create_connection(config: &Config) -> Connection {
    let url = format!("postgresql://{}:{}@{}:{}/{}", config.username, config.password, config.host, config.port, config.database);
    Connection { conn: postgres::Connection::connect(url, TlsMode::None).unwrap() }
}

pub struct Connection {
    conn: postgres::Connection
}

impl Connection {
    pub fn get_all_tables(&self) -> Vec<String> {
        self.conn.query("SELECT * FROM pg_catalog.pg_tables WHERE schemaname <> 'pg_catalog' AND schemaname <> 'information_schema'", &[])
                .unwrap()
                .into_iter()
                .map(|row| {
                    let schemaname: String = row.get("schemaname");
                    let tablename: String = row.get("tablename");
                    format!("{}.{}", schemaname, tablename)
                })
                .collect()
    }
}
