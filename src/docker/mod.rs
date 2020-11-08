mod structs;

use self::structs::{Container, DockerErrorBody};
use bytes::buf::BufExt;
use hyper::{Body, Client, Error as HyperError};
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
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

pub struct DockerClient {
    client: Client<UnixConnector, Body>,
    sock_path: String,
}

impl Default for DockerClient {
    fn default() -> Self {
        Self::new("/var/run/docker.sock")
    }
}

impl DockerClient {
    pub fn new(sock_path: &str) -> Self {
        DockerClient {
            client: Client::unix(),
            sock_path: sock_path.into(),
        }
    }

    pub async fn list(&self) -> Result<Vec<Container>, DockerError> {
        self.call_api::<Vec<Container>>("/v1.40/containers/json")
            .await
    }

    async fn call_api<T: de::DeserializeOwned>(&self, url_path: &str) -> Result<T, DockerError> {
        let url = Uri::new(Path::new(&self.sock_path), url_path).into();

        let res = self.client.get(url).await?;
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
}
