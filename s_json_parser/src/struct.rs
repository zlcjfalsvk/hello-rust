use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename(deserialize = "_id"))]
    pub id: String,
    pub index: i32,
    pub name: String,
    pub gender: String,
    pub company: String,
    pub email: String,
    #[serde(rename(deserialize = "ProjectUserMap"))]
    pub project_user_map: Vec<ProjectUserMap>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectUserMap {
    pub user_id: String,
    pub project_id: String,
    #[serde(rename(deserialize = "Project"))]
    pub project: Option<Project>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
}

impl From<Map<String, Value>> for User {
    fn from(map: Map<String, Value>) -> Self {
        let user: User = serde_json::from_value(serde_json::Value::from(map)).unwrap();

        user
    }
}
