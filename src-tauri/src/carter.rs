use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatResponse {
    pub output: Option<CarterOutput>,
    pub input: Option<String>,
    pub forced_behaviours: Option<Vec<ForcedBehaviour>>,
    pub agent: Option<Agent>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PluginList {
    pub plugins: Vec<Plugin>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OpenerResponse {
    pub output: Option<CarterOutput>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PersonaliseResponse {
    pub output: CarterOutput,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CarterOutput {
    pub text: String,
    pub audio: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForcedBehaviour {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Agent {
    pub name: String,
    pub image: Option<String>,
}

pub struct Carter {
    key: String,
    user_id: String,
}

impl Carter {
    pub fn new(key: String, user_id: String) -> Self {
        Self { key, user_id }
    }

    async fn send_request<T>(path: &str, data: &Value) -> Result<T, reqwest::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let body: T = reqwest::Client::new()
            .post(format!("https://api.carterlabs.ai/{}", path))
            .json(data)
            .send()
            .await?
            .json()
            .await?;

        Ok(body)
    }

    pub async fn chat(&self, text: &str) -> Result<ChatResponse, reqwest::Error> {
        let data = json!({
            "key": self.key,
            "text": text,
            "user_id": self.user_id,
            "speak": true,
        });

        Carter::send_request::<ChatResponse>("api/chat", &data).await
    }

    pub async fn opener(&self) -> Result<OpenerResponse, reqwest::Error> {
        let data = json!({
            "key": self.key,
            "user_id": self.user_id,
        });

        Carter::send_request::<OpenerResponse>("api/opener", &data).await
    }

    pub async fn get_plugin_list(&self) -> Option<PluginList> {
        let data = json!({
            "key": self.key,
            "text": "/plugin list",
            "user_id": self.user_id,
            "speak": false,
        });

        let body = Carter::send_request::<ChatResponse>("api/chat", &data).await;
        if let Ok(result) = body {
            if let Some(output) = result.output {
                let mut plugins = vec![];

                let text = output.text;
                let mut lines = text.lines();

                lines.next();
                for line in lines {
                    let mut parts = line.split(" - ");

                    let id = parts.next().unwrap().trim();
                    let info = self.get_plugin_info(id).await.unwrap();

                    plugins.push(info);
                }

                return Some(PluginList { plugins });
            }
        }

        None
    }

    pub async fn get_plugin_info(&self, plugin_id: &str) -> Option<Plugin> {
        let data = json!({
            "key": self.key,
            "text": format!("/plugin info {}", plugin_id),
            "user_id": self.user_id,
            "speak": false,
        });

        let body = Carter::send_request::<ChatResponse>("api/chat", &data).await;
        if let Ok(result) = body {
            if let Some(output) = result.output {
                let text = output.text;
                let mut lines = text.lines();

                let mut split = lines.next().unwrap().split("#");
                let id = split.nth(1).unwrap().replace(":", "");

                let mut split = lines.next().unwrap().split(": ");
                let name = split.nth(1).unwrap();

                let mut description = None;
                for line in lines {
                    let mut parts = line.split(": ");
                    let key = parts.next().unwrap().trim();
                    let value = parts.next().unwrap().trim();

                    match key {
                        "Description" => description = Some(value.to_string()),
                        _ => {}
                    }
                }

                return Some(Plugin {
                    id: id.to_string(),
                    name: name.to_string(),
                    description,
                });
            }
        }

        None
    }

    pub async fn personalise(&self, text: &str) -> Result<PersonaliseResponse, reqwest::Error> {
        let data = json!({
            "key": self.key,
            "text": text,
            "user_id": self.user_id,
            "speak": true,
        });

        Carter::send_request::<PersonaliseResponse>("api/personalise", &data).await
    }
}
