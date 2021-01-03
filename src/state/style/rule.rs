use crate::entity::Entity;

use super::Property;

use crate::state::storage::dense_storage::DenseStorage;

use crate::style::{Relation, Selector, Specificity};

#[derive(Clone, Debug)]
pub struct StyleRule {
    pub selectors: Vec<Selector>,
    pub properties: Vec<Property>,
}

impl StyleRule {
    pub fn new() -> Self {
        StyleRule {
            selectors: Vec::new(),
            properties: Vec::new(),
        }
    }

    pub fn selector(mut self, selector: Selector) -> Self {
        self.selectors.push(selector);

        self
    }

    pub fn parent_selector(mut self, mut selector: Selector) -> Self {
        selector.relation = Relation::Parent;
        self.selectors.push(selector);

        self
    }

    pub fn property(mut self, property: Property) -> Self {
        self.properties.push(property);

        self
    }

    pub fn specificity(&self) -> Specificity {
        let mut specificity = Specificity([0, 0, 0]);
        for selector in self.selectors.iter() {
            specificity += selector.specificity();
        }

        return specificity;
    }
}
