use serde::{Deserialize, Serialize};

/// Will avoid higher class roads unless the country overrides allows motor scooters on these roads.
///
/// Motor scooter routes follow regular roads when needed, but avoid roads without motor_scooter,
/// moped, or mofa access. The costing model recognizes factors unique to motor_scooter travel and
/// offers options for tuning motor_scooter routes.
///
/// Factors unique to travel by motor_scooter influence the resulting route.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MotorScooterCostingOptions {
    maneuver_penalty: Option<f32>,
    gate_cost: Option<f32>,
    gate_penalty: Option<f32>,
    private_access_penalty: Option<f32>,
    destination_only_penalty: Option<f32>,
    toll_booth_cost: Option<f32>,
    toll_booth_penalty: Option<f32>,
    ferry_cost: Option<f32>,
    use_ferry: Option<f32>,
    use_highways: Option<f32>,
    use_tolls: Option<f32>,
    use_living_streets: Option<f32>,
    use_tracks: Option<f32>,
    service_penalty: Option<f32>,
    service_factor: Option<f32>,
    country_crossing_cost: Option<f32>,
    country_crossing_penalty: Option<f32>,
    shortest: Option<bool>,
    use_distance: Option<f32>,
    disable_hierarchy_pruning: Option<bool>,
    top_speed: Option<f32>,
    fixed_speed: Option<u32>,
    closure_factor: Option<f32>,
    ignore_closures: Option<bool>,
    ignore_restrictions: Option<bool>,
    ignore_oneways: Option<bool>,
    ignore_non_vehicular_restrictions: Option<bool>,
    ignore_access: Option<bool>,
    // -- ↓ auto/motor_scooter only ↓ --
    speed_types: Option<UsedSpeedSources>,
    height: Option<f32>,
    width: Option<f32>,
    exclude_unpaved: Option<bool>,
    exclude_cash_only_tolls: Option<bool>,
    include_hov2: Option<bool>,
    include_hov3: Option<bool>,
    include_hot: Option<bool>,
    // -- ↓ motor_scooter only ↓ --
    use_primary: Option<f32>,
    use_hills: Option<f32>,
}

impl MotorScooterCostingOptions {
    #[must_use]
    pub fn builder() -> Self {
        Self::default()
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
    /// Default:
    /// - `0` trucks and
    /// - `15` for cars, buses, motor scooters and motorcycles.
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
    /// Will determine which speed sources are used, if available.
    ///
    /// A list of strings with the following possible values:
    /// - [`UsedSpeedSources::All`]
    /// - [`UsedSpeedSources::Freeflow`]
    /// - [`UsedSpeedSources::Constrained`]
    /// - [`UsedSpeedSources::Predicted`]
    /// - [`UsedSpeedSources::Current`]
    ///
    /// Default: [`UsedSpeedSources::All`] sources (again, only if available)
    pub fn speed_types(mut self, speed_types: UsedSpeedSources) -> Self {
        if speed_types == UsedSpeedSources::All {
            self.speed_types = None
        } else {
            self.speed_types = Some(speed_types);
        }
        self
    }

    /// The height of the vehicle (in meters).
    ///
    /// Default:
    /// - `car`/`bus`/`taxi`: `1.9` and
    /// - `truck`: `4.11`
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }
    /// The width of the vehicle (in meters).
    ///
    /// Default:
    /// - `car`/`bus`/`taxi`: `1.6` and
    /// - `truck`: `2.6`
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
    /// Exclude unpaved roads.
    ///
    /// If exclude_unpaved is set it is allowed to start and end with unpaved roads,
    /// but is not allowed to have them in the middle of the route path,
    /// otherwise they are allowed.
    ///
    /// Default: `false`.
    pub fn exclude_unpaved(mut self) -> Self {
        self.exclude_unpaved = Some(true);
        self
    }
    /// Desire to avoid routes with cash-only tolls.
    ///
    /// Default: `false`.
    pub fn exclude_cash_only_tolls(mut self, exclude_cash_only_tolls: bool) -> Self {
        self.exclude_cash_only_tolls = Some(exclude_cash_only_tolls);
        self
    }
    /// Include HOV roads with a 2-occupant requirement in the route when advantageous.
    ///
    /// Default: `false`.
    pub fn include_hov2(mut self, include_hov2: bool) -> Self {
        self.include_hov2 = Some(include_hov2);
        self
    }
    /// Include HOV roads with a 3-occupant requirement in the route when advantageous.
    ///
    /// Default: `false`.
    pub fn include_hov3(mut self, include_hov3: bool) -> Self {
        self.include_hov3 = Some(include_hov3);
        self
    }
    /// Include tolled HOV roads which require the driver to pay a toll if the occupant requirement isn't met.
    ///
    /// Default: `false`.
    pub fn include_hot(mut self, include_hot: bool) -> Self {
        self.include_hot = Some(include_hot);
        self
    }
    /// A rider's propensity to use primary roads.
    ///
    /// This is a range of values from `0` to `1`:
    /// - A value near `0` attempts to avoid primary roads, and
    /// - a value near `1` indicates the rider is more comfortable riding on primary roads.
    ///
    /// Based on this factor, roads with certain classifications and higher speeds are penalized
    /// in an attempt to avoid them when finding the best path.
    ///
    /// Default: `0.5`
    pub fn use_primary(mut self, use_primary: f32) -> Self {
        self.use_primary = Some(use_primary);
        self
    }
    /// A rider's desire to tackle hills in their routes.
    ///
    /// This is a range of values from `0` to `1`
    /// - where `0` attempts to avoid hills and steep grades even if it means a longer
    ///   (time/distance) path,
    /// - while `1` indicates the rider does not fear hills and steeper grades.
    ///
    /// Based on this factor, penalties are applied to roads based on elevation change and grade.
    /// These penalties help the path avoid hilly roads in favor of flatter roads or less steep
    /// grades where available.
    /// **Note:** that it is not always possible to find alternate paths to avoid hills
    /// (for example when route locations are in mountainous areas).
    ///
    /// Default: `0.5`
    pub fn use_hills(mut self, use_hills: f32) -> Self {
        self.use_hills = Some(use_hills);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum UsedSpeedSources {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "freeflow")]
    Freeflow,
    #[serde(rename = "constrained")]
    Constrained,
    #[serde(rename = "predicted")]
    Predicted,
    #[serde(rename = "current")]
    Current,
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn serialisation() {
        assert_eq!(
            serde_json::to_value(MotorScooterCostingOptions::default()).unwrap(),
            serde_json::json!({})
        );
    }
}
