mod structs;

use self::structs::{Container, DockerErrorBody};
use bytes::buf::BufExt;
use hyper::{Client, Error as HyperError};
use hyperlocal::{UnixClientExt, Uri};
use serde::de;
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
    call_api::<Vec<Container>>("/v1.40/containers/json").await
}

async fn call_api<'a, T: de::DeserializeOwned>(url_path: &str) -> Result<T, DockerError> {
    let path = Path::new("/var/run/docker.sock");
    let url = Uri::new(path, url_path).into();

    let client = Client::unix();

    let res = client.get(url).await?;
    let status = res.status();
    let body = hyper::body::aggregate(res).await?;

    if status == 200 {
        Ok(serde_json::from_reader(body.reader())?)
    } else {
        Err(DockerError::APIError(serde_json::from_reader(
            body.reader(),
        )?))
    }
}
