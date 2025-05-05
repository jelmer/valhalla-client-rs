pub mod auto;
pub mod bicycle;
pub mod motor_scooter;
pub mod motorcycle;
pub mod multimodal;
pub mod pedestrian;
pub mod transit;
pub mod truck;

pub use auto::AutoCostingOptions;
pub use bicycle::BicycleCostingOptions;
pub use motor_scooter::MotorScooterCostingOptions;
pub use motorcycle::MotorcycleCostingOptions;
pub use multimodal::MultimodalCostingOptions;
pub use pedestrian::PedestrianCostingOptions;
use serde::Serialize;
pub use transit::TransitCostingOptions;
pub use truck::TruckCostingOptions;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "costing", content = "costing_options")]
#[allow(clippy::large_enum_variant)]
/// Costing options for different travel modes.
pub enum Costing {
    /// Standard costing for driving routes by car, motorcycle, truck, and so on.
    ///
    /// Obeys automobile driving rules, such as access and turn restrictions.
    /// This provides a short time path (though not guaranteed to be the shortest time) and
    /// uses intersection costing to minimize turns and maneuvers or road name changes.
    /// Routes also tend to favor highways and higher classification roads,
    /// such as motorways and trunks.
    #[serde(rename = "auto")]
    Auto(AutoCostingOptions),

    /// Standard costing for travel by bicycle.
    ///
    /// Has a slight preference for using cycleways or roads with bicycle lanes.
    /// Bicycle routes follow regular roads when needed, but avoid roads without bicycle access.
    #[serde(rename = "bicycle")]
    Bicycle(BicycleCostingOptions),

    /// Standard costing for bus routes.
    ///
    /// Bus costing inherits the [`Costing::Auto`] behaviors, but checks for bus access on the roads.
    #[serde(rename = "bus")]
    Bus(AutoCostingOptions),
    /// A combination of pedestrian and bicycle.
    ///
    /// Use bike share station (indicated by [`amenity:bicycle_rental`](https://wiki.openstreetmap.org/wiki/Tag:amenity%3Dbicycle_rental)) to change the travel mode
    #[serde(rename = "bikeshare")]
    Bikeshare(BicycleCostingOptions),
    /// Standard costing for trucks.
    ///
    /// Truck costing inherits the [`Costing::Auto`] behaviors, but checks for:
    /// - truck access,
    /// - width/height restrictions and
    /// - weight limits
    #[serde(rename = "truck")]
    Truck(TruckCostingOptions),
    /// Standard costing for taxi routes.
    ///
    /// Taxi costing inherits the [`Costing::Auto`] behaviors, but checks and favors
    /// taxi lane access on roads.
    #[serde(rename = "taxi")]
    Taxi(AutoCostingOptions),
    /// Standard costing for travel by motor scooter or moped.
    ///
    /// By default, this will avoid higher class roads unless the country overrides allows motor
    /// scooters on these roads. Motor scooter routes follow regular roads when needed,
    /// but avoid roads without motor_scooter, moped, or mofa access.
    #[serde(rename = "motor_scooter")]
    MotorScooter(MotorScooterCostingOptions),
    /// Standard costing for travel by motorcycle.
    ///
    /// This costing model provides options to tune the route to take roadways (road touring) vs.
    /// tracks and trails (adventure motorcycling).
    #[serde(rename = "motorcycle")]
    Motorcycle(MotorcycleCostingOptions),
    /// Combines different modalities.
    ///
    /// **Currently supports pedestrian and transit.**
    /// In the future, multimodal will support a combination of all of the above.
    #[serde(rename = "multimodal")]
    Multimodal(MultimodalCostingOptions),
    /// Standard walking route that excludes roads without pedestrian access.
    ///
    /// In general, pedestrian routes are the shortest distance with the following exceptions:
    /// - walkways and footpaths are slightly favored and
    /// - steps or stairs and alleys are slightly avoided
    #[serde(rename = "pedestrian")]
    Pedestrian(PedestrianCostingOptions),
}

impl Default for Costing {
    fn default() -> Self {
        Self::Auto(Default::default())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn serialisation() {
        assert_eq!(
            serde_json::to_value(Costing::default()).unwrap(),
            serde_json::json!({"costing": "auto", "costing_options": {"auto":{}}})
        );
    }
}
