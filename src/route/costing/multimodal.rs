use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MultimodalCostingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pedestrian: Option<super::pedestrian::PedestrianCostingOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit: Option<super::transit::TransitCostingOptions>,
}
impl MultimodalCostingOptions {
    pub fn builder() -> Self {
        Default::default()
    }
    /// Allows configuration of the transit Costing options
    ///
    /// See [`super::transit::TransitCostingOptions`] for further details on options
    pub fn transit(mut self, transit: super::transit::TransitCostingOptions) -> Self {
        self.transit = Some(transit);
        self
    }
    /// Allows configuration of the Pedestrian Costing options
    ///
    /// See [`super::pedestrian::PedestrianCostingOptions`] for further details on options
    pub fn pedestrian(mut self, pedestrian: super::pedestrian::PedestrianCostingOptions) -> Self {
        self.pedestrian = Some(pedestrian);
        self
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn serialisation(){
        insta::assert_json_snapshot!(MultimodalCostingOptions::default(),@"{}")
    }
}
