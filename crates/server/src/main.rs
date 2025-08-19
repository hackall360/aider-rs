use axum::{extract::{Path, WebSocketUpgrade}, response::IntoResponse, routing::get, Router};
use axum::extract::ws::{Message, WebSocket};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use std::time::Duration;

pub mod pb {
    tonic::include_proto!("aider.v1");
}
use pb::session_service_server::{SessionService, SessionServiceServer};
use pb::repo_map_service_server::{RepoMapService, RepoMapServiceServer};
use pb::diff_service_server::{DiffService, DiffServiceServer};
use pb::*;

const SERVER_VERSION: &str = "1.0.0";

#[derive(Default)]
struct SessionSvc;

#[tonic::async_trait]
impl SessionService for SessionSvc {
    async fn open(&self, request: Request<OpenRequest>) -> Result<Response<OpenResponse>, Status> {
        let req = request.into_inner();
        let client = semver::Version::parse(&req.client_version).unwrap_or_else(|_| semver::Version::new(0,0,0));
        let server = semver::Version::parse(SERVER_VERSION).unwrap();
        if client.major != server.major {
            return Err(Status::failed_precondition("incompatible client"));
        }
        Ok(Response::new(OpenResponse {
            session_id: uuid::Uuid::new_v4().to_string(),
            server_version: SERVER_VERSION.to_string(),
        }))
    }

    async fn close(&self, _req: Request<CloseRequest>) -> Result<Response<CloseResponse>, Status> {
        Ok(Response::new(CloseResponse {}))
    }

    type SendMessageStream = ReceiverStream<Result<TokenChunk, Status>>;

    async fn send_message(&self, _req: Request<SendMessageRequest>) -> Result<Response<Self::SendMessageStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            let _ = tx.send(Ok(TokenChunk{ text: "hello".into(), done: false })).await;
            let _ = tx.send(Ok(TokenChunk{ text: "".into(), done: true })).await;
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn set_files(&self, _req: Request<SetFilesRequest>) -> Result<Response<Empty>, Status> {
        Ok(Response::new(Empty {}))
    }
}

#[derive(Default)]
struct RepoMapSvc;

#[tonic::async_trait]
impl RepoMapService for RepoMapSvc {
    async fn get_map(&self, _req: Request<GetMapRequest>) -> Result<Response<GetMapResponse>, Status> {
        Ok(Response::new(GetMapResponse { map_json: "{}".into() }))
    }
}

#[derive(Default)]
struct DiffSvc;

#[tonic::async_trait]
impl DiffService for DiffSvc {
    async fn preview(&self, _req: Request<PreviewRequest>) -> Result<Response<PreviewResponse>, Status> {
        Ok(Response::new(PreviewResponse { preview: "".into() }))
    }

    async fn apply(&self, _req: Request<ApplyRequest>) -> Result<Response<ApplyResponse>, Status> {
        Ok(Response::new(ApplyResponse {}))
    }
}

async fn ws_handler(Path(_): Path<String>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        if socket.send(Message::Text("token".into())).await.is_err() {
            break;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();

    let grpc_addr = "[::1]:50051".parse().unwrap();

    let grpc_server = Server::builder()
        .add_service(SessionServiceServer::new(SessionSvc::default()))
        .add_service(RepoMapServiceServer::new(RepoMapSvc::default()))
        .add_service(DiffServiceServer::new(DiffSvc::default()))
        .serve(grpc_addr);

    let ws_app = Router::new().route("/stream/:session_id", get(ws_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    let ws_server = axum::serve(listener, ws_app);

    tokio::try_join!(
        async { grpc_server.await.map_err(|e| Box::<dyn std::error::Error + Send + Sync>::from(e)) },
        async { ws_server.await.map_err(|e| Box::<dyn std::error::Error + Send + Sync>::from(e)) },
    )?;
    Ok(())
}
