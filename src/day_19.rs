use crate::{Day19Rooms, Day19Views};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};

pub async fn task_1(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(task_1_handle_socket)
}

async fn task_1_handle_socket(mut socket: WebSocket) {
    let mut game_started = false;

    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        if let Message::Text(request) = msg {
            if request == "serve" {
                game_started = true;
            } else if game_started && request == "ping" {
                let response = Message::Text("pong".to_string());

                if socket.send(response).await.is_err() {
                    // client disconnected
                    return;
                }
            }
        }
    }
}

pub async fn task_2_reset(State(day_19_views): State<Day19Views>) {
    *day_19_views.write().await = 0;
}

pub async fn task_2_views(State(day_19_views): State<Day19Views>) -> String {
    day_19_views.read().await.to_string()
}

#[derive(Deserialize, Serialize)]
struct Tweet {
    user: Option<String>,
    message: String,
}

pub async fn task_2_room(
    Path((room_number, user)): Path<(u32, String)>,
    State(day_19_views): State<Day19Views>,
    State(day_19_rooms): State<Day19Rooms>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(move |socket| {
        task_2_handle_socket(socket, room_number, user, day_19_views, day_19_rooms)
    })
}

async fn task_2_handle_socket(
    socket: WebSocket,
    room_number: u32,
    user: String,
    day_19_views: Day19Views,
    day_19_rooms: Day19Rooms,
) {
    let (sender, mut receiver) = socket.split();

    let mut day_19_rooms_locked = day_19_rooms.write().await;
    if let Some(senders) = day_19_rooms_locked.get_mut(&room_number) {
        senders.push(sender);
    } else {
        day_19_rooms_locked.insert(room_number, vec![sender]);
    }
    drop(day_19_rooms_locked);

    while let Some(msg) = receiver.next().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        match msg {
            Message::Text(tweet) => {
                if let Ok(mut tweet) = serde_json::from_str::<Tweet>(&tweet) {
                    if tweet.user.is_none() {
                        if tweet.message.len() <= 128 {
                            tweet.user = Some(user.clone());

                            let message = Message::Text(serde_json::to_string(&tweet).unwrap());

                            let mut day_19_rooms_locked = day_19_rooms.write().await;
                            if let Some(senders) = day_19_rooms_locked.get_mut(&room_number) {
                                for sender in senders {
                                    if sender.send(message.clone()).await.is_ok() {
                                        *day_19_views.write().await += 1;
                                        // if error, client disconnected
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Message::Close(_) => {
                // client disconnected
                return;
            }
            _ => (),
        };
    }
}
