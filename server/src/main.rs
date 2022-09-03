use futures::{SinkExt, StreamExt, TryFutureExt};
use nanoid::nanoid;
use std::{collections::HashMap, env, sync::Arc};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{ws::Message, Filter};

mod checkers;
mod game;

type Games = Arc<RwLock<HashMap<String, RwLock<game::Game>>>>;

#[tokio::main]
async fn main() {
    let games = Games::default();
    let game = warp::path!("game" / String)
        .and(warp::ws())
        .and(warp::any().map(move || games.clone()))
        .map(|game_id, ws: warp::ws::Ws, games| {
            ws.on_upgrade(move |websocket| on_upgrade(game_id, websocket, games))
        });
    let index = warp::path::end().map(|| "Endpoint for checkers game");
    let routes = index.or(game);
    warp::serve(routes)
        .run((
            [0, 0, 0, 0],
            env::var("PORT")
                .unwrap_or("3030".to_string())
                .parse()
                .unwrap(),
        ))
        .await;
}

async fn on_upgrade(game_id: String, websocket: warp::ws::WebSocket, games: Games) {
    let (mut ws_tx, mut ws_rx) = websocket.split();
    let player_id = nanoid!();

    // Create buffer for messages
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);
    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });

    if join_game(&game_id, &player_id, &games, tx).await.is_err() {
        return;
    }

    // Receive messages from the websocket
    while let Some(res) = ws_rx.next().await {
        match res {
            Ok(msg) => game_message(&game_id, &player_id, msg, &games).await,
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }

    leave_game(&game_id, &player_id, &games).await;
}

async fn join_game(
    game_id: &str,
    player_id: &str,
    games: &Games,
    tx: mpsc::UnboundedSender<Message>,
) -> Result<(), ()> {
    games
        .write()
        .await
        .entry(game_id.to_string())
        .or_default()
        .write()
        .await
        .add_player(player_id.to_string(), tx)?;
    println!("Player {} joined game {}", &player_id, &game_id);
    Ok(())
}

async fn leave_game(game_id: &str, player_id: &str, games: &Games) {
    let mut empty = false;
    if let Some(game) = games.read().await.get(game_id) {
        let mut game = game.write().await;
        game.remove_player(player_id);
        empty = game.is_empty();
    }
    if empty {
        games.write().await.remove(game_id);
    }
    println!("Player {} left game {}", &player_id, &game_id);
}

async fn game_message(game_id: &str, player_id: &str, msg: Message, games: &Games) {
    if let Some(game) = games.read().await.get(game_id) {
        game.write().await.try_move_piece(player_id, msg);
    }
}
