use axum::extract::ws::{Message, WebSocket};
use axum::{
    extract::{Path, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde_json::json;
use std::cmp::min;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

pub mod pb {
    tonic::include_proto!("aider.v1");
}
use pb::diff_service_server::{DiffService, DiffServiceServer};
use pb::repo_map_service_server::{RepoMapService, RepoMapServiceServer};
use pb::session_service_server::{SessionService, SessionServiceServer};
use pb::*;

const SERVER_VERSION: &str = "1.0.0";

#[derive(Default)]
struct SessionSvc;

#[tonic::async_trait]
impl SessionService for SessionSvc {
    async fn open(&self, request: Request<OpenRequest>) -> Result<Response<OpenResponse>, Status> {
        let req = request.into_inner();
        let client = semver::Version::parse(&req.client_version)
            .unwrap_or_else(|_| semver::Version::new(0, 0, 0));
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

    async fn send_message(
        &self,
        _req: Request<SendMessageRequest>,
    ) -> Result<Response<Self::SendMessageStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            let _ = tx
                .send(Ok(TokenChunk {
                    text: "hello".into(),
                    done: false,
                }))
                .await;
            let _ = tx
                .send(Ok(TokenChunk {
                    text: "".into(),
                    done: true,
                }))
                .await;
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
    async fn get_map(
        &self,
        req: Request<GetMapRequest>,
    ) -> Result<Response<GetMapResponse>, Status> {
        let budget = req.into_inner().token_budget;
        let map = json!({
            "budget": budget,
            "total_tokens": 10,
            "files": [
                {
                    "path": "README.md",
                    "relevance": 0.8,
                    "tokens": 10,
                    "symbols": [
                        {"name": "intro", "line": 1, "relevance": 0.5, "tokens": 5}
                    ]
                }
            ]
        });
        Ok(Response::new(GetMapResponse {
            map_json: map.to_string(),
        }))
    }

    async fn get_snippet(
        &self,
        req: Request<SnippetRequest>,
    ) -> Result<Response<SnippetResponse>, Status> {
        let req = req.into_inner();
        let content = std::fs::read_to_string(&req.path).unwrap_or_default();
        let lines: Vec<&str> = content.lines().collect();
        let line = req.line as usize;
        let ctx = req.context as usize;
        let start = if line > ctx { line - ctx - 1 } else { 0 };
        let end = min(lines.len(), line + ctx);
        let snippet = lines[start..end].join("\n");
        Ok(Response::new(SnippetResponse { content: snippet }))
    }
}

#[derive(Default)]
struct DiffSvc;

#[tonic::async_trait]
impl DiffService for DiffSvc {
    async fn preview(
        &self,
        _req: Request<PreviewRequest>,
    ) -> Result<Response<PreviewResponse>, Status> {
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
        .add_service(SessionServiceServer::new(SessionSvc))
        .add_service(RepoMapServiceServer::new(RepoMapSvc))
        .add_service(DiffServiceServer::new(DiffSvc))
        .serve(grpc_addr);

    let ws_app = Router::new().route("/stream/:session_id", get(ws_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    let ws_server = axum::serve(listener, ws_app);

    tokio::try_join!(
        async {
            grpc_server
                .await
                .map_err(Box::<dyn std::error::Error + Send + Sync>::from)
        },
        async {
            ws_server
                .await
                .map_err(Box::<dyn std::error::Error + Send + Sync>::from)
        },
    )?;
    Ok(())
}
