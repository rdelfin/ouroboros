use bollard::service::{
    Port as BollardPort, PortBinding as BollardPortBinding, PortTypeEnum as BollardPortType,
    RestartPolicy as BollardRestartPolicy, RestartPolicyNameEnum as BollardRestartPolicyNameEnum,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub name: String,
    pub id: String,
    pub command: String,
    pub image: String,
    pub state: Option<String>,
    pub status: Option<String>,
    pub ports: Vec<PortMapping>,
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
pub struct ContainerStopRequest {
    pub name: String,
    pub stop_timeout: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerStopResponse {
    pub ok: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerRemoveRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerRemoveResponse {
    pub ok: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortMapping {
    pub container_port: i64,
    pub host_port: Option<i64>,
    pub ip_addr: Option<String>,
    pub protocol: PortProtocol,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PortProtocol {
    #[serde(rename = "tcp")]
    TCP,
    #[serde(rename = "udp")]
    UDP,
    #[serde(rename = "sctp")]
    SCTP,
}

impl PortProtocol {
    pub fn from_opt(port: Option<BollardPortType>) -> Option<Self> {
        match port {
            None => None,
            Some(port) => match port {
                BollardPortType::EMPTY => None,
                BollardPortType::TCP => Some(PortProtocol::TCP),
                BollardPortType::UDP => Some(PortProtocol::UDP),
                BollardPortType::SCTP => Some(PortProtocol::SCTP),
            },
        }
    }

    pub fn to_str_name(&self) -> String {
        match self {
            PortProtocol::TCP => "tcp",
            PortProtocol::UDP => "udp",
            PortProtocol::SCTP => "sctp",
        }
        .to_string()
    }
}

impl From<BollardPort> for PortMapping {
    fn from(port: BollardPort) -> Self {
        PortMapping {
            container_port: port.private_port,
            host_port: port.public_port,
            ip_addr: port.ip,
            protocol: PortProtocol::from_opt(port.typ).unwrap_or(PortProtocol::TCP),
        }
    }
}

impl PortMapping {
    pub fn to_exposed_port(&self) -> (String, HashMap<(), ()>) {
        return (self.port_key(), HashMap::new());
    }

    pub fn to_port_binding(&self) -> (String, Option<Vec<BollardPortBinding>>) {
        if self.host_port.is_some() || self.ip_addr.is_some() {
            (
                self.port_key(),
                Some(vec![BollardPortBinding {
                    host_ip: self.ip_addr.clone(),
                    host_port: self.host_port.map(|p| p.to_string()),
                }]),
            )
        } else {
            (self.port_key(), None)
        }
    }

    pub fn port_key(&self) -> String {
        format!("{}/{}", self.container_port, self.protocol.to_str_name())
    }
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
