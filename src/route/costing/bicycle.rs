use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub enum BicycleType {
    /// Road: a road-style bicycle with narrow tires that is generally lightweight and designed for speed on paved surfaces.
    #[serde(rename = "road")]
    Road,
    /// Hybrid or City: a bicycle made mostly for city riding or casual riding on roads and paths with good surfaces.
    #[default]
    #[serde(rename = "hybrid")]
    Hybrid,
    /// Cross: a cyclo-cross bicycle, which is similar to a road bicycle but with wider tires suitable to rougher surfaces.
    #[serde(rename = "cross")]
    Cross,
    /// Mountain: a mountain bicycle suitable for most surfaces but generally heavier and slower on paved surfaces.
    #[serde(rename = "mountain")]
    Mountain,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BicycleCostingOptions {
    pub bicycle_type: BicycleType,

    /// Cycling speed is the average travel speed along smooth, flat roads. This is meant to be
    /// the speed a rider can comfortably maintain over the desired distance of the route. It
    /// can be modified (in the costing method) by surface type in conjunction with bicycle
    /// type and (coming soon) by hilliness of the road section. When no speed is specifically
    /// provided, the default speed is determined by the bicycle type and are as follows: Road
    /// = 25 KPH (15.5 MPH), Cross = 20 KPH (13 MPH), Hybrid/City = 18 KPH (11.5 MPH), and
    /// Mountain = 16 KPH (10 MPH).
    pub cycling_speed: Option<f64>,

    /// A cyclist's propensity to use roads alongside other vehicles. This is a range of values
    /// from 0 to 1, where 0 attempts to avoid roads and stay on cycleways and paths, and 1
    /// indicates the rider is more comfortable riding on roads. Based on the use_roads factor,
    /// roads with certain classifications and higher speeds are penalized in an attempt to
    /// avoid them when finding the best path. The default value is 0.5.
    pub use_roads: Option<f64>,

    /// A cyclist's desire to tackle hills in their routes. This is a range of values from 0 to
    /// 1, where 0 attempts to avoid hills and steep grades even if it means a longer (time and
    /// distance) path, while 1 indicates the rider does not fear hills and steeper grades.
    /// Based on the use_hills factor, penalties are applied to roads based on elevation change
    /// and grade. These penalties help the path avoid hilly roads in favor of flatter roads or
    /// less steep grades where available. Note that it is not always possible to find
    /// alternate paths to avoid hills (for example when route locations are in mountainous
    /// areas). The default value is 0.5.
    pub use_hills: Option<f64>,

    /// This value indicates the willingness to take ferries. This is a range of values between
    /// 0 and 1. Values near 0 attempt to avoid ferries and values near 1 will favor ferries.
    /// Note that sometimes ferries are required to complete a route so values of 0 are not
    /// guaranteed to avoid ferries entirely. The default value is 0.5.
    pub use_ferry: Option<f64>,

    /// This value indicates the willingness to take living streets. This is a range of values
    /// between 0 and 1. Values near 0 attempt to avoid living streets and values from 0.5 to 1
    /// will currently have no effect on route selection. The default value is 0.5. Note that
    /// sometimes living streets are required to complete a route so values of 0 are not
    /// guaranteed to avoid living streets entirely.
    pub use_living_streets: Option<f64>,

    /// This value is meant to represent how much a cyclist wants to avoid roads with poor
    /// surfaces relative to the bicycle type being used. This is a range of values between 0
    /// and 1. When the value is 0, there is no penalization of roads with different surface
    /// types; only bicycle speed on each surface is taken into account. As the value
    /// approaches 1, roads with poor surfaces for the bike are penalized heavier so that they
    /// are only taken if they significantly improve travel time. When the value is equal to 1,
    /// all bad surfaces are completely disallowed from routing, including start and end
    /// points. The default value is 0.25.
    pub avoid_bad_surfaces: Option<f64>,

    /// This value is useful when bikeshare is chosen as travel mode. It is meant to give the
    /// time will be used to return a rental bike. This value will be displayed in the final
    /// directions and used to calculate the whole duation. The default value is 120 seconds.
    pub bss_return_cost: Option<f64>,

    /// This value is useful when bikeshare is chosen as travel mode. It is meant to describe
    /// the potential effort to return a rental bike. This value won't be displayed and used
    /// only inside of the algorithm.
    pub bss_return_penalty: Option<f64>,

    /// Changes the metric to quasi-shortest, i.e. purely distance-based costing. Note, this
    /// will disable all other costings & penalties. Also note, shortest will not disable
    /// hierarchy pruning, leading to potentially sub-optimal routes for some costing models.
    /// The default is false.
    pub shortest: Option<bool>,

    /// A penalty applied when transitioning between roads that do not have consistent
    /// naming–in other words, no road names in common. This penalty can be used to create
    /// simpler routes that tend to have fewer maneuvers or narrative guidance instructions.
    /// The default maneuver penalty is five seconds.
    pub maneuver_penalty: Option<f64>,

    /// A cost applied when a gate with undefined or private access is encountered. This cost
    /// is added to the estimated time / elapsed time. The default gate cost is 30 seconds.
    pub gate_cost: Option<f64>,

    /// A penalty applied when a gate with no access information is on the road. The default
    /// gate penalty is 300 seconds.
    pub gate_penalty: Option<f64>,

    /// A cost applied when encountering an international border. This cost is added to the
    /// estimated and elapsed times. The default cost is 600 seconds.
    pub country_crossing_cost: Option<f64>,

    /// A penalty applied for a country crossing. This penalty can be used to create paths that
    /// avoid spanning country boundaries. The default penalty is 0.
    pub country_crossing_penalty: Option<f64>,

    /// A penalty applied for transition to generic service road. The default penalty is 0
    /// trucks and 15 for cars, buses, motor scooters and motorcycles.
    pub service_penalty: Option<f64>,
}
