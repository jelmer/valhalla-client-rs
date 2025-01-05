use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub(crate) struct TransitCostingOptionsInner {
    use_bus: Option<f32>,
    use_rail: Option<f32>,
    use_transfers: Option<f32>,
    filters: Option<Filters>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TransitCostingOptions {
    pub(crate) transit: TransitCostingOptionsInner,
}
impl TransitCostingOptions {
    #[must_use]
    pub fn builder() -> Self {
        Self::default()
    }
    /// User's desire to use buses.
    ///
    /// Range of values from
    /// - `0` (try to avoid buses) to
    /// - `1` (strong preference for riding buses).
    pub fn use_bus(mut self, use_bus: f32) -> Self {
        self.transit.use_bus = Some(use_bus);
        self
    }
    /// User's desire to use rail/subway/metro.
    ///
    /// Range of values from
    /// - `0` (try to avoid rail) to
    /// - `1` (strong preference for riding rail).
    pub fn use_rail(mut self, use_rail: f32) -> Self {
        self.transit.use_rail = Some(use_rail);
        self
    }
    /// User's desire to favor transfers.
    ///
    /// Range of values from
    /// - `0` (try to avoid transfers) to
    /// - `1` (totally comfortable with transfers).
    pub fn use_transfers(mut self, use_transfers: f32) -> Self {
        self.transit.use_transfers = Some(use_transfers);
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
        let new_filter = Filter {
            ids: ids.into_iter().map(|s| s.to_string()).collect(),
            action,
        };
        if let Some(ref mut filters) = self.transit.filters {
            filters.stops = Some(new_filter);
        } else {
            self.transit.filters = Some(Filters {
                stops: Some(new_filter),
                ..Default::default()
            });
        }
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
    pub fn filter_routes(
        mut self,
        ids: impl IntoIterator<Item = impl ToString>,
        action: Action,
    ) -> Self {
        let new_filter = Filter {
            ids: ids.into_iter().map(|s| s.to_string()).collect(),
            action,
        };
        if let Some(ref mut filters) = self.transit.filters {
            filters.routes = Some(new_filter);
        } else {
            self.transit.filters = Some(Filters {
                routes: Some(new_filter),
                ..Default::default()
            });
        }
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
        let new_filter = Filter {
            ids: ids.into_iter().map(|s| s.to_string()).collect(),
            action,
        };
        if let Some(ref mut filters) = self.transit.filters {
            filters.operators = Some(new_filter);
        } else {
            self.transit.filters = Some(Filters {
                operators: Some(new_filter),
                ..Default::default()
            });
        }
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

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Filters {
    routes: Option<Filter>,
    operators: Option<Filter>,
    stops: Option<Filter>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Filter {
    ids: Vec<String>,
    action: Action,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn serialisation() {
        assert_eq!(
            serde_json::to_value(TransitCostingOptions::default()).unwrap(),
            serde_json::json!({"transit":{}})
        );
    }
}
