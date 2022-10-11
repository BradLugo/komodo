use std::collections::HashMap;

use mungos::ObjectId;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub type PermissionsMap = HashMap<String, u8>;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub enabled: bool,
    pub admin: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    // used with auth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub host: String,
    pub permissions: PermissionsMap,
    pub to_notify: Vec<String>,
    pub cpu_alert: f64,
    pub mem_alert: f64,
    pub disk_alert: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_core: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_interval: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            id: None,
            name: String::new(),
            host: String::new(),
            permissions: HashMap::new(),
            to_notify: Vec::new(),
            cpu_alert: 50.0,
            mem_alert: 75.0,
            disk_alert: 75.0,
            passkey: None,
            is_core: None,
            stats_interval: None,
            region: None,
            instance_id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String, // must be formatted to be compat with docker
    pub server_id: String,
    pub permissions: PermissionsMap,
    pub docker_run_args: DockerRunArgs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_core: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub permissions: PermissionsMap,
    pub version: Version,

    // git related
    pub repo: Option<String>,
    pub branch: Option<String>,
    pub github_account: Option<String>,
    pub on_clone: Option<Command>,

    // build related
    pub pre_build: Option<Command>,
    pub docker_build_args: Option<DockerBuildArgs>,
    pub docker_account: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BuildRecord {
    pub start_ts: i64,
    pub end_ts: i64,
    pub successful: bool,
    pub logs: Vec<Log>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Procedure {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub procedure: Vec<Operation>,
    pub permissions: PermissionsMap,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Update {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<EntityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    pub operation: Operation,
    pub command: String,
    pub log: Vec<Log>,
    pub ts: i64,
    pub is_error: bool,
    pub operator: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DockerBuildArgs {
    pub build_path: String,
    pub dockerfile_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DockerRunArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    pub ports: Vec<Conversion>,
    pub volumes: Vec<Conversion>,
    pub environment: Vec<EnvironmentVar>,
    pub network: String,
    pub restart: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_account: Option<String>,
}

impl Default for DockerRunArgs {
    fn default() -> Self {
        Self {
            image: None,
            ports: Vec::new(),
            volumes: Vec::new(),
            environment: Vec::new(),
            network: "bridge".to_string(),
            restart: "no".to_string(),
            post_image: None,
            container_user: None,
            docker_account: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub stage: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_out: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_err: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Command {
    pub path: String,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub major: u64,
    pub minor: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Conversion {
    pub local: String,
    pub container: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentVar {
    pub variable: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EntityType {
    Build,
    Deployment,
    Server,
}

#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Operation {
    // server
    PruneImagesServer,
    PruneContainersServer,
    PruneNetworksServer,

    // build
    BuildBuild,
    RecloneBuild,

    // deployment
    DeployDeployment,
    StopDeployment,
    StartDeployment,
    PullDeployment,
    RecloneDeployment,
}
