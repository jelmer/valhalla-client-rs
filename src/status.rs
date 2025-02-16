use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Default, Debug)]
pub struct Manifest {
    verbose: Option<bool>,
}
impl Manifest {
    pub fn builder() -> Self {
        Default::default()
    }

    /// If set to `true` will add [`VerboseStatus`] to the output.
    ///
    /// **Note:**
    /// Gathering this additional information can be computationally expensive.
    /// Hence, the verbose flag can be disallowed in the configuration JSON.
    /// See `service_limits.status.allow_verbose`, with default `false`.
    ///
    /// Default: `false`
    pub fn verbose_output(mut self, verbose: bool) -> Self {
        self.verbose = Some(verbose);
        self
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Response {
    /// Current Valhalla version
    ///
    /// Example: `3.1.4`
    pub version: semver::Version,
    /// Time the tile_extract or tile_dir were last modified
    #[serde(with = "chrono::serde::ts_seconds")]
    pub tileset_last_modified: chrono::DateTime<chrono::Utc>,
    /// Actions which are available to a consumer.
    ///
    /// Can be used in applications to enable/disable parts of the UI such as an elevation map.
    /// Example: `["expansion","height","status","trace_attributes","trace_route","optimized_route","sources_to_targets","isochrone","route","locate"]`
    pub available_actions: HashSet<String>,
    /// Verbose information about the deployment
    ///
    /// Only included if
    /// - requested via [`Manifest::verbose_output`] (default: `false`) and
    /// - allowed via the configuration option `service_limits.status.allow_verbose` (default: `false`)
    #[serde(flatten)]
    pub verbose: Option<VerboseStatus>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct VerboseStatus {
    /// Whether a valid tileset is currently loaded
    pub has_tiles: bool,
    /// Whether the current tileset was built using the admin database
    pub has_admins: bool,
    /// Whether the current tileset was built using the timezone database
    pub has_timezones: bool,
    /// Whether live traffic tiles are currently available
    pub has_live_traffic: bool,
    /// GeoJSON of the tileset extent
    ///
    /// This is likely humongous, be cautions
    pub bbox: Value,
    /// May contain warning objects informing about
    /// - deprecated request parameters,
    /// - clamped values
    /// - etc.
    #[serde(default = "Vec::new")]
    pub warnings: Vec<Value>,
}

#[cfg(all(test, feature = "blocking"))]
mod tests {
    use super::*;
    use crate::blocking::Valhalla;
    #[test]
    fn test_status_verbose() {
        let request = Manifest::builder().verbose_output(true);
        let response = Valhalla::default().status(request).unwrap();
        assert!(response.version >= semver::Version::parse("3.1.4").unwrap());
        assert!(response.tileset_last_modified.timestamp() > 0);
        let verbose = response.verbose.unwrap();
        assert!(verbose.bbox.is_object());
        assert!(verbose.warnings.is_empty());
    }
}
