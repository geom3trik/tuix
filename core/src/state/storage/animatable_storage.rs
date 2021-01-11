use crate::state::animator::{AnimationState, Interpolator};
use crate::state::Entity;

#[derive(Copy, Clone)]
pub struct Index(usize);

impl Index {
    pub fn new(val: usize) -> Self {
        let mask = std::usize::MAX / 4;
        Index(val & mask)
    }

    pub fn inherited(mut self, val: bool) -> Self {
        let mask = !(std::usize::MAX / 2);
        // Set first bit to 1 to indicate that the value is inhertied
        if val {
            self.0 = self.0 | mask;
        }

        self
    }

    pub fn inline(mut self, val: bool) -> Self {
        let mask = !(std::usize::MAX / 2) >> 1;
        if val {
            self.0 = self.0 | mask;
        }

        self
    }

    pub fn set_inherited(&mut self, val: bool) -> &mut Self {
        let mask = !(std::usize::MAX / 2);
        // Set first bit to 1 to indicate that the value is inhertied
        if val {
            self.0 = self.0 | mask;
        }

        self
    }
    // Second bit set to 1 to indicate that the value is inline
    pub fn set_inline(&mut self, val: bool) -> &mut Self {
        let mask = !(std::usize::MAX / 2) >> 1;

        if val {
            self.0 = self.0 | mask;
        }

        self
    }

    pub fn set_value(&mut self, val: usize) -> &mut Self {
        let mask = !(std::usize::MAX / 2) | !(std::usize::MAX / 2) >> 1;
        let flags = self.0 & mask;
        self.0 = val | flags;

        self
    }

    pub fn index(&self) -> usize {
        let mask = std::usize::MAX / 4;
        return self.0 & mask;
    }

    pub fn is_inherited(&self) -> bool {
        let mask = !(std::usize::MAX / 2);
        return (self.0 & mask).rotate_left(1) != 0;
    }

    pub fn is_inline(&self) -> bool {
        let mask = !(std::usize::MAX / 2) >> 1;
        return (self.0 & mask).rotate_left(2) != 0;
    }
}

impl Default for Index {
    fn default() -> Self {
        Index(std::usize::MAX & (std::usize::MAX / 2).rotate_right(1))
    }
}

#[derive(Copy, Clone)]
pub struct AnimationIndex(usize);

impl AnimationIndex {
    pub fn new(val: usize) -> Self {
        let mask = std::usize::MAX / 4;
        AnimationIndex(val & mask)
    }

    pub fn inherited(mut self, val: bool) -> Self {
        let mask = !(std::usize::MAX / 2);
        // Set first bit to 1 to indicate that the value is inhertied
        if val {
            self.0 = self.0 | mask;
        }

        self
    }

    pub fn set_inherited(&mut self, val: bool) -> &mut Self {
        let mask = !(std::usize::MAX / 2);
        // Set first bit to 1 to indicate that the value is inhertied
        if val {
            self.0 = self.0 | mask;
        }

        self
    }

    pub fn active(mut self, val: bool) -> Self {
        let mask = !(std::usize::MAX / 2) >> 1;
        if val {
            self.0 = self.0 | mask;
        }

        self
    }

    pub fn index(&self) -> usize {
        let mask = std::usize::MAX / 4;
        return self.0 & mask;
    }

    pub fn is_inherited(&self) -> bool {
        let mask = !(std::usize::MAX / 2);
        return (self.0 & mask).rotate_left(1) != 0;
    }

    pub fn is_active(&self) -> bool {
        let mask = !(std::usize::MAX / 2) >> 1;
        return (self.0 & mask).rotate_left(2) != 0;
    }
}

impl Default for AnimationIndex {
    fn default() -> Self {
        AnimationIndex(std::usize::MAX & (std::usize::MAX / 2).rotate_right(1))
    }
}

pub enum LinkType {
    NewLink,
    AlreadyLinked,
    NoRule,
    NoData,
}

#[derive(Copy, Clone)]
pub struct DataIndex {
    pub data_index: Index,
    pub animation_id: usize,
}

impl DataIndex {
    // pub fn new(data_index: usize, animation_index: usize) -> Self {
    //     DataIndex {
    //         data_index,
    //         animation_index,
    //     }
    // }

    // pub fn from_index(data_index: usize) -> Self {
    //     DataIndex {
    //         data_index,
    //         animation_index: std::usize::MAX,
    //     }
    // }

    pub fn index(&self) -> Index {
        self.data_index
    }
    pub fn anim_id(&self) -> usize {
        self.animation_id
    }
}

impl Default for DataIndex {
    fn default() -> Self {
        DataIndex {
            data_index: Index::default(),
            animation_id: std::usize::MAX,
        }
    }
}

pub struct AnimatableStorage<T: Interpolator> {
    // Mapping from entity index to data and animations
    pub entity_indices: Vec<DataIndex>,
    // Mapping from rule index to data
    pub rule_indices: Vec<DataIndex>,
    // An index to the animation either in definitions or active
    //pub animation_indices: Vec<usize>,
    // The actual data as determined by the rules
    pub data: Vec<T>,
    pub inline_data: Vec<T>,
    // Animation descriptions
    pub animations: Vec<AnimationState<T>>,
    // Active Animations
    pub active_animations: Vec<AnimationState<T>>,
}

impl<T> AnimatableStorage<T>
where
    T: Default + Clone + Interpolator + std::fmt::Debug + PartialEq + 'static,
{
    pub fn new() -> Self {
        AnimatableStorage {
            entity_indices: Vec::new(),
            rule_indices: Vec::new(),
            //animation_indices: Vec::new(),
            data: Vec::new(),
            inline_data: Vec::new(),
            animations: Vec::new(),
            active_animations: Vec::new(),
        }
    }

    // Insert inline data
    pub fn insert(&mut self, entity: Entity, value: T) {
        if entity.index() >= self.entity_indices.len() {
            // Resize entity indices to include new entity
            self.entity_indices
                .resize(entity.index() + 1, Default::default());
            // Set the data index to the data position
            self.entity_indices[entity.index()].data_index =
                Index::new(self.inline_data.len()).inherited(false).inline(true);
            // Add the data
            self.inline_data.push(value);
        } else {
            let data_index = self.entity_indices[entity.index()].data_index;

            if data_index.index() >= self.inline_data.len() {
                self.entity_indices[entity.index()].data_index =
                    Index::new(self.inline_data.len()).inherited(false).inline(true);
                //self.entity_indices[entity.index()].animation_index = AnimationIndex::default();
                self.inline_data.push(value);
            } else {
                self.entity_indices[entity.index()]
                    .data_index
                    .set_inherited(false)
                    .set_inline(true);
                //self.entity_indices[entity.index()].animation_index = AnimationIndex::default();
                self.inline_data[data_index.index()] = value;
            }

            //self.entity_indices[entity.index()].animation_index = std::usize::MAX - 1;
        }
    }

    // Insert an animation definition
    pub fn insert_animation(&mut self, animation_state: AnimationState<T>) -> usize {
        let animation_id = self.animations.len();

        self.animations.push(animation_state);

        return animation_id;
    }

    pub fn play_animation(&mut self, entity: Entity, description_id: usize) {
        // Check if animation exists
        if description_id >= self.animations.len() {
            return;
        }

        // Link the entity to the animation
        if entity.index() >= self.entity_indices.len() {
            self.entity_indices
                .resize(entity.index() + 1, Default::default());
        }

        let animation_index = self.entity_indices[entity.index()].animation_id;

        if animation_index < self.active_animations.len() {
            let animation = self.animations.get(description_id).unwrap();
            self.active_animations[animation_index].t0 = 0.0;
            self.active_animations[animation_index].active = true;
            self.active_animations[animation_index].t = 0.0;
            self.active_animations[animation_index].start_time = std::time::Instant::now();
            self.active_animations[animation_index].duration = animation.duration;
            self.active_animations[animation_index].delay = animation.delay;
            self.active_animations[animation_index].keyframes = animation.keyframes.clone();
            // FIX ME (Needed because sometimes drawing happens before animation for some reason. Stops output being null if accessed before animated)
            self.active_animations[animation_index].output =
                Some(animation.keyframes.first().unwrap().1.clone());
        } else {
            let mut animation = self.animations[description_id].clone();
            animation.active = true;
            animation.t0 = 0.0;
            animation.t = 0.0;
            animation.start_time = std::time::Instant::now();
            animation.entities.push(entity);

            animation.output = Some(animation.keyframes.first().unwrap().1.clone());
            self.entity_indices[entity.index()].animation_id = self.active_animations.len();
            self.active_animations.push(animation);
        }


    }

    pub fn animate(&mut self, current_time: std::time::Instant) {
        for state in self.active_animations.iter_mut() {
            // If the animation is already finished then return false
            if state.t0 == 1.0 {
                continue;
            }

            let start = state.keyframes.first().unwrap();
            let end = state.keyframes.last().unwrap();

            if start.1 == end.1 {
                state.t0 = 1.0;
                state.output = Some(end.1.clone());
                continue;
            }

            let elapsed_time = current_time.duration_since(state.start_time);

            // Store previous time state
            state.t0 = state.t;

            // Update time state
            state.t = (elapsed_time.as_secs_f32() / state.duration.as_secs_f32()) - state.delay;

            if state.t >= 1.0 {
                //Animation is finished
                state.output = Some(T::interpolate(&start.1, &end.1, 1.0));

                if !state.persistent {
                    state.t = 1.0;
                    state.active = false;
                } else {
                    state.t = 1.0;
                }
            } else if state.t <= 0.0 {
                state.output = Some(start.1.clone());
            } else {
                state.output = Some(T::interpolate(&start.1, &end.1, state.t));
            }
        }

        self.remove_innactive_animations();
    }

    pub fn remove_innactive_animations(&mut self) {
        // Create a list of finished animations
        let inactive: Vec<AnimationState<T>> = self
            .active_animations
            .iter()
            .filter(|e| e.t0 == 1.0 && !e.persistent)
            .cloned()
            .collect();

        // Remove inactive animation states from active animations list
        // Retains persistent animations
        self.active_animations.retain(|e| e.t0 < 1.0 || e.persistent);

        for state in inactive.into_iter() {
            for entity in state.entities.iter() {
                self.entity_indices[entity.index()].animation_id = std::usize::MAX;
            }
        }

        for (index, state) in self.active_animations.iter().enumerate() {
            for entity in state.entities.iter() {
                self.entity_indices[entity.index()].animation_id = index;
            }
        }
    }

    // WIP

    // pub fn cascade(&mut self, entity: Entity, parent: Entity) {

    //     //println!("Parent: {}", parent);

    //     if parent.index() >= self.entity_indices.len() {
    //         return;
    //     }

    //     let parent_data_index = self.entity_indices[parent.index()].data_index;
    //     //let data_index = self.entity_indices[entity.index()].data_index;

    //     if entity.index() >= self.entity_indices.len() {
    //         self.entity_indices
    //             .resize(entity.index() + 1, Default::default());
    //     }

    //     let data_index = self.entity_indices[entity.index()].data_index;

    //     if !data_index.is_inline() {
    //         self.entity_indices[entity.index()].data_index = parent_data_index;
    //     }
    // }

    // When the style system has determined the matching rule with the highest
    // specificity for an entity. The entity can be "linked" to the rule by pointing the
    // same computed property.
    pub fn link(&mut self, entity: Entity, rule: usize) -> LinkType {
        // Check if rule exists
        if rule >= self.rule_indices.len() {
            return LinkType::NoRule;
        }

        let rule_data_index = self.rule_indices[rule].data_index.index();

        // Check if the rule has any associated data
        if rule_data_index >= self.data.len() {
            return LinkType::NoData;
        }

        // Check if entity exists, else add the entity
        if entity.index() >= self.entity_indices.len() {
            self.entity_indices
                .resize(entity.index() + 1, Default::default());
        }

        // Check if the entity is already linked to the rule
        if self.entity_indices[entity.index()].data_index.index() == rule_data_index {
            return LinkType::AlreadyLinked;
        }

        // Get the animation id for any transition on the rule
        let rule_animation_id = self.rule_indices[rule].animation_id;

        // Check if the entity is already animating with a transition
        
        let animation_index = self.entity_indices[entity.index()].animation_id;
        if animation_index < self.active_animations.len() {
            // Check here is the active animation belongs to the transition of the currently linked data
            let from_rule = self.active_animations[animation_index].from_rule;
            //let to_rule = self.active_animations[animation_index].to_rule;

            // If the transition is already going from A to B and the request is to go to A, then reverse the transition.
            if rule_data_index == from_rule {
                let transition = self.active_animations.get_mut(animation_index).unwrap();

                transition.from_rule = transition.to_rule;
                transition.to_rule = rule_data_index;

                *transition.keyframes.first_mut().unwrap() =
                    (0.0, self.data[transition.from_rule].clone());
                *transition.keyframes.last_mut().unwrap() =
                    (1.0, self.data[transition.to_rule].clone());

                //transition.duration = transition.duration.mul_f32(transition.t);
                transition.delay = transition.t - 1.0;

                transition.start_time = std::time::Instant::now();
            }
        } else {
            if rule_animation_id < self.animations.len() {
                // Get the transition animation definition
                let transition = self.animations.get_mut(rule_animation_id).unwrap();
                let current_data_index = self.entity_indices[entity.index()].data_index.index();
                let start = self
                    .data
                    .get(current_data_index)
                    .cloned()
                    .unwrap_or_default();
                let end = self.data.get(rule_data_index).cloned().unwrap_or_default();

                *transition.keyframes.first_mut().unwrap() = (0.0, start);
                *transition.keyframes.last_mut().unwrap() = (1.0, end);

                transition.from_rule = self.entity_indices[entity.index()].data_index.index();
                transition.to_rule = rule_data_index;

                // Play any transition animation
                self.play_animation(entity, rule_animation_id);
            }
        }
        

        // Link the entity to the same data as the rule
        self.entity_indices[entity.index()].data_index = Index::new(rule_data_index);

        LinkType::NewLink
    }

    pub fn has_animations(&self) -> bool {
        for state in self.active_animations.iter() {
            if state.t0 < 1.0 {
                return true;
            }
        }

        false
    }

    pub fn unlink(&mut self, entity: Entity) {
        if entity.index() >= self.entity_indices.len() {
            return;
        }

        self.entity_indices[entity.index()].data_index = Index::default();
    }

    pub fn link_rule(&mut self, entity: Entity, rule_list: &Vec<usize>) -> bool {
        // Check if the entity already has an inline style. If so then rules don't affect it.
        if entity.index() < self.entity_indices.len() {
            if self.entity_indices[entity.index()].data_index.is_inline() {
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
        // Cascading could happen here but would need to pass in the hierarchy

        self.unlink(entity);

        false
    }

    // Insert rule data
    pub fn insert_rule(&mut self, rule: usize, value: T) {
        if rule >= self.rule_indices.len() {
            self.rule_indices.resize(rule + 1, Default::default());
            self.rule_indices[rule].data_index = Index::new(self.data.len());
            self.data.push(value);
        } else {
            let data_index = self.rule_indices[rule].data_index.index();
            if data_index >= self.data.len() {
                self.rule_indices[rule].data_index = Index::new(self.data.len());
                self.data.push(value);
            } else {
                self.data[data_index] = value;
            }
        }
    }

    // Links a rule to a transition animation
    pub fn insert_transition(&mut self, rule: usize, animation_state: AnimationState<T>) {
        self.rule_indices[rule].animation_id = self.animations.len();
        self.animations.push(animation_state);
    }

    // Get the current value (either animation or data rule)
    pub fn get(&self, entity: Entity) -> Option<&T> {
        if entity.index() >= self.entity_indices.len() {
            return None;
        }

        let animation_index = self.entity_indices[entity.index()].animation_id;

        if animation_index < self.active_animations.len() {
            
            return self.active_animations[animation_index].get_output();
        }

        let data_index = self.entity_indices[entity.index()].data_index;

        if data_index.is_inline() {
            if data_index.index() >= self.inline_data.len() {
                return None;
            }

            Some(&self.inline_data[data_index.index()])

        } else {
            if data_index.index() >= self.data.len() {
                return None;
            }

            Some(&self.data[data_index.index()])            
        }


    }

    // Returns true if the entity is linked to a currently active animation
    pub fn is_animating(&self, entity: Entity) -> bool {
        if entity.index() >= self.entity_indices.len() {
            return false;
        }

        let animation_index = self.entity_indices[entity.index()].animation_id;

        if animation_index >= self.active_animations.len() {
            return false;
        }

        true
    }

    pub fn get_rule_mut(&mut self, rule: usize) -> Option<&mut T> {
        if rule >= self.rule_indices.len() {
            return None;
        }

        let data_index = self.rule_indices[rule].data_index.index();

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

        let data_index = self.rule_indices[rule].data_index.index();

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

        let data_index = self.rule_indices[rule].data_index.index();

        if data_index >= self.data.len() {
            return false;
        }

        true
    }

    pub fn get_animation_mut(&mut self, animation_id: usize) -> Option<&mut AnimationState<T>> {
        if animation_id >= self.animations.len() {
            return None;
        }

        return self.animations.get_mut(animation_id);
    }

    // // Removes data at data_index
    // pub remove_data(&mut self, data_index: usize) {
    //     // Unlink any entities from the data
    //     // Remove any 
    // }

    // Removes css styles but leaves inline styles and animations
    pub fn remove_styles(&mut self) {

        // Remove rules
        self.rule_indices.clear();
        // Remove rule data
        self.data.clear();
        
        // Unlink non-inline entities from the rules
        for entity in self.entity_indices.iter_mut() {
            if !entity.index().is_inline() {
                entity.data_index = Index::default();
            }
        }
    
    }
}
