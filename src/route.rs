use crate::costing;
pub use crate::shapes::ShapePoint;
pub use crate::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Response {
    pub(crate) trip: Trip,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Trip {
    /// Status code
    pub status: i32,
    /// Status message
    pub status_message: String,
    /// The via [`Manifest::units`] specified units of length are returned.
    ///
    /// Either [`super::Units::Metric`] or [`super::Units::Imperial`].
    pub units: super::Units,
    /// The language of the narration instructions.
    ///
    /// If the user specified a language via [`Manifest::language`] in the directions options and the specified language was supported.
    /// This returned value will be equal to the specified value.
    /// Otherwise, this value will be the default (`en-US`) language.
    pub language: String,
    /// Location information is returned in the same form as it is entered.
    ///
    /// Additional fields are added to indicate the side of the street.
    /// Output can be changed via  via [`Manifest::locations`].
    pub locations: Vec<Location>,
    /// This array may contain warning objects informing about deprecated request parameters, clamped values etc.
    pub warnings: Option<Vec<String>>,
    /// Name of your route request.
    ///
    /// If an id is specified via [`Manifest::id`], the naming will be sent thru to the response.
    pub id: Option<String>,
    /// List of [`Leg`]s constituting a [`Trip`]
    pub legs: Vec<Leg>,
    /// Basic information about the entire [`Trip`]
    pub summary: Summary,
}
#[cfg(feature = "gpx")]
impl From<Trip> for gpx::Gpx {
    fn from(trip: Trip) -> Self {
        let mut gpx = Self {
            version: gpx::GpxVersion::Gpx11,
            creator: Some("valhalla".to_string()),
            ..Default::default()
        };
        let track = gpx::Track {
            name: Some("route".to_string()),
            segments: trip.legs.iter().map(|leg| leg.into()).collect(),
            ..Default::default()
        };
        gpx.tracks.push(track);

        let ps = trip
            .legs
            .iter()
            .flat_map(|leg| {
                leg.maneuvers.iter().map(|m| {
                    let p = &leg.shape[m.begin_shape_index];

                    gpx::Waypoint::new(p.into())
                })
            })
            .collect();
        let route = gpx::Route {
            name: Some("route".to_string()),
            points: ps,
            ..Default::default()
        };
        gpx.routes.push(route);
        gpx
    }
}
#[derive(Deserialize, Debug, Clone)]
pub struct Summary {
    /// Estimated elapsed time in seconds
    pub time: f64,
    /// Distance traveled
    ///
    /// Unit is either [`super::Units::Metric`] or [`super::Units::Imperial`] and specified in [`Trip`] for clarification.
    /// See [`Manifest::units`] to change the units.
    pub length: f64,
    /// If the path uses one or more toll segments
    pub has_toll: bool,
    /// If the path uses one or more highway segments
    pub has_highway: bool,
    ///  if the path uses one or more ferry segments
    pub has_ferry: bool,
    /// Minimum latitude of the sections bounding box
    pub min_lat: f64,
    /// Minimum longitude of the sections bounding box
    pub min_lon: f64,
    /// Maximum latitude of the sections bounding box
    pub max_lat: f64,
    /// Maximum longitude of the sections bounding box
    pub max_lon: f64,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TravelMode {
    #[serde(rename = "drive")]
    Drive,
    #[serde(rename = "pedestrian")]
    Pedestrian,
    #[serde(rename = "bicycle")]
    Bicycle,
    #[serde(rename = "transit")]
    Transit,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(untagged)]
pub enum TravelType {
    Drive(DriveTravelType),
    Pedestrian(costing::pedestrian::PedestrianType),
    Bicycle(costing::bicycle::BicycleType),
    Transit(TransitTravelType),
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriveTravelType {
    #[serde(rename = "car")]
    Car,
    #[serde(rename = "motorcycle")]
    Motorcycle,
    #[serde(rename = "truck")]
    Truck,
    #[serde(rename = "motor_scooter")]
    MotorScooter,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitTravelType {
    #[serde(rename = "tram")]
    Tram,
    #[serde(rename = "metro")]
    Metro,
    #[serde(rename = "rail")]
    Rail,
    #[serde(rename = "bus")]
    Bus,
    #[serde(rename = "ferry")]
    Ferry,
    #[serde(rename = "cable_car")]
    CableCar,
    #[serde(rename = "gondola")]
    Gondola,
    #[serde(rename = "funicular")]
    Funicular,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BssManeuverType {
    #[serde(rename = "NoneAction")]
    NoneAction,
    #[serde(rename = "RentBikeAtBikeShare")]
    RentBikeAtBikeShare,
    #[serde(rename = "ReturnBikeAtBikeShare")]
    ReturnBikeAtBikeShare,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Leg {
    pub summary: Summary,

    pub maneuvers: Vec<Maneuver>,

    #[serde(deserialize_with = "crate::shapes::deserialize_shape")]
    pub shape: Vec<ShapePoint>,
}

#[cfg(feature = "gpx")]
impl From<&Leg> for gpx::TrackSegment {
    fn from(leg: &Leg) -> Self {
        Self {
            points: leg.shape[leg.maneuvers[0].begin_shape_index
                ..leg.maneuvers[leg.maneuvers.len() - 1].end_shape_index]
                .iter()
                .map(|location| gpx::Waypoint::new(location.into()))
                .collect(),
        }
    }
}

#[derive(serde_repr::Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum ManeuverType {
    None = 0,
    Start,
    StartRight,
    StartLeft,
    Destination,
    DestinationRight,
    DestinationLeft,
    Becomes,
    Continue,
    SlightRight,
    Right,
    SharpRight,
    UturnRight,
    UturnLeft,
    SharpLeft,
    Left,
    SlightLeft,
    RampStraight,
    RampRight,
    RampLeft,
    ExitRight,
    ExitLeft,
    StayStraight,
    StayRight,
    StayLeft,
    Merge,
    RoundaboutEnter,
    RoundaboutExit,
    FerryEnter,
    FerryExit,
    Transit,
    TransitTransfer,
    TransitRemainOn,
    TransitConnectionStart,
    TransitConnectionTransfer,
    TransitConnectionDestination,
    PostTransitConnectionDestination,
    MergeRight,
    MergeLeft,
    ElevatorEnter,
    StepsEnter,
    EscalatorEnter,
    BuildingEnter,
    BuildingExit,
}

#[derive(Deserialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Sign {
    /// list of exit number elements.
    ///
    /// If an exit number element exists, it is typically just one value
    ///
    /// Example: `91B`
    pub exit_number_elements: Vec<ManeuverSignElement>,
    /// Exit branch elements.
    ///
    /// The exit branch element text is the subsequent road name or route number after the sign
    ///
    /// Example: `I 95 North`
    pub exit_branch_elements: Vec<ManeuverSignElement>,
    /// Exit toward elements.
    ///
    /// The exit toward element text is the location where the road ahead goes.
    /// The location is typically a control city, but may also be a future road name or route number.
    ///
    /// Example: `New York`
    pub exit_toward_elements: Vec<ManeuverSignElement>,
    /// Exit name elements.
    ///
    /// The exit name element is the interchange identifier.
    /// Typically not used in the US.
    ///
    /// Example: `Gettysburg Pike`
    pub exit_name_elements: Vec<ManeuverSignElement>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ManeuverSignElement {
    /// Interchange sign text.
    ///
    /// Examples:
    /// - exit number: `91B`
    /// - exit branch: `I 95 North`
    /// - exit toward: `New York`
    /// - exit name: `Gettysburg Pike`
    pub text: String,
    /// The frequency of this sign element within a set a consecutive signs
    pub consecutive_count: Option<usize>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Maneuver {
    /// Type of maneuver
    #[serde(rename = "type")]
    pub type_: ManeuverType,
    /// Written maneuver instruction, describing the maneuver.
    ///
    /// Example: "Turn right onto Main Street".
    pub instruction: String,

    /// Text suitable for use as a verbal alert in a navigation application.
    ///
    /// The transition alert instruction will prepare the user for the forthcoming transition.
    ///
    /// Example: "Turn right onto North Prince Street"
    pub verbal_transition_alert_instruction: Option<String>,

    /// Text suitable for use as a verbal message immediately prior to the maneuver transition.
    ///
    /// Example: "Turn right onto North Prince Street, U.S. 2 22"
    pub verbal_pre_transition_instruction: Option<String>,
    /// Text suitable for use as a verbal message immediately after the maneuver transition.
    ///
    /// Example: "Continue on U.S. 2 22 for 3.9 miles"
    pub verbal_post_transition_instruction: Option<String>,

    /// List of street names that are consistent along the entire nonobvious maneuver
    pub street_names: Option<Vec<String>>,

    /// When present, these are the street names at the beginning (transition point) of the
    /// nonobvious maneuver (if they are different than the names that are consistent along the
    /// entire nonobvious maneuver).
    pub begin_street_names: Option<Vec<String>>,
    /// Estimated time along the maneuver in seconds.
    pub time: f64,
    /// Maneuver length in the [`super::Units`] specified via [`Manifest::units`]
    pub length: f64,
    /// Index into the list of shape points for the start of the maneuver.
    pub begin_shape_index: usize,
    /// Index into the list of shape points for the end of the maneuver.
    pub end_shape_index: usize,
    /// `true` if a toll booth is encountered on this maneuver.
    pub toll: Option<bool>,
    /// `true` if a highway is encountered on this maneuver.
    pub highway: Option<bool>,
    /// `true` if the maneuver is unpaved or rough pavement, or has any portions that have rough
    /// pavement.
    pub rough: Option<bool>,
    /// `true` if a gate is encountered on this maneuver.
    pub gate: Option<bool>,
    /// `true` if a ferry is encountered on this maneuver.
    pub ferry: Option<bool>,
    /// Contains the interchange guide information at a road junction associated with this
    /// maneuver.
    ///
    /// See [`Sign`] for details.
    pub sign: Option<Sign>,
    /// The spoke to exit roundabout after entering.
    pub roundabout_exit_count: Option<i64>,
    /// Written depart time instruction.
    ///
    /// Typically used with a transit maneuver, such as "Depart: 8:04 AM from 8 St - NYU".
    pub depart_instruction: Option<String>,
    /// Text suitable for use as a verbal depart time instruction.
    ///
    /// Typically used with a transit maneuver, such as "Depart at 8:04 AM from 8 St - NYU".
    pub verbal_depart_instruction: Option<String>,
    /// Written arrive time instruction.
    ///
    /// Typically used with a transit maneuver, such as "Arrive: 8:10 AM at 34 St - Herald Sq".
    pub arrive_instruction: Option<String>,
    /// Text suitable for use as a verbal arrive time instruction.
    ///
    /// Typically used with a transit maneuver, such as "Arrive at 8:10 AM at 34 St - Herald Sq".
    pub verbal_arrive_instruction: Option<String>,
    /// Contains the attributes that describe a specific transit route.
    ///
    /// See [`TransitInfo`] for details.
    pub transit_info: Option<TransitInfo>,
    /// `true` if [`Self::verbal_pre_transition_instruction`] has been appended with
    /// the verbal instruction of the next maneuver and thus contains more than one instruction.
    pub verbal_multi_cue: Option<bool>,
    /// Travel mode
    pub travel_mode: TravelMode,
    /// Travel type
    pub travel_type: TravelType,
    /// Describes bike share maneuver.
    ///
    /// Used when travel_mode is [`TravelMode::Bicycle`].
    ///
    /// Default: [`BssManeuverType::NoneAction`]
    pub bss_maneuver_type: Option<BssManeuverType>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransitInfo {
    /// Global transit route identifier.
    pub onestop_id: String,
    /// Short name describing the transit route
    ///
    /// Example: "N"
    pub short_name: String,
    /// Long name describing the transit route
    ///
    /// Example: "Broadway Express"
    pub long_name: String,
    /// The sign on a public transport vehicle that identifies the route destination to passengers.
    ///
    /// Example: "ASTORIA - DITMARS BLVD"
    pub headsign: String,
    /// The numeric color value associated with a transit route.
    ///
    /// The value for yellow would be "16567306".
    pub color: i32,
    /// The numeric text color value associated with a transit route.
    ///
    /// The value for black would be "0".
    pub text_color: String,
    /// The description of the transit route
    ///
    /// Example: "Trains operate from Ditmars Boulevard, Queens, to Stillwell Avenue, Brooklyn, at all times
    /// N trains in Manhattan operate along Broadway and across the Manhattan Bridge to and from Brooklyn.
    /// Trains in Brooklyn operate along 4th Avenue, then through Borough Park to Gravesend.
    /// Trains typically operate local in Queens, and either express or local in Manhattan and Brooklyn,
    /// depending on the time. Late night trains operate via Whitehall Street, Manhattan.
    /// Late night service is local"
    pub description: String,
    /// Global operator/agency identifier.
    pub operator_onestop_id: String,
    /// Operator/agency name
    ///
    /// Short name is used over long name.
    ///
    /// Example: "BART", "King County Marine Division", and so on.
    pub operator_name: String,
    /// Operator/agency URL
    ///
    /// Example: `http://web.mta.info/`.
    pub operator_url: String,
    /// A list of the stops/stations associated with a specific transit route.
    ///
    /// See [`TransitStop`] for details.
    pub transit_stops: Vec<TransitStop>,
}

#[derive(serde_repr::Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TransitStopType {
    /// Simple stop.
    Stop = 0,
    /// Station.
    Station,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransitStop {
    #[serde(rename = "type")]
    pub type_: TransitStopType,
    /// Name of the stop or station
    ///
    /// Example: "14 St - Union Sq"
    pub name: String,
    /// Arrival date and time
    pub arrival_date_time: chrono::NaiveDateTime,
    /// Departure date and time
    pub departure_date_time: chrono::NaiveDateTime,
    /// `true` if this stop is a marked as a parent stop.
    pub is_parent_stop: bool,
    /// `true` if the times are based on an assumed schedule because the actual schedule is not
    /// known.
    pub assumed_schedule: bool,
    /// Latitude of the transit stop in degrees.
    pub lat: f64,
    /// Longitude of the transit stop in degrees.
    pub lon: f64,
}

#[derive(Serialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectionsType {
    /// indicating no maneuvers or instructions should be returned.
    #[serde(rename = "none")]
    None,

    /// indicating that only maneuvers be returned.
    #[serde(rename = "maneuvers")]
    Maneuvers,

    /// indicating that maneuvers with instructions should be returned (this is the default if not
    /// specified).
    #[default]
    #[serde(rename = "instructions")]
    Instructions,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Default, Debug)]
pub struct Manifest {
    #[serde(flatten)]
    costing: Option<costing::Costing>,
    locations: Vec<Location>,
    units: Option<super::Units>,
    id: Option<String>,
    language: Option<String>,
    directions_type: Option<DirectionsType>,
    alternates: Option<i32>,
    exclude_locations: Option<Vec<Location>>,
    exclude_polygons: Option<Vec<Vec<super::Coordinate>>>,
    linear_references: Option<bool>,
    prioritize_bidirectional: Option<bool>,
    roundabout_exits: Option<bool>,
    date_time: Option<DateTime>,
}

impl Manifest {
    #[must_use]
    pub fn builder() -> Self {
        Self::default()
    }
    /// Configures the costing model
    ///
    /// Valhalla's routing service uses dynamic, run-time costing to generate the route path.
    /// Can be configured with different settings depending on the costing model used.
    ///
    /// Default: [`costing::Costing::Auto`]
    pub fn costing(mut self, costing: costing::Costing) -> Self {
        self.costing = Some(costing);
        self
    }

    /// Specify locations to visit as an ordered list
    ///
    /// Minimum number of locations: 2
    ///
    /// A location must include a latitude and longitude in decimal degrees.
    /// The coordinates can come from many input sources, such as a GPS location, a point or a
    /// click on a map, a geocoding service, and so on.
    ///
    /// **Note:** Valhalla cannot search for names or addresses or perform geocoding or reverse geocoding.
    /// External search services, such as [Mapbox Geocoding](https://www.mapbox.com/api-documentation/#geocoding),
    /// can be used to find places and geocode addresses, which must be converted to coordinates for input.
    ///
    /// To build a route, you need to specify two [`LocationType::Break`] locations.
    /// In addition, you can include [`LocationType::Through`], [`LocationType::Via`] or
    /// [`LocationType::BreakThrough`] locations to influence the route path.
    /// See [`LocationType`] for further information.
    pub fn locations(mut self, locations: impl IntoIterator<Item = Location>) -> Self {
        self.locations = locations.into_iter().collect();
        debug_assert!(self.locations.len() >= 2);
        self
    }

    /// Sets the distance units for output.
    ///
    /// Possible unit types are
    /// - miles via [`super::Units::Imperial`] and
    /// - kilometers via [`super::Units::Metric`].
    ///
    /// Default: [`super::Units::Metric`]
    pub fn units(mut self, units: super::Units) -> Self {
        self.units = Some(units);
        self
    }

    /// Name of the route request
    ///
    /// If id is specified, the naming will be sent through to the response.
    pub fn id(mut self, id: impl ToString) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// The language of the narration instructions based on the
    /// [IETF BCP 47](https://en.wikipedia.org/wiki/IETF_language_tag) language tag string.
    ///
    /// If unsupported, the language `en-US` (United States-based English) is used
    /// Currently supported language list can be found here:
    /// <https://valhalla.github.io/valhalla/api/turn-by-turn/api-reference/#supported-language-tags>
    ///
    /// Default: `en-US` (United States-based English)
    pub fn language(mut self, language: impl ToString) -> Self {
        self.language = Some(language.to_string());
        self
    }
    /// Sets the directions type
    ///
    /// [`DirectionsType`] is an enum with 3 values:
    /// - [`DirectionsType::None`] indicates no maneuvers or instructions should be returned.
    /// - [`DirectionsType::Maneuvers`] indicates that only maneuvers be returned.
    /// - [`DirectionsType::Instructions`] indicates that maneuvers with instructions should be returned
    ///
    /// Default: [`DirectionsType::Instructions`]
    pub fn directions_type(mut self, directions_type: DirectionsType) -> Self {
        self.directions_type = Some(directions_type);
        self
    }

    /// How many alternate routes should be provided
    ///
    /// There may be no alternates or fewer alternates than the user specifies.
    ///
    /// Alternates are not yet supported on
    /// - multipoint routes (i.e. routes with more than 2 locations) and
    /// - time dependent routes
    pub fn alternates(mut self, alternates: i32) -> Self {
        self.alternates = Some(alternates);
        self
    }

    /// A set of [`Location`]s to exclude or avoid within a route
    ///
    /// They are mapped to the closest road or roads and these roads are excluded
    /// from the route path computation.
    pub fn exclude_locations(
        mut self,
        exclude_locations: impl IntoIterator<Item = Location>,
    ) -> Self {
        self.exclude_locations = Some(exclude_locations.into_iter().collect());
        self
    }

    /// Sets at least one exterior rings of excluded polygons.
    ///
    /// **Note:** Contrary to [`Self::exclude_polygon`], this OVERRIDES previously set excluded polygons.
    ///
    /// Roads intersecting these rings will be avoided during path finding.
    /// If you only need to avoid a few specific roads, it's much more efficient to use
    /// [`Self::exclude_locations`].
    /// Valhalla will close open rings (i.e. copy the first coordinate to the last position).
    ///
    /// # Example:
    /// ```rust,no_run
    /// use valhalla_client::blocking::Valhalla;
    /// use valhalla_client::route::{Location, Manifest};
    /// use valhalla_client::costing::{Costing};
    ///
    /// let polygon_around_midrecht_between_amsterdam_and_utrecht = vec![(4.9904022, 52.2528761), (4.8431168, 52.2392163), (4.8468933, 52.1799052), (4.9845657, 52.2102016), (4.9904022, 52.2528761)];
    /// let polygon_around_leiden = vec![(4.5891266, 52.1979985),(4.4105987, 52.2560249),(4.3034820, 52.1592721),(4.5005493, 52.0935286),(4.5726471, 52.1373684),(4.5898132, 52.1984193),(4.5891266, 52.1979985)];
    /// let amsterdam = Location::new(4.9041, 52.3676);
    /// let utrecht = Location::new(5.1214, 52.0907);
    ///
    /// let manifest = Manifest::builder()
    ///   .locations([amsterdam, utrecht])
    ///   .exclude_polygons([polygon_around_leiden, polygon_around_midrecht_between_amsterdam_and_utrecht])
    ///   .costing(Costing::MotorScooter(Default::default()));
    ///
    /// let response = Valhalla::default()
    ///   .route(manifest)
    ///   .unwrap();
    /// # assert!(!response.legs.is_empty());
    /// ```
    pub fn exclude_polygons(
        mut self,
        exclude_polygons: impl IntoIterator<Item = impl IntoIterator<Item = super::Coordinate>>,
    ) -> Self {
        let new_excluded_polygons = exclude_polygons
            .into_iter()
            .map(|e| e.into_iter().collect())
            .collect();
        self.exclude_polygons = Some(new_excluded_polygons);
        self
    }
    /// Add one exterior rings as an excluded polygon.
    ///
    /// **Note:** Contrary to [`Self::exclude_polygons`], this APPENDS to the previously set excluded polygons.
    ///
    /// Roads intersecting these rings will be avoided during path finding.
    /// If you only need to avoid a few specific roads, it's much more efficient to use
    /// exclude_locations.
    /// Valhalla will close open rings (i.e. copy the first coordinate to the last position).
    ///
    /// # Example:
    /// ```rust,no_run
    /// use valhalla_client::blocking::Valhalla;
    /// use valhalla_client::route::{Location, Manifest};
    /// use valhalla_client::costing::{Costing};
    ///
    /// let polygon_around_leiden = vec![(4.5891266, 52.1979985),(4.4105987, 52.2560249),(4.3034820, 52.1592721),(4.5005493, 52.0935286),(4.5726471, 52.1373684),(4.5898132, 52.1984193),(4.5891266, 52.1979985)];
    /// let amsterdam = Location::new(4.9041, 52.3676);
    /// let utrecht = Location::new(5.1214, 52.0907);
    ///
    /// let manifest = Manifest::builder()
    ///   .locations([amsterdam, utrecht])
    ///   .exclude_polygon(polygon_around_leiden)
    ///   .costing(Costing::Auto(Default::default()));
    ///
    /// let response = Valhalla::default()
    ///   .route(manifest)
    ///   .unwrap();
    /// # assert!(!response.legs.is_empty());
    /// ```
    pub fn exclude_polygon(
        mut self,
        exclude_polygon: impl IntoIterator<Item = super::Coordinate>,
    ) -> Self {
        let new_excluded_polygon = exclude_polygon.into_iter().collect();
        if let Some(ref mut polygons) = self.exclude_polygons {
            polygons.push(new_excluded_polygon);
        } else {
            self.exclude_polygons = Some(vec![new_excluded_polygon]);
        }
        self
    }

    /// When present and true, the successful route response will include a key `linear_references`.
    ///
    /// Its value is an array of base64-encoded [OpenLR location references](https://en.wikipedia.org/wiki/OpenLR),
    /// one for each graph edge of the road network matched by the input trace.
    #[doc(hidden)] // TODO: need to implement the linear_references field
    pub fn include_linear_references(mut self) -> Self {
        self.linear_references = Some(true);
        self
    }

    /// Prioritize bidirectional A* when `date_time.type = depart_at/current`.
    ///
    /// Currently, it does not update the time (and speeds) when searching for the route path, but
    /// the ETA on that route is recalculated based on the time-dependent speeds
    ///
    /// Default: time_dependent_forward A* is used in these cases, but bidirectional A* is much faster
    pub fn prioritize_bidirectional(mut self) -> Self {
        self.prioritize_bidirectional = Some(true);
        self
    }

    /// Don't include instructions at roundabouts to the output
    ///
    /// Default: `true`
    pub fn roundabout_exits(mut self) -> Self {
        self.roundabout_exits = Some(false);
        self
    }
    /// Shortcut for configuring the arrival/departure date_time settings globally
    /// instead of specifying it for each of the [locations](Location::date_time).
    ///
    /// See [`Location::date_time`] if you want a more granular API.
    pub fn date_time(mut self, date_time: DateTime) -> Self {
        self.date_time = Some(date_time);
        self
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocationType {
    #[default]
    #[serde(rename = "break")]
    Break,

    #[serde(rename = "through")]
    Through,

    #[serde(rename = "via")]
    Via,

    #[serde(rename = "break_through")]
    BreakThrough,
}

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    #[serde(rename = "same")]
    Same,

    #[serde(rename = "opposite")]
    Opposite,

    #[default]
    #[serde(rename = "either")]
    Either,
}

#[cfg(feature = "gpx")]
impl From<&Location> for gpx::Waypoint {
    fn from(location: &Location) -> Self {
        let point =
            geo_types::Point::new(f64::from(location.longitude), f64::from(location.latitude));
        let mut p = Self::new(point);
        p.name.clone_from(&location.name);
        p
    }
}
impl From<super::Coordinate> for Location {
    fn from((latitude, longitude): super::Coordinate) -> Self {
        Self {
            latitude,
            longitude,
            ..Default::default()
        }
    }
}

impl Location {
    /// Create a Location from latitude/longitude of the location in degrees.
    ///
    /// This is assumed to be both routing location and display location is equal.
    /// See [`Self::display_coordinates`] to change the display location
    pub fn new(longitude: f32, latitude: f32) -> Self {
        Self {
            latitude,
            longitude,
            ..Default::default()
        }
    }
    /// Display Coordinate location in degrees.
    ///
    /// Will be used to determine the side of street.
    /// Must be valid to achieve the desired effect.
    pub fn display_coordinates(mut self, display_lat: f32, display_lon: f32) -> Self {
        self.display_lat = Some(display_lat);
        self.display_lon = Some(display_lon);
        self
    }

    /// Sets the Street name.
    ///
    /// May be used to assist finding the correct routing location at the specified coordinate.
    /// **This is not currently implemented.**
    pub fn street_name(mut self, street: impl ToString) -> Self {
        self.street = Some(street.to_string());
        self
    }

    /// Sets the OpenStreetMap identification number for a polyline way.
    ///
    /// The way ID may be used to assist finding the correct routing location at the specified coordinate.
    /// **This is not currently implemented.**
    pub fn way_id(mut self, way_id: i64) -> Self {
        self.way_id = Some(way_id);
        self
    }

    /// Sets the Minimum number of nodes (intersections) reachable for a given edge (road between
    /// intersections) to consider that edge as belonging to a connected region.
    ///
    /// When correlating this location to the route network, try to find candidates who are reachable
    /// from this many or more nodes (intersections). If a given candidate edge reaches less than
    /// this number of nodes it is considered to be a disconnected island, and we will search for more
    /// candidates until we find at least one that isn't considered a disconnected island.
    /// If this value is larger than the configured service limit it will be clamped to that limit.
    ///
    /// Default: `50` reachable nodes.
    pub fn minimum_reachability(mut self, minimum_reachability: i32) -> Self {
        self.minimum_reachability = Some(minimum_reachability);
        self
    }

    /// The number of meters about this input location within which edges (roads between
    /// intersections) will be considered as candidates for said location.
    ///
    /// When correlating this location to the route network, try to only return results within
    /// this distance (meters) from this location. If there are no candidates within this distance
    /// it will return the closest candidate within reason.
    /// If this value is larger than the configured service limit it will be clamped to that limit.
    ///
    /// Default: `0` meters
    pub fn radius(mut self, radius: i32) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Whether or not to rank the edge candidates for this location.
    ///
    /// The ranking is used as a penalty within the routing algorithm so that some edges will be
    /// penalized more heavily than others:
    /// - If `true`, candidates will be ranked according to their distance from the input and
    ///   various other attributes.
    /// - If `false` the candidates will all be treated as equal which should lead to routes that
    ///   are just the most optimal path with emphasis about which edges were selected.
    pub fn rank_candidates(mut self, rank_candidates: bool) -> Self {
        self.rank_candidates = Some(rank_candidates);
        self
    }
    /// Which side of the road the location should be visited from.
    ///
    /// Whether the location should be visited from the [`Side::Same`], [`Side::Opposite`] or [`Side::Either`] side of
    /// the road with respect to the side of the road the given locale drives on:
    /// - In Germany (driving on the right side of the road), passing a value of same will only allow
    ///   you to leave from or arrive at a location such that the location will be on your right.
    /// - In Australia (driving on the left side of the road), passing a value of same will force the location to be on
    ///   your left.
    ///
    /// A value of opposite will enforce arriving/departing from a location on the opposite side
    /// of the road from that which you would be driving on while a value of either will make
    /// no attempt limit the side of street that is available for the route.
    ///
    /// **Note:** If the location is not offset from the road centerline or is closest to an intersection
    /// this option has no effect.
    pub fn preferred_side(mut self, preferred_side: Side) -> Self {
        self.preferred_side = Some(preferred_side);
        self
    }
    /// Sets the type of the location
    ///
    /// Either [`LocationType::Break`], [`LocationType::Through`], [`LocationType::Via`] or [`LocationType::BreakThrough`].
    /// The types of the first and last locations are ignored and are treated as [`LocationType::Break`].
    /// Each type controls two characteristics:
    /// - whether or not to allow an u-turn at the location and
    /// - whether or not to generate guidance/legs at the location.
    ///
    /// Here is their behaviour:
    /// - A [`LocationType::Break`] is a location at which we allows u-turns and generate legs and
    ///   arrival/departure maneuvers.
    /// - A [`LocationType::Through`] location is a location at which we neither allow u-turns
    ///   nor generate legs or arrival/departure maneuvers.
    /// - A [`LocationType::Via`] location is a location at which we allow u-turns,
    ///   but do not generate legs or arrival/departure maneuvers.
    /// - A [`LocationType::BreakThrough`] location is a location at which we do not allow u-turns,
    ///   but do generate legs and arrival/departure maneuvers.
    ///
    /// Default: [`LocationType::Break`]
    pub fn r#type(mut self, r#type: LocationType) -> Self {
        self.r#type = Some(r#type);
        self
    }

    /// Preferred direction of travel for the start from the location.
    ///
    /// This can be useful for mobile routing where a vehicle is traveling in a specific direction
    /// along a road, and the route should start in that direction.
    /// The heading is indicated in degrees from north in a clockwise direction:
    /// - where north is `0°`,
    /// - east is `90°`,
    /// - south is `180°`, and
    /// - west is `270°`.
    pub fn heading(mut self, heading: u32) -> Self {
        self.heading = Some(heading);
        self
    }
    /// How close in degrees a given street's heading angle must be in order for it to be considered
    /// as in the same direction of the heading parameter.
    ///
    /// The heading angle can be set via [`Self::heading`]
    ///
    /// Default: `60` degrees
    pub fn heading_tolerance(mut self, heading_tolerance: u32) -> Self {
        self.heading_tolerance = Some(heading_tolerance);
        self
    }
    /// Location or business name.
    ///
    /// May be used in the route narration directions.
    /// Example: `"You have arrived at <business name>"`
    pub fn name(mut self, name: impl ToString) -> Self {
        self.name = Some(name.to_string());
        self
    }
    /// Cutoff at which we will assume the input is too far away from civilisation to be worth
    /// correlating to the nearest graph elements.
    ///
    /// Default: `35 km`
    pub fn search_cutoff(mut self, search_cutoff: f32) -> Self {
        self.search_cutoff = Some(search_cutoff);
        self
    }
    /// During edge correlation this is the tolerance used to determine whether or not to snap to
    /// the intersection rather than along the street, if the snap location is within this distance
    /// from the intersection is used instead.
    ///
    /// Default: `5 meters`
    pub fn node_snap_tolerance(mut self, node_snap_tolerance: f32) -> Self {
        self.node_snap_tolerance = Some(node_snap_tolerance);
        self
    }
    /// Sets the tolerance for street side changes.
    ///
    /// The value means:
    /// - If your input coordinate is less than this tolerance away from the edge centerline then we
    ///   set your side of street to none.
    /// - Otherwise your side of street will be left or right depending on direction of travel.
    ///
    /// Default: `5 meters`
    pub fn street_side_tolerance(mut self, street_side_tolerance: f32) -> Self {
        self.street_side_tolerance = Some(street_side_tolerance);
        self
    }
    /// The max distance in meters that the input coordinates or display ll can be from the edge
    /// centerline for them to be used for determining the side of street.
    ///
    /// Beyond this distance the side of street is set to none.
    ///
    /// Default: `1000 meters`
    pub fn street_side_max_distance(mut self, street_side_max_distance: f32) -> Self {
        self.street_side_max_distance = Some(street_side_max_distance);
        self
    }

    /// Allows configuring the preferred side selection.
    ///
    /// Disables the preferred side (set via [`Self::preferred_side`]) when set to [`Side::Same`]
    /// or [`Side::Opposite`], if the edge has a road class less than that provided by this value.
    ///
    /// The road class must be one of the following strings:
    /// - `motorway`,
    /// - `trunk`,
    /// - `primary`,
    /// - `secondary`,
    /// - `tertiary`,
    /// - `unclassified`,
    /// - `residential` or
    /// - `service_other`.
    ///
    /// Default: `service_other` so that the preferred side will not be disabled for any edges
    pub fn street_side_cutoff(mut self, street_side_cutoff: f32) -> Self {
        self.street_side_cutoff = Some(street_side_cutoff);
        self
    }
    /// Expected date/time for the user to be at the location in the local time zone of departure or arrival.
    ///
    /// Offers more granularity over setting time than the global [`Manifest::date_time`].
    ///
    /// If waiting was set on this location in the request, and it's an intermediate location,
    /// the date_time will describe the departure time at this location.
    pub fn date_time(mut self, date_time: chrono::NaiveDateTime) -> Self {
        self.date_time = Some(date_time);
        self
    }
    /// The waiting time at this location.
    ///
    /// Only works if [`Manifest::r#type`] was set to
    /// - [`LocationType::Break`] or
    /// - [`LocationType::BreakThrough`]
    ///
    /// Example:
    /// A route describes a pizza delivery tour.
    /// Each location has a service time, which can be respected by setting waiting on the location.
    /// Then the departure will be delayed by this duration.
    pub fn waiting(mut self, waiting: chrono::Duration) -> Self {
        self.waiting = Some(waiting.num_seconds());
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Location {
    #[serde(rename = "lat")]
    latitude: f32,
    #[serde(rename = "lon")]
    longitude: f32,
    display_lat: Option<f32>,
    display_lon: Option<f32>,
    street: Option<String>,
    way_id: Option<i64>,
    minimum_reachability: Option<i32>,
    radius: Option<i32>,
    rank_candidates: Option<bool>,
    preferred_side: Option<Side>,
    #[serde(rename = "type")]
    r#type: Option<LocationType>,
    heading: Option<u32>,
    heading_tolerance: Option<u32>,
    name: Option<String>,
    search_cutoff: Option<f32>,
    node_snap_tolerance: Option<f32>,
    street_side_tolerance: Option<f32>,
    street_side_max_distance: Option<f32>,
    street_side_cutoff: Option<f32>,
    /// The waiting time in seconds at this location
    waiting: Option<i64>,
    /// Expected date/time for the user to be at the location.
    #[serde(serialize_with = "super::serialize_naive_date_time_opt")]
    date_time: Option<chrono::NaiveDateTime>,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn serialisation() {
        assert_eq!(
            serde_json::to_value(Manifest::default()).unwrap(),
            serde_json::json!({"locations": []})
        );
    }
}
