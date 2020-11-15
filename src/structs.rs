use bollard::service::{
    RestartPolicy as BollardRestartPolicy, RestartPolicyNameEnum as BollardRestartPolicyNameEnum,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub name: String,
    pub id: String,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageCreateRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageCreateResponse {
    pub ok: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerCreateRequest {
    pub name: String,
    pub image: String,
    pub port_mappings: Vec<PortMapping>,
    pub environment: HashMap<String, Option<String>>,
    pub volumes: Vec<Volume>,
    pub restart_policy: Option<RestartPolicy>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerCreateResponse {
    pub id: String,
    pub warnings: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerStartRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerStartResponse {
    pub ok: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortMapping {
    pub container_port: u32,
    pub host_port: u32,
    pub ip_addr: String,
    pub protocol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Volume {
    pub host_dir: String,
    pub container_dir: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestartPolicy {
    pub name: RestartPolicyName,
    pub max_retries: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RestartPolicyName {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "unless_stopped")]
    UnlessStopped,
    #[serde(rename = "on_failure")]
    OnFailure,
    #[serde(rename = "no")]
    NoRestart,
}

impl Into<BollardRestartPolicyNameEnum> for RestartPolicyName {
    fn into(self) -> BollardRestartPolicyNameEnum {
        match self {
            RestartPolicyName::Always => BollardRestartPolicyNameEnum::ALWAYS,
            RestartPolicyName::UnlessStopped => BollardRestartPolicyNameEnum::UNLESS_STOPPED,
            RestartPolicyName::OnFailure => BollardRestartPolicyNameEnum::ON_FAILURE,
            RestartPolicyName::NoRestart => BollardRestartPolicyNameEnum::NO,
        }
    }
}

impl Into<BollardRestartPolicy> for &RestartPolicy {
    fn into(self) -> BollardRestartPolicy {
        BollardRestartPolicy {
            name: Some(self.name.into()),
            maximum_retry_count: self.max_retries,
        }
    }
}
