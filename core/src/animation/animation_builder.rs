use morphorm::Units;

use crate::{Animation, AnimationState, Color, Opacity, State};


pub(crate) struct AnimationDescription {
    duration: std::time::Duration,
    delay: std::time::Duration,
    persistent: bool,
}

/// A builder for constructing animations.
///
/// Returned from `state.create_animation(duration)`.
///
/// ## Example
/// ```
/// let animation_id = state.create_animation(std::time::Duration::from_secs(1))
///     .add_keyframe(0.0, |keyframe| 
///         keyframe
///             .set_background_color(Color::red())
///             .set_border_color(Color::blue())
///     )
///     .add_keyframe(1.0, |keyframe| 
///         keyframe
///             .set_background_color(Color::blue()))
///             .set_border_color(Color::red())
///     .build();
/// ```
pub struct AnimationBuilder<'a> {
    id: Animation,
    state: &'a mut State,
    animation_description: AnimationDescription,
}

impl<'a> AnimationBuilder<'a> {
    pub fn new(id: Animation, state: &'a mut State, duration: std::time::Duration) -> Self {
        Self {
            id, 
            state,
            animation_description: AnimationDescription {
                duration,
                delay: std::time::Duration::from_secs(0),
                persistent: false,
            }
        }
    }


    /// Sets the delay before the animation will play. 
    ///
    /// Needs to be called before setting keyframes.
    pub fn with_delay(mut self, delay: std::time::Duration) -> Self {
        self.animation_description.delay = delay;

        self
    }

    /// Sets the animation to persist after completion.
    ///
    /// Normally, after an animation is finished, the animated property will return to the the previous value 
    /// before the animation was played. Setting an animation to persistent causes the property to be set to the last
    /// value of the animation.
    pub fn persistent(mut self) -> Self {
        self.animation_description.persistent = true;

        self
    }

    /// Adds a keyframe to the animation.
    ///
    /// 
    pub fn add_keyframe<F>(self, time: f32, keyframe: F) -> KeyframeBuilder<'a> 
    where F: FnOnce(KeyframeBuilder<'a>) -> KeyframeBuilder<'a>
    {
        (keyframe)(KeyframeBuilder::new(self.id, self.state, time, self.animation_description))
    }
}


/// A builder for constructing keyframes.
pub struct KeyframeBuilder<'a> {
    id: Animation,
    state: &'a mut State,
    time: f32,
    animation_description: AnimationDescription,
}

impl<'a> KeyframeBuilder<'a> {
    pub(crate) fn new(id: Animation, state: &'a mut State, time: f32, animation_description: AnimationDescription) -> Self {
        Self {
            id,
            state,
            time,
            animation_description,
        }
    } 

    /// Finish building the animation, returning an [Animation] id.
    pub fn build(self) -> Animation {
        self.id
    }

    /// Adds another keyframe to the animation.
    pub fn add_keyframe<F>(self, time: f32, keyframe: F) -> Self 
    where F: FnOnce(KeyframeBuilder<'a>) -> KeyframeBuilder<'a>
    {
        (keyframe)(KeyframeBuilder::new(self.id, self.state, time, self.animation_description))
    }

    pub fn set_background_color(self, color: Color) -> Self {

        if let Some(anim_state) = self.state.style.background_color.get_animation_mut(self.id) {
            anim_state.keyframes.push((self.time, color));
        } else {
            let anim_state = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, color));
                
            self.state.style.background_color.insert_animation(self.id, anim_state);

        }

        self
        
    }

    pub fn set_left(self, value: Units) -> Self {

        if let Some(anim_state) = self.state.style.left.get_animation_mut(self.id) {
            anim_state.keyframes.push((self.time, value));
        } else {
            let anim_state = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));
                
            self.state.style.left.insert_animation(self.id, anim_state);

        }

        self   
    }

    pub fn set_right(self, value: Units) -> Self {

        if let Some(anim_state) = self.state.style.right.get_animation_mut(self.id) {
            anim_state.keyframes.push((self.time, value));
        } else {
            let anim_state = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));
                
            self.state.style.right.insert_animation(self.id, anim_state);

        }

        self   
    }

    pub fn set_top(self, value: Units) -> Self {

        if let Some(anim_state) = self.state.style.top.get_animation_mut(self.id) {
            anim_state.keyframes.push((self.time, value));
        } else {
            let anim_state = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));
                
            self.state.style.top.insert_animation(self.id, anim_state);

        }

        self   
    }

    pub fn set_bottom(self, value: Units) -> Self {

        if let Some(anim_state) = self.state.style.bottom.get_animation_mut(self.id) {
            anim_state.keyframes.push((self.time, value));
        } else {
            let anim_state = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));
                
            self.state.style.bottom.insert_animation(self.id, anim_state);

        }

        self   
    }

    pub fn set_width(self, value: Units) -> Self {

        if let Some(anim_state) = self.state.style.width.get_animation_mut(self.id) {
            anim_state.keyframes.push((self.time, value));
        } else {
            let anim_state = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));
                
            self.state.style.width.insert_animation(self.id, anim_state);

        }

        self   
    }

    pub fn set_height(self, value: Units) -> Self {

        if let Some(anim_state) = self.state.style.height.get_animation_mut(self.id) {
            anim_state.keyframes.push((self.time, value));
        } else {
            let anim_state = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));
                
            self.state.style.height.insert_animation(self.id, anim_state);

        }

        self   
    }

    pub fn set_rotate(self, value: f32) -> Self {

        if let Some(anim_state) = self.state.style.rotate.get_animation_mut(self.id) {
            anim_state.keyframes.push((self.time, value));
            println!("Modify previous animation keyframe: {} {:?}", self.id, anim_state);
        } else {
            
            let anim_state = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, value));
            println!("Insert new animation keyframe: {} {:?}", self.id, anim_state);
                
            self.state.style.rotate.insert_animation(self.id, anim_state);

        }

        self   
    }

    pub fn set_opacity(self, value: f32) -> Self {
        if let Some(anim_state) = self.state.style.opacity.get_animation_mut(self.id) {
            anim_state.keyframes.push((self.time, Opacity(value)));
        } else {
            let anim_state = AnimationState::new(self.id)
                .with_duration(self.animation_description.duration)
                .with_delay(self.animation_description.delay)
                .set_persistent(self.animation_description.persistent)
                .with_keyframe((self.time, Opacity(value)));
                
            self.state.style.opacity.insert_animation(self.id, anim_state);

        }

        self 
    }


}