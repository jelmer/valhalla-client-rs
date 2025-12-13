//! Transit-specific costing options
use serde::Serialize;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, Default, PartialEq)]
pub(crate) struct TransitCostingOptionsInner {
    use_bus: Option<f32>,
    use_rail: Option<f32>,
    use_transfers: Option<f32>,
    filters: Option<Filters>,
}

#[derive(Serialize, Debug, Clone, Default, PartialEq)]
/// Transit costing options
pub struct TransitCostingOptions {
    pub(crate) transit: TransitCostingOptionsInner,
}
impl TransitCostingOptions {
    #[must_use]
    /// Builder for [`TransitCostingOptions`]
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
    pub fn filter_stops<S>(mut self, ids: impl IntoIterator<Item = S>, action: Action) -> Self
    where
        S: Into<String>,
    {
        let new_filter = Filter {
            ids: ids.into_iter().map(Into::into).collect(),
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
    pub fn filter_routes<S>(mut self, ids: impl IntoIterator<Item = S>, action: Action) -> Self
    where
        S: Into<String>,
    {
        let new_filter = Filter {
            ids: ids.into_iter().map(Into::into).collect(),
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
    pub fn filter_operators<S>(mut self, ids: impl IntoIterator<Item = S>, action: Action) -> Self
    where
        S: Into<String>,
    {
        let new_filter = Filter {
            ids: ids.into_iter().map(Into::into).collect(),
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

#[derive(Serialize, Debug, Clone, Copy, Default, PartialEq, Eq)]
/// Action to take when filtering
pub enum Action {
    /// Include only the `ids` listed in the filter
    #[default]
    Include,
    /// Exclude all the `ids` listed in the filter
    Exclude,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, Default, PartialEq)]
struct Filters {
    routes: Option<Filter>,
    operators: Option<Filter>,
    stops: Option<Filter>,
}

#[derive(Serialize, Debug, Clone, Default, PartialEq, Eq)]
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

    #[test]
    fn builder_returns_default() {
        assert_eq!(
            TransitCostingOptions::builder(),
            TransitCostingOptions::default()
        );
    }

    #[test]
    fn use_bus_sets_value() {
        let opts = TransitCostingOptions::builder().use_bus(0.8);
        assert_eq!(opts.transit.use_bus, Some(0.8));
    }

    #[test]
    fn use_rail_sets_value() {
        let opts = TransitCostingOptions::builder().use_rail(0.9);
        assert_eq!(opts.transit.use_rail, Some(0.9));
    }

    #[test]
    fn use_transfers_sets_value() {
        let opts = TransitCostingOptions::builder().use_transfers(0.5);
        assert_eq!(opts.transit.use_transfers, Some(0.5));
    }

    #[test]
    fn filter_routes_sets_value() {
        let opts = TransitCostingOptions::builder()
            .filter_routes(vec!["NYC_AUR", "NYC_BX"], Action::Include);
        assert!(opts.transit.filters.is_some());
        let filters = opts.transit.filters.unwrap();
        assert!(filters.routes.is_some());
        let route_filter = filters.routes.unwrap();
        assert_eq!(route_filter.ids, vec!["NYC_AUR", "NYC_BX"]);
        assert_eq!(route_filter.action, Action::Include);
    }

    #[test]
    fn filter_operators_sets_value() {
        let opts =
            TransitCostingOptions::builder().filter_operators(vec!["NYC_MTA"], Action::Exclude);
        assert!(opts.transit.filters.is_some());
        let filters = opts.transit.filters.unwrap();
        assert!(filters.operators.is_some());
        let op_filter = filters.operators.unwrap();
        assert_eq!(op_filter.ids, vec!["NYC_MTA"]);
        assert_eq!(op_filter.action, Action::Exclude);
    }

    #[test]
    fn chaining_works() {
        let opts = TransitCostingOptions::builder()
            .use_bus(0.7)
            .use_rail(0.9)
            .use_transfers(0.4);
        assert_eq!(opts.transit.use_bus, Some(0.7));
        assert_eq!(opts.transit.use_rail, Some(0.9));
        assert_eq!(opts.transit.use_transfers, Some(0.4));
    }
}
