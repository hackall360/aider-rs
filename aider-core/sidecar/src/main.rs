use aider_analytics::Analytics;
use axum::{
    Json, Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
#[cfg(unix)]
use crossterm::tty::IsTty;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, net::SocketAddr};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpListener,
    process::Command,
};

#[derive(Clone)]
struct AppState {
    token: Option<String>,
    analytics: Analytics,
}

async fn ping() -> Json<&'static str> {
    Json(aider_core::ping())
}

#[derive(Deserialize)]
struct RpcRequest {
    method: String,
    params: Value,
}

#[derive(Serialize)]
struct RpcResponse {
    result: Option<Value>,
    error: Option<String>,
}

impl RpcResponse {
    fn result(v: Value) -> Self {
        Self {
            result: Some(v),
            error: None,
        }
    }
    fn error(msg: String) -> Self {
        Self {
            result: None,
            error: Some(msg),
        }
    }
}

async fn rpc_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<RpcRequest>,
) -> (StatusCode, Json<RpcResponse>) {
    if let Some(token) = &state.token {
        match headers.get("authorization") {
            Some(h) if h.to_str().ok() == Some(&format!("Bearer {token}")) => {}
            _ => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(RpcResponse::error("unauthorized".into())),
                );
            }
        }
    }

    let resp = match req.method.as_str() {
        "git" => {
            #[derive(Deserialize)]
            struct GitParams {
                args: Vec<String>,
            }
            let params: GitParams =
                serde_json::from_value(req.params).unwrap_or(GitParams { args: vec![] });
            match aider_core::git(params.args) {
                Ok(out) => RpcResponse::result(Value::String(out)),
                Err(e) => RpcResponse::error(e.to_string()),
            }
        }
        "repo_map" => {
            let map = aider_core::repo_map();
            RpcResponse::result(Value::String(map))
        }
        "llm.chat" => {
            #[derive(Deserialize)]
            struct ChatParams {
                messages: Vec<aider_core::chat::ChatMessage>,
            }
            let params: ChatParams =
                serde_json::from_value(req.params).unwrap_or(ChatParams { messages: vec![] });
            match aider_core::chat::chat(&params.messages).await {
                Ok(answer) => RpcResponse::result(Value::String(answer)),
                Err(e) => RpcResponse::error(e.to_string()),
            }
        }
        "llm.models" => match aider_core::models::fetch_models().await {
            Ok(models) => {
                let v = serde_json::to_value(models).unwrap_or(Value::Null);
                RpcResponse::result(v)
            }
            Err(e) => RpcResponse::error(e.to_string()),
        },
        "scrape.url" => {
            #[derive(Deserialize)]
            struct ScrapeParams {
                url: String,
            }
            let params: ScrapeParams =
                serde_json::from_value(req.params).unwrap_or(ScrapeParams { url: String::new() });
            match aider_core::scrape::scrape_url(&params.url).await {
                Ok(md) => RpcResponse::result(Value::String(md)),
                Err(e) => RpcResponse::error(e.to_string()),
            }
        }
        "analytics_event" => {
            #[derive(Deserialize)]
            struct AnalyticsParams {
                event: String,
                properties: Value,
            }
            let params: AnalyticsParams =
                serde_json::from_value(req.params).unwrap_or(AnalyticsParams {
                    event: String::new(),
                    properties: Value::Null,
                });
            match state
                .analytics
                .event(&params.event, params.properties)
                .await
            {
                Ok(_) => RpcResponse::result(Value::Bool(true)),
                Err(e) => RpcResponse::error(e.to_string()),
            }
        }
        _ => RpcResponse::error("unknown method".into()),
    };

    (StatusCode::OK, Json(resp))
}

async fn command_ws(ws: WebSocketUpgrade) -> impl axum::response::IntoResponse {
    ws.on_upgrade(handle_command)
}

async fn handle_command(socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();
    if let Some(Ok(Message::Text(txt))) = receiver.next().await {
        #[derive(Deserialize)]
        struct CmdReq {
            cmd: String,
            args: Vec<String>,
        }
        let req: CmdReq = serde_json::from_str(&txt).unwrap_or(CmdReq {
            cmd: String::new(),
            args: vec![],
        });

        #[cfg(unix)]
        let mut cmd = {
            let _ = std::io::stdout().is_tty();
            let mut c = Command::new(&req.cmd);
            c.args(&req.args);
            c
        };

        #[cfg(windows)]
        let mut cmd = {
            use std::ffi::OsString;
            let mut c = if req.cmd.ends_with(".ps1") {
                let mut p = Command::new("powershell");
                p.arg("-File").arg(&req.cmd);
                p
            } else {
                let mut p = Command::new("cmd.exe");
                p.arg("/C").arg(&req.cmd);
                p
            };
            c
        };

        cmd.stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        match cmd.spawn() {
            Ok(mut child) => {
                let mut stdout = BufReader::new(child.stdout.take().unwrap()).lines();
                let mut stderr = BufReader::new(child.stderr.take().unwrap()).lines();
                loop {
                    tokio::select! {
                        Ok(Some(line)) = stdout.next_line() => {
                            let msg = serde_json::json!({"type":"stdout","data":line});
                            if sender.send(Message::Text(msg.to_string())).await.is_err() { break; }
                        }
                        Ok(Some(line)) = stderr.next_line() => {
                            let msg = serde_json::json!({"type":"stderr","data":line});
                            if sender.send(Message::Text(msg.to_string())).await.is_err() { break; }
                        }
                        status = child.wait() => {
                            let code = status.map(|s| s.code().unwrap_or(-1)).unwrap_or(-1);
                            let msg = serde_json::json!({"type":"exit","code":code});
                            let _ = sender.send(Message::Text(msg.to_string())).await;
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                let msg = serde_json::json!({"type":"error","message":e.to_string()});
                let _ = sender.send(Message::Text(msg.to_string())).await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("SIDECAR_TOKEN").ok();
    let host = env::var("POSTHOG_HOST").unwrap_or_else(|_| "https://us.i.posthog.com".into());
    let api_key = env::var("POSTHOG_PROJECT_API_KEY")
        .unwrap_or_else(|_| "phc_99T7muzafUMMZX15H8XePbMSreEUzahHbtWjy3l5Qbv".into());
    let analytics = Analytics::new(&host, &api_key);
    let state = AppState { token, analytics };
    let app = Router::new()
        .route("/ping", get(ping))
        .route("/rpc", post(rpc_handler))
        .route("/command", get(command_ws))
        .with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {addr}");
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
