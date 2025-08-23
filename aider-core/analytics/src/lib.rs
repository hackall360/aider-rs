use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{fs, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
struct Store {
    uuid: String,
    permanently_disable: bool,
    asked_opt_in: bool,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            permanently_disable: false,
            asked_opt_in: false,
        }
    }
}

const HISTORY_VERSION: u32 = 1;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SessionEvent {
    pub event: String,
    pub properties: Value,
    pub user_id: String,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SessionHistory {
    pub version: u32,
    pub events: Vec<SessionEvent>,
}

impl Default for SessionHistory {
    fn default() -> Self {
        Self { version: HISTORY_VERSION, events: Vec::new() }
    }
}

#[derive(Clone)]
pub struct Analytics {
    store: Store,
    path: PathBuf,
    history_path: PathBuf,
    history: SessionHistory,
    client: reqwest::Client,
    api_key: String,
    host: String,
}

impl Analytics {
    pub fn new(host: &str, api_key: &str) -> Self {
        Self::new_with_dir(host, api_key, None)
    }

    pub fn new_with_dir(host: &str, api_key: &str, dir: Option<PathBuf>) -> Self {
        let path = data_file(dir.as_ref());
        let history_path = history_file(dir.as_ref());
        let store = load_store(&path);
        let history = load_history(&history_path);
        Self {
            store,
            path,
            history_path,
            history,
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
            host: host.to_string(),
        }
    }

    pub async fn event(&mut self, event: &str, properties: Value) -> Result<(), reqwest::Error> {
        let props_clone = properties.clone();
        self.log_event(event, props_clone);
        if self.store.permanently_disable || !self.store.asked_opt_in {
            return Ok(());
        }
        println!("Sending analytics event '{}', please wait...", event);
        let mut props = match properties {
            Value::Object(map) => map,
            _ => serde_json::Map::new(),
        };
        props.insert(
            "distinct_id".to_string(),
            Value::String(self.store.uuid.clone()),
        );
        let payload = json!({
            "api_key": self.api_key,
            "event": event,
            "properties": Value::Object(props),
        });
        self.client
            .post(format!("{}/capture/", self.host))
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;
        println!("Analytics event '{}' sent.", event);
        Ok(())
    }

    fn log_event(&mut self, event: &str, properties: Value) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let entry = SessionEvent {
            event: event.to_string(),
            properties,
            user_id: self.store.uuid.clone(),
            time: timestamp,
        };
        self.history.events.push(entry);
        let _ = save_history(&self.history_path, &self.history);
    }

    pub fn events(&self) -> &[SessionEvent] {
        &self.history.events
    }

    pub fn opt_in(&mut self) {
        self.store.asked_opt_in = true;
        let _ = save_store(&self.path, &self.store);
    }

    pub fn opt_out(&mut self) {
        self.store.permanently_disable = true;
        self.store.asked_opt_in = true;
        let _ = save_store(&self.path, &self.store);
    }
}

fn data_file(dir: Option<&PathBuf>) -> PathBuf {
    if let Some(base) = dir {
        let _ = fs::create_dir_all(base);
        base.join("analytics.json")
    } else if let Some(dirs) = ProjectDirs::from("com", "aider", "aider") {
        let dir = dirs.config_dir();
        let _ = fs::create_dir_all(dir);
        dir.join("analytics.json")
    } else {
        PathBuf::from("analytics.json")
    }
}

fn history_file(dir: Option<&PathBuf>) -> PathBuf {
    if let Some(base) = dir {
        let _ = fs::create_dir_all(base);
        base.join("session_history.json")
    } else if let Some(dirs) = ProjectDirs::from("com", "aider", "aider") {
        let dir = dirs.config_dir();
        let _ = fs::create_dir_all(dir);
        dir.join("session_history.json")
    } else {
        PathBuf::from("session_history.json")
    }
}

fn load_store(path: &PathBuf) -> Store {
    if let Ok(contents) = fs::read_to_string(path) {
        if let Ok(store) = serde_json::from_str(&contents) {
            return store;
        }
    }
    let store = Store::default();
    let _ = save_store(path, &store);
    store
}

fn save_store(path: &PathBuf, store: &Store) -> std::io::Result<()> {
    let contents = serde_json::to_string_pretty(store)?;
    fs::write(path, contents)
}

fn load_history(path: &PathBuf) -> SessionHistory {
    if let Ok(contents) = fs::read_to_string(path) {
        if let Ok(history) = serde_json::from_str::<SessionHistory>(&contents) {
            if history.version == HISTORY_VERSION {
                return history;
            }
        }
    }
    let history = SessionHistory::default();
    let _ = save_history(path, &history);
    history
}

fn save_history(path: &PathBuf, history: &SessionHistory) -> std::io::Result<()> {
    let contents = serde_json::to_string_pretty(history)?;
    fs::write(path, contents)
}
