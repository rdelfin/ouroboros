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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerCreateResponse {
    pub id: String,
    pub warnings: Vec<String>,
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
