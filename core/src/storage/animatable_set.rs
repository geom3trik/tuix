

use crate::{Animation, AnimationState, Entity, GenerationalId, Interpolator, Rule};

use super::sparse_set::{DenseIndex, SparseSet, SparseSetGeneric};



const INDEX_MASK: u32 =  std::u32::MAX / 2;

/// Represents an index that can either be used to retrieve inline or shared data
///
/// Since inline data will override shared data, this allows the same index to be used 
/// with a flag to indicate which data the index refers to.
/// The first bit of the u32 internal value is used to signify if the data index
/// refers to shared (default) or inline data:
/// - 0 - shared
/// - 1 - inline
#[derive(Clone, Copy, PartialEq)]
struct DataIndex(u32);

impl DataIndex {

    /// Create a new data index with the first bit set to 1, indicating that
    /// the index refers to inline data.
    ///
    /// # Example
    /// ```
    /// let data_index = DataIndex::inline(42);
    /// ```
    pub fn inline(index: usize) -> Self {
        assert!((index as u32) < INDEX_MASK);
        let value = (index as u32) | !INDEX_MASK;
        Self(value)
    }

    /// Create a new data index with the first bit set to 0, indicating that
    /// the index refers to shared data.
    ///
    /// # Example
    /// ```
    /// let data_index = DataIndex::shared(42);
    /// ```
    pub fn shared(index: usize) -> Self {
        assert!((index as u32) < INDEX_MASK);
        Self(index as u32)
    }

    /// Retrieve the inline or shared data index.
    ///
    ///
    pub fn index(&self) -> usize {
        (self.0 & INDEX_MASK) as usize
    }

    /// Returns true if the data index refers to inline data.
    ///
    ///
    pub fn is_inline(&self) -> bool {
        (self.0 & !INDEX_MASK).rotate_left(1) != 0
    }

    /// Create a null data index.
    ///
    /// A null data index is used to signify that the index refers to no data.
    ///
    pub fn null() -> Self {
        Self(std::u32::MAX >> 1)
    }
}

impl std::fmt::Debug for DataIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_inline() {
            write!(f, "Inline: {}", self.index())
        } else {
            write!(f, "Shared: {}", self.index())
        }
    }
}


/// An Index is used by the AnimatableStorage and contains a data index and an animation index.
///
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct InlineIndex {
    data_index: DataIndex,
    anim_index: u32,
}

impl Default for InlineIndex {
    fn default() -> Self {
        InlineIndex { 
            data_index: DataIndex::null(), 
            anim_index: std::u32::MAX, 
        }
    }
}

impl DenseIndex for InlineIndex {
    fn new(index: usize) -> Self {
        InlineIndex {
            data_index: DataIndex::inline(index),
            anim_index: std::u32::MAX,
        }
    }

    fn null() -> Self {
        Self::default()
    }

    fn index(&self) -> usize {
        self.data_index.index()
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SharedIndex {
    data_index: u32,
    animation: Animation,
}

impl Default for SharedIndex {
    fn default() -> Self {
        SharedIndex { 
            data_index: std::u32::MAX, 
            animation: Animation::null(), 
        }
    }
}

impl DenseIndex for SharedIndex {
    fn new(index: usize) -> Self {
        SharedIndex {
            data_index: index as u32,
            animation: Animation::null(),
        }
    }

    fn null() -> Self {
        Self::default()
    }

    fn index(&self) -> usize {
        self.data_index as usize
    }
}


/// Animatable storage is used for storing inline and shared data for entities as well as definitions for
/// animations, which can be played for entities, and transitions, which play when an entity matches a new shared style
/// rule which defines a trnasition.
///
/// Animations are moved from animations to active_animations when played. This allows the active
/// animations to be quickly iterated to update the value.
#[derive(Default, Debug)]
pub struct AnimatableSet<T: Interpolator> {
    /// Shared data determined by style rules
    pub(crate) shared_data: SparseSetGeneric<T,SharedIndex>,
    /// Inline data defined on specific entities
    pub(crate) inline_data: SparseSetGeneric<T,InlineIndex>,
    /// Animation descriptions
    animations: SparseSet<AnimationState<T>>,
    /// Animations which are currently playing
    active_animations: Vec<AnimationState<T>>,
}

impl<T> AnimatableSet<T>
where
    T: 'static + Default + Clone + Interpolator + PartialEq + std::fmt::Debug
{
    /// Create a new empty animatable storage
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert an inline value for an entity
    ///
    /// # Example
    /// ```
    /// let mut background_color = AnimatableStorage<Color>;
    /// background_color.insert(entity, Color::red())
    /// ```
    pub fn insert(&mut self, entity: Entity, value: T) {
        self.inline_data.insert(entity, value).unwrap();
    }

    /// Remove an entity and any inline data
    ///
    /// # Example
    /// ```
    /// let mut background_color = AnimatableStorage<Color>;
    /// background_color.insert(entity, Color::red())
    /// background_color.remove(entity);
    /// ```
    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let entity_index = entity.index();
        
        if entity_index < self.inline_data.sparse.len() {
            let data_index = self.inline_data.sparse[entity_index].data_index;
            if data_index.is_inline() {
                self.inline_data.remove(entity)
            } else {
                self.inline_data.sparse[entity_index] = InlineIndex::null();
                None
            }
        } else {
            None
        }
    }

    /// Inserts an animation
    ///
    /// Animations exist separately to inline (entity) data and shared (rule) data.
    /// Playing an aimation for a particular entity will clone the animation state to the
    /// active animations and then link the entity to it.  
    pub fn insert_animation(&mut self, animation: Animation, animation_description: AnimationState<T>) {
        self.animations.insert(animation, animation_description).unwrap();
    }

    pub fn remove_animation(&mut self, animation: Animation) -> Option<AnimationState<T>> {
        self.animations.remove(animation)
    }

    pub fn insert_rule(&mut self, rule: Rule, value: T) {
        self.shared_data.insert(rule, value).unwrap();
    }

    pub fn remove_rule(&mut self, rule: Rule) -> Option<T> {
        self.shared_data.remove(rule)
    }

    /// Inserts a transition for a given rule
    ///
    /// Transitions are animations which are defined for a particular rule. When an entity is linked to
    /// a rule any transition associated with that rule will play for that entity.
    ///
    pub fn insert_transition(&mut self, rule: Rule, animation: Animation) {
        // Check if the rule exists
        if self.shared_data.contains(rule) && self.animations.contains(animation) {
            self.shared_data.sparse[rule.index()].animation = animation; 
        }
    }

    pub fn play_animation(&mut self, entity: Entity, animation: Animation) {
        let entity_index = entity.index();

        if !self.animations.contains(animation) {
            return;
        }

        // If there is no inline or shared data for the entity then add the entity as animation only
        if entity_index >= self.inline_data.sparse.len() {
            self.inline_data.sparse.resize(entity_index + 1, InlineIndex::null());
        }

        if entity_index < self.inline_data.sparse.len() {
            let active_anim_index = self.inline_data.sparse[entity_index].anim_index as usize;
            if active_anim_index < self.active_animations.len() {
                let anim_state = &mut self.active_animations[active_anim_index];
                if anim_state.id == animation {
                    anim_state.t0 = 0.0;
                    anim_state.active = true;
                    anim_state.t = 0.0;
                    anim_state.start_time = std::time::Instant::now();
                    anim_state.output = Some(self.animations.get(animation).cloned().unwrap().keyframes.first().unwrap().1.clone());
                } else {
                    anim_state.output = Some(self.animations.get(animation).cloned().unwrap().keyframes.first().unwrap().1.clone());
                    anim_state.entities.remove(&entity);
                }
                //println!("Already playing: {:?}", anim_state);
                //anim_state.play(entity);
            }
            //else {
                // Safe to unwrap because already checked that the animation exists
            let mut anim_state = self.animations.get(animation).cloned().unwrap();
            //println!("Start playing: {} {:?}", animation, anim_state);
            anim_state.output = Some(self.animations.get(animation).cloned().unwrap().keyframes.first().unwrap().1.clone());
            anim_state.play(entity);
            self.inline_data.sparse[entity_index].anim_index = self.active_animations.len() as u32;
            self.active_animations.push(anim_state);
            //}
        }
    }

    pub fn tick(&mut self, time: std::time::Instant) {
        
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

            let elapsed_time = time.duration_since(state.start_time);

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

            //println!("Tick: {:?}", state.get_output());
        }

        self.remove_innactive_animations();
    }

    pub fn is_animating(&mut self, entity: Entity, animation: Animation) -> bool {
        if self.animations.contains(animation) {
            let entity_index = entity.index();
            if entity_index < self.inline_data.sparse.len() {
                let anim_index = self.inline_data.sparse[entity_index].anim_index as usize;
                if anim_index < self.active_animations.len() {
                    return true;
                }
            }
        }

        false
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
        self.active_animations
            .retain(|e| e.t0 < 1.0 || e.persistent);

        for state in inactive.into_iter() {
            for entity in state.entities.iter() {
                self.inline_data.sparse[entity.index()].anim_index = std::u32::MAX;
            }
        }

        for (index, state) in self.active_animations.iter().enumerate() {
            for entity in state.entities.iter() {
                self.inline_data.sparse[entity.index()].anim_index = index as u32;
            }
        }
    }

    pub fn has_animations(&self) -> bool {
        for state in self.active_animations.iter() {
            if state.t0 < 1.0 {
                return true;
            }
        }

        false
    }

    

    /// Returns a reference to any inline data on the entity if it exists
    ///
    ///
    /// # Example
    /// ```
    /// animatable_storage.get_inline(entity);
    /// ```
    pub fn get_inline(&self, entity: Entity) -> Option<&T> {
        let entity_index = entity.index();
        if entity_index < self.inline_data.sparse.len() {
            let data_index = self.inline_data.sparse[entity_index].data_index;
            if data_index.is_inline() {
                return self.inline_data.get(entity);
            }
        }

        None
    }

    /// Returns a mutable reference to any inline data on the entity if it exists
    ///
    ///
    /// # Example
    /// ```
    /// animatable_storage.get_inline_mut(entity);
    /// ```
    pub fn get_inline_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let entity_index = entity.index();
        if entity_index < self.inline_data.sparse.len() {
            let data_index = self.inline_data.sparse[entity_index].data_index;
            if data_index.is_inline() {
                return self.inline_data.get_mut(entity);
            }
        }

        None
    }

    /// Returns a reference to any shared data for a given rule if it exists
    ///
    ///
    /// # Example
    /// ```
    /// animatable_storage.get_shared(entity);
    /// ```
    pub fn get_shared(&self, rule: Rule) -> Option<&T> {
        self.shared_data.get(rule)
    }

    /// Returns a mutable reference to any shared data for a given rule if it exists
    ///
    ///
    /// # Example
    /// ```
    /// animatable_storage.get_shared_mut(Entity::root());
    /// ```
    pub fn get_shared_mut(&mut self, rule: Rule) -> Option<&mut T> {
        self.shared_data.get_mut(rule)
    }

    pub fn get_animation(&self, animation: Animation) -> Option<&AnimationState<T>> {
        self.animations.get(animation)
    }

    pub fn get_animation_mut(&mut self, animation: Animation) -> Option<&mut AnimationState<T>> {
        self.animations.get_mut(animation)
    }

    /// Get the animated, inline, or shared data value from the storage
    ///
    /// # Example 
    /// ```
    /// animatable_storage.get(entity);
    /// ```
    pub fn get(&self, entity: Entity) -> Option<&T> {
        let entity_index = entity.index();
        if entity_index < self.inline_data.sparse.len() {

            // Animations override inline and shared styling
            let animation_index = self.inline_data.sparse[entity_index].anim_index as usize;

            if animation_index < self.active_animations.len() {
                return self.active_animations[animation_index].get_output();
            }

            let data_index = self.inline_data.sparse[entity_index].data_index;
            if data_index.is_inline() {
                if data_index.index() < self.inline_data.dense.len() {
                    return Some(&self.inline_data.dense[data_index.index()].value);
                }
            } else {
                if data_index.index() < self.shared_data.dense.len() {
                    return Some(&self.shared_data.dense[data_index.index()].value);
                }
            }
        }

        None
    } 

    /// Link an entity to some shared data
    pub fn link(&mut self, entity: Entity, rules: &[Rule]) -> bool {
        let entity_index = entity.index();

        // Check if the entity already has some data
        if entity_index < self.inline_data.sparse.len() {
            let data_index = self.inline_data.sparse[entity_index].data_index;
            // If the data is inline then skip linking as inline data overrides shared data
            if data_index.is_inline() {
                return false;
            }
        }

        

        // Loop through matched rules and link to the first valid rule
        for rule in rules.iter() {
            if let Some(shared_data_index) = self.shared_data.dense_idx(*rule) {
                
                // If the entity doesn't have any previous shared data then create space for it
                if entity_index >= self.inline_data.sparse.len() {
                    self.inline_data.sparse.resize(entity_index + 1, InlineIndex::null());
                }

                // Get the animation state index of any animations (transitions) defined for the rule
                let rule_animation = shared_data_index.animation;

                //if let Some(transition_state) = self.animations.get_mut(rule_animation) {
                    let entity_anim_index = self.inline_data.sparse[entity_index].anim_index as usize;
                    if entity_anim_index < self.active_animations.len() {
                        let current_anim_state = &mut self.active_animations[entity_anim_index];
                        let rule_data_index = shared_data_index.data_index as usize;
                        if rule_data_index == current_anim_state.from_rule {
                            current_anim_state.from_rule = current_anim_state.to_rule;
                            current_anim_state.to_rule = rule_data_index;
                            *current_anim_state.keyframes.first_mut().unwrap() = (0.0, self.shared_data.dense[current_anim_state.from_rule].value.clone());
                            *current_anim_state.keyframes.last_mut().unwrap() = (1.0, self.shared_data.dense[current_anim_state.to_rule].value.clone());
                            current_anim_state.delay = current_anim_state.t - 1.0;
                            current_anim_state.start_time = std::time::Instant::now();
                        }
                    } else {

                        if rule_animation.index() < self.animations.dense.len() {
                            let transition_state = &mut self.animations.dense[rule_animation.index()].value;
                            // Safe to unwrap because already checked that the rule exists
                            let end = self.shared_data.get(*rule).unwrap();
                            //println!("End: {:?}", end);

                            let entity_data_index = self.inline_data.sparse[entity_index].data_index;

                            if !entity_data_index.is_inline() && entity_data_index.index() < self.shared_data.dense.len() {
                                let start_data = self.shared_data.dense[entity_data_index.index()].value.clone();
                                *transition_state.keyframes.first_mut().unwrap() = (0.0, start_data);
                            } else {
                                *transition_state.keyframes.first_mut().unwrap() = (0.0, end.clone());
                            }

                            *transition_state.keyframes.last_mut().unwrap() = (1.0, end.clone());
                            transition_state.from_rule = self.inline_data.sparse[entity_index].data_index.index();
                            transition_state.to_rule = shared_data_index.index();

                            self.play_animation(entity, rule_animation);                            
                        }

                    }
                //}

                let data_index = self.inline_data.sparse[entity_index].data_index;
                // Already linked
                if data_index.index() == shared_data_index.index() {
                    return false;
                }
                
                self.inline_data.sparse[entity_index].data_index = DataIndex::shared(shared_data_index.index());
                
                return true;
            
            } else {
                // No rule
                // if entity_index < self.inline_data.sparse.len() {
                //     self.inline_data.sparse[entity_index].data_index = DataIndex::null();
                // }
                // return true;
            }
        }

        false
    }

    pub fn clear_rules(&mut self) {
        // Remove transitions (TODO)
        for _index in self.shared_data.sparse.iter() {
            //let anim_index = index.anim_index as usize;
        }

        self.shared_data.clear();

        for index in self.inline_data.sparse.iter_mut() {
            if !index.data_index.is_inline() {
                index.data_index = DataIndex::null();
            }
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    
    // DataIndex tests

    /// Test for creating an inline data index and retrieving the index
    #[test]
    fn inline() {
        let data_index = DataIndex::inline(5);
        assert_eq!(data_index.0, !INDEX_MASK + 5);
        assert_eq!(data_index.index(), 5);
    }

    /// Test that an invalid (too large) inline index causes a panic
    #[test]
    #[should_panic]
    fn invalid_inline() {
        DataIndex::inline(std::usize::MAX);
    }

    /// Test for creating a shared data index and retrieving the index
    #[test]
    fn shared() {
        let data_index = DataIndex::shared(5);
        assert_eq!(data_index.0, 5);
        assert_eq!(data_index.index(), 5);
    }

    /// Test that an invalid (too large) shared index causes a panic
    #[test]
    #[should_panic]
    fn invalid_shared() {
        DataIndex::shared(std::usize::MAX);
    }

    /// Test of the is_inline() method
    #[test]
    fn is_inline() {
        let data_index1 = DataIndex::inline(5);
        assert_eq!(data_index1.is_inline(), true);
        let data_index2 = DataIndex::shared(5);
        assert_eq!(data_index2.is_inline(), false);
    }

    /// Test that a null data index is the correct value #7FFFFFFF (i.e. all bits = 1 except the first bit)
    #[test]
    fn null() {
        let data_index = DataIndex::null();
        assert_eq!(data_index.0, 2147483647);
    }



    // AnimatableStorage tests

    /// Test for constructing a new empty animatable storage
    #[test]
    fn new() {
        let animatable_storage = AnimatableSet::<f32>::new();
        assert_eq!(animatable_storage.inline_data.is_empty(), true);
        assert_eq!(animatable_storage.shared_data.is_empty(), true);
        assert_eq!(animatable_storage.animations.is_empty(), true);
        assert_eq!(animatable_storage.active_animations.is_empty(), true);
    }

    /// Test inserting inline data into the storage
    #[test]
    fn insert_inline() {
        let mut animatable_storage = AnimatableSet::new();
        animatable_storage.insert(Entity::root(), 5.0);
        //assert_eq!(animatable_storage.entity_indices.first().unwrap().data_index, DataIndex::inline(0));
    }

}