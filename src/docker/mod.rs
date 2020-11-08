mod structs;

use self::structs::{Container, DockerError};
use anyhow::Result;
use hyper::Client;
use hyperlocal::{UnixClientExt, Uri};
use serde_json;
use std::path::Path;

pub async fn list() -> Result<()> {
    let path = Path::new("/var/run/docker.sock");
    let url = Uri::new(path, "/v1.40/containers/json").into();

    let client = Client::unix();

    let res = client.get(url).await?;

    if res.status() == 200 {
        let buf = hyper::body::to_bytes(res).await?;
        let msg: Vec<Container> = serde_json::from_slice(&buf[..])?;
        println!("Body: {:?}", msg);
    } else {
        let buf = hyper::body::to_bytes(res).await?;
        let msg: Vec<DockerError> = serde_json::from_slice(&buf[..])?;
        println!("Body: {:?}", msg);
    }

    Ok(())
}
