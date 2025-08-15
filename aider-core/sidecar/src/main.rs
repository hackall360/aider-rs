use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    token: Option<String>,
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
        "llm" => {
            #[derive(Deserialize)]
            struct LlmParams {
                prompt: String,
            }
            let params: LlmParams = serde_json::from_value(req.params).unwrap_or(LlmParams {
                prompt: String::new(),
            });
            let ans = aider_core::llm(params.prompt);
            RpcResponse::result(Value::String(ans))
        }
        _ => RpcResponse::error("unknown method".into()),
    };

    (StatusCode::OK, Json(resp))
}

#[tokio::main]
async fn main() {
    let token = env::var("SIDECAR_TOKEN").ok();
    let state = AppState { token };
    let app = Router::new()
        .route("/ping", get(ping))
        .route("/rpc", post(rpc_handler))
        .with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {addr}");
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
