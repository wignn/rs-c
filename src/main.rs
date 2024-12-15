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



fn handle_post_request(request: &str) -> (String, String) {
    match (get_user_request_body(&request), Client::connect(DATABASE_URL, NoTls)) {
        (Ok(user), Ok(mut client)) => {
            let result = client.execute(
                "INSERT INTO users (name, email) VALUES ($1, $2)",
                &[&user.name, &user.email],
            ).unwarp();
            (OK_RESPONSE.to_string(), "User created".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR_RESPONSE.to_string(), "Failed to parse request or connect to database".to_string()),
    }
}

fn handle_get_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) =>
            match client.query_one("SELECT * FROM users WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let user = User {
                        id: row.get(0),
                        name: row.get(1),
                        email: row.get(2),
                    };

                    (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
                }
                _ => (NOT_FOUND.to_string(), "User not found".to_string()),
            }

        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

fn handle_get_all_request(request: &str) -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut users = Vec::new();

            for row in client.query("SELECT * FROM users", &[]).unwrap() {
                users.push(User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                });
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}


fn handle_put_request(request: &str) -> (String, String) {
    match
        (
            get_id(&request).parse::<i32>(),
            get_user_request_body(&request),
            Client::connect(DB_URL, NoTls),
        )
    {
        (Ok(id), Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                    &[&user.name, &user.email, &id]
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "User updated".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}


fn handle_delete_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&id]).unwrap();

            if rows_affected == 0 {
                return (NOT_FOUND.to_string(), "User not found".to_string());
            }

            (OK_RESPONSE.to_string(), "User deleted".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}



fn get_id(request: &str)->&str{
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}


fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    let body = request.split("\r\n\r\n").last().unwrap_or_default();
    serde_json::from_str(body)
}