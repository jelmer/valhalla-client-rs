#![forbid(unsafe_code)]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

pub mod route;
pub mod shapes;

use log::debug;
use serde::Deserialize;

const VALHALLA_PUBLIC_API_URL: &str = "https://valhalla1.openstreetmap.de/";

pub struct Valhalla {
    client: reqwest::blocking::Client,
    base_url: url::Url,
}

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Url(url::ParseError),
    Serde(serde_json::Error),
    RemoteError(RemoteError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Reqwest(e) => write!(f, "reqwest error: {e}"),
            Self::Url(e) => write!(f, "url error: {e}"),
            Self::Serde(e) => write!(f, "serde error: {e}"),
            Self::RemoteError(e) => write!(f, "remote error: {e:?}"),
        }
    }
}

impl std::error::Error for Error {}

impl Default for Valhalla {
    fn default() -> Self {
        Self::new(
            url::Url::parse(VALHALLA_PUBLIC_API_URL)
                .expect("VALHALLA_PUBLIC_API_URL is not a valid url"),
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct RemoteError {
    pub error_code: isize,
    pub error: String,
    pub status_code: isize,
    pub status: String,
}

impl Valhalla {
    pub fn new(base_url: url::Url) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            base_url,
        }
    }

    /// Make a routing request
    ///
    /// See <https://valhalla.github.io/valhalla/api/turn-by-turn/api-reference> for details
    pub fn route(&self, manifest: route::Manifest) -> Result<route::Trip, Error> {
        debug!(
            "Sending routing request: {}",
            serde_json::to_string(&manifest).unwrap()
        );
        let mut url = self.base_url.clone();
        url.path_segments_mut()
            .expect("base_url is not a valid base url")
            .push("route");
        let response = self
            .client
            .post(url)
            .json(&manifest)
            .send()
            .map_err(Error::Reqwest)?;
        if response.status().is_client_error() {
            return Err(Error::RemoteError(response.json().map_err(Error::Reqwest)?));
        }
        response.error_for_status_ref().map_err(Error::Reqwest)?;
        let text = response.text().map_err(Error::Reqwest)?;
        // let route: Trip = response.json().map_err(Error::Reqwest)?;
        let response: route::Response = serde_json::from_str(&text).map_err(Error::Serde)?;
        Ok(response.trip)
    }
}
