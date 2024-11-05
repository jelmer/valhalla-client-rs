use serde::{Deserialize, Serialize};
pub mod costing;
#[derive(Deserialize, Debug, Clone)]
pub struct Response {
    pub trip: Trip,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Trip {
    pub status: i32,
    pub status_message: String,
    pub units: Units,
    pub language: String,
    pub locations: Vec<Location>,
    pub warnings: Option<Vec<String>>,
    pub id: Option<String>,
    pub legs: Vec<Leg>,
    pub summary: Summary,
}
#[cfg(feature = "gpx")]
impl From<Trip> for gpx::Gpx {
    fn from(trip: Trip) -> Self {
        let mut gpx = gpx::Gpx {
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
    pub time: f64,
    pub length: f64,
    pub has_toll: bool,
    pub has_highway: bool,
    pub has_ferry: bool,
    pub min_lat: f64,
    pub min_lon: f64,
    pub max_lat: f64,
    pub max_lon: f64,
}

#[derive(Deserialize, Debug, Clone, Copy)]
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

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum CarTravelType {
    #[serde(rename = "car")]
    Car,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum PedestrianTravelType {
    #[serde(rename = "foot")]
    Foot,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum BicycleTravelType {
    #[serde(rename = "road")]
    Road,
}

#[derive(Deserialize, Debug, Clone, Copy)]
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

#[derive(Deserialize, Debug, Clone, Copy)]
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
    pub shape: Vec<crate::shapes::ShapePoint>,
}

#[cfg(feature = "gpx")]
impl From<&Leg> for gpx::TrackSegment {
    fn from(leg: &Leg) -> Self {
        gpx::TrackSegment {
            points: leg.shape[leg.maneuvers[0].begin_shape_index
                ..leg.maneuvers[leg.maneuvers.len() - 1].end_shape_index]
                .iter()
                .map(|location| gpx::Waypoint::new(location.into()))
                .collect(),
        }
    }
}

#[derive(serde_repr::Deserialize_repr, Debug, Clone, Copy)]
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

#[derive(Deserialize, Clone, Debug)]
pub struct Sign {}

#[derive(Deserialize, Clone, Debug)]
pub struct Maneuver {
    #[serde(rename = "type")]
    pub type_: ManeuverType,

    pub instruction: String,

    /// Text suitable for use as a verbal alert in a navigation application. The transition alert
    /// instruction will prepare the user for the forthcoming transition. For example: "Turn right
    /// onto North Prince Street".
    pub verbal_transition_alert_instruction: Option<String>,

    /// Text suitable for use as a verbal message immediately prior to the maneuver transition. For
    /// example "Turn right onto North Prince Street, U.S. 2 22".
    pub verbal_pre_transition_instruction: Option<String>,
    /// Text suitable for use as a verbal message immediately after the maneuver transition. For
    /// example "Continue on U.S. 2 22 for 3.9 miles".
    pub verbal_post_transition_instruction: Option<String>,

    ///  List of street names that are consistent along the entire nonobvious maneuver.
    pub street_names: Option<Vec<String>>,

    /// When present, these are the street names at the beginning (transition point) of the
    /// nonobvious maneuver (if they are different than the names that are consistent along the
    /// entire nonobvious maneuver).
    pub begin_street_names: Option<Vec<String>>,

    /// Estimated time along the maneuver in seconds.
    pub time: f64,

    /// Maneuver length in the units specified.
    pub length: f64,

    /// Index into the list of shape points for the start of the maneuver.
    pub begin_shape_index: usize,
    /// Index into the list of shape points for the end of the maneuver.
    pub end_shape_index: usize,
    /// True if a toll booth is encountered on this maneuver.
    pub toll: Option<bool>,
    /// True if a highway is encountered on this maneuver.
    pub highway: Option<bool>,
    /// True if the maneuver is unpaved or rough pavement, or has any portions that have rough
    /// pavement.
    pub rough: Option<bool>,
    /// True if a gate is encountered on this maneuver.
    pub gate: Option<bool>,
    /// True if a ferry is encountered on this maneuver.
    pub ferry: Option<bool>,
    /// Contains the interchange guide information at a road junction associated with this
    /// maneuver. See below for details.
    pub sign: Option<Sign>,
    /// The spoke to exit roundabout after entering.
    pub roundabout_exit_count: Option<i64>,
    /// Written depart time instruction. Typically used with a transit maneuver, such as "Depart:
    /// 8:04 AM from 8 St - NYU".
    pub depart_instruction: Option<String>,
    /// Text suitable for use as a verbal depart time instruction. Typically used with a transit
    /// maneuver, such as "Depart at 8:04 AM from 8 St - NYU".
    pub verbal_depart_instruction: Option<String>,
    /// Written arrive time instruction. Typically used with a transit maneuver, such as "Arrive:
    /// 8:10 AM at 34 St - Herald Sq".
    pub arrive_instruction: Option<String>,
    /// Text suitable for use as a verbal arrive time instruction. Typically used with a transit
    /// maneuver, such as "Arrive at 8:10 AM at 34 St - Herald Sq".
    pub verbal_arrive_instruction: Option<String>,
    /// Contains the attributes that describe a specific transit route. See below for details.
    pub transit_info: Option<TransitInfo>,
    /// Contains the attributes that describe a specific transit stop. See below for details.
    /// True if the verbal_pre_transition_instruction has been appended with the verbal instruction
    /// of the next maneuver.
    pub verbal_multi_cue: Option<bool>,

    /// Travel mode.
    pub travel_mode: TravelMode,

    /// Used when travel_mode is bikeshare. Describes bike share maneuver. The default value is
    /// NoneAction
    pub bss_maneuver_type: Option<BssManeuverType>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransitInfo {
    /// Global transit route identifier.
    pub onestop_id: String,
    /// Short name describing the transit route. For example "N".
    pub short_name: String,
    /// Long name describing the transit route. For example "Broadway Express".
    pub long_name: String,
    /// The sign on a public transport vehicle that identifies the route destination to passengers.
    /// For example "ASTORIA - DITMARS BLVD".
    pub headsign: String,
    /// The numeric color value associated with a transit route. The value for yellow would be
    /// "16567306".
    pub color: i32,
    /// The numeric text color value associated with a transit route. The value for black would be
    /// "0".
    pub text_color: String,
    /// The description of the the transit route. For example "Trains operate from Ditmars
    /// Boulevard, Queens, to Stillwell Avenue, Brooklyn, at all times. N trains in Manhattan
    /// operate along Broadway and across the Manhattan Bridge to and from Brooklyn. Trains in
    /// Brooklyn operate along 4th Avenue, then through Borough Park to Gravesend. Trains typically
    /// operate local in Queens, and either express or local in Manhattan and Brooklyn, depending
    /// on the time. Late night trains operate via Whitehall Street, Manhattan. Late night service
    /// is local".
    pub description: String,
    /// Global operator/agency identifier.
    pub operator_onestop_id: String,
    /// Operator/agency name. For example, "BART", "King County Marine Division", and so on. Short
    /// name is used over long name.
    pub operator_name: String,
    /// Operator/agency URL. For example, `http://web.mta.info/`.
    pub operator_url: String,
    /// A list of the stops/stations associated with a specific transit route. See below for
    /// details.
    pub transit_stops: Vec<TransitStop>,
}

#[derive(serde_repr::Deserialize_repr, Debug, Clone, Copy)]
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
    /// Name of the stop or station. For example "14 St - Union Sq".
    pub name: String,
    /// Arrival date and time using the ISO 8601 format (YYYY-MM-DDThh:mm). For example,
    /// "2015-12-29T08:06".
    pub arrival_date_time: String,
    /// Departure date and time using the ISO 8601 format (YYYY-MM-DDThh:mm). For example,
    /// "2015-12-29T08:06".
    pub departure_date_time: String,
    /// True if this stop is a marked as a parent stop.
    pub is_parent_stop: bool,
    /// True if the times are based on an assumed schedule because the actual schedule is not
    /// known.
    pub assumed_schedule: bool,
    /// Latitude of the transit stop in degrees.
    pub lat: f64,
    /// Longitude of the transit stop in degrees.
    pub lon: f64,
}

#[derive(Serialize, Default, Debug, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub enum Units {
    #[default]
    #[serde(rename = "kilometers")]
    Metric,

    #[serde(rename = "miles")]
    Imperial,
}

#[derive(Serialize, Default, Debug)]
pub struct Manifest {
    #[serde(flatten)]
    costing: costing::Costing,
    locations: Vec<Location>,
    units: Units,
    id: String,
    language: String,
    directions_type: DirectionsType,
    alternates: i32,
    exclude_locations: Vec<Location>,
    exclude_polygons: Vec<Vec<(f32, f32)>>,
    linear_references: Option<bool>,
    prioritize_bidirectional: Option<bool>,
    roundabout_exits: Option<bool>,
}

impl Manifest {
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
        self.costing = costing;
        self
    }

    /// You specify locations as an ordered list of two or more locations.
    ///
    /// Locations are visited in the order specified.
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
    pub fn locations(mut self, locations: impl IntoIterator<Item = Location>) -> Self {
        self.locations = locations.into_iter().collect();
        debug_assert!(self.locations.len() >= 2);
        self
    }

    /// Sets the distance units for output.
    ///
    /// Possible unit types are miles via [`Units::Imperial`] and kilometers via [`Units::Metric`].
    ///
    /// Default: [`Units::Metric`]
    pub fn units(mut self, units: Units) -> Self {
        self.units = units;
        self
    }

    /// Name your route request.
    ///
    /// If id is specified, the naming will be sent through to the response.
    pub fn id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
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
        self.language = language.to_string();
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
        self.directions_type = directions_type;
        self
    }

    /// A number denoting how many alternate routes should be provided.
    ///
    /// There may be no alternates or fewer alternates than the user specifies.
    ///
    /// Alternates are not yet supported on
    /// - multipoint routes (i.e. routes with more than 2 locations) and
    /// - time dependent routes
    pub fn alternates(mut self, alternates: i32) -> Self {
        self.alternates = alternates;
        self
    }

    /// A set of locations to exclude or avoid within a route can be specified using a JSON array
    /// of avoid_locations.
    ///
    /// The avoid_locations have the same format as the locations list.
    /// At a minimum each avoid location must include latitude and longitude.
    /// The avoid_locations are mapped to the closest road or roads and these roads are excluded
    /// from the route path computation.
    pub fn exclude_locations(
        mut self,
        exclude_locations: impl IntoIterator<Item = Location>,
    ) -> Self {
        self.exclude_locations = exclude_locations.into_iter().collect();
        self
    }

    /// Sets at least one exterior rings of excluded polygons.
    ///
    /// **Note:** Contrary to [`Self::exclude_polygon`], this OVERRIDES previously set excluded polygons.
    ///
    /// Roads intersecting these rings will be avoided during path finding.
    /// If you only need to avoid a few specific roads, it's much more efficient to use
    /// exclude_locations.
    /// Valhalla will close open rings (i.e. copy the first coordinate to the last position).
    ///
    /// # Example:
    /// ```rust,no_run
    /// use valhalla_client::Valhalla;
    /// use valhalla_client::route::{Location, Manifest};
    /// use valhalla_client::route::costing::{Costing};
    ///
    /// let polygon_around_midrecht_between_amsterdam_and_utrecht = vec![(4.9904022, 52.2528761), (4.8431168, 52.2392163), (4.8468933, 52.1799052), (4.9845657, 52.2102016), (4.9904022, 52.2528761)];
    /// let polygon_around_leiden = vec![(4.5891266, 52.1979985),(4.4105987, 52.2560249),(4.3034820, 52.1592721),(4.5005493, 52.0935286),(4.5726471, 52.1373684),(4.5898132, 52.1984193),(4.5891266, 52.1979985)];
    /// let amsterdam = Location::new(4.9041, 52.3676);
    /// let utrecht = Location::new(5.1214, 52.0907);
    ///
    /// let manifest = Manifest::builder()
    ///   .locations([amsterdam, utrecht])
    ///   .exclude_polygons([polygon_around_leiden, polygon_around_midrecht_between_amsterdam_and_utrecht])
    ///   .costing(Costing::Bicycle(Default::default()));
    ///
    /// let response = Valhalla::default()
    ///   .route(manifest)
    ///   .unwrap();
    /// # assert!(!response.legs.is_empty());
    /// ```
    pub fn exclude_polygons(
        mut self,
        exclude_polygons: impl IntoIterator<Item = impl IntoIterator<Item = (f32, f32)>>,
    ) -> Self {
        self.exclude_polygons = exclude_polygons
            .into_iter()
            .map(|e| e.into_iter().collect())
            .collect();
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
    /// use valhalla_client::Valhalla;
    /// use valhalla_client::route::{Location, Manifest};
    /// use valhalla_client::route::costing::{Costing};
    ///
    /// let polygon_around_leiden = vec![(4.5891266, 52.1979985),(4.4105987, 52.2560249),(4.3034820, 52.1592721),(4.5005493, 52.0935286),(4.5726471, 52.1373684),(4.5898132, 52.1984193),(4.5891266, 52.1979985)];
    /// let amsterdam = Location::new(4.9041, 52.3676);
    /// let utrecht = Location::new(5.1214, 52.0907);
    ///
    /// let manifest = Manifest::builder()
    ///   .locations([amsterdam, utrecht])
    ///   .exclude_polygon(polygon_around_leiden)
    ///   .costing(Costing::Bicycle(Default::default()));
    ///
    /// let response = Valhalla::default()
    ///   .route(manifest)
    ///   .unwrap();
    /// # assert!(!response.legs.is_empty());
    /// ```
    pub fn exclude_polygon(
        mut self,
        exclude_polygon: impl IntoIterator<Item = (f32, f32)>,
    ) -> Self {
        self.exclude_polygons
            .push(exclude_polygon.into_iter().collect());
        self
    }

    /// When present and true, the successful route response will include a key linear_references.
    ///
    /// Its value is an array of base64-encoded [OpenLR location references](https://en.wikipedia.org/wiki/OpenLR),
    /// one for each graph edge of the road network matched by the input trace.
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
}

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
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

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
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
        let mut p =
            gpx::Waypoint::new(geo_types::Point::new(location.longitude, location.latitude));
        p.name = location.name.clone();
        p
    }
}

impl Location {
    pub fn new(longitude: f64, latitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Location {
    /// Latitude of the location in degrees. This is assumed to be both the routing location and
    /// the display location if no display_lat and display_lon are provided.
    #[serde(rename = "lat")]
    pub latitude: f64,

    /// Longitude of the location in degrees. This is assumed to be both the routing location and
    /// the display location if no display_lat and display_lon are provided.
    #[serde(rename = "lon")]
    pub longitude: f64,

    /// (optional) Street name. The street name may be used to assist finding the correct routing
    /// location at the specified latitude, longitude. This is not currently implemented.
    pub street: Option<String>,

    /// (optional) OpenStreetMap identification number for a polyline way. The way ID may be used
    /// to assist finding the correct routing location at the specified latitude, longitude. This
    /// is not currently implemented.
    pub way_id: Option<i64>,

    /// Minimum number of nodes (intersections) reachable for a given edge (road between
    /// intersections) to consider that edge as belonging to a connected region. When correlating
    /// this location to the route network, try to find candidates who are reachable from this many
    /// or more nodes (intersections). If a given candidate edge reaches less than this number of
    /// nodes its considered to be a disconnected island and we'll search for more candidates until
    /// we find at least one that isn't considered a disconnected island. If this value is larger
    /// than the configured service limit it will be clamped to that limit. The default is a
    /// minimum of 50 reachable nodes.
    pub minimum_reachability: Option<i32>,

    /// The number of meters about this input location within which edges (roads between
    /// intersections) will be considered as candidates for said location. When correlating this
    /// location to the route network, try to only return results within this distance (meters)
    /// from this location. If there are no candidates within this distance it will return the
    /// closest candidate within reason. If this value is larger than the configured service limit
    /// it will be clamped to that limit. The default is 0 meters.
    pub radius: Option<i32>,

    /// Whether or not to rank the edge candidates for this location. The ranking is used as a
    /// penalty within the routing algorithm so that some edges will be penalized more heavily than
    /// others. If true candidates will be ranked according to their distance from the input and
    /// various other attributes. If false the candidates will all be treated as equal which should
    /// lead to routes that are just the most optimal path with emphasis about which edges were
    /// selected.
    pub rank_candidates: Option<bool>,

    /// If the location is not offset from the road centerline or is closest to an intersection
    /// this option has no effect. Otherwise the determined side of street is used to determine
    /// whether or not the location should be visited from the same, opposite or either side of the
    /// road with respect to the side of the road the given locale drives on. In Germany (driving
    /// on the right side of the road), passing a value of same will only allow you to leave from
    /// or arrive at a location such that the location will be on your right. In Australia (driving
    /// on the left side of the road), passing a value of same will force the location to be on
    /// your left. A value of opposite will enforce arriving/departing from a location on the
    /// opposite side of the road from that which you would be driving on while a value of either
    /// will make no attempt limit the side of street that is available for the route.
    pub preferred_side: Option<Side>,

    ///  Type of location, either break, through, via or break_through. Each type controls two
    ///  characteristics: whether or not to allow a u-turn at the location and whether or not to
    ///  generate guidance/legs at the location. A break is a location at which we allows u-turns
    ///  and generate legs and arrival/departure maneuvers. A through location is a location at
    ///  which we neither allow u-turns nor generate legs or arrival/departure maneuvers. A via
    ///  location is a location at which we allow u-turns but do not generate legs or
    ///  arrival/departure maneuvers. A break_through location is a location at which we do not
    ///  allow u-turns but do generate legs and arrival/departure maneuvers. If no type is
    ///  provided, the type is assumed to be a break. The types of the first and last locations are
    ///  ignored and are treated as breaks.
    #[serde(rename = "type")]
    pub type_: LocationType,

    /// (optional) Preferred direction of travel for the start from the location. This can be
    /// useful for mobile routing where a vehicle is traveling in a specific direction along a
    /// road, and the route should start in that direction. The heading is indicated in degrees
    /// from north in a clockwise direction, where north is 0°, east is 90°, south is 180°, and
    /// west is 270°.
    pub heading: Option<String>,

    /// (optional) How close in degrees a given street's angle must be in order for it to be
    /// considered as in the same direction of the heading parameter. The default value is 60
    /// degrees.
    pub heading_tolerance: Option<String>,

    pub name: Option<String>,

    /// Latitude of the map location in degrees. If provided the lat and lon parameters will be
    /// treated as the routing location and the display_lat and display_lon will be used to
    /// determine the side of street. Both display_lat and display_lon must be provided and valid
    /// to achieve the desired effect.
    pub display_lat: Option<f64>,

    /// Longitude of the map location in degrees. If provided the lat and lon parameters will be
    /// treated as the routing location and the display_lat and display_lon will be used to
    /// determine the side of street. Both display_lat and display_lon must be provided and valid
    /// to achieve the desired effect.
    pub display_lon: Option<f64>,

    /// The cutoff at which we will assume the input is too far away from civilisation to be worth
    /// correlating to the nearest graph elements. The default is 35 km.
    pub search_cutoff: Option<f64>,

    /// During edge correlation this is the tolerance used to determine whether or not to snap to
    /// the intersection rather than along the street, if the snap location is within this distance
    /// from the intersection the intersection is used instead. The default is 5 meters.
    pub node_snap_tolerance: Option<f64>,

    /// If your input coordinate is less than this tolerance away from the edge centerline then we
    /// set your side of street to none otherwise your side of street will be left or right
    /// depending on direction of travel. The default is 5 meters.
    pub street_side_tolerance: Option<f64>,

    /// The max distance in meters that the input coordinates or display ll can be from the edge
    /// centerline for them to be used for determining the side of street. Beyond this distance the
    /// side of street is set to none. The default is 1000 meters.
    pub street_side_max_distance: Option<f64>,

    /// Disables the preferred_side when set to same or opposite if the edge has a road class less
    /// than that provided by street_side_cutoff. The road class must be one of the following
    /// strings: motorway, trunk, primary, secondary, tertiary, unclassified, residential,
    /// service_other. The default value is service_other so that preferred_side will not be
    /// disabled for any edges.
    pub street_side_cutoff: Option<f64>,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn serialisation_snapshots() {
        let manifest = Manifest::default();
        assert_eq!(
            serde_json::to_value(manifest).unwrap(),
            serde_json::json!({
                  "costing": "auto",
                  "costing_options": {
                      "maneuver_penalty": null,
                      "gate_cost": null,
                      "gate_penalty": null,
                      "private_access_penalty": null,
                      "destination_only_penalty": null,
                      "toll_booth_cost": null,
                      "toll_booth_penalty": null,
                      "ferry_cost": null,
                      "use_ferry": null,
                      "use_highways": null,
                      "use_tolls": null,
                      "use_living_streets": null,
                      "use_tracks": null,
                      "service_penalty": null,
                      "service_factor": null,
                      "country_crossing_cost": null,
                      "country_crossing_penalty": null,
                      "shortest": null,
                      "use_distance": null,
                      "disable_hierarchy_pruning": null,
                      "top_speed": null,
                      "fixed_speed": null,
                      "closure_factor": null,
                      "ignore_closures": null,
                      "ignore_restrictions": null,
                      "ignore_oneways": null,
                      "ignore_non_vehicular_restrictions": null,
                      "ignore_access": null,
                      "speed_types": null,
                      "height": null,
                      "width": null,
                      "exclude_unpaved": null,
                      "exclude_cash_only_tolls": null,
                      "include_hov2": null,
                      "include_hov3": null,
                      "include_hot": null
                    },
               "locations": [],
               "units": "kilometers",
               "id": "",
               "language": "",
               "directions_type": "instructions",
               "alternates": 0,
               "exclude_locations": [],
               "exclude_polygons": [],
               "linear_references": null,
               "prioritize_bidirectional": null,
               "roundabout_exits": null
            })
        )
    }
}
