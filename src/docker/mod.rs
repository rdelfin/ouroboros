use std::path::Path;

use anyhow::Result;
use hyper::Client;
use hyperlocal::{UnixClientExt, Uri};

pub async fn list() -> Result<()> {
    let path = Path::new("/var/run/docker.sock");
    let url = Uri::new(path, "/v1.40/containers/json").into();

    let client = Client::unix();

    let res = client.get(url).await?;

    println!("Status: {}", res.status());

    let buf = hyper::body::to_bytes(res).await?;

    println!("Body: {:?}", buf);

    Ok(())
}
