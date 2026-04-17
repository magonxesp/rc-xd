use std::collections::VecDeque;
use std::sync::{LazyLock, Mutex};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

static COMMANDS: LazyLock<Mutex<VecDeque<String>>> = LazyLock::new(|| Mutex::new(VecDeque::new()));

#[get("/command")]
async fn get_command() -> impl Responder {
    let mut commands = COMMANDS.lock().unwrap();
    let command = commands.pop_front();

    HttpResponse::Ok().body(command.unwrap_or("".to_string()))
}

#[post("/command")]
async fn post_command(command: String) -> impl Responder {
    let mut commands = COMMANDS.lock().unwrap();
    commands.push_back(command.clone());
    HttpResponse::NoContent().finish()
}

#[post("/command/output")]
async fn post_command_output(output: String) -> impl Responder {
    println!("{}", output);
    HttpResponse::NoContent().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(get_command)
            .service(post_command)
            .service(post_command_output)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}