use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MultimodalCostingOptions {
    pedestrian: Option<super::pedestrian::PedestrianCostingOptions>,
    transit: Option<super::transit::TransitCostingOptions>,
}
impl MultimodalCostingOptions {
    #[must_use]
    pub fn builder() -> Self {
        Self::default()
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
mod test {
    use super::*;
    #[test]
    fn serialisation() {
        assert_eq!(
            serde_json::to_value(MultimodalCostingOptions::default()).unwrap(),
            serde_json::json!({})
        )
    }
}
