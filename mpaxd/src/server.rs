use std::sync::mpsc::Sender;
use std::sync::Arc;

use anyhow::Result;
use axum::body::Body;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use log::{error, info};
use racros::AutoDebug;
use serde::Deserialize;
use tokio::net::TcpListener;

use libmpax::api::{DEFAULT_SERVER_URL, ROUTE_ACTION_PAUSE, ROUTE_ACTION_PLAY};

use crate::player::PlayAction;

#[derive(AutoDebug, Clone)]
struct AppState {
    tx: Arc<Sender<PlayAction>>,
}

#[derive(AutoDebug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActionPlayParam {
    file_path: Option<String>,
}

#[derive(AutoDebug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActionPauseParam {}

/// Launch a thread for the socket to run on.
///
/// Listen for the messages from client (mpaxctl) and act to it.
///
/// This function takes a [Sender] type argument [tx] to send
/// Operations to the [Player].
pub async fn launch_server_thread(tx: Sender<PlayAction>) -> Result<()> {
    info!("server thread start");

    let app_state = Arc::new(AppState { tx: Arc::new(tx) });

    let server = Router::new()
        .route(ROUTE_ACTION_PLAY, get(handle_action_play))
        .route(ROUTE_ACTION_PAUSE, get(handle_action_pause))
        .with_state(app_state);
    let listener = TcpListener::bind(DEFAULT_SERVER_URL).await.unwrap();
    axum::serve(listener, server).await.unwrap();
    info!("server thread exit");
    Ok(())
}

async fn handle_action_play(
    State(app_state): State<Arc<AppState>>,
    params: Option<Query<ActionPlayParam>>,
) -> Response {
    info!("{ROUTE_ACTION_PLAY}: params={:#?}", params);

    if let Some(Query(ActionPlayParam {
        file_path: Some(file_path),
    })) = params
    {
        info!("{ROUTE_ACTION_PLAY} file_path={file_path}");
        let tx = app_state.tx.clone();
        if let Err(err) = tx.send(PlayAction::Play(String::from(file_path))) {
            error!("{ROUTE_ACTION_PLAY} error when handling root: {}", err);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("{err}")))
                .unwrap();
        };
    } else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("invalid argument"))
            .unwrap();
    }

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap()
}

async fn handle_action_pause(
    State(app_state): State<Arc<AppState>>,
    params: Option<Query<ActionPauseParam>>,
) -> Response {
    info!("{ROUTE_ACTION_PAUSE} params = {:#?}", params);
    if let Err(err) = app_state.tx.send(PlayAction::Pause) {
        error!("{ROUTE_ACTION_PAUSE} error: {}", err);
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(format!("{err}")))
            .unwrap();
    }
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap()
}
