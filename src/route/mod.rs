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
    pub costing: costing::Costing,

    pub locations: Vec<Location>,

    /// Distance units for output. Allowable unit types are miles (or mi) and kilometers (or km).
    /// If no unit type is specified, the units default to kilometers.
    pub units: Units,

    /// Name your route request. If id is specified, the naming will be sent thru to the response.
    pub id: String,

    /// The language of the narration instructions based on the IETF BCP 47 language tag string. If
    /// no language is specified or the specified language is unsupported, United States-based
    /// English (en-US) is used. Currently supported language list
    pub language: String,

    pub directions_type: DirectionsType,

    /// A number denoting how many alternate routes should be provided. There may be no alternates
    /// or less alternates than the user specifies. Alternates are not yet supported on multipoint
    /// routes (that is, routes with more than 2 locations). They are also not supported on time
    /// dependent routes.
    pub alternates: i32,

    /// A set of locations to exclude or avoid within a route can be specified using a JSON array
    /// of avoid_locations. The avoid_locations have the same format as the locations list. At a
    /// minimum each avoid location must include latitude and longitude. The avoid_locations are
    /// mapped to the closest road or roads and these roads are excluded from the route path
    /// computation.
    pub exclude_locations: Vec<Location>,

    /// One or multiple exterior rings of polygons in the form of nested JSON arrays.
    ///
    /// Example: `[[[lon1, lat1], [lon2,lat2]],[[lon1,lat1],[lon2,lat2]]]`
    /// Roads intersecting these rings will be
    /// avoided during path finding. If you only need to avoid a few specific roads, it's much more
    /// efficient to use exclude_locations. Valhalla will close open rings (i.e. copy the first
    /// coordinate to the last position).
    pub exclude_polygons: Vec<Vec<(f64, f64)>>,

    /// When present and true, the successful route response will include a key linear_references.
    /// Its value is an array of base64-encoded OpenLR location references, one for each graph edge
    /// of the road network matched by the input trace.
    pub linear_references: bool,

    /// Prioritize bidirectional a* when date_time.type = depart_at/current. By default
    /// time_dependent_forward a* is used in these cases, but bidirectional a* is much faster.
    /// Currently, it does not update the time (and speeds) when searching for the route path, but
    /// the ETA on that route is recalculated based on the time-dependent speeds
    pub prioritize_bidirectional: bool,

    /// A boolean indicating whether exit instructions at roundabouts should be added to the output
    /// or not. Default is true.
    pub roundabout_exits: bool,
}

impl Manifest {
    pub fn builder() -> Self {
        Self::default()
    }
    /// Configures the costing model
    ///
    /// Valhalla's routing service uses dynamic, run-time costing to generate the route path.
    /// Can be configured with different settings depending on the costing model used.
    /// Default: [`costing::Costing::Auto`]
    pub fn costing(mut self, costing: costing::Costing) -> Self {
        self.costing = costing;
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
        insta::assert_json_snapshot!(manifest, @r#"
        {
          "costing": "auto",
          "costing_options": {},
          "locations": [],
          "units": "kilometers",
          "id": "",
          "language": "",
          "directions_type": "instructions",
          "alternates": 0,
          "exclude_locations": [],
          "exclude_polygons": [],
          "linear_references": false,
          "prioritize_bidirectional": false,
          "roundabout_exits": false
        }
        "#)
    }
}
