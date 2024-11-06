use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TransitCostingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    use_bus: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_rail: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_transfers: Option<f32>,
    #[serde(skip_serializing_if = "Filters::is_empty")]
    filters: Filters,
}
impl TransitCostingOptions {
    pub fn builder() -> Self {
        Default::default()
    }
    /// User's desire to use buses.
    ///
    /// Range of values from
    /// - `0` (try to avoid buses) to
    /// - `1` (strong preference for riding buses).
    pub fn use_bus(mut self, use_bus: f32) -> Self {
        self.use_bus = Some(use_bus);
        self
    }
    /// User's desire to use rail/subway/metro.
    ///
    /// Range of values from
    /// - `0` (try to avoid rail) to
    /// - `1` (strong preference for riding rail).
    pub fn use_rail(mut self, use_rail: f32) -> Self {
        self.use_rail = Some(use_rail);
        self
    }
    /// User's desire to favor transfers.
    ///
    /// Range of values from
    /// - `0` (try to avoid transfers) to
    /// - `1` (totally comfortable with transfers).
    pub fn use_transfers(mut self, use_transfers: f32) -> Self {
        self.use_transfers = Some(use_transfers);
        self
    }
    /// Sets a filter for one or more ~~`stops`~~ (TODO: need to re-enable)
    ///
    /// Filters must contain a list of so-called Onestop IDs, which is (supposed to be) a
    /// unique identifier for GTFS data, and an [`Action`].
    /// The OneStop ID is simply the feeds's directory name and the object's GTFS ID separated
    /// by an underscore.
    ///
    /// Example:
    /// A route with `route_id: AUR` in `routes.txt` from the feed `NYC` would have
    /// the OneStop ID `NYC_AUR`, similar with operators/agencies
    ///
    /// **Tip**: Can be combined with [`Self::filter_routes`] and/or [`Self::filter_operators`]
    #[doc(hidden)] // TODO: enable once this works in valhalla
    pub fn filter_stops(
        mut self,
        ids: impl IntoIterator<Item = impl ToString>,
        action: Action,
    ) -> Self {
        self.filters.stops = Some(Filter {
            ids: ids.into_iter().map(|s| s.to_string()).collect(),
            action,
        });
        self
    }
    /// Sets a filter for one or more `routes`
    ///
    /// Filters must contain a list of so-called Onestop IDs, which is (supposed to be) a
    /// unique identifier for GTFS data, and an [`Action`].
    /// The OneStop ID is simply the feeds's directory name and the object's GTFS ID separated
    /// by an underscore.
    ///
    /// Example:
    /// A route with `route_id: AUR` in `routes.txt` from the feed `NYC` would have
    /// the OneStop ID `NYC_AUR`, similar with operators/agencies
    ///
    /// **Tip**: Can be combined with [`Self::filter_stops`] and/or [`Self::filter_operators`]
    pub fn filter_routes<S>(
        mut self,
        ids: impl IntoIterator<Item = impl ToString>,
        action: Action,
    ) -> Self {
        self.filters.routes = Some(Filter {
            ids: ids.into_iter().map(|s| s.to_string()).collect(),
            action,
        });
        self
    }
    /// Sets a filter for one or more `operators`.
    ///
    /// Filters must contain a list of so-called Onestop IDs, which is (supposed to be) a
    /// unique identifier for GTFS data, and an [`Action`].
    /// The OneStop ID is simply the feeds's directory name and the object's GTFS ID separated
    /// by an underscore.
    ///
    /// Example:
    /// A route with `route_id: AUR` in `routes.txt` from the feed `NYC` would have
    /// the OneStop ID `NYC_AUR`, similar with operators/agencies
    ///
    /// **Tip**: Can be combined with [`Self::filter_stops`] and/or [`Self::filter_routes`]
    pub fn filter_operators(
        mut self,
        ids: impl IntoIterator<Item = impl ToString>,
        action: Action,
    ) -> Self {
        self.filters.operators = Some(Filter {
            ids: ids.into_iter().map(|s| s.to_string()).collect(),
            action,
        });
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub enum Action {
    /// Include only the `ids` listed in the filter
    #[default]
    Include,
    /// Exclude all the `ids` listed in the filter
    Exclude,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Filters {
    #[serde(skip_serializing_if = "Option::is_none")]
    routes: Option<Filter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operators: Option<Filter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stops: Option<Filter>,
}
impl Filters {
    fn is_empty(&self) -> bool {
        self.routes.is_none() && self.operators.is_none() && self.stops.is_none()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Filter {
    ids: Vec<String>,
    action: Action,
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn serialisation(){
        insta::assert_json_snapshot!(TransitCostingOptions::default(),@"{}")
    }
}
