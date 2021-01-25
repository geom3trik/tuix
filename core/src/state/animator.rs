#![allow(warnings)]

use crate::entity::Entity;
use crate::state::storage::dense_storage::DenseStorage;
pub use crate::state::style::*;
use crate::{PropSet, State};
use std::time::{Duration, Instant};

use crate::state::style::Color;

use std::collections::HashMap;

// #[derive(Clone)]
// pub struct Animation {
//     pub name: String,
//     pub duration: f32,
// }

// impl Animation {
//     pub fn new(name: &str) -> Self {
//         Animation {
//             name: name.to_string(),
//             duration: 0.0,
//         }
//     }

//     pub fn with_duration(mut self, duration: f32) -> Self {
//         self.duration = duration;

//         self
//     }
// }

// #[derive(Clone)]
// pub struct Keyframe {
//     time: f32,
//     property: Property,
// }

// impl Keyframe {
//     pub fn new(time: f32) -> Self {
//         Keyframe {
//             time,
//             property: Property::None,
//         }
//     }

//     pub fn property(mut self, property: Property) -> Self {
//         self.property = property;

//         self
//     }
// }

// pub struct AnimationRule {
//     pub name: String,
//     //pub keyframes: Vec<(f32, Property)>,
//     pub keyframes: Vec<Keyframe>,
//     pub duration: Duration,
// }

// impl AnimationRule {
//     pub fn new(name: &str) -> Self {
//         AnimationRule {
//             name: name.to_string(),
//             keyframes: Vec::new(),
//             duration: Duration::new(0, 0),
//         }
//     }

//     pub fn with_duration(mut self, duration: Duration) -> Self {
//         self.duration = duration;

//         self
//     }

//     pub fn keyframe(mut self, keyframe: Keyframe) -> Self {
//         self.keyframes.push(keyframe);

//         self
//     }
// }

#[derive(Debug, Clone)]
pub struct Transition {
    // List of properties affected by transition
    pub property: String,
    // Duration of the transition
    pub duration: f32,
    // Delay of the transition
    pub delay: f32,
}

impl Transition {
    pub fn new() -> Self {
        Transition {
            property: String::new(),
            duration: 0.0,
            delay: 0.0,
        }
    }
}

pub trait Interpolator {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self;
}

#[derive(Clone, Debug)]
pub struct AnimationState<Prop: Interpolator> {
    // List of property indices that this animation applies to
    pub indices: Vec<usize>,
    // The start time of the animation
    pub start_time: Instant,
    // The duration of the animation
    pub duration: Duration,
    //
    pub delay: f32,
    // Animation keyframes (time, value)
    pub keyframes: Vec<(f32, Prop)>,
    // The output of the animation
    pub output: Option<Prop>,
    // A flag used to check if the animation is finished
    pub persistent: bool,
    pub t0: f32,
    // How far through the animation between 0.0 and 1.0 (used for transitions)
    pub t: f32,

    pub active: bool,

    // For transitions. The starting rule for this transition.
    pub from_rule: usize,
    // For tansitions. The ending rule for this transition.
    pub to_rule: usize,

    // List of entities connected to this animation (used when animation is removed from active list)
    pub entities: Vec<Entity>,
}

impl<Prop> AnimationState<Prop>
where
    Prop: Interpolator,
{
    pub fn new() -> Self {
        AnimationState {
            indices: Vec::new(),
            start_time: Instant::now(),
            duration: Duration::new(0, 0),
            delay: 0.0,
            keyframes: Vec::new(),
            output: None,
            persistent: false,
            t0: 0.0,
            t: 0.0,
            active: false,
            entities: Vec::new(),
            from_rule: std::usize::MAX,
            to_rule: std::usize::MAX,
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;

        self
    }

    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay.as_secs_f32() / self.duration.as_secs_f32();

        self
    }

    pub fn with_keyframe(mut self, key: (f32, Prop)) -> Self {
        self.keyframes.push(key);

        self
    }

    pub fn interpolate(&mut self, current_time: Instant) -> bool {
        if current_time > self.start_time + self.duration {
            return false;
        }
        // println!("Animating");

        //let point = self.start_time.elapsed().as_secs_f32() / self.duration.as_secs_f32();

        //let value = Prop::interpolate((0.0,1.0), (&self.keyframes[0].1, &self.keyframes[1].1), point);
        // use the keyframes to interpolate the value and store the result in output.
        //let mut pos = Positioning::default();

        //let i = Prop::interpolate(0.0, Prop::default(), 1.0, Prop::default())

        //let i = pos.interpolate();

        true
    }

    pub fn set_persistent(mut self, flag: bool) -> Self {
        self.persistent = flag;

        self
    }

    pub fn get_output(&self) -> Option<&Prop> {
        self.output.as_ref()
    }
}

impl<Prop> Default for AnimationState<Prop>
where
    Prop: Interpolator,
{
    fn default() -> Self {
        AnimationState {
            indices: Vec::new(),
            start_time: Instant::now(),
            duration: Duration::new(0, 0),
            delay: 0.0,
            keyframes: Vec::new(),
            output: None,
            persistent: true,
            t0: 0.0,
            t: 0.0,
            active: false,
            entities: Vec::new(),
            from_rule: std::usize::MAX,
            to_rule: std::usize::MAX,
        }
    }
}

impl Interpolator for Color {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        Color::interpolate(start.clone(), end.clone(), t as f64)
    }
}

impl Interpolator for f32 {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        return start + (end - start) * t;
    }
}

impl Interpolator for i32 {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        return ((start + (end - start)) as f32 * t).round() as i32;
    }
}
