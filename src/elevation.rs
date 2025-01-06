pub use crate::shapes::ShapePoint;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Default, Debug, PartialEq, Clone)]
pub struct Manifest {
    id: Option<String>,
    height_precision: Option<HeightPrecision>,
    range: Option<bool>,
    resample_distance: Option<f64>,
    shape: Option<Vec<ShapePoint>>,
    encoded_polyline: Option<String>,
    shape_format: Option<ShapeFormat>,
}
impl Manifest {
    pub fn builder() -> Self {
        Default::default()
    }
    /// Name your route request.
    ///
    /// If id is specified, the naming will be sent through to the response.
    pub fn id(mut self, id: impl ToString) -> Self {
        self.id = Some(id.to_string());
        self
    }
    /// Controls whether the returned array is one-dimensional (height only, the default) or two-dimensional (range and height).
    ///
    /// This can be used to generate a graph along a route, because a 2D-array has values for x (the range) and y (the height) at each shape point.
    /// Steepness or gradient can also be computed from a profile request (e.g. when range = `true`).
    ///
    /// Default: `false`
    pub fn include_range(mut self) -> Self {
        self.range = Some(true);
        self
    }
    /// Specifying the distance (in meters) at which the input polyline is sampled in order to provide uniform distances between samples along the polyline.
    pub fn resample_distance(mut self, resample_distance_meters: f64) -> Self {
        self.resample_distance = Some(resample_distance_meters);
        self
    }
    /// Allows increasing the returned precision for heights.
    ///
    /// The default of returning as integer values works fine for most cases.
    /// However, when charting elevation results along a nearly flat road can lead to "stair step" changes in elevation.
    ///
    /// Default: [`HeightPrecision::ZeroDecimalPlaces`]
    pub fn height_precision(mut self, height_precision: HeightPrecision) -> Self {
        self.height_precision = Some(height_precision);
        self
    }
    /// Specifies whether the polyline is encoded with
    /// - 6 digit precision ([`ShapeFormat::Polyline6`]) or
    /// - 5 digit precision ([`ShapeFormat::Polyline5`]).
    ///
    /// Default: [`ShapeFormat::Polyline6`], meaning the encoded polyline is expected to be 6 digit precision.
    pub fn shape_format(mut self, shape_format: ShapeFormat) -> Self {
        debug_assert!(self.shape.is_none(), "shape is set and setting the shape_format is requested. This combination does not make sense: shapes and encoded_polylines as input are mutually exclusive.");
        self.shape_format = Some(shape_format);
        self
    }
    /// Latitudes/longitudes where the elevation data is desired in decimal degrees.
    ///
    /// The input coordinates can come from many input sources, such as a GPS location, a point or a click on a map, a geocoding service, and so on.
    /// The locations are visited in the order specified.
    pub fn shape(mut self, shape: impl IntoIterator<Item = impl Into<ShapePoint>>) -> Self {
        debug_assert!(self.shape_format.is_none(), "shape_format is set and setting a shape is requested. This combination does not make sense: shapes and encoded_polylines as input are mutually exclusive.");
        debug_assert!(self.encoded_polyline.is_none(), "encoded_polyline is set and setting a shape is requested. This combination does not make sense: shapes and encoded_polylines as input are mutually exclusive.");
        self.shape = Some(shape.into_iter().map(|s| s.into()).collect());
        self
    }
    /// A set of polyline encoded latitude/longitude pairs of a line or shape where the elevation data is desired.
    ///
    /// Details on polyline encoding and decoding can be found [here](https://valhalla.github.io/valhalla/decoding/).
    /// See [`Self::shape_format`] to set the precision of the polyline.
    pub fn encoded_polyline(mut self, encoded_polyline: impl ToString) -> Self {
        debug_assert!(self.shape.is_none(), "shape is set and setting the encoded_polyline is requested. This combination does not make sense: shapes and encoded_polylines as input are mutually exclusive.");
        self.encoded_polyline = Some(encoded_polyline.to_string());
        self
    }
}

/// Specifies the precision (number of decimal places) of all returned height values.
#[derive(serde_repr::Serialize_repr, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum HeightPrecision {
    /// Zero decimal places (="integer precision") of precision for all height values
    ///
    /// Example: `123`
    #[default]
    ZeroDecimalPlaces = 0,
    /// One decimal places of precision for all height values
    ///
    /// Example: `123.3`
    OneDecimalPlace,
    /// Two decimal places of precision for all height values
    ///
    /// Example: `123.34`
    TwoDecimalPlaces,
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShapeFormat {
    /// polyline is encoded with 6 digit precision
    #[serde(rename = "polyline6")]
    #[default]
    Polyline6,
    /// polyline is encoded with 5 digit precision
    #[serde(rename = "polyline5")]
    Polyline5,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Response {
    /// Name of the route request.
    ///
    /// If id is specified via [`Manifest::id`] the naming will be sent through to the response.
    pub id: Option<String>,
    /// The specified shape coordinates from the input request.
    ///
    /// `None` if requested enabled via [`Manifest::shape`]
    pub shape: Option<Vec<ShapePoint>>,
    /// The specified encoded polyline, with six degrees of precision, coordinates from the input request.
    ///
    /// `None` if requested enabled via [`Manifest::encoded_polyline`]
    pub encoded_polyline: Option<String>,
    /// The 2D array of range (x) and height (y) per input latitude, longitude coordinate.
    ///
    /// `None` if not enabled via [`Manifest::include_range`]
    pub range_height: Vec<Option<(f64, Option<f64>)>>,
    /// The range or distance along the input locations.
    ///
    /// It is the cumulative distance along the previous latitiude, longitude coordinates up to the current coordinate.
    /// The x-value for the first coordinate in the shape will always be 0.
    pub x_coordinate: Option<f64>,
    /// The height or elevation of the associated latitude, longitude pair.
    ///
    /// The height is returned as null if no height data exists for a given location.
    pub y_coordinate: Option<f64>,
    /// An array of height for the associated latitude, longitude coordinates.
    #[serde(default = "Vec::new")]
    pub height: Vec<f32>,
    /// This array may contain warning objects informing about deprecated
    /// - request parameters,
    /// - clamped values
    /// - etc.
    #[serde(default = "Vec::new")]
    pub warnings: Vec<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn serialisation() {
        let manifest = Manifest::builder();
        assert_eq!(
            serde_json::to_value(&manifest).unwrap(),
            serde_json::json!({})
        );
    }

    #[test]
    fn test_serialize_encoded_polyline() {
        let manifest = Manifest::builder()
            .id("some_id")
            .height_precision(HeightPrecision::OneDecimalPlace)
            .include_range()
            .encoded_polyline("polyline")
            .shape_format(ShapeFormat::Polyline6);
        assert_eq!(
            serde_json::to_value(&manifest).unwrap(),
            serde_json::json!({"id":"some_id","height_precision":1,"range":true,"encoded_polyline":"polyline","shape_format":"polyline6"})
        );
    }
}
