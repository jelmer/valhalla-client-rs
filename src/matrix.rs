use crate::costing;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::shapes::ShapeFormat;

#[derive(Serialize, Default, Debug)]
pub struct Manifest {
    pub(crate) targets: Vec<Location>,
    pub(crate) sources: Vec<Location>,
    #[serde(flatten)]
    costing: costing::Costing,
    id: String,
    matrix_locations:Option<u32>,
    date_time:Option<DateTime>,
    verbose: Option<bool>,
    shape_format: Option<ShapeFormat>,
}
impl Manifest {
    pub fn builder()->Self{
        Default::default()
    }
    /// Sets the source and targets of the matrix
    pub fn sources_to_targets(mut self, sources: impl IntoIterator<Item=Location>,targets:impl IntoIterator<Item=Location>) -> Self {
        self.sources=sources.into_iter().collect();
        self.targets=targets.into_iter().collect();
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
        self.id = id.to_string();
        self
    }
    /// Sets the minimum number of locations that need to be found satisfying the request
    ///
    /// Allows a partial result to be returned.
    ///
    /// This is basically equivalent to:
    /// > "find the closest or best N locations out of the full location set"
    pub fn minimum_matrix_locations_count(mut self,count:u32)->Self{
        self.matrix_locations=Some(count);
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
    pub fn date_time(mut self, date_time:DateTime)->Self{
        self.date_time=Some(date_time);
        self
    }
    /// Modifies the verbosity of the output:
    /// - `true` will output a flat list of objects for distances & durations explicitly specifying
    ///   the `source` & `target` indices.
    /// - `false` will return more compact, nested row-major distances & durations arrays and
    ///   not echo `sources` and `targets`
    /// 
    /// Default: `true`
    pub fn verbose_output(mut self, verbose:bool)->Self{
        self.verbose=Some(verbose);
        self
    }
    /// Specifies the [`ShapeFormat`] for the path shape of each connection.
    pub fn shape_format(mut self,shape_format:ShapeFormat)->Self{
        self.shape_format=Some(shape_format);
        self
    }

}

/// The local date and time at the location
#[derive(Serialize, Debug)]
pub struct DateTime{
    r#type: MatrixDateType,
    value: chrono::NaiveDateTime,
}
impl DateTime {
    /// Current departure time
    pub fn from_current_departure_time()->Self{
        Self{
            r#type:MatrixDateType::CurrentDepartureTime,
            value:chrono::Local::now().naive_local(),
        }
    }
    /// Specified departure time
    pub  fn from_departure_time(depart_after: chrono::NaiveDateTime)->Self{
        Self{
            r#type:MatrixDateType::SpecifiedDepartureTime,
            value: depart_after
        }
    }
    /// Specified arrival time
    pub  fn from_arrival_time(arrive_by: chrono::NaiveDateTime)->Self{
        Self{
            r#type:MatrixDateType::SpecifiedArrivalTime,
            value:arrive_by
        }
    }
}

#[derive(Serialize, Debug, Clone,Copy)]
#[repr(u8)]
enum MatrixDateType{
    CurrentDepartureTime = 0,
    SpecifiedDepartureTime,
    SpecifiedArrivalTime,
}


#[derive(Serialize, Default, Debug)]
pub struct Location {
    lat:f32,
    lon:f32,
    date_time: Option<chrono::NaiveDateTime>,
}
impl From<super::Coordinate> for Location{
    fn from((longitude, latitude): super::Coordinate) -> Self {
        Self{
            lat:latitude,
            lon:longitude,
            date_time:None,
        }
    }
}
impl Location {
    /// Creates a new location from a longitude/latitude
    pub fn new(longitude: f32, latitude: f32)->Self{
        Self::from((longitude,latitude))
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
    pub fn date_time(mut self, date_time:chrono::NaiveDateTime)->Self{
        self.date_time=Some(date_time);
        self
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Response{
    /// Row-ordered time and distances between the sources and the targets.
    /// 
    /// The time and distance from the first location to all others forms the first row of the array,
    /// followed by the time and distance from the second source location to all target locations,
    /// etc.
    pub sources_to_targets: Value,
    /// The computed distance between each set of points.
    /// 
    /// Distance will always be `0.00` for
    /// - the first element of the time-distance array for one_to_many,
    /// - the last element in a many_to_one, and
    /// - the first and last elements of a many_to_many.
    pub distance:Value,
    /// The computed time between each set of points.
    /// 
    /// Time will always be `0` for
    /// - the first element of the time-distance array for one_to_many,
    /// - the last element in a many_to_one, and
    /// - the first and last elements of a many_to_many
    pub time:Value,
    /// The destination index into the locations array
    pub to_index:Value,
    /// The origin index into the locations array
    pub from_index:Value,
    /// If the date_time was valid for an origin, date_time will return the local time at the destination.
    pub date_time:Option<Value>,
    /// The specified array of lat/lngs from the input request.
    pub locations: Value,
    /// Distance units for output.
    /// 
    /// Allowable unit types are mi (miles) and km (kilometers).
    /// If no unit type is specified, the units default to kilometers.
    pub units: Value,
    /// This array may contain warning objects informing about deprecated request parameters, clamped values etc.
    pub warnings:Option<Value>,
}