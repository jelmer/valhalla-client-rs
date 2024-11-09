use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PedestrianCostingOptions {
    walking_speed: Option<f32>,
    walkway_factor: Option<f32>,
    sidewalk_factor: Option<f32>,
    alley_factor: Option<f32>,
    driveway_factor: Option<f32>,
    step_penalty: Option<f32>,
    use_ferry: Option<f32>,
    use_living_streets: Option<f32>,
    use_tracks: Option<f32>,
    use_hills: Option<f32>,
    use_lit: Option<f32>,
    service_penalty: Option<f32>,
    service_factor: Option<f32>,
    destination_only_penalty: Option<f32>,
    max_hiking_difficulty: Option<f32>,
    bss_rent_cost: Option<f32>,
    bss_rent_penalty: Option<f32>,
    shortest: Option<bool>,
    max_distance: Option<f32>,
    transit_start_end_max_distance: Option<f32>,
    transit_transfer_max_distance: Option<f32>,
    r#type: Option<PedestrianType>,
    mode_factor: Option<f32>,
}
impl PedestrianCostingOptions {
    #[must_use]
    pub fn builder() -> Self {
        Self::default()
    }

    /// Walking speed in kilometers per hour.
    ///
    /// Must be between  `0.5`  and `25 km/hr`.
    ///
    /// Default: `5.1 km/hr` (`3.1 miles/hour`)
    pub fn walking_speed(mut self, walking_speed: f32) -> Self {
        debug_assert!(walking_speed >= 0.5);
        debug_assert!(walking_speed <= 25.0);
        self.walking_speed = Some(walking_speed);
        self
    }
    /// A factor that modifies the cost when encountering roads classified as `footway`
    /// (no motorized vehicles allowed), which may be designated footpaths or designated sidewalks
    /// along residential roads.
    ///
    /// Pedestrian routes generally attempt to favor using these [walkways and sidewalks](https://wiki.openstreetmap.org/wiki/Sidewalks).
    ///
    /// Default: `1.0`
    pub fn walkway_factor(mut self, walkway_factor: f32) -> Self {
        self.walkway_factor = Some(walkway_factor);
        self
    }
    /// A factor that modifies the cost when encountering roads with dedicated sidewalks.
    ///
    /// Pedestrian routes generally attempt to favor using [sidewalks](https://wiki.openstreetmap.org/wiki/Key:sidewalk).
    ///
    /// Default: `1.0`
    pub fn sidewalk_factor(mut self, sidewalk_factor: f32) -> Self {
        self.sidewalk_factor = Some(sidewalk_factor);
        self
    }
    /// A factor that modifies (multiplies) the cost when
    /// [alleys](http://wiki.openstreetmap.org/wiki/Tag:service%3Dalley) are encountered.
    ///
    /// Pedestrian routes generally want to avoid alleys or narrow service roads between buildings.
    ///
    /// Default: `2.0`
    pub fn alley_factor(mut self, alley_factor: f32) -> Self {
        self.alley_factor = Some(alley_factor);
        self
    }
    /// A factor that modifies (multiplies) the cost when encountering a
    /// [driveway](http://wiki.openstreetmap.org/wiki/Tag:service%3Ddriveway), which is often a
    /// private, service road.
    ///
    /// Pedestrian routes generally want to avoid driveways (private).
    ///
    /// Default: `5.0`
    pub fn driveway_factor(mut self, driveway_factor: f32) -> Self {
        self.driveway_factor = Some(driveway_factor);
        self
    }
    /// A penalty in seconds added to each transition onto a path with
    /// [steps or stairs](http://wiki.openstreetmap.org/wiki/Tag:highway%3Dsteps).
    ///
    /// Higher values apply larger cost penalties to avoid paths that contain flights of steps
    pub fn step_penalty(mut self, step_penalty: f32) -> Self {
        self.step_penalty = Some(step_penalty);
        self
    }
    /// Willingness to take ferries.
    ///
    /// This is range of values between `0` and `1`:
    /// - `0` attempt to avoid ferries and
    /// - `1` will favor ferries.
    ///
    /// **Note:** Sometimes ferries are required to complete a route so values of `0` are not
    /// guaranteed to avoid ferries entirely
    ///
    /// Default: `0.5`
    pub fn use_ferry(mut self, use_ferry: f32) -> Self {
        debug_assert!(use_ferry >= 0.0);
        debug_assert!(use_ferry <= 1.0);
        self.use_ferry = Some(use_ferry);
        self
    }
    /// Willingness to take living streets.
    ///
    /// This is a range of values between `0` and `1`:
    /// - `0` attempt to avoid living streets and
    /// - `1` will favor living streets.
    ///
    /// **Note:** Sometimes living streets are required to complete a route so values of `0` are
    /// not guaranteed to avoid living streets entirely
    ///
    /// Default: `0.6`
    pub fn use_living_streets(mut self, use_living_streets: f32) -> Self {
        debug_assert!(use_living_streets >= 0.0);
        debug_assert!(use_living_streets <= 1.0);
        self.use_living_streets = Some(use_living_streets);
        self
    }
    /// Willingness to take track roads.
    ///
    /// This is a range of values between `0` and `1`.
    /// - `0` attempts to avoid tracks and
    /// - `1` will favor tracks a little bit.
    ///
    /// **Note:** Sometimes tracks are required to complete a route so values of `0` are not
    /// guaranteed to avoid tracks entirely
    ///
    /// Default: `0.5`
    pub fn use_tracks(mut self, use_tracks: f32) -> Self {
        debug_assert!(use_tracks >= 0.0);
        debug_assert!(use_tracks <= 1.0);
        self.use_tracks = Some(use_tracks);
        self
    }
    /// Desire to tackle hills in routes.
    ///
    /// This is a range of values from `0` to `1`:
    /// - `0` attempts to avoid hills and steep grades even if it means a longer (time/distance) path,
    /// - `1` indicates the walker does not fear hills and steeper grades.
    ///
    /// Based on the factor, penalties are applied to roads based on elevation change and grade.
    /// These penalties help the path avoid hilly roads in favor of flatter roads or less steep
    /// grades where available. Note that it is not always possible to find alternate paths to
    /// avoid hills (for example when route locations are in mountainous areas).
    ///
    /// Default: `0.5`
    pub fn use_hills(mut self, use_hills: f32) -> Self {
        self.use_hills = Some(use_hills);
        self
    }

    /// (Un)willingness to take lit streets.
    ///
    /// This is a range of values between `0` and `1`:
    /// - Values near `0` indicate indifference towards lit streets
    /// - values near `1` indicates that unlit streets should be avoided
    ///
    /// **Note:** even values near `1`, there is no guarantee the returned route will include lit segments.
    ///
    /// Default: `0`
    pub fn use_lit(mut self, use_lit: f32) -> Self {
        debug_assert!(use_lit >= 0.0);
        debug_assert!(use_lit <= 1.0);
        self.use_lit = Some(use_lit);
        self
    }
    /// A penalty applied for transition to generic service road.
    ///
    /// Default: `0`
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
    /// A penalty applied when entering a road which is only allowed to enter if necessary to reach
    /// the [destination](https://wiki.openstreetmap.org/wiki/Tag:vehicle%3Ddestination)
    pub fn destination_only_penalty(mut self, destination_only_penalty: f32) -> Self {
        self.destination_only_penalty = Some(destination_only_penalty);
        self
    }
    /// Maximum difficulty of hiking trails that is allowed.
    ///
    /// Values between `0` and `6` are allowed.
    /// The values correspond to *sac_scale* values within OpenStreetMap, see reference
    /// [here](https://wiki.openstreetmap.org/wiki/Key:sac_scale).
    /// Higher difficulty trails can be allowed by specifying a higher value for max_hiking_difficulty
    ///
    /// Default: `1` (well cleared trails that are mostly flat or slightly sloped are allowed)
    pub fn max_hiking_difficulty(mut self, max_hiking_difficulty: f32) -> Self {
        debug_assert!(max_hiking_difficulty >= 0.0);
        debug_assert!(max_hiking_difficulty <= 6.0);
        self.max_hiking_difficulty = Some(max_hiking_difficulty);
        self
    }
    /// Time to rent a bike from a bike share station.
    ///
    /// This value will be displayed in the final directions and used to calculate the whole duration.
    /// Useful when [`super::Costing::Bikeshare`] is chosen as travel mode.
    ///
    /// Default: `120 seconds`
    pub fn bss_rent_cost(mut self, bss_rent_cost: f32) -> Self {
        self.bss_rent_cost = Some(bss_rent_cost);
        self
    }
    /// Potential effort to rent a bike from a bike share station.
    ///
    /// This value won't be displayed and used only inside the algorithm
    /// Useful when [`super::Costing::Bikeshare`] is chosen as travel mode.
    pub fn bss_rent_penalty(mut self, bss_rent_penalty: f32) -> Self {
        self.bss_rent_penalty = Some(bss_rent_penalty);
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
    /// Sets the maximum total walking distance of a route.
    ///
    /// Default: `100 km` (`~62 miles`)
    pub fn max_distance(mut self, max_distance: f32) -> Self {
        self.max_distance = Some(max_distance);
        self
    }
    /// Maximum walking distance at the beginning or end of a route
    ///
    /// Default: `2145 meters` (`~1.5 miles`)
    pub fn transit_start_end_max_distance(mut self, transit_start_end_max_distance: f32) -> Self {
        self.transit_start_end_max_distance = Some(transit_start_end_max_distance);
        self
    }
    /// Maximum walking distance between transfers
    ///
    /// Default: `800 meters` (`~0.5 miles`)
    pub fn transit_transfer_max_distance(mut self, transit_transfer_max_distance: f32) -> Self {
        self.transit_transfer_max_distance = Some(transit_transfer_max_distance);
        self
    }
    /// Changes the type of pedestrian.
    ///
    /// If set to [`PedestrianType::Blind`], enables additional route instructions, especially useful for blind users:
    /// - Announcing crossed streets,
    /// - the stairs,
    /// - bridges,
    /// - tunnels,
    /// - gates,
    /// - bollards (which need to be passed on route)
    /// - information about traffic signals on crosswalks
    ///
    /// Route numbers are not announced for named routes.
    ///
    /// Default: [`PedestrianType::Foot`]
    pub fn r#type(mut self, r#type: PedestrianType) -> Self {
        self.r#type = Some(r#type);
        self
    }
    /// A factor which the cost of a pedestrian edge will be multiplied with on multimodal request, e.g.
    /// `bss` or `multimodal/transit`.
    ///
    /// Default: `1.5`, i.e. avoiding walking
    pub fn mode_factor(mut self, mode_factor: f32) -> Self {
        self.mode_factor = Some(mode_factor);
        self
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub enum PedestrianType {
    #[default]
    #[serde(rename = "foot")]
    Foot,
    #[serde(rename = "blind")]
    Blind,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn serialisation() {
        assert_eq!(
            serde_json::to_value(PedestrianCostingOptions::default()).unwrap(),
            serde_json::json!({})
        );
    }
}
