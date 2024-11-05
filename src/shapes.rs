use serde::{Deserialize,Serialize};

/// Specifies the optional format for the path shape of each connection
#[derive(Serialize,Debug, Clone, Copy)]
pub enum ShapeFormat{
    #[serde(rename = "polyline6")]
    Polyline6,
    #[serde(rename = "polyline5")]
    Polyline5,
    #[serde(rename = "geojson")]
    GeoJSON,
    #[serde(rename = "no_shape")]
    NoShape
}

#[derive(Debug, Clone)]
pub struct ShapePoint {
    lon: f64,
    lat: f64,
}

impl From<&ShapePoint> for geo_types::Point {
    fn from(p: &ShapePoint) -> Self {
        Self::new(p.lon, p.lat)
    }
}

fn decode_shape(encoded: &str) -> Vec<ShapePoint> {
    let inv = 1.0 / 1e6;
    let mut decoded = Vec::new();
    let mut previous = [0, 0];
    let mut i = 0;

    while i < encoded.len() {
        let mut ll = [0, 0];

        for j in 0..2 {
            let mut shift = 0;
            let mut byte = 0x20;

            while byte >= 0x20 {
                byte = i32::from(encoded.as_bytes()[i]) - 63;
                i += 1;
                ll[j] |= (byte & 0x1f) << shift;
                shift += 5;
            }

            ll[j] = previous[j] + ((!ll[j] >> 1) ^ -(ll[j] & 1));
            previous[j] = ll[j];
        }

        decoded.push(ShapePoint {
            lon: -1.0 * f64::from(ll[1]) * inv,
            lat: -1.0 * f64::from(ll[0]) * inv,
        });
    }

    decoded
}
pub(crate) fn deserialize_shape<'de, D>(deserializer: D) -> Result<Vec<ShapePoint>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(decode_shape(s.as_str()))
}
