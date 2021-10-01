

use std::cmp::{Eq, PartialEq};
use std::hash::Hash;

use crate::state::id::GenerationalId;

const ANIMATION_INDEX_BITS: u32 = 24;
const ANIMATION_INDEX_MASK: u32  = (1<<ANIMATION_INDEX_BITS)-1;

const ANIMATION_GENERATION_BITS: u32 = 8;
const ANIMATION_GENERATION_MASK: u32 = (1<<ANIMATION_GENERATION_BITS)-1;

const ANIMATION_MAX: u32 = std::u32::MAX>>8;

const MINIMUM_FREE_INDICES: usize = 1024;


/// An animation is an id used to reference to get/set properties in State.
///
/// Rather than having widgets own their data, all state is stored in a single database and
/// is stored and loaded using entities.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Animation(u32);

impl Default for Animation {
    fn default() -> Self {
        Animation::null()
    }
}

impl std::fmt::Display for Animation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.index())
    }
}

impl std::fmt::Debug for Animation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Animation {{index: {}, generation: {}}}", self.index(), self.generation())
    }
}

impl Animation {
    /// Creates a null animation
    ///
    /// A null animation can be used as a placeholder within a widget struct but cannot be used to get/set properties
    pub fn null() -> Animation {
        Animation(std::u32::MAX)
    }

    /// Creates a root animation
    ///
    /// The root animation represents the main window and is always valid. 
    /// The root animation can be used to set properties on the primary window, such as background color, 
    /// as well as sending events to the window such as Restyle and Redraw events.
    pub fn root() -> Animation {
        Animation(0)
    }

    /// Creates a new animation with a given index and generation
    pub(crate) fn new(index: u32, generation: u32) -> Animation {
        assert!(index < ANIMATION_INDEX_MASK);
        assert!(generation < ANIMATION_GENERATION_MASK);
        Animation(index | generation << ANIMATION_INDEX_BITS)
    }

}

impl GenerationalId for Animation {
    fn new(index: usize, generation: usize) -> Self {
        Animation::new(index as u32, generation as u32)
    }

    fn index(&self) -> usize {
        (self.0 & ANIMATION_INDEX_MASK) as usize
    }

    fn generation(&self) -> u8 {
        ((self.0 >> ANIMATION_INDEX_BITS) & ANIMATION_GENERATION_MASK) as u8
    }

    /// Returns true if the animation is null
    fn is_null(&self) -> bool {
        self.0 == std::u32::MAX
    }
}