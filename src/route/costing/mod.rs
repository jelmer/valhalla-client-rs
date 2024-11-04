pub mod auto;
pub mod bicycle;
pub mod bikeshare;
pub mod hov;
pub mod motor;
pub mod motorcycle;
pub mod multimodal;
pub mod pedestrian;
pub mod truck;

use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "costing", content = "costing_options")]
#[allow(clippy::large_enum_variant)]
pub enum Costing {
    /// Standard costing for driving routes by car, motorcycle, truck, and so on.
    ///
    /// Obeys automobile driving rules, such as access and turn restrictions.
    /// This provides a short time path (though not guaranteed to be the shortest time) and
    /// uses intersection costing to minimize turns and maneuvers or road name changes.
    /// Routes also tend to favor highways and higher classification roads,
    /// such as motorways and trunks.
    #[serde(rename = "auto")]
    Auto(auto::AutoCostingOptions),

    /// Standard costing for travel by bicycle.
    ///
    /// Has a slight preference for using cycleways or roads with bicycle lanes.
    /// Bicycle routes follow regular roads when needed, but avoid roads without bicycle access.
    #[serde(rename = "bicycle")]
    Bicycle(bicycle::BicycleCostingOptions),

    /// Standard costing for bus routes.
    ///
    /// Bus costing inherits the [`Costing::Auto`] behaviors, but checks for bus access on the roads.
    #[serde(rename = "bus")]
    Bus(auto::AutoCostingOptions),
    /// A combination of pedestrian and bicycle.
    ///
    /// Use bike share station (indicated by [`amenity:bicycle_rental`](https://wiki.openstreetmap.org/wiki/Tag:amenity%3Dbicycle_rental)) to change the travel mode
    #[serde(rename = "bikeshare")]
    Bikeshare(bikeshare::BikeshareCostingOptions),
    /// Standard costing for trucks.
    ///
    /// Truck costing inherits the [`Costing::Auto`] behaviors, but checks for:
    /// - truck access,
    /// - width/height restrictions and
    /// - weight limits
    #[serde(rename = "truck")]
    Truck(truck::TruckCostingOptions),
    /// Standard costing for taxi routes.
    ///
    /// Taxi costing inherits the [`Costing::Auto`] behaviors, but checks and favors
    /// taxi lane access on roads.
    #[serde(rename = "taxi")]
    Taxi(auto::AutoCostingOptions),
    /// Standard costing for travel by motor scooter or moped.
    ///
    /// By default, this will avoid higher class roads unless the country overrides allows motor
    /// scooters on these roads. Motor scooter routes follow regular roads when needed,
    /// but avoid roads without motor_scooter, moped, or mofa access.
    #[serde(rename = "motor_scooter")]
    MotorScooter(motor::MotorCostingOptions),
    /// Standard costing for travel by motorcycle.
    ///
    /// This costing model provides options to tune the route to take roadways (road touring) vs.
    /// tracks and trails (adventure motorcycling).
    #[serde(rename = "motorcycle")]
    Motorcycle(motorcycle::MotorcycleCostingOptions),
    /// Combines different modalities.
    ///
    /// **Currently supports pedestrian and transit.**
    /// In the future, multimodal will support a combination of all of the above.
    #[serde(rename = "multimodal")]
    Multimodal(multimodal::MultimodalCostingOptions),
    /// Standard walking route that excludes roads without pedestrian access.
    ///
    /// In general, pedestrian routes are the shortest distance with the following exceptions:
    /// - walkways and footpaths are slightly favored and
    /// - steps or stairs and alleys are slightly avoided
    #[serde(rename = "pedestrian")]
    Pedestrian(pedestrian::PedestrianCostingOptions),
}

impl Default for Costing {
    fn default() -> Self {
        Self::Auto(Default::default())
    }
}
