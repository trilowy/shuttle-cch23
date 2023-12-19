use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};

pub async fn task_1(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
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
