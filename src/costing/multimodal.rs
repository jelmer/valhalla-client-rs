//! Multimodal costing options
use serde::Serialize;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, Default, PartialEq)]
/// The multimodal costing options
pub struct MultimodalCostingOptions {
    pedestrian: Option<super::pedestrian::PedestrianCostingOptionsInner>,
    transit: Option<super::transit::TransitCostingOptionsInner>,
}
impl MultimodalCostingOptions {
    #[must_use]
    /// Creates a new instance of [`MultimodalCostingOptions`]
    pub fn builder() -> Self {
        Self::default()
    }
    /// Allows configuration of the transit Costing options
    ///
    /// See [`super::transit::TransitCostingOptions`] for further details on options
    pub fn transit(mut self, transit: super::transit::TransitCostingOptions) -> Self {
        self.transit = Some(transit.transit);
        self
    }
    /// Allows configuration of the Pedestrian Costing options
    ///
    /// See [`super::pedestrian::PedestrianCostingOptions`] for further details on options
    pub fn pedestrian(mut self, pedestrian: super::pedestrian::PedestrianCostingOptions) -> Self {
        self.pedestrian = Some(pedestrian.pedestrian);
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
