use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub actor: Option<Actor>,
    pub repo: Option<Repo>,
    pub public: bool,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Actor {
    // #[serde(deserialize_with = "deserialize_id")]
    pub id: Option<i64>,
    pub login: Option<String>,
    pub display_login: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "avatar_url")]
    pub avatar_url: Option<String>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Repo {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub url: Option<String>,
}
