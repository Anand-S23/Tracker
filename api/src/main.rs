use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use tower_http::cors::CorsLayer;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Player {
    name: String,
    img: String,
    points: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct State {
    players: Vec<Player>,
}

#[derive(Deserialize)]
struct Win {
    value: u32,
}

const PLAYERS: [&str; 5] = ["Anand", "Corey", "Jake", "Nicholas", "Nikhil"];

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = Router::new()
        .route("/api/tracker", get(tracking_info_handler))
        .route("/api/tracker", post(update_tracking_info_handler))
        .layer(cors);

    println!("Tracker is now running!");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn read_json_file(filename: &str) -> Vec<Player> {
    let file = File::open(filename).expect("Invalid file");
    let current_state: State = serde_json::from_reader(file).expect("Invalid JSON in file");
    current_state.players
}

fn write_json_file(filename: &str, data: String) -> Result<(), impl IntoResponse> {
    let mut file = match std::fs::OpenOptions::new()
        .write(true)
        .append(false)
        .open(filename)
    {
        Ok(val) => val,
        Err(_) => {
            return Err(Json(
                serde_json::json!({"error": "Error Occured when writing to file"}),
            ))
        }
    };

    match file.write_all(data.as_bytes()) {
        Ok(_) => print!(""),
        Err(_) => {
            return Err(Json(
                serde_json::json!({"error": "Error Occured when writing to file"}),
            ))
        }
    };

    match file.flush() {
        Ok(_) => print!(""),
        Err(_) => {
            return Err(Json(
                serde_json::json!({"error": "Error Occured when writing to file"}),
            ))
        }
    };

    Ok(())
}

async fn tracking_info_handler() -> impl IntoResponse {
    Json(read_json_file("data/players.json"))
}

// Read from file, update values, write to file, send to frontend???
async fn update_tracking_info_handler(Json(new_win): Json<Win>) -> impl IntoResponse {
    let player_index: usize = new_win.value as usize;
    if player_index >= PLAYERS.len() {
        return Json(serde_json::json!({
            "status": "error",
            "message": "Not a valid player"
        }));
    }

    let current_player = PLAYERS[player_index];

    let mut players = read_json_file("data/players.json");
    let index = players
        .iter()
        .position(|player| player.name == current_player)
        .unwrap();

    let mut current_winner = players[index].clone();
    current_winner.points += 1;

    players.remove(index);
    players.insert(0, current_winner);

    // Write out to file
    let new_state = State { players: players };
    let data = serde_json::to_string(&new_state).expect("Not a serializable type");
    match write_json_file("data/players.json", data) {
        Ok(_) => {
            return Json(serde_json::json!({
                "status": "success",
                "message": "Updated"
            }))
        }
        Err(_) => {
            return Json(
                serde_json::json!({"status": "error", "message": "Error Occured when writing to file"}),
            )
        }
    };
}
