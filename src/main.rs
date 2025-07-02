use std::{collections::HashMap, sync::Arc};

use axum::{
    Router,
    extract::{Path, State, WebSocketUpgrade, ws::Message, ws::WebSocket},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use futures_util::{SinkExt, StreamExt};
use rand::{Rng, distr::Alphanumeric};
use serde::{Deserialize, Serialize};
use tokio::{
    net::TcpListener,
    select,
    sync::{
        Mutex,
        mpsc::{UnboundedSender, unbounded_channel},
    },
};

mod app_state;
mod handler;
mod room;
mod types;
use app_state::AppState;
use room::Room;

use crate::types::{ClientRole, SignalingMessage};

#[tokio::main]
async fn main() {
    let mut feeded_data = HashMap::new();
    feeded_data.insert("1234".to_string(), Room::new());

    let app_state = AppState::new();
    let room_id = app_state.create_new_room().await;

    println!("Roomif {} is connected as sender", room_id);

    let router = Router::new()
        .route("/ws/{room_id}", get(handle_room_upgrade))
        .with_state(app_state);

    let server_addr = "0.0.0.0:8000";
    let listener = TcpListener::bind(server_addr).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

async fn handle_room_upgrade(
    Path(room_id): Path<String>,
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    //TODO:check the room exist or not
    let may_room = {
        let state = state.rooms.lock().await;
        state.get(&room_id).cloned()
    };
    if let Some(room) = may_room {
        return ws.on_upgrade(|socket| handle_room(socket, room));
    }
    (StatusCode::NOT_ACCEPTABLE, "No room found").into_response()
}

async fn handle_room(socket: WebSocket, mut state: Room) {
    let (tx, mut rx) = unbounded_channel::<SignalingMessage>();

    let is_sender = state.is_sender_available().await;
    if is_sender {
        //this means it is sender
        println!("Connected as the sender");
        state.set_sender(tx).await;
    } else {
        let unique_client_id = uuid::Uuid::new_v4().to_string();
        println!("Connected as receiver clientid {}", unique_client_id);
        state.add_receiver(unique_client_id, tx);
    }

    let (mut writer, mut reader) = socket.split();
    //this is the task responsible for receiving from channel and writing to the socket of sender
    println!("Spinup the writer task");
    let writer_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            writer
                .send(Message::Text(
                    serde_json::to_string(&message).unwrap().into(),
                ))
                .await
                .unwrap();
        }
    });

    println!("Spinup the reader task");
    let reader_task = tokio::spawn(async move {
        while let Some(msg) = reader.next().await {
            match msg {
                Ok(Message::Text(msg)) => {
                    println!("new message is received");

                    if let Ok(signaling_msg) = serde_json::from_str::<SignalingMessage>(&msg) {
                        if is_sender {
                            //lets fuck off the receivers
                            println!("receiver is fucking off");
                            state.broadcast_to_receivers(SignalingMessage::Join {
                                client_id: "jcj".to_string(),
                            });
                        } else {
                            //this is receiver

                            state.send_to_sender(SignalingMessage::Join {
                                client_id: "heh".to_string(),
                            });
                        }
                        // match signaling_msg {}
                    }
                }
                Ok(Message::Close(_)) => {}
                Err(e) => {}
                _ => {
                    println!("Unsupported message");
                }
            }
        }
    });

    select! {
        _=reader_task=>{},
        _=writer_task=>{
            //clean up garnu parxa yeta
        }
    }
}

async fn received_handle_message_processing(
    client_role: ClientRole,
    room: Room,
    message: SignalingMessage,
) {
}
