use morphorm::Units;

use crate::{Animation, AnimationState, Color, Opacity, State};


pub struct AnimationDescription {
    duration: std::time::Duration,
    delay: std::time::Duration,
    persistent: bool,
}


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


    /// Needs to be called before setting keyframes
    pub fn with_delay(mut self, delay: std::time::Duration) -> Self {
        self.animation_description.delay = delay;

        self
    }

    pub fn persistent(mut self) -> Self {
        self.animation_description.persistent = true;

        self
    }

    pub fn add_keyframe<F>(self, time: f32, keyframe: F) -> KeyframeBuilder<'a> 
    where F: FnOnce(KeyframeBuilder<'a>) -> KeyframeBuilder<'a>
    {
        (keyframe)(KeyframeBuilder::new(self.id, self.state, time, self.animation_description))
    }
}

pub struct KeyframeBuilder<'a> {
    id: Animation,
    state: &'a mut State,
    time: f32,
    animation_description: AnimationDescription,
}

impl<'a> KeyframeBuilder<'a> {
    pub fn new(id: Animation, state: &'a mut State, time: f32, animation_description: AnimationDescription) -> Self {
        Self {
            id,
            state,
            time,
            animation_description,
        }
    } 

    pub fn build(self) -> Animation {
        self.id
    }

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