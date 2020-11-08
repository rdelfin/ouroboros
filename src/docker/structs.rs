use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Names")]
    pub names: Vec<String>,
    #[serde(rename = "ImageID")]
    pub image_id: String,
    #[serde(rename = "Image")]
    pub image: Option<String>,
    #[serde(rename = "Command")]
    pub command: String,
    #[serde(rename = "Created")]
    pub created: i64,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Ports")]
    pub ports: Vec<Port>,
    #[serde(rename = "Labels")]
    pub labels: HashMap<String, String>,
    #[serde(rename = "SizeRW")]
    pub size_rw: Option<i64>,
    #[serde(rename = "SizeRootFS")]
    pub size_rootfs: Option<i64>,
    #[serde(rename = "HostConfig")]
    pub host_config: HostConfig,
    #[serde(rename = "NetworkSettings")]
    pub network_settings: NetworkSettings,
    #[serde(rename = "Mounts")]
    pub mounts: Vec<Mount>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerError {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Port {
    #[serde(rename = "IP")]
    pub ip: Option<String>,
    #[serde(rename = "PrivatePort")]
    pub private_port: u32,
    #[serde(rename = "PublicPort")]
    pub public_port: Option<u32>,
    #[serde(rename = "Type")]
    pub conn_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mount {
    #[serde(rename = "Destination")]
    pub destination: String,
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Driver")]
    pub driver: Option<String>,
    #[serde(rename = "Type")]
    pub mount_type: String,
    #[serde(rename = "Mode")]
    pub mode: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "RW")]
    pub rw: bool,
    #[serde(rename = "Propagation")]
    pub propagation: String,
    #[serde(rename = "VolumeOptions")]
    pub volume_options: Option<VolumeOptions>,
    #[serde(rename = "TmpfsOptions")]
    pub tmpfs_options: Option<TmpfsOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostConfig {
    #[serde(rename = "NetworkMode")]
    pub network_mode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkSettings {
    #[serde(rename = "Networks")]
    pub networks: HashMap<String, Network>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    #[serde(rename = "Links")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "Aliases")]
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "NetworkID")]
    pub network_id: String,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    #[serde(rename = "Gateway")]
    pub gateway: String,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: u32,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: i64,
    #[serde(rename = "MacAddress")]
    pub mac_address: String,
    #[serde(rename = "DriverOpts")]
    pub driver_opts: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BindOptions {
    #[serde(rename = "Propagation")]
    pub propagation: String,
    #[serde(rename = "NonRecursive")]
    pub non_recursive: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VolumeOptions {
    #[serde(rename = "NoCopy")]
    pub no_copy: bool,
    #[serde(rename = "Labels")]
    pub labels: HashMap<String, String>,
    #[serde(rename = "DriverConfig")]
    pub driver_config: DriverConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TmpfsOptions {
    #[serde(rename = "SizeBytes")]
    pub size_bytes: i64,
    #[serde(rename = "Mode")]
    pub mode: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriverConfig {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Options")]
    pub options: HashMap<String, String>,
}
