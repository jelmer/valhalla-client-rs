use crate::costing;
use crate::shapes::ShapeFormat;
pub use crate::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Default, Debug)]
/// Matrix request
pub struct Manifest {
    pub(crate) targets: Vec<Location>,
    pub(crate) sources: Vec<Location>,
    #[serde(flatten)]
    costing: costing::Costing,
    id: Option<String>,
    matrix_locations: Option<u32>,
    date_time: Option<DateTime>,
    verbose: Option<bool>,
    shape_format: Option<ShapeFormat>,
}
impl Manifest {
    /// Create a builder for the matrix request
    pub fn builder() -> Self {
        Default::default()
    }
    /// Sets the source and targets of the matrix
    pub fn sources_to_targets(
        mut self,
        sources: impl IntoIterator<Item = Location>,
        targets: impl IntoIterator<Item = Location>,
    ) -> Self {
        self.sources = sources.into_iter().collect();
        self.targets = targets.into_iter().collect();
        self
    }
    /// Configures the costing model
    ///
    /// Valhalla's routing service uses dynamic, run-time costing to generate the route path.
    /// Can be configured with different settings depending on the costing model used.
    ///
    /// **Note:** multimodal costing is not supported for the time-distance matrix service at this time.
    ///
    /// Default: [`costing::Costing::Auto`]
    pub fn costing(mut self, costing: costing::Costing) -> Self {
        self.costing = costing;
        self
    }
    /// Name your route request.
    ///
    /// If id is specified, the naming will be sent through to the response.
    pub fn id(mut self, id: impl ToString) -> Self {
        self.id = Some(id.to_string());
        self
    }
    /// Sets the minimum number of locations that need to be found satisfying the request
    ///
    /// Allows a partial result to be returned.
    ///
    /// This is basically equivalent to:
    /// > "find the closest or best N locations out of the full location set"
    pub fn minimum_matrix_locations_count(mut self, count: u32) -> Self {
        self.matrix_locations = Some(count);
        self
    }
    /// Shortcut for configuring the arrival/departure date_time settings globally
    /// instead of specifying it for each source/target.
    ///
    /// See [`Location::date_time`] if you want a more granular API.
    /// Valhalla will translate this to setting the value on all `source` locations when
    /// [`DateTime::from_current_departure_time`] or [`DateTime::from_departure_time`] is used and
    /// on all `target` locations when [`DateTime::from_arrival_time`].
    ///
    /// **Note:**
    /// There are important limitations to time awareness.
    /// Due to algorithmic complexity, we disallow time-dependence for certain combinations
    /// of date_time on locations:
    /// - when there are more sources than `target`s:
    ///   - [`Location::date_time`] on any `source`
    ///   - using [`Self::date_time`] with [`DateTime::from_current_departure_time`] and [`DateTime::from_departure_time`]
    /// - when there's more or equal amount of `target`s than/as `source`s
    ///   - [`Location::date_time`] on any `target`
    ///   - [`DateTime::from_arrival_time`]
    pub fn date_time(mut self, date_time: DateTime) -> Self {
        self.date_time = Some(date_time);
        self
    }
    /// Modifies the verbosity of the output:
    /// - `true` will output a flat list of objects for distances & durations explicitly specifying
    ///   the `source` & `target` indices.
    /// - `false` will return more compact, nested row-major distances & durations arrays and
    ///   not echo `sources` and `targets`
    ///
    /// Default: `true`
    pub fn verbose_output(mut self, verbose: bool) -> Self {
        self.verbose = Some(verbose);
        self
    }
    /// Specifies the [`ShapeFormat`] for the path shape of each connection.
    pub fn shape_format(mut self, shape_format: ShapeFormat) -> Self {
        self.shape_format = Some(shape_format);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Default, Clone, Copy, PartialEq, Debug)]
/// Location of a point on the map
pub struct Location {
    lat: f32,
    lon: f32,
    #[serde(serialize_with = "super::serialize_naive_date_time_opt")]
    date_time: Option<chrono::NaiveDateTime>,
}
impl From<super::Coordinate> for Location {
    fn from((longitude, latitude): super::Coordinate) -> Self {
        Self {
            lat: latitude,
            lon: longitude,
            date_time: None,
        }
    }
}
impl Location {
    /// Creates a new location from a longitude/latitude
    pub fn new(longitude: f32, latitude: f32) -> Self {
        Self::from((longitude, latitude))
    }
    /// Expected date/time for the user to be at the location in the local time zone of departure or arrival.
    ///
    /// Offers more granularity over setting time than the global [`Manifest::date_time`].
    ///
    /// **Note:**
    /// This behaves differently for the matrix in comparison to the route API:
    /// - If set on the sources and there's more targets than sources,
    ///   it will behave like a *"Specified departure time"* on the sources.
    /// - If set on the targets and there's less targets than sources,
    ///   it will behave like a *"Specified arrival time"* on the targets.
    ///
    /// **Note:**
    /// There are important limitations to time awareness.
    /// Due to algorithmic complexity, we disallow time-dependence for certain combinations
    /// of date_time on locations:
    /// - when there are more sources than `target`s:
    ///   - [`Location::date_time`] on any `source`
    ///   - using [`Self::date_time`] with [`DateTime::from_current_departure_time`] and [`DateTime::from_departure_time`]
    /// - when there's more or equal amount of `target`s than/as `source`s
    ///   - [`Location::date_time`] on any `target`
    ///   - [`DateTime::from_arrival_time`]
    pub fn date_time(mut self, date_time: chrono::NaiveDateTime) -> Self {
        self.date_time = Some(date_time);
        self
    }
}

/// [`Location`] which was configured in the input
///
/// Present only in `verbose` mode. Verbosity can be set via [`Manifest::verbose_output`]
#[derive(Deserialize, Default, Clone, Copy, PartialEq, Debug)]
pub struct VerboseLocation {
    /// Latitude as defined in [`super::Coordinate`]
    pub lat: f32,
    /// Longitude as defined in [`super::Coordinate`]
    pub lon: f32,
    /// time configured via [`Location::date_time`]
    pub date_time: Option<chrono::NaiveDateTime>,
}

impl From<Location> for VerboseLocation {
    fn from(value: Location) -> Self {
        Self {
            lat: value.lat,
            lon: value.lon,
            date_time: value.date_time,
        }
    }
}

impl From<VerboseLocation> for Location {
    fn from(value: VerboseLocation) -> Self {
        Self {
            lat: value.lat,
            lon: value.lon,
            date_time: value.date_time,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Response to the matrix request
pub enum Response {
    /// Returned in `verbose` mode.
    ///
    /// Verbosity can be set via [`Manifest::verbose_output`]
    Verbose(VerboseResponse),
    /// Returned in non-`verbose` mode.
    ///
    /// Verbosity can be set via [`Manifest::verbose_output`]
    Concise(ConciseResponse),
}
#[derive(Deserialize, Debug, Clone)]
/// Verbose response to the matrix request
pub struct VerboseResponse {
    /// Name of the route request.
    ///
    /// If id is specified via [`Manifest::id`] the naming will be sent through to the response.
    pub id: Option<String>,
    /// Algorithm used
    pub algorithm: String,
    /// Distance units for output.
    ///
    /// Possible unit types are miles via [`super::Units::Imperial`] and kilometers via [`super::Units::Metric`].
    ///
    /// Default: [`super::Units::Metric`]
    pub units: super::Units,
    /// This array may contain warning objects informing about deprecated request parameters, clamped values etc.
    #[serde(default = "Vec::new")]
    pub warnings: Vec<Value>,
    /// The sources of the matrix
    pub sources: Vec<VerboseLocation>,
    /// The targets of the matrix
    pub targets: Vec<VerboseLocation>,
    /// A flat list of objects for distances & durations explicitly specifying the `source` & `target` indices.
    ///
    /// The arrays rows are:
    /// - time and distance from the first source location to all target locations,
    /// - time and distance from the second source location to all target locations,
    /// - etc.
    pub sources_to_targets: Vec<Vec<VerboseSourceToTarget>>,
}
#[derive(Deserialize, Debug, Clone)]
/// Concise response to the matrix request
pub struct ConciseResponse {
    /// Name of the route request.
    ///
    /// If id is specified via [`Manifest::id`] the naming will be sent through to the response.
    pub id: Option<String>,
    /// Algorithm used
    pub algorithm: String,
    /// Distance units for output.
    ///
    /// Possible unit types are miles via [`super::Units::Imperial`] and kilometers via [`super::Units::Metric`].
    ///
    /// Default: [`super::Units::Metric`]
    pub units: super::Units,
    /// This array may contain warning objects informing about deprecated request parameters, clamped values etc.
    #[serde(default = "Vec::new")]
    pub warnings: Vec<Value>,
    /// More compact, nested row-major distances & durations
    ///
    /// The arrays rows are:
    /// - time and distance from the first source location to all target locations,
    /// - time and distance from the second source location to all target locations,
    /// - etc.
    pub sources_to_targets: ConciseSourceToTargets,
}

#[derive(Deserialize, Debug, Clone)]
/// Concise source to target
pub struct ConciseSourceToTargets {
    /// The computed time between each set of points.
    ///
    /// Time will always be `0` for
    /// - the first element of the time-distance array for one_to_many,
    /// - the last element in a many_to_one, and
    /// - the first and last elements of a many_to_many
    pub durations: Vec<Vec<u32>>,
    /// The computed distance between each set of points.
    ///
    /// Distance will always be `0.00` for
    /// - the first element of the time-distance array for one_to_many,
    /// - the last element in a many_to_one, and
    /// - the first and last elements of a many_to_many.
    pub distances: Vec<Vec<f32>>,
}

#[derive(Deserialize, Debug, Clone)]
/// Verbose source to target
pub struct VerboseSourceToTarget {
    /// The computed distance between each set of points.
    ///
    /// Distance will always be `0.00` for
    /// - the first element of the time-distance array for one_to_many,
    /// - the last element in a many_to_one, and
    /// - the first and last elements of a many_to_many.
    pub distance: f32,
    /// The computed time between each set of points.
    ///
    /// Time will always be `0` for
    /// - the first element of the time-distance array for one_to_many,
    /// - the last element in a many_to_one, and
    /// - the first and last elements of a many_to_many
    pub time: u32,
    /// The destination index into the locations array
    pub from_index: usize,
    /// The origin index into the locations array
    pub to_index: usize,
    /// Timezone of when a user will arrive at this location or has to depart from the start point.
    ///
    /// For information how to differentiate between departure/arrival time, please see [`Manifest::date_time`] or [`Location::date_time`].
    ///
    /// This field is included only if:
    /// - valhalla is build with timezone support,
    /// - the time is below the settings `max_timedep_distance_matrix` or `max_timedep_distance`
    /// - departure/arrival time is unspecified via [`Manifest::date_time`] or [`Location::date_time`]
    ///
    /// Example: `"Europe/Berlin"`
    pub time_zone_name: Option<String>,
    /// Timezone of when a user will arrive at this location or has to depart from the start point.
    ///
    /// For information how to differentiate between departure/arrival time, please see [`Manifest::date_time`] or [`Location::date_time`].
    ///
    /// This field is included only if:
    /// - valhalla is build with timezone support,
    /// - the time is below the settings `max_timedep_distance_matrix` or `max_timedep_distance`
    /// - departure/arrival time is unspecified via [`Manifest::date_time`] or [`Location::date_time`]
    ///
    /// Example: `"+01:00"`
    pub time_zone_offset: Option<String>,
    /// When a user will arrive at this location or has to depart from the start point.
    ///
    /// For information how to differentiate between departure/arrival time, please see [`Manifest::date_time`] or [`Location::date_time`].
    ///
    /// This field is included only if:
    /// - the time is below the settings `max_timedep_distance_matrix` or `max_timedep_distance`
    /// - departure/arrival time is unspecified via [`Manifest::date_time`] or [`Location::date_time`]
    ///
    /// Example: `"2024-11-07T15:26"`
    pub date_time: Option<chrono::NaiveDateTime>,
}
