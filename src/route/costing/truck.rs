use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TruckCostingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    maneuver_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gate_cost: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gate_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_access_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_only_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toll_booth_cost: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toll_booth_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ferry_cost: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_ferry: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_highways: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_tolls: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_living_streets: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_tracks: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_factor: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_crossing_cost: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_crossing_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shortest: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_distance: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_hierarchy_pruning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_speed: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_speed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    closure_factor: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_closures: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_restrictions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_oneways: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_non_vehicular_restrictions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_access: Option<bool>,
    // -- ↓ truck only ↓ --
    #[serde(skip_serializing_if = "Option::is_none")]
    length: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    axle_load: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    axle_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hazmat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hgv_no_access_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    low_class_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_truck_route: Option<f32>,
}
impl TruckCostingOptions {
    pub fn builder() -> Self {
        Default::default()
    }

    /// A cost applied when a [gate](http://wiki.openstreetmap.org/wiki/Tag:barrier%3Dgate) with
    /// undefined or private access is encountered.
    ///
    /// This cost is added to the estimated time / elapsed time.
    ///
    /// Default: `30` seconds
    pub fn gate_cost(mut self, gate_cost: f32) -> Self {
        self.gate_cost = Some(gate_cost);
        self
    }
    /// A penalty applied when a [gate](https://wiki.openstreetmap.org/wiki/Tag:barrier%3Dgate) with
    /// no access information is on the road.
    ///
    /// Default: `300` seconds
    pub fn gate_penalty(mut self, gate_penalty: f32) -> Self {
        self.gate_penalty = Some(gate_penalty);
        self
    }
    /// A penalty applied when a [gate](https://wiki.openstreetmap.org/wiki/Tag:barrier%3Dgate) or
    /// [bollard](https://wiki.openstreetmap.org/wiki/Tag:barrier%3Dbollard) with `access=private`
    /// is encountered.
    ///
    /// Default: `450` seconds
    pub fn private_access_penalty(mut self, private_access_penalty: f32) -> Self {
        self.private_access_penalty = Some(private_access_penalty);
        self
    }
    /// A penalty applied when entering a road which is only allowed to enter if necessary to reach
    /// the [destination](https://wiki.openstreetmap.org/wiki/Tag:vehicle%3Ddestination).
    pub fn destination_only_penalty(mut self, destination_only_penalty: f32) -> Self {
        self.destination_only_penalty = Some(destination_only_penalty);
        self
    }
    /// A cost applied when a [toll booth](http://wiki.openstreetmap.org/wiki/Tag:barrier%3Dtoll_booth)
    /// is encountered.
    ///
    /// This cost is added to the estimated and elapsed times.
    ///
    /// Default: `15` seconds
    pub fn toll_booth_cost(mut self, toll_booth_cost: f32) -> Self {
        self.toll_booth_cost = Some(toll_booth_cost);
        self
    }
    /// A penalty applied to the cost when a
    /// [toll booth](http://wiki.openstreetmap.org/wiki/Tag:barrier%3Dtoll_booth) is encountered.
    ///
    /// This penalty can be used to create paths that avoid toll roads.
    ///
    /// Default: `0`
    pub fn toll_booth_penalty(mut self, toll_booth_penalty: f32) -> Self {
        self.toll_booth_penalty = Some(toll_booth_penalty);
        self
    }
    /// A cost applied when entering a ferry.
    ///
    /// This cost is added to the estimated and elapsed times.
    ///
    /// Default: `300` seconds (5 minutes)
    pub fn ferry_cost(mut self, ferry_cost: f32) -> Self {
        self.ferry_cost = Some(ferry_cost);
        self
    }
    /// This value indicates the willingness to take ferries.
    ///
    /// This is a range of values between `0` and `1`:
    /// - Values near `0` attempt to avoid ferries and
    /// - values near `1` will favor ferries.
    ///
    /// **Note:** sometimes ferries are required to complete a route so values of `0` are not guaranteed to avoid ferries entirely.
    ///
    /// Default: `0.5`
    pub fn use_ferry(mut self, use_ferry: f32) -> Self {
        debug_assert!(use_ferry >= 0.0);
        debug_assert!(use_ferry <= 1.0);
        self.use_ferry = Some(use_ferry);
        self
    }
    /// This value indicates the willingness to take highways.
    ///
    /// This is a range of values between `0` and 1:
    /// - Values near `0` attempt to avoid highways and
    /// - values near `1` will favor highways.
    ///
    /// **Note:** sometimes highways are required to complete a route so values of `0` are not guaranteed to avoid highways entirely.
    ///
    /// Default: `1.0`
    pub fn use_highways(mut self, use_highways: f32) -> Self {
        debug_assert!(use_highways >= 0.0);
        debug_assert!(use_highways <= 1.0);
        self.use_highways = Some(use_highways);
        self
    }
    /// This value indicates the willingness to take roads with tolls.
    ///
    /// This is a range of values between `0` and 1:
    /// - Values near `0` attempt to avoid tolls and
    /// - values near `1` will not attempt to avoid them.
    ///
    /// **Note:** sometimes roads with tolls are required to complete a route so values of `0` are not guaranteed to avoid them entirely.
    ///
    /// Default: `0.5`
    pub fn use_tolls(mut self, use_tolls: f32) -> Self {
        debug_assert!(use_tolls >= 0.0);
        debug_assert!(use_tolls <= 1.0);
        self.use_tolls = Some(use_tolls);
        self
    }
    /// This value indicates the willingness to take living streets.
    ///
    /// This is a range of values between `0` and 1:
    /// - Values near `0` attempt to avoid living streets and
    /// - values near `1` will favor living streets.
    ///
    /// **Note:** sometimes living streets are required to complete a route so values of `0` are not guaranteed to avoid living streets entirely.
    ///
    /// Default:
    /// - `truck`: `0`
    /// - `cars`/`buses`/`motor scooters`/`motorcycles`: `0.1`
    pub fn use_living_streets(mut self, use_living_streets: f32) -> Self {
        debug_assert!(use_living_streets >= 0.0);
        debug_assert!(use_living_streets <= 1.0);
        self.use_living_streets = Some(use_living_streets);
        self
    }
    /// This value indicates the willingness to take track roads.
    ///
    /// This is a range of values between `0` and 1:
    /// - Values near `0` attempt to avoid tracks and
    /// - values near `1` will favor tracks a little bit.
    ///
    /// **Note:** sometimes tracks are required to complete a route so values of `0` are not guaranteed to avoid tracks entirely.
    ///
    /// Default:
    /// - `0` for autos,
    /// - `0.5` for motor scooters and motorcycles.
    pub fn use_tracks(mut self, use_tracks: f32) -> Self {
        debug_assert!(use_tracks >= 0.0);
        debug_assert!(use_tracks <= 1.0);
        self.use_tracks = Some(use_tracks);
        self
    }
    /// A penalty applied for transition to generic service road.
    ///
    /// Default: `0`trucks and 15 for cars, buses, motor scooters and motorcycles.
    pub fn service_penalty(mut self, service_penalty: f32) -> Self {
        self.service_penalty = Some(service_penalty);
        self
    }
    /// A factor that modifies (multiplies) the cost when generic service roads are encountered.
    ///
    /// Default: `1`
    pub fn service_factor(mut self, service_factor: f32) -> Self {
        self.service_factor = Some(service_factor);
        self
    }
    /// A cost applied when encountering an international border.
    ///
    /// This cost is added to the estimated and elapsed times.
    ///
    /// Default: `600` seconds
    pub fn country_crossing_cost(mut self, country_crossing_cost: f32) -> Self {
        self.country_crossing_cost = Some(country_crossing_cost);
        self
    }
    /// A penalty applied for a country crossing.
    ///
    /// This penalty can be used to create paths that avoid spanning country boundaries.
    ///
    /// Default: `0`
    pub fn country_crossing_penalty(mut self, country_crossing_penalty: f32) -> Self {
        self.country_crossing_penalty = Some(country_crossing_penalty);
        self
    }
    /// Changes the metric to quasi-shortest, i.e. **purely distance-based costing**.
    ///
    /// Disables ALL other costings & penalties.
    /// Also note, shortest will not disable hierarchy pruning, leading to potentially sub-optimal
    /// routes for some costing models.
    ///
    /// Default: `false`
    pub fn only_consider_quasi_shortest(mut self) -> Self {
        self.shortest = Some(true);
        self
    }

    /// A factor that allows controlling the contribution of distance and time to the route costs.
    ///
    /// The value is in range between `0` and 1, where
    /// - `0` only takes time into account (default),
    /// - `0.5` will weight them roughly equally
    /// - `1` only distance.
    ///
    /// **Note:** this costing is currently only available for [`super::Costing::Auto`].
    pub fn use_distance(mut self, use_distance: f32) -> Self {
        debug_assert!(use_distance >= 0.0);
        debug_assert!(use_distance <= 1.0);
        self.use_distance = Some(use_distance);
        self
    }
    /// Disable hierarchies to calculate the actual optimal route.
    ///
    /// **Note:** This could be quite a performance drainer so there is an upper limit of distance.
    /// If the upper limit is exceeded, this option will always be `false`.
    ///
    /// Default: `false`
    pub fn disable_hierarchy_pruning(mut self) -> Self {
        self.disable_hierarchy_pruning = Some(true);
        self
    }
    /// Top speed the vehicle can go.
    ///
    /// Also used to avoid roads with higher speeds than this value.
    /// Must be between `10` and `252 KPH`.
    ///
    /// Default:
    /// - `truck`: `120 KPH`
    /// - `auto`/`bus`: `140 KPH`
    pub fn top_speed(mut self, top_speed: f32) -> Self {
        debug_assert!(top_speed >= 10.0);
        debug_assert!(top_speed <= 252.0);
        self.top_speed = Some(top_speed);
        self
    }
    /// Fixed speed the vehicle can go. Used to override the calculated speed.
    ///
    /// Can be useful if speed of vehicle is known.
    /// Must be between `1` and `252 KPH`.
    ///
    /// Default: `0KPH` which disables fixed speed and falls back to the standard calculated speed
    /// based on the road attribution.
    pub fn fixed_speed(mut self, fixed_speed: u32) -> Self {
        debug_assert!(fixed_speed >= 1);
        debug_assert!(fixed_speed <= 252);
        self.fixed_speed = Some(fixed_speed);
        self
    }
    /// A factor that penalizes the cost when traversing a closed edge
    ///
    /// Example:
    /// If `search_filter.exclude_closures` is `false` for origin and/or destination
    /// location and the route starts/ends on closed edges.
    ///
    /// Its value can range from
    /// - `1.0` don't penalize closed edges,
    /// - to `10.0` apply high cost penalty to closed edges.
    ///
    /// **Note:** This factor is applicable only for motorized modes of transport, i.e `auto`, `motorcycle`, `motor_scooter`, `bus`, `truck` & `taxi`.
    ///
    /// Default: `9.0`
    pub fn closure_factor(mut self, closure_factor: f32) -> Self {
        self.closure_factor = Some(closure_factor);
        self
    }
    /// If set ignores all closures, marked due to live traffic closures, during routing.
    ///
    /// **Note:** This option cannot be set if `location.search_filter.exclude_closures` is also
    /// specified in the request and will return an error if it is
    pub fn ignore_closures(mut self) -> Self {
        self.ignore_closures = Some(true);
        self
    }
    /// If set, ignores any restrictions (e.g. turn/dimensional/conditional restrictions).
    ///
    /// Especially useful for matching GPS traces to the road network regardless of restrictions.
    ///
    /// Default: `false`
    pub fn ignore_restrictions(mut self) -> Self {
        self.ignore_restrictions = Some(true);
        self
    }
    /// If set, ignores one-way restrictions.
    ///
    /// Especially useful for matching GPS traces to the road network ignoring uni-directional traffic rules.
    /// Not included in [`Self::ignore_restrictions`] option.
    ///
    /// Default: `false`
    pub fn ignore_oneways(mut self) -> Self {
        self.ignore_oneways = Some(true);
        self
    }
    /// Similar to [`Self::ignore_restrictions`], but will respect restrictions that impact vehicle safety,
    /// such as weight and size restrictions.
    ///
    /// Default: `false`
    pub fn ignore_non_vehicular_restrictions(mut self) -> Self {
        self.ignore_non_vehicular_restrictions = Some(true);
        self
    }
    /// Ignore mode-specific access tags.
    ///
    /// Especially useful for matching GPS traces to the road network regardless of restrictions.
    ///
    /// Default `false`
    pub fn ignore_access(mut self) -> Self {
        self.ignore_access = Some(true);
        self
    }
    ///The length of the truck (in meters).
    ///
    /// Default: `21.64`
    pub fn length(mut self, length: f32) -> Self {
        self.length = Some(length);
        self
    }
    ///The weight of the truck (in metric tons)
    ///
    /// Default: `21.77`
    pub fn weight(mut self, weight: f32) -> Self {
        self.weight = Some(weight);
        self
    }
    ///The axle load of the truck (in metric tons)
    ///
    /// Default: `9.07`
    pub fn axle_load(mut self, axle_load: f32) -> Self {
        self.axle_load = Some(axle_load);
        self
    }
    ///The axle count of the truck
    ///
    /// Default: `5`
    pub fn axle_count(mut self, axle_count: u32) -> Self {
        self.axle_count = Some(axle_count);
        self
    }
    /// Indicates that the truck is carrying hazardous materials
    ///
    /// Default: `false`
    pub fn carries_hazardous_materials(mut self) -> Self {
        self.hazmat = Some(true);
        self
    }
    ///A penalty applied to roads with no HGV/truck access.
    ///
    /// If set to a value less than `43200` seconds,
    /// HGV will be allowed on these roads and the penalty applies
    ///
    /// Default: `43200`, i.e. trucks are not allowed
    pub fn hgv_no_access_penalty(mut self, hgv_no_access_penalty: f32) -> Self {
        self.hgv_no_access_penalty = Some(hgv_no_access_penalty);
        self
    }
    ///A penalty (in seconds) which is applied when going to residential or service roads
    ///
    /// Default: `30` seconds
    pub fn low_class_penalty(mut self, low_class_penalty: f32) -> Self {
        self.low_class_penalty = Some(low_class_penalty);
        self
    }
    ///This value is a range of values from `0` to `1`:
    ///- `0` indicates a light preference for streets marked as truck routes, and
    /// - `1` indicates that streets not marked as truck routes should be avoided.
    ///
    /// This information is derived from the [`hgv=designated`](https://wiki.openstreetmap.org/wiki/Key:hgv) tag.
    /// **Note:** even with values near `1`, there is no guarantee the returned route will include
    /// streets marked as truck routes.
    ///
    /// Default: `0`
    pub fn use_truck_route(mut self, use_truck_route: f32) -> Self {
        self.use_truck_route = Some(use_truck_route);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn serialisation() {
        insta::assert_json_snapshot!(TruckCostingOptions::default(),@"{}")
    }
}
