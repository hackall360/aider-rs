use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{fs, path::PathBuf};
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

#[derive(Clone)]
pub struct Analytics {
    store: Store,
    path: PathBuf,
    client: reqwest::Client,
    api_key: String,
    host: String,
}

impl Analytics {
    pub fn new(host: &str, api_key: &str) -> Self {
        let path = data_file();
        let store = load_store(&path);
        Self {
            store,
            path,
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
            host: host.to_string(),
        }
    }

    pub async fn event(&self, event: &str, properties: Value) -> Result<(), reqwest::Error> {
        if self.store.permanently_disable || !self.store.asked_opt_in {
            return Ok(());
        }
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
        Ok(())
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

fn data_file() -> PathBuf {
    if let Some(dirs) = ProjectDirs::from("com", "aider", "aider") {
        let dir = dirs.config_dir();
        let _ = fs::create_dir_all(dir);
        dir.join("analytics.json")
    } else {
        PathBuf::from("analytics.json")
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
