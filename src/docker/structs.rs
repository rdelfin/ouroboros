use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Names")]
    names: Vec<String>,
    #[serde(rename = "ImageID")]
    image_id: String,
    #[serde(rename = "Image")]
    image: Option<String>,
    #[serde(rename = "Command")]
    command: String,
    #[serde(rename = "Created")]
    created: i64,
    #[serde(rename = "State")]
    state: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "Ports")]
    ports: Vec<Port>,
    #[serde(rename = "Labels")]
    labels: HashMap<String, String>,
    #[serde(rename = "SizeRW")]
    size_rw: Option<i64>,
    #[serde(rename = "SizeRootFS")]
    size_rootfs: Option<i64>,
    #[serde(rename = "HostConfig")]
    host_config: HostConfig,
    #[serde(rename = "NetworkSettings")]
    network_settings: NetworkSettings,
    #[serde(rename = "Mounts")]
    mounts: Vec<Mount>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerError {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Port {
    #[serde(rename = "IP")]
    ip: Option<String>,
    #[serde(rename = "PrivatePort")]
    private_port: u32,
    #[serde(rename = "PublicPort")]
    public_port: Option<u32>,
    #[serde(rename = "Type")]
    conn_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mount {
    #[serde(rename = "Destination")]
    destination: String,
    #[serde(rename = "Source")]
    source: String,
    #[serde(rename = "Driver")]
    driver: Option<String>,
    #[serde(rename = "Type")]
    mount_type: String,
    #[serde(rename = "Mode")]
    mode: String,
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "RW")]
    rw: bool,
    #[serde(rename = "Propagation")]
    propagation: String,
    #[serde(rename = "VolumeOptions")]
    volume_options: Option<VolumeOptions>,
    #[serde(rename = "TmpfsOptions")]
    tmpfs_options: Option<TmpfsOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostConfig {
    #[serde(rename = "NetworkMode")]
    network_mode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkSettings {
    #[serde(rename = "Networks")]
    networks: HashMap<String, Network>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    #[serde(rename = "Links")]
    links: Option<Vec<String>>,
    #[serde(rename = "Aliases")]
    aliases: Option<Vec<String>>,
    #[serde(rename = "NetworkID")]
    network_id: String,
    #[serde(rename = "EndpointID")]
    endpoint_id: String,
    #[serde(rename = "Gateway")]
    gateway: String,
    #[serde(rename = "IPAddress")]
    ip_address: String,
    #[serde(rename = "IPPrefixLen")]
    ip_prefix_len: u32,
    #[serde(rename = "IPv6Gateway")]
    ipv6_gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    global_ipv6_address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    global_ipv6_prefix_len: i64,
    #[serde(rename = "MacAddress")]
    mac_address: String,
    #[serde(rename = "DriverOpts")]
    driver_opts: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BindOptions {
    #[serde(rename = "Propagation")]
    propagation: String,
    #[serde(rename = "NonRecursive")]
    non_recursive: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VolumeOptions {
    #[serde(rename = "NoCopy")]
    no_copy: bool,
    #[serde(rename = "Labels")]
    labels: HashMap<String, String>,
    #[serde(rename = "DriverConfig")]
    driver_config: DriverConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TmpfsOptions {
    #[serde(rename = "SizeBytes")]
    size_bytes: i64,
    #[serde(rename = "Mode")]
    mode: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriverConfig {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Options")]
    options: HashMap<String, String>,
}
