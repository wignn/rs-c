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



fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

   match stream.read(&mut buffer) {
        Ok(size) => {
            request.push(String::from_utf8_lossy(&buffer[..size]).as_ref());
            let (status_line, content)= match &*request {
            r if &request_with("POST /users") =>handle_post_request(r), 
            r if &request_with("GET /users") =>handle_get_request(r),
            r if &request_with("PUT /users") =>handle_put_request(r),
            r if &request_with("DELETE /users") =>handle_delete_request(r),
            r if &request_with("GET /users/") =>handle_get_all_request(r),
            _=>(NOT_FOUND_RESPONSE.to_string(), "Not Found".to_string())
            };
            stream.write_all(status_line.as_bytes()).unwrap();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }

    }
}



fn handle_post_request(request: &str)->(String, String){
    macth (get_user_request_body(&request), Client::connect(DATABASE_URL)
)
}


fn get_id(request: &str)->&str{
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}


fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    let body = request.split("\r\n\r\n").last().unwrap_or_default();
    serde_json::from_str(body)
}