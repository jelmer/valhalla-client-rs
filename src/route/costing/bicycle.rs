use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub enum BicycleType {
    /// Road
    ///
    /// A road-style bicycle with narrow tires that is generally lightweight and designed for speed on paved surfaces.
    #[serde(rename = "road")]
    Road,
    /// Hybrid or City
    ///
    /// A bicycle made mostly for city riding or casual riding on roads and paths with good surfaces.
    #[default]
    #[serde(rename = "hybrid")]
    Hybrid,
    /// Cross
    ///
    /// A cyclo-cross bicycle, which is similar to a road bicycle but with wider tires suitable to rougher surfaces.
    #[serde(rename = "cross")]
    Cross,
    /// Mountain
    ///
    /// A mountain bicycle suitable for most surfaces but generally heavier and slower on paved surfaces.
    #[serde(rename = "mountain")]
    Mountain,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BicycleCostingOptions {
    bicycle_type: Option<BicycleType>,
    cycling_speed: Option<f32>,
    use_roads: Option<f32>,
    use_hills: Option<f32>,
    use_ferry: Option<f32>,
    use_living_streets: Option<f32>,
    avoid_bad_surfaces: Option<f32>,
    bss_return_cost: Option<f32>,
    bss_return_penalty: Option<f32>,
    shortest: Option<bool>,
    maneuver_penalty: Option<f32>,
    gate_cost: Option<f32>,
    gate_penalty: Option<f32>,
    country_crossing_cost: Option<f32>,
    country_crossing_penalty: Option<f32>,
    service_penalty: Option<f32>,
}
impl BicycleCostingOptions {
    #[must_use]
    pub fn builder() -> Self {
        Self::default()
    }

    /// Specifies the [`BicycleType`].
    ///
    /// Adapts routing speeds and penalties.
    ///
    /// Default: [`BicycleType::Hybrid`]
    pub fn bicycle_type(mut self, bicycle_type: BicycleType) -> Self {
        self.bicycle_type = Some(bicycle_type);
        self
    }

    /// Cycling speed is the average travel speed along smooth, flat roads.
    ///
    /// The speed a rider can comfortably maintain over the desired distance of the route.
    /// It can be modified (in the costing method) by surface type in conjunction with bicycle
    /// type and (coming soon) by hilliness of the road section.
    ///
    /// When no speed is specifically provided, the default speed is determined by the bicycle type
    /// and are as follows:
    /// - [`BicycleType::Road`] = 25 KPH (15.5 MPH),
    /// - [`BicycleType::Cross`] = 20 KPH (13 MPH),
    /// - [`BicycleType::Hybrid`] = 18 KPH (11.5 MPH), and
    /// - [`BicycleType::Mountain`] = 16 KPH (10 MPH).
    pub fn cycling_speed(mut self, speed: f32) -> Self {
        self.cycling_speed = Some(speed);
        self
    }

    /// A cyclist's propensity to use roads alongside other vehicles.
    ///
    /// This is a range of values from `0` to `1`:
    /// - `0` attempts to avoid roads and stay on cycleways and paths,
    /// - and `1` indicates the rider is more comfortable riding on roads.
    ///
    /// Based on this factor, roads with certain classifications and higher speeds are penalized
    /// in an attempt to avoid them when finding the best path.
    ///
    /// Default: `0.5`
    pub fn use_roads(mut self, willingness: f32) -> Self {
        self.use_roads = Some(willingness);
        self
    }

    /// Desire to tackle hills in routes.
    ///
    /// This is a range of values from 0 to 1:
    /// - 0 attempts to avoid hills and steep grades even if it means a longer (time/distance) path,
    /// - while `1` indicates the rider does not fear hills and steeper grades.
    ///
    /// Based on the use_hills factor, penalties are applied to roads based on elevation change
    /// and grade. These penalties help the path avoid hilly roads in favor of flatter roads or
    /// less steep grades where available. Note that it is not always possible to find
    /// alternate paths to avoid hills (for example when route locations are in mountainous
    /// areas).
    ///
    /// Default: `0.5`
    pub fn use_hills(mut self, willingness: f32) -> Self {
        self.use_hills = Some(willingness);
        self
    }

    /// Willingness to take ferries.
    ///
    /// This is a range of values between `0` and `1`.
    /// - Values near `0` attempt to avoid ferries and
    /// - values near `1` will favor ferries.
    ///
    /// Note that sometimes ferries are required to complete a route so values of `0` are not
    /// guaranteed to avoid ferries entirely.
    ///
    /// Default: `0.5`
    pub fn use_ferry(mut self, willingness: f32) -> Self {
        self.use_ferry = Some(willingness);
        self
    }

    /// Willingness to take living streets.
    ///
    /// This is a range of values between `0` and `1`:
    /// - Values near `0` attempt to avoid living streets and
    /// - values from `0.5` to `1` will currently have no effect on route selection.
    ///
    /// Note that sometimes living streets are required to complete a route so values of `0` are not
    /// guaranteed to avoid living streets entirely.
    ///
    /// Default: `0.5`
    pub fn use_living_streets(mut self, willingness: f32) -> Self {
        self.use_living_streets = Some(willingness);
        self
    }

    /// How much a cyclist wants to avoid roads with poor surfaces relative to the bicycle type used.
    ///
    /// This is a range of values between `0` and 1:
    /// - When the value is 0, there is no penalization of roads with different surface types; only
    ///   bicycle speed on each surface is taken into account.
    /// - As the value approaches 1, roads with poor surfaces for the bike are penalized heavier
    ///   so that they are only taken if they significantly improve travel time.
    /// - When the value is equal to 1, all bad surfaces are completely disallowed from routing,
    ///   including start and end points.
    ///
    /// Default: `0.25`
    pub fn avoid_bad_surfaces(mut self, willingness: f32) -> Self {
        self.avoid_bad_surfaces = Some(willingness);
        self
    }

    /// This value is useful when bikeshare is chosen as travel mode.
    ///
    /// It is meant to give the time will be used to return a rental bike.
    /// This value will be displayed in the final directions and used to calculate the whole duration.
    ///
    /// Default: `120` seconds
    pub fn bss_return_cost(mut self, cost: f32) -> Self {
        self.bss_return_cost = Some(cost);
        self
    }

    /// This value is useful when bikeshare is chosen as travel mode.
    ///
    /// It is meant to describe the potential effort to return a rental bike.
    /// This value won't be displayed and used only inside the algorithm.
    pub fn bss_return_penalty(mut self, penalty: f32) -> Self {
        self.bss_return_penalty = Some(penalty);
        self
    }

    /// Changes the metric to quasi-shortest, i.e. **purely distance-based costing**.
    ///
    /// Disables ALL other costings & penalties.
    /// Also note, shortest will not disable hierarchy pruning, leading to potentially sub-optimal
    /// routes for some costing models.
    ///
    /// Default: `false`.
    pub fn only_consider_quasi_shortest(mut self) -> Self {
        self.shortest = Some(true);
        self
    }

    /// A penalty applied when transitioning between roads that do not have consistent naming–in
    /// other words, no road names in common.
    ///
    /// This penalty can be used to create simpler routes that tend to have fewer maneuvers or
    /// narrative guidance instructions.
    ///
    /// Default: `5` seconds
    pub fn maneuver_penalty(mut self, penalty: f32) -> Self {
        self.maneuver_penalty = Some(penalty);
        self
    }

    /// A cost applied when a gate with undefined or private access is encountered.
    ///
    /// This cost is added to the estimated time / elapsed time.
    ///
    /// Default: `30` seconds
    pub fn gate_cost(mut self, cost: f32) -> Self {
        self.gate_cost = Some(cost);
        self
    }

    /// A penalty applied when a gate with no access information is on the road.
    ///
    /// Default: `300` seconds
    pub fn gate_penalty(mut self, penalty: f32) -> Self {
        self.gate_penalty = Some(penalty);
        self
    }

    /// A cost applied when encountering an international border.
    ///
    /// This cost is added to the estimated and elapsed times.
    ///
    /// Default: `600` seconds
    pub fn country_crossing_cost(mut self, cost: f32) -> Self {
        self.country_crossing_cost = Some(cost);
        self
    }

    /// A penalty applied for a country crossing.
    ///
    /// This penalty can be used to create paths that avoid spanning country boundaries.
    ///
    /// Default: `0`
    pub fn country_crossing_penalty(mut self, penalty: f32) -> Self {
        self.country_crossing_penalty = Some(penalty);
        self
    }

    /// A penalty applied for transition to generic service road.
    ///
    /// Default: `0` for trucks and `15` for cars, buses, motor scooters and motorcycles.
    pub fn service_penalty(mut self, penalty: f32) -> Self {
        self.service_penalty = Some(penalty);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn serialisation() {
        assert_eq!(
            serde_json::to_value(BicycleCostingOptions::default()).unwrap(),
            serde_json::json!({})
        );
    }
}
