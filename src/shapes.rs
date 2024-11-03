use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct ShapePoint {
    lon: f64,
    lat: f64,
}

impl From<&ShapePoint> for geo_types::Point {
    fn from(p: &ShapePoint) -> Self {
        geo_types::Point::new(p.lon, p.lat)
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
                byte = encoded.as_bytes()[i] as i32 - 63;
                i += 1;
                ll[j] |= (byte & 0x1f) << shift;
                shift += 5;
            }

            ll[j] = previous[j] + ((!ll[j] >> 1) ^ -(ll[j] & 1));
            previous[j] = ll[j];
        }

        decoded.push(ShapePoint {
            lon: -1.0 * ll[1] as f64 * inv,
            lat: -1.0 * ll[0] as f64 * inv,
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
