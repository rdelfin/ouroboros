mod structs;

use self::structs::{Container, DockerErrorBody};
use hyper::{Client, Error as HyperError};
use hyperlocal::{UnixClientExt, Uri};
use serde_json::Error as JsonError;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DockerError {
    #[error("Error thrown by the API: {0:?}")]
    APIError(DockerErrorBody),
    #[error("Error parsing JSON {0:?}")]
    JSONError(#[from] JsonError),
    #[error("HTTP Error {0:?}")]
    HTTPError(#[from] HyperError),
}

pub async fn list() -> Result<Vec<Container>, DockerError> {
    let path = Path::new("/var/run/docker.sock");
    let url = Uri::new(path, "/v1.40/containers/json").into();

    let client = Client::unix();

    let res = client.get(url).await?;

    if res.status() == 200 {
        let buf = hyper::body::to_bytes(res).await?;
        Ok(serde_json::from_slice::<Vec<Container>>(&buf[..])?)
    } else {
        let buf = hyper::body::to_bytes(res).await?;
        Err(DockerError::APIError(serde_json::from_slice::<
            DockerErrorBody,
        >(&buf[..])?))
    }
}
