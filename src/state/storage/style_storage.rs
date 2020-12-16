use crate::state::Entity;
// Could use last bit of entity_indices index to denote whether the data is from a rule or an inline property

pub enum LinkType {
    NewLink,
    AlreadyLinked,
    NoRule,
    NoData,
}

#[derive(Copy, Clone)]
pub struct DataIndex {
    pub data_index: usize,
    pub animation_index: usize,
}

impl DataIndex {
    pub fn new(data_index: usize, animation_index: usize) -> Self {
        DataIndex {
            data_index,
            animation_index,
        }
    }

    pub fn from_index(data_index: usize) -> Self {
        DataIndex {
            data_index,
            animation_index: std::usize::MAX,
        }
    }

    pub fn index(&self) -> usize {
        self.data_index
    }
    pub fn anim_index(&self) -> usize {
        self.animation_index
    }
}

impl Default for DataIndex {
    fn default() -> Self {
        DataIndex {
            data_index: std::usize::MAX,
            animation_index: std::usize::MAX,
        }
    }
}

pub struct StyleStorage<T> {
    // Mapping from entity to data
    pub entity_indices: Vec<DataIndex>,
    // Mapping from rule to data
    pub rule_indices: Vec<usize>,
    pub data: Vec<T>,
}

impl<T> StyleStorage<T>
where
    T: Default + Clone + std::fmt::Debug,
{
    pub fn new() -> Self {
        StyleStorage {
            entity_indices: Vec::new(),
            rule_indices: Vec::new(),
            data: Vec::new(),
        }
    }

    //Use std::usize::MAX to represent inline style
    pub fn insert(&mut self, entity: Entity, value: T) {
        if entity.index() >= self.entity_indices.len() {
            //println!("Insert New: {:?} - Data {:?}", entity, value);
            self.entity_indices
                .resize(entity.index() + 1, Default::default());
            self.entity_indices[entity.index()].data_index = self.data.len();
            self.entity_indices[entity.index()].animation_index = std::usize::MAX - 1;
            self.data.push(value);
        } else {
            let data_index = self.entity_indices[entity.index()].data_index;

            if data_index >= self.data.len() {
                self.entity_indices[entity.index()].data_index = self.data.len();

                self.data.push(value);
            } else {
                self.data[data_index] = value;
            }

            self.entity_indices[entity.index()].animation_index = std::usize::MAX - 1;
        }
    }

    // When the style system has determined the matching rule with the highest
    // specificity for an entity. The entity can be "linked" to the rule by pointing the
    // same computed property.

    // Will return false if the link was unsuccessful, or the entity is already linked to the rule
    // Will return true if a new link is established, which may trigger a relayout
    pub fn link(&mut self, entity: Entity, rule: usize) -> LinkType {
        // Check if rule exists
        if rule >= self.rule_indices.len() {
            return LinkType::NoRule;
        }

        let rule_data_index = self.rule_indices[rule];

        // Check if the rule has any associated data
        // BUG - If there is no rule then reverse transitions wont work
        if rule_data_index >= self.data.len() {
            return LinkType::NoData;
        }

        // Check if entity exists, else add the entity
        if entity.index() >= self.entity_indices.len() {
            self.entity_indices
                .resize(entity.index() + 1, Default::default());
        }
        // Link the entity to the same data as the rule

        // Check if the entity is already linked to the rule
        if self.entity_indices[entity.index()].data_index == rule_data_index {
            return LinkType::AlreadyLinked;
        }

        self.entity_indices[entity.index()].data_index = rule_data_index;

        LinkType::NewLink
    }

    pub fn unlink(&mut self, entity: Entity) {
        if entity.index() >= self.entity_indices.len() {
            return;
        }

        self.entity_indices[entity.index()] = DataIndex::default();
    }

    // Returns true if
    pub fn link_rule(&mut self, entity: Entity, rule_list: &Vec<usize>) -> bool {
        // Check if the entity already has an inline style. If so then rules don't affect it.
        if entity.index() < self.entity_indices.len() {
            if self.entity_indices[entity.index()].anim_index() == std::usize::MAX - 1 {
                return false;
            }
        }

        for rule in rule_list {
            match self.link(entity, *rule) {
                LinkType::NewLink => {
                    return true;
                }

                LinkType::AlreadyLinked => {
                    return false;
                }

                _ => {}
            }
        }

        // If none of the matching rules have a specified property then unlink the entity from any rules

        self.unlink(entity);

        false
    }

    // Insert data
    pub fn insert_rule(&mut self, rule: usize, value: T) {
        if rule >= self.rule_indices.len() {
            self.rule_indices.resize(rule + 1, std::usize::MAX);
            self.rule_indices[rule] = self.data.len();
            self.data.push(value);
        } else {
            println!("Is this ever used?");
            let data_index = self.rule_indices[rule] as usize;
            if data_index >= self.data.len() {
                self.rule_indices[rule] = self.data.len();
                self.data.push(value);
            } else {
                self.data[data_index] = value;
            }
        }
    }

    // Get data linked to entity
    pub fn get(&self, entity: Entity) -> Option<&T> {
        if entity.index() >= self.entity_indices.len() {
            return None;
        }

        let data_index = self.entity_indices[entity.index()].index();

        if data_index >= self.data.len() {
            return None;
        }

        Some(&self.data[data_index])
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        if entity.index() >= self.entity_indices.len() {
            return None;
        }

        let data_index = self.entity_indices[entity.index()].index();

        if data_index >= self.data.len() {
            return None;
        }

        Some(&mut self.data[data_index])
    }

    pub fn get_rule_mut(&mut self, rule: usize) -> Option<&mut T> {
        if rule >= self.rule_indices.len() {
            return None;
        }

        let data_index = self.rule_indices[rule];

        if data_index >= self.data.len() {
            return None;
        }

        Some(&mut self.data[data_index])
    }

    pub fn set_rule(&mut self, rule: usize, value: T) {
        if rule >= self.rule_indices.len() {
            self.insert_rule(rule, value);
            return;
        }

        let data_index = self.rule_indices[rule];

        if data_index >= self.data.len() {
            self.insert_rule(rule, value);
            return;
        }

        self.data[data_index] = value;
    }

    pub fn has_rule(&self, rule: usize) -> bool {
        if rule >= self.rule_indices.len() {
            return false;
        }

        let data_index = self.rule_indices[rule];

        if data_index >= self.data.len() {
            return false;
        }

        true
    }
}
