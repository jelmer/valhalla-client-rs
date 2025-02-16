#![forbid(unsafe_code)]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

/// [`costing`] model-configuration for different transport modes
pub mod costing;
/// Models connected to the [`elevation`]-api
pub mod elevation;
/// Models connected to the Time-distance [`matrix`]-api
pub mod matrix;
/// Models connected to the Turn-by-turn [`route`]ing-api
pub mod route;
/// Shape decoding support for [`route`] and [`elevation`]
pub mod shapes;
/// Models connected to the healthcheck via the [`status`]-API
pub mod status;

use log::trace;
use serde::{Deserialize, Serialize};

/// A longitude, latitude coordinate in degrees
///
/// See <https://en.wikipedia.org/wiki/Geographic_coordinate_system> for further context
pub type Coordinate = (f32, f32);
impl From<Coordinate> for shapes::ShapePoint {
    fn from((lon, lat): Coordinate) -> Self {
        Self {
            lon: lon as f64,
            lat: lat as f64,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CodedDescription {
    pub code: u64,
    pub description: String,
}

/// valhalla needs `date_time` fields to be in the `YYYY-MM-DDTHH:MM` format
pub(crate) fn serialize_naive_date_time_opt<S>(
    value: &Option<chrono::NaiveDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        None => serializer.serialize_none(),
        Some(value) => serialize_naive_date_time(value, serializer),
    }
}

/// valhalla needs `date_time` fields to be in the `YYYY-MM-DDTHH:MM` format
fn serialize_naive_date_time<S>(
    value: &chrono::NaiveDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.format("%Y-%m-%dT%H:%M").to_string())
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Units {
    #[default]
    #[serde(rename = "kilometers")]
    Metric,

    #[serde(rename = "miles")]
    Imperial,
}
/// The local date and time at the location
#[derive(Serialize, Debug)]
pub struct DateTime {
    r#type: MatrixDateTimeType,
    #[serde(serialize_with = "serialize_naive_date_time")]
    value: chrono::NaiveDateTime,
}

impl DateTime {
    /// Current departure time
    pub fn from_current_departure_time() -> Self {
        Self {
            r#type: MatrixDateTimeType::CurrentDeparture,
            value: chrono::Local::now().naive_local(),
        }
    }
    /// Specified departure time
    pub fn from_departure_time(depart_after: chrono::NaiveDateTime) -> Self {
        Self {
            r#type: MatrixDateTimeType::SpecifiedDeparture,
            value: depart_after,
        }
    }
    /// Specified arrival time
    pub fn from_arrival_time(arrive_by: chrono::NaiveDateTime) -> Self {
        Self {
            r#type: MatrixDateTimeType::SpecifiedArrival,
            value: arrive_by,
        }
    }
}

#[derive(serde_repr::Serialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
enum MatrixDateTimeType {
    CurrentDeparture = 0,
    SpecifiedDeparture,
    SpecifiedArrival,
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

#[derive(Debug, Deserialize)]
pub struct RemoteError {
    pub error_code: isize,
    pub error: String,
    pub status_code: isize,
    pub status: String,
}

/// synchronous ("blocking") client implementation
#[cfg(feature = "blocking")]
pub mod blocking {
    use crate::{elevation, matrix, route, status, Error, VALHALLA_PUBLIC_API_URL};
    use std::sync::Arc;

    #[derive(Debug, Clone)]
    pub struct Valhalla {
        runtime: Arc<tokio::runtime::Runtime>,
        client: super::Valhalla,
    }
    impl Valhalla {
        /// Create a sync [Valhalla](https://valhalla.github.io/valhalla/) client
        pub fn new(base_url: url::Url) -> Self {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .build()
                .expect("tokio runtime can be created");
            Self {
                runtime: Arc::new(runtime),
                client: super::Valhalla::new(base_url),
            }
        }

        /// Make a turn-by-turn routing request
        ///
        /// See <https://valhalla.github.io/valhalla/api/turn-by-turn/api-reference> for details
        ///
        /// # Example:
        /// ```rust,no_run
        /// use valhalla_client::blocking::Valhalla;
        /// use valhalla_client::route::{Location, Manifest,};
        /// use valhalla_client::costing::Costing;
        ///
        /// let amsterdam = Location::new(4.9041, 52.3676);
        /// let utrecht = Location::new(5.1214, 52.0907);
        ///
        /// let manifest = Manifest::builder()
        ///   .locations([utrecht,amsterdam])
        ///   .alternates(2)
        ///   .costing(Costing::Auto(Default::default()))
        ///   .language("de-De");
        ///
        /// let response = Valhalla::default().route(manifest).unwrap();
        /// # use valhalla_client::matrix::Response;
        /// # assert!(response.warnings.is_none());
        /// # assert_eq!(response.locations.len(), 2);
        /// ```
        pub fn route(&self, manifest: route::Manifest) -> Result<route::Trip, Error> {
            self.runtime
                .block_on(async move { self.client.route(manifest).await })
        }
        /// Make a time-distance matrix routing request
        ///
        /// See <https://valhalla.github.io/valhalla/api/matrix/api-reference> for details
        ///
        /// # Example:
        /// ```rust,no_run
        /// use valhalla_client::blocking::Valhalla;
        /// use valhalla_client::matrix::{DateTime, Location, Manifest,};
        /// use valhalla_client::costing::Costing;
        ///
        /// let amsterdam = Location::new(4.9041, 52.3676);
        /// let utrecht = Location::new(5.1214, 52.0907);
        /// let rotterdam = Location::new(4.4775302894411, 51.92485867761482);
        /// let den_haag = Location::new(4.324908478055228, 52.07934071633195);
        ///
        /// let manifest = Manifest::builder()
        ///   .verbose_output(true)
        ///   .sources_to_targets([utrecht],[amsterdam,rotterdam,den_haag])
        ///   .date_time(DateTime::from_departure_time(chrono::Local::now().naive_local()))
        ///   .costing(Costing::Auto(Default::default()));
        ///
        /// let response = Valhalla::default()
        ///   .matrix(manifest)
        ///   .unwrap();
        /// # use valhalla_client::matrix::Response;
        /// # if let Response::Verbose(r) = response{
        /// #   assert!(r.warnings.is_empty());
        /// #   assert_eq!(r.sources.len(),1);
        /// #   assert_eq!(r.targets.len(),3);
        /// # };
        /// ```
        pub fn matrix(&self, manifest: matrix::Manifest) -> Result<matrix::Response, Error> {
            self.runtime
                .block_on(async move { self.client.matrix(manifest).await })
        }
        /// Make an elevation request
        ///
        /// Valhalla's elevation lookup service provides digital elevation model (DEM) data as the result of a query.
        /// The elevation service data has many applications when combined with other routing and navigation data, including computing the steepness of roads and paths or generating an elevation profile chart along a route.
        ///
        /// For example, you can get elevation data for a point, a trail, or a trip.
        /// You might use the results to consider hills for your bicycle trip, or when estimating battery usage for trips in electric vehicles.
        ///
        /// See <https://valhalla.github.io/valhalla/api/elevation/api-reference/> for details
        ///
        /// # Example:
        ///
        /// ```rust,no_run
        /// use valhalla_client::blocking::Valhalla;
        /// use valhalla_client::elevation::Manifest;
        ///
        /// let request = Manifest::builder()
        ///   .shape([
        ///     (40.712431, -76.504916),
        ///     (40.712275, -76.605259),
        ///     (40.712122, -76.805694),
        ///     (40.722431, -76.884916),
        ///     (40.812275, -76.905259),
        ///     (40.912122, -76.965694),
        ///   ])
        ///   .include_range();
        /// let response = Valhalla::default()
        ///   .elevation(request).unwrap();
        /// # assert!(response.height.is_empty());
        /// # assert_eq!(response.range_height.len(), 6);
        /// # assert!(response.encoded_polyline.is_none());
        /// # assert!(response.warnings.is_empty());
        /// # assert_eq!(response.x_coordinate, None);
        /// # assert_eq!(response.y_coordinate, None);
        /// # assert_eq!(response.shape.map(|s|s.len()),Some(6));
        /// ```
        pub fn elevation(
            &self,
            manifest: elevation::Manifest,
        ) -> Result<elevation::Response, Error> {
            self.runtime
                .block_on(async move { self.client.elevation(manifest).await })
        }
        /// Make a status request
        ///
        /// This can be used as a health endpoint for the HTTP API or to toggle features in a frontend.
        ///
        /// See <https://valhalla.github.io/valhalla/api/status/api-reference/> for details
        ///
        /// # Example:
        /// ```rust,no_run
        /// use valhalla_client::blocking::Valhalla;
        /// use valhalla_client::status::Manifest;
        ///
        /// let request = Manifest::builder()
        ///   .verbose_output(false);
        /// let response = Valhalla::default()
        ///   .status(request).unwrap();
        /// # assert!(response.version >= semver::Version::parse("3.1.4").unwrap());
        /// # assert!(response.tileset_last_modified.timestamp() > 0);
        /// # assert!(response.verbose.is_none());
        /// ```
        pub fn status(&self, manifest: status::Manifest) -> Result<status::Response, Error> {
            self.runtime
                .block_on(async move { self.client.status(manifest).await })
        }
    }
    impl Default for Valhalla {
        fn default() -> Self {
            Self::new(
                url::Url::parse(VALHALLA_PUBLIC_API_URL)
                    .expect("VALHALLA_PUBLIC_API_URL is not a valid url"),
            )
        }
    }
}

const VALHALLA_PUBLIC_API_URL: &str = "https://valhalla1.openstreetmap.de/";
#[derive(Debug, Clone)]
pub struct Valhalla {
    client: reqwest::Client,
    base_url: url::Url,
}

impl Valhalla {
    /// Create an async [Valhalla](https://valhalla.github.io/valhalla/) client
    pub fn new(base_url: url::Url) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }

    /// Make a turn-by-turn routing request
    ///
    /// See <https://valhalla.github.io/valhalla/api/turn-by-turn/api-reference> for details
    ///
    /// # Example:
    /// ```rust
    /// # async fn route(){
    /// use valhalla_client::Valhalla;
    /// use valhalla_client::route::{Location, Manifest,};
    /// use valhalla_client::costing::Costing;
    ///
    /// let amsterdam = Location::new(4.9041, 52.3676);
    /// let utrecht = Location::new(5.1214, 52.0907);
    ///
    /// let manifest = Manifest::builder()
    ///   .locations([utrecht,amsterdam])
    ///   .alternates(2)
    ///   .costing(Costing::Auto(Default::default()))
    ///   .language("de-De");
    ///
    /// let response = Valhalla::default().route(manifest).await.unwrap();
    /// # assert!(response.warnings.is_none());
    /// # assert_eq!(response.locations.len(), 2);
    /// # }
    /// ```
    pub async fn route(&self, manifest: route::Manifest) -> Result<route::Trip, Error> {
        let response: route::Response = self.do_request(manifest, "route", "route").await?;
        Ok(response.trip)
    }

    /// Make a time-distance matrix routing request
    ///
    /// See <https://valhalla.github.io/valhalla/api/matrix/api-reference> for details
    ///
    /// # Example:
    /// ```rust
    /// # async fn matrix(){
    /// use valhalla_client::Valhalla;
    /// use valhalla_client::matrix::{DateTime, Location, Manifest,};
    /// use valhalla_client::costing::Costing;
    ///
    /// let amsterdam = Location::new(4.9041, 52.3676);
    /// let utrecht = Location::new(5.1214, 52.0907);
    /// let rotterdam = Location::new(4.4775302894411, 51.92485867761482);
    /// let den_haag = Location::new(4.324908478055228, 52.07934071633195);
    ///
    /// let manifest = Manifest::builder()
    ///   .verbose_output(true)
    ///   .sources_to_targets([utrecht],[amsterdam,rotterdam,den_haag])
    ///   .date_time(DateTime::from_departure_time(chrono::Local::now().naive_local()))
    ///   .costing(Costing::Auto(Default::default()));
    ///
    /// let response = Valhalla::default()
    ///   .matrix(manifest).await
    ///   .unwrap();
    /// # use valhalla_client::matrix::Response;
    /// # if let Response::Verbose(r) = response{
    /// #   assert!(r.warnings.is_empty());
    /// #   assert_eq!(r.sources.len(),1);
    /// #   assert_eq!(r.targets.len(),3);
    /// # };
    /// # }
    /// ```
    pub async fn matrix(&self, manifest: matrix::Manifest) -> Result<matrix::Response, Error> {
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

        self.do_request(manifest, "sources_to_targets", "matrix")
            .await
    }
    /// Make an elevation request
    ///
    /// Valhalla's elevation lookup service provides digital elevation model (DEM) data as the result of a query.
    /// The elevation service data has many applications when combined with other routing and navigation data, including computing the steepness of roads and paths or generating an elevation profile chart along a route.
    ///
    /// For example, you can get elevation data for a point, a trail, or a trip.
    /// You might use the results to consider hills for your bicycle trip, or when estimating battery usage for trips in electric vehicles.
    ///
    /// See <https://valhalla.github.io/valhalla/api/elevation/api-reference/> for details
    ///
    /// # Example:
    ///
    /// ```rust,no_run
    /// # async fn elevation() {
    /// use valhalla_client::Valhalla;
    /// use valhalla_client::elevation::Manifest;
    ///
    /// let request = Manifest::builder()
    ///   .shape([
    ///     (40.712431, -76.504916),
    ///     (40.712275, -76.605259),
    ///     (40.712122, -76.805694),
    ///     (40.722431, -76.884916),
    ///     (40.812275, -76.905259),
    ///     (40.912122, -76.965694),
    ///   ])
    ///   .include_range();
    /// let response = Valhalla::default()
    ///   .elevation(request).await.unwrap();
    /// # assert!(response.height.is_empty());
    /// # assert_eq!(response.range_height.len(), 6);
    /// # assert!(response.encoded_polyline.is_none());
    /// # assert!(response.warnings.is_empty());
    /// # assert_eq!(response.x_coordinate, None);
    /// # assert_eq!(response.y_coordinate, None);
    /// # assert_eq!(response.shape.map(|s|s.len()),Some(6));
    /// # }
    /// ```
    pub async fn elevation(
        &self,
        manifest: elevation::Manifest,
    ) -> Result<elevation::Response, Error> {
        self.do_request(manifest, "height", "elevation").await
    }
    /// Make a status request
    ///
    /// This can be used as a health endpoint for the HTTP API or to toggle features in a frontend.
    ///
    /// See <https://valhalla.github.io/valhalla/api/status/api-reference/> for details
    ///
    /// # Example:
    /// ```rust,no_run
    /// # async fn status(){
    /// use valhalla_client::Valhalla;
    /// use valhalla_client::status::Manifest;
    ///
    /// let request = Manifest::builder()
    ///   .verbose_output(false);
    /// let response = Valhalla::default()
    ///   .status(request).await.unwrap();
    /// # assert!(response.version >= semver::Version::parse("3.1.4").unwrap());
    /// # assert!(response.tileset_last_modified.timestamp() > 0);
    /// # assert!(response.verbose.is_none());
    /// # }
    /// ```
    pub async fn status(&self, manifest: status::Manifest) -> Result<status::Response, Error> {
        self.do_request(manifest, "status", "status").await
    }

    async fn do_request<Resp: for<'de> serde::Deserialize<'de>>(
        &self,
        manifest: impl serde::Serialize,
        path: &'static str,
        name: &'static str,
    ) -> Result<Resp, Error> {
        if log::log_enabled!(log::Level::Trace) {
            let request = serde_json::to_string(&manifest).unwrap();
            trace!("Sending {name} request: {request}");
        }
        let mut url = self.base_url.clone();
        url.path_segments_mut()
            .expect("base_url is not a valid base url")
            .push(path);
        let response = self
            .client
            .post(url)
            .json(&manifest)
            .send()
            .await
            .map_err(Error::Reqwest)?;
        if response.status().is_client_error() {
            return Err(Error::RemoteError(
                response.json().await.map_err(Error::Reqwest)?,
            ));
        }
        response.error_for_status_ref().map_err(Error::Reqwest)?;
        let text = response.text().await.map_err(Error::Reqwest)?;
        trace!("{name} responded: {text}");
        let response: Resp = serde_json::from_str(&text).map_err(Error::Serde)?;
        Ok(response)
    }
}

impl Default for Valhalla {
    fn default() -> Self {
        Self::new(
            url::Url::parse(VALHALLA_PUBLIC_API_URL)
                .expect("VALHALLA_PUBLIC_API_URL is not a valid url"),
        )
    }
}
