use postgres::Error as PostgresError;
use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};
use std::env;
use std::env;
use std::fmt::format;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

const DATABASE_URL: &str = env!("DATABASE_URL");

//FUNCTION TO CONNECT TO DATABASE
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\ncContent-Type: application/json\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR_RESPONSE: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

//main
fn main() {
    if let Err(e) = set_database() {
        eprintln!("Error: {}", e);
    }

    // Create a listener on port 8080
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind to address");
    println!(
        "Server is running on 
        http://localhost:8080"
    );

    for stram in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn set_database() -> Resul<(), PostgresError> {
    let mut client = Client::connect(DATABASE_URL, NoTls)?;
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )",
        &[],
    )?;
    Ok(())
}
