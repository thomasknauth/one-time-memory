use actix_web::{web, App, HttpResponse, HttpServer, Responder, post};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Deserialize)]
struct ProgramInput {
    v0: String,
    v1: String,
}

#[derive(Serialize)]
struct ProgramResponse {
    uuid: String,
}

#[derive(Deserialize)]
struct ReadInput {
    uuid: String,
    selector: String,
}

#[derive(Serialize)]
struct ReadResponse {
    value: String,
}

struct AppState {
    data_store: Mutex<HashMap<String, HashMap<String, String>>>,
}

#[post("/program")]
async fn program(data: web::Json<ProgramInput>, state: web::Data<AppState>) -> impl Responder {
    let uid = Uuid::new_v4().to_string();
    let mut data_store = state.data_store.lock().unwrap();

    data_store.insert(uid.clone(), HashMap::from([
        ("0".to_string(), data.v0.clone()),
        ("1".to_string(), data.v1.clone()),
    ]));

    HttpResponse::Ok().json(ProgramResponse { uuid: uid })
}

#[post("/read")]
async fn read(data: web::Json<ReadInput>, state: web::Data<AppState>) -> impl Responder {
    let mut data_store = state.data_store.lock().unwrap();

    match data_store.remove(&data.uuid) {
        Some(values) => {
            if let Some(value) = values.get(&data.selector) {
                HttpResponse::Ok().json(ReadResponse { value: value.clone() })
            } else {
                HttpResponse::BadRequest().body("Selector must be '0' or '1'")
            }
        }
        None => HttpResponse::NotFound().body("Invalid or already consumed UUID"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_state = web::Data::new(AppState {
        data_store: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .service(program)
            .service(read)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
