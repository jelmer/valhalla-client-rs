#![forbid(unsafe_code)]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

pub mod costing;
pub mod matrix;
pub mod route;
pub mod shapes;

use log::debug;
use serde::{Deserialize, Serialize};

/// A longitude, latitude coordinate in degrees
///
/// See <https://en.wikipedia.org/wiki/Geographic_coordinate_system> for further context
pub type Coordinate = (f32, f32);

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

/// valhalla needs `date_time` fields to be in the `YYYY-MM-DDTHH:MM` format
pub(crate) fn serialize_naive_date_time_opt<S>(value:&Option<chrono::NaiveDateTime>,serializer: S)-> Result<S::Ok, S::Error> where S: serde::Serializer{
    match value { 
        None=> serializer.serialize_none(),
        Some(value)=>serialize_naive_date_time(value,serializer)
    }
}

/// valhalla needs `date_time` fields to be in the `YYYY-MM-DDTHH:MM` format
pub(crate) fn serialize_naive_date_time<S>(value:&chrono::NaiveDateTime,serializer: S)-> Result<S::Ok, S::Error> where S: serde::Serializer{
    serializer.serialize_str(&value.format("%Y-%m-%dT%H:%M").to_string())
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub enum Units {
    #[default]
    #[serde(rename = "kilometers")]
    Metric,

    #[serde(rename = "miles")]
    Imperial,
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

const VALHALLA_PUBLIC_API_URL: &str = "https://valhalla1.openstreetmap.de/";
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

    /// Make a turn-by-turn routing request
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
        let response: route::Response = serde_json::from_str(&text).map_err(Error::Serde)?;
        Ok(response.trip)
    }
    /// Make a time-distance matrix routing request
    ///
    /// See <https://valhalla.github.io/valhalla/api/matrix/api-reference> for details
    ///
    /// # Example:
    /// ```rust,no_run
    /// use chrono::Local;
    /// use valhalla_client::Valhalla;
    /// use valhalla_client::matrix::{DateTime, Location, Manifest};
    /// use valhalla_client::costing::Costing;
    ///
    /// let amsterdam = Location::new(4.9041, 52.3676);
    /// let utrecht = Location::new(5.1214, 52.0907);
    /// let rotterdam = Location::new(4.4775302894411, 51.92485867761482);
    /// let den_haag = Location::new(4.324908478055228, 52.07934071633195);
    ///
    /// let manifest = Manifest::builder()
    ///   .sources_to_targets([utrecht],[amsterdam,rotterdam,den_haag])
    ///   .date_time(DateTime::from_departure_time(Local::now().naive_local()))
    ///   .costing(Costing::Auto(Default::default()));
    ///
    /// let response = Valhalla::default()
    ///   .matrix(manifest)
    ///   .unwrap();
    /// # assert!(response.warnings.is_empty());
    /// # assert_eq!(response.sources.unwrap().len(),1);
    /// # assert_eq!(response.targets.unwrap().len(),3);
    /// ```
    pub fn matrix(&self, manifest: matrix::Manifest) -> Result<matrix::Response, Error> {
        debug_assert_ne!(
            manifest.targets.len(),
            0,
            "a matrix route needs at least one target specified"
        );
        debug_assert_ne!(
            manifest.sources.len(),
            0,
            "a matrix route needs at least one source specified"
        );

        debug!(
            "Sending routing request: {}",
            serde_json::to_string(&manifest).unwrap()
        );
        let mut url = self.base_url.clone();
        url.path_segments_mut()
            .expect("base_url is not a valid base url")
            .push("sources_to_targets");
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
        let response: matrix::Response = serde_json::from_str(&text).map_err(Error::Serde)?;
        Ok(response)
    }
}
