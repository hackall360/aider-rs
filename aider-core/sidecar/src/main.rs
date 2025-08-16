use aider_analytics::Analytics;
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
                Ok(ans) => RpcResponse::result(Value::String(ans)),
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
        .with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {addr}");
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
