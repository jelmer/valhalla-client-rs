//! Models connected to the [`trace_attributes`] map-matching API
//!
//! See <https://valhalla.github.io/valhalla/api/map-matching/api-reference/> for details.

use crate::costing;
pub use crate::shapes::ShapePoint;
use serde::{Deserialize, Serialize};

/// A shape point for the trace_attributes request.
#[derive(Serialize, Debug, Clone)]
pub struct TracePoint {
    /// Latitude in degrees
    pub lat: f64,
    /// Longitude in degrees
    pub lon: f64,
}

impl TracePoint {
    /// Create a new trace point
    pub fn new(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }
}

/// How to match the shape to the road network.
#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum ShapeMatch {
    /// Try edge walking first, fall back to map matching
    #[default]
    WalkOrSnap,
    /// Use map matching algorithm
    MapSnap,
    /// Use edge walking algorithm (requires very precise input)
    EdgeWalk,
}

/// Filter to include or exclude specific attributes in the response.
#[derive(Serialize, Debug, Clone)]
pub struct Filter {
    /// List of attribute names to include or exclude
    pub attributes: Vec<String>,
    /// Whether to include or exclude the listed attributes
    pub action: FilterAction,
}

/// Whether to include or exclude filtered attributes.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FilterAction {
    /// Include only the listed attributes
    Include,
    /// Exclude the listed attributes
    Exclude,
}

/// Request manifest for the trace_attributes API.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone)]
pub struct Manifest {
    /// Input shape points to match
    pub shape: Vec<TracePoint>,
    /// Costing model to use
    #[serde(flatten)]
    pub costing: costing::Costing,
    /// Shape matching mode
    pub shape_match: ShapeMatch,
    /// Attribute filter
    pub filters: Option<Filter>,
}

impl Manifest {
    /// Create a builder with the given shape points and costing.
    pub fn builder(shape: impl IntoIterator<Item = TracePoint>, costing: costing::Costing) -> Self {
        Self {
            shape: shape.into_iter().collect(),
            costing,
            shape_match: ShapeMatch::default(),
            filters: None,
        }
    }

    /// Set the shape matching mode.
    pub fn shape_match(mut self, shape_match: ShapeMatch) -> Self {
        self.shape_match = shape_match;
        self
    }

    /// Set the attribute filter to include specific edge attributes.
    pub fn include_attributes(
        mut self,
        attributes: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.filters = Some(Filter {
            attributes: attributes.into_iter().map(|a| a.into()).collect(),
            action: FilterAction::Include,
        });
        self
    }
}

/// Surface type of a road edge.
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Surface {
    /// Smooth paved surface
    PavedSmooth,
    /// Paved surface
    Paved,
    /// Rough paved surface
    PavedRough,
    /// Compacted surface
    Compacted,
    /// Dirt surface
    Dirt,
    /// Gravel surface
    Gravel,
    /// Path surface
    Path,
    /// Impassable surface
    Impassable,
}

/// Road classification of an edge.
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RoadClass {
    /// Motorway
    Motorway,
    /// Trunk road
    Trunk,
    /// Primary road
    Primary,
    /// Secondary road
    Secondary,
    /// Tertiary road
    Tertiary,
    /// Unclassified road
    Unclassified,
    /// Residential road
    Residential,
    /// Service or other road
    ServiceOther,
}

/// Use type of an edge.
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EdgeUse {
    /// Standard road
    Road,
    /// Ramp (highway on/off)
    Ramp,
    /// Turn channel
    TurnChannel,
    /// Track
    Track,
    /// Driveway
    Driveway,
    /// Alley
    Alley,
    /// Parking aisle
    ParkingAisle,
    /// Emergency access
    EmergencyAccess,
    /// Drive through
    DriveThrough,
    /// Cul-de-sac
    Culdesac,
    /// Cycleway
    Cycleway,
    /// Mountain bike trail
    MountainBike,
    /// Sidewalk
    Sidewalk,
    /// Footway
    Footway,
    /// Steps/stairs
    Steps,
    /// Ferry
    Ferry,
    /// Rail ferry
    #[serde(rename = "rail-ferry")]
    RailFerry,
    /// Other use
    Other,
}

/// A matched edge in the trace_attributes response.
#[derive(Deserialize, Debug, Clone)]
pub struct Edge {
    /// Road surface type
    #[serde(default)]
    pub surface: Option<Surface>,
    /// Road classification
    #[serde(default)]
    pub road_class: Option<RoadClass>,
    /// Edge use type
    #[serde(default)]
    pub r#use: Option<EdgeUse>,
    /// Length of the edge in the response units (km or miles)
    #[serde(default)]
    pub length: Option<f64>,
    /// Road names
    #[serde(default)]
    pub names: Option<Vec<String>>,
    /// Index into the response shape where this edge begins
    #[serde(default)]
    pub begin_shape_index: Option<u32>,
    /// Index into the response shape where this edge ends
    #[serde(default)]
    pub end_shape_index: Option<u32>,
    /// OSM way ID
    #[serde(default)]
    pub way_id: Option<u64>,
    /// Percentage along the edge where the source point lies (first edge only)
    #[serde(default)]
    pub source_percent_along: Option<f64>,
    /// Percentage along the edge where the target point lies (last edge only)
    #[serde(default)]
    pub target_percent_along: Option<f64>,
}

/// A matched point in the trace_attributes response.
#[derive(Deserialize, Debug, Clone)]
pub struct MatchedPoint {
    /// Latitude of the matched point
    pub lat: f64,
    /// Longitude of the matched point
    pub lon: f64,
    /// Match type
    #[serde(default)]
    pub r#type: Option<String>,
    /// Index of the edge this point was matched to
    #[serde(default)]
    pub edge_index: Option<u32>,
    /// Distance along the edge
    #[serde(default)]
    pub distance_along_edge: Option<f64>,
}

/// Response from the trace_attributes API.
#[derive(Deserialize, Debug, Clone)]
pub struct Response {
    /// Matched edges with attributes
    #[serde(default)]
    pub edges: Vec<Edge>,
    /// Matched input points
    #[serde(default)]
    pub matched_points: Vec<MatchedPoint>,
    /// Encoded polyline of the matched path
    #[serde(default)]
    pub shape: Option<String>,
    /// Units used in the response
    #[serde(default)]
    pub units: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_manifest() {
        let manifest = Manifest::builder(
            [TracePoint::new(48.1, 11.5), TracePoint::new(48.2, 11.6)],
            costing::Costing::Auto(Default::default()),
        );
        let value = serde_json::to_value(&manifest).unwrap();
        assert_eq!(
            value,
            serde_json::json!({
                "shape": [{"lat": 48.1, "lon": 11.5}, {"lat": 48.2, "lon": 11.6}],
                "costing": "auto",
                "costing_options": {"auto": {}},
                "shape_match": "walk_or_snap"
            })
        );
    }

    #[test]
    fn test_serialize_manifest_with_filter() {
        let manifest = Manifest::builder(
            [TracePoint::new(48.1, 11.5)],
            costing::Costing::Pedestrian(Default::default()),
        )
        .shape_match(ShapeMatch::MapSnap)
        .include_attributes(["edge.surface", "edge.road_class"]);
        let value = serde_json::to_value(&manifest).unwrap();
        assert_eq!(
            value,
            serde_json::json!({
                "shape": [{"lat": 48.1, "lon": 11.5}],
                "costing": "pedestrian",
                "costing_options": {"pedestrian": {}},
                "shape_match": "map_snap",
                "filters": {
                    "attributes": ["edge.surface", "edge.road_class"],
                    "action": "include"
                }
            })
        );
    }

    #[test]
    fn test_deserialize_response() {
        let json = serde_json::json!({
            "edges": [{
                "surface": "paved",
                "road_class": "primary",
                "use": "road",
                "length": 0.123,
                "names": ["Main Street"],
                "begin_shape_index": 0,
                "end_shape_index": 5,
                "way_id": 12345,
                "source_percent_along": 0.1,
                "target_percent_along": 0.9
            }],
            "matched_points": [{
                "lat": 48.1,
                "lon": 11.5,
                "type": "matched",
                "edge_index": 0,
                "distance_along_edge": 0.5
            }],
            "shape": "encoded_shape_string",
            "units": "km"
        });
        let response: Response = serde_json::from_value(json).unwrap();
        assert_eq!(response.edges.len(), 1);
        assert_eq!(response.edges[0].surface, Some(Surface::Paved));
        assert_eq!(response.edges[0].road_class, Some(RoadClass::Primary));
        assert_eq!(response.edges[0].r#use, Some(EdgeUse::Road));
        assert_eq!(response.edges[0].length, Some(0.123));
        assert_eq!(
            response.edges[0].names,
            Some(vec!["Main Street".to_string()])
        );
        assert_eq!(response.edges[0].begin_shape_index, Some(0));
        assert_eq!(response.edges[0].end_shape_index, Some(5));
        assert_eq!(response.edges[0].way_id, Some(12345));
        assert_eq!(response.edges[0].source_percent_along, Some(0.1));
        assert_eq!(response.edges[0].target_percent_along, Some(0.9));
        assert_eq!(response.matched_points.len(), 1);
        assert_eq!(response.matched_points[0].lat, 48.1);
        assert_eq!(response.matched_points[0].lon, 11.5);
        assert_eq!(
            response.matched_points[0].r#type,
            Some("matched".to_string())
        );
        assert_eq!(response.matched_points[0].edge_index, Some(0));
        assert_eq!(response.matched_points[0].distance_along_edge, Some(0.5));
        assert_eq!(response.shape, Some("encoded_shape_string".to_string()));
        assert_eq!(response.units, Some("km".to_string()));
    }

    #[test]
    fn test_deserialize_response_with_defaults() {
        let json = serde_json::json!({});
        let response: Response = serde_json::from_value(json).unwrap();
        assert_eq!(response.edges.len(), 0);
        assert_eq!(response.matched_points.len(), 0);
        assert_eq!(response.shape, None);
        assert_eq!(response.units, None);
    }

    #[test]
    fn test_deserialize_edge_with_defaults() {
        let json = serde_json::json!({});
        let edge: Edge = serde_json::from_value(json).unwrap();
        assert_eq!(edge.surface, None);
        assert_eq!(edge.road_class, None);
        assert_eq!(edge.r#use, None);
        assert_eq!(edge.length, None);
        assert_eq!(edge.names, None);
        assert_eq!(edge.way_id, None);
    }

    #[test]
    fn test_serialize_shape_match_variants() {
        assert_eq!(
            serde_json::to_value(ShapeMatch::WalkOrSnap).unwrap(),
            serde_json::json!("walk_or_snap")
        );
        assert_eq!(
            serde_json::to_value(ShapeMatch::MapSnap).unwrap(),
            serde_json::json!("map_snap")
        );
        assert_eq!(
            serde_json::to_value(ShapeMatch::EdgeWalk).unwrap(),
            serde_json::json!("edge_walk")
        );
    }
}
