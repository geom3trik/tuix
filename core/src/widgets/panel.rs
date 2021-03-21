#![allow(dead_code)]

use crate::widgets::*;
use crate::widgets::{Element, Label};
use crate::AnimationState;

use crate::state::style::*;

const ICON_DOWN_OPEN_BIG: &str = "\u{e75c}";
const ICON_RIGHT_OPEN_BIG: &str = "\u{e75e}";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelEvent {
    Open,
    Close,
}

pub struct Panel {
    header: Entity,
    container1: Entity,
    container2: Entity,
    arrow: Entity,
    collapsed: bool,
    title: String,

    container_height: f32,
    container_width: f32,

    expand_height_animation: usize,
    collapse_height_animation: usize,
    expand_width_animation: usize,
    collapse_width_animation: usize,

    fade_in_animation: usize,
    fade_out_animation: usize,

    move_up_animation: usize,
    move_down_animation: usize,

    arrow_cw_animation: usize,
    arrow_ccw_animation: usize,
}

impl Panel {
    pub fn new(title: &str) -> Self {
        Panel {
            header: Entity::default(),
            container1: Entity::default(),
            container2: Entity::default(),
            arrow: Entity::default(),
            title: title.to_string(),
            collapsed: false,

            container_height: 0.0,
            container_width: 0.0,

            expand_height_animation: std::usize::MAX,
            collapse_height_animation: std::usize::MAX,  
            expand_width_animation: std::usize::MAX,
            collapse_width_animation: std::usize::MAX,  

            fade_in_animation: std::usize::MAX,
            fade_out_animation: std::usize::MAX,

            move_up_animation: std::usize::MAX,
            move_down_animation: std::usize::MAX,

            arrow_cw_animation: std::usize::MAX,
            arrow_ccw_animation: std::usize::MAX,
        }
    }
}

impl Widget for Panel {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        entity.set_focusability(state, false);

        self.header = Button::new()
        .on_release(Event::new(PanelEvent::Open).target(entity))
        .build(state, entity, |builder| {
            builder
                //.set_flex_direction(FlexDirection::Row)
                .class("header")
        });

        self.arrow = Element::new().build(state, self.header, |builder| {
            builder
                .set_text(ICON_DOWN_OPEN_BIG)
                .set_font("icons")
                .set_text_justify(Justify::Center)
                .set_text_align(Align::Center)
                .set_flex_basis(Length::Pixels(20.0))
                .set_hoverability(false)
                .set_focusability(false)
                .class("icon")
        });

        // Label
        Label::new(&self.title).build(state, self.header, |builder| {
            builder
                .set_flex_grow(1.0)
                .set_hoverability(false)
                .set_focusability(false)
                .class("label")
        });

        self.container1 = Element::new().build(state, entity, |builder| {
            builder.class("container1").set_focusability(false)
        });

        self.container2 = Element::new().build(state,self.container1,|builder| 
            builder.class("container2").set_focusability(false).set_clip_widget(self.container1)
        );

        state.style.insert_element(entity, "panel");


        // Animations
        let container_expand_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Length::Pixels(0.0)))
            .with_keyframe((1.0, Length::Pixels(0.0)));

        self.expand_height_animation = state
            .style
            .height
            .insert_animation(container_expand_animation.clone());
        
        self.expand_width_animation = state
            .style
            .width
            .insert_animation(container_expand_animation.clone());

        let container_collapse_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_delay(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Length::Pixels(0.0)))
            .with_keyframe((1.0, Length::Pixels(0.0)));

        self.collapse_height_animation = state
            .style
            .height
            .insert_animation(container_collapse_animation.clone());

        self.collapse_width_animation = state
            .style
            .width
            .insert_animation(container_collapse_animation.clone());

        let container_fade_in_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_delay(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Opacity(0.0)))
            .with_keyframe((1.0, Opacity(1.0)));

        let container_move_up_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_delay(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Length::Pixels(0.0)))
            .with_keyframe((1.0, Length::Pixels(0.0)));
        
        self.move_up_animation = state.style.top.insert_animation(container_move_up_animation);


        let container_move_down_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Length::Pixels(0.0)))
            .with_keyframe((1.0, Length::Pixels(0.0)));
    
        self.move_down_animation = state.style.top.insert_animation(container_move_down_animation); 

        self.fade_in_animation = state
            .style
            .opacity
            .insert_animation(container_fade_in_animation);

        let container_fade_out_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Opacity(1.0)))
            .with_keyframe((1.0, Opacity(0.0)));

        self.fade_out_animation = state
            .style
            .opacity
            .insert_animation(container_fade_out_animation);

        let arrow_cw_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, -90.0))
            .with_keyframe((1.0, 0.0));

        self.arrow_cw_animation = state.style.rotate.insert_animation(arrow_cw_animation);

        let arrow_ccw_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, 0.0))
            .with_keyframe((1.0, -90.0));

        self.arrow_ccw_animation = state
            .style
            .rotate
            .insert_animation(arrow_ccw_animation);

        self.container2
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(panel_event) = event.message.downcast::<PanelEvent>() {
            match panel_event {
                PanelEvent::Open | PanelEvent::Close => {
                    if self.collapsed {

                        self.collapsed = false;

                        entity.set_checked(state, true);
                        // state.insert_event(
                        //     Event::new(PanelEvent::Open).target(entity),
                        // );

                        match entity.get_flex_direction(state) {
                            FlexDirection::Column | FlexDirection::ColumnReverse => {
                                state
                                    .style
                                    .height
                                    .play_animation(self.container1, self.expand_height_animation);
                                
                                self.container1.set_height(state, Length::Auto);

                                if let Some(animation) =
                                    state.style.rotate.get_animation_mut(self.arrow_cw_animation)
                                {
                                    animation.set_delay(std::time::Duration::from_millis(0));
                                }

                                if let Some(animation) =
                                    state.style.rotate.get_animation_mut(self.arrow_ccw_animation)
                                {
                                    animation.set_delay(std::time::Duration::from_millis(100));
                                }

                                state
                                    .style
                                    .rotate
                                    .play_animation(self.arrow, self.arrow_cw_animation);
                                self.arrow.set_rotate(state, 0.0);
                            }

                            FlexDirection::Row | FlexDirection::RowReverse => {
                                state
                                    .style
                                    .width
                                    .play_animation(self.container1, self.expand_width_animation);
                                
                                self.container1.set_width(state, Length::Auto);

                                if let Some(animation) =
                                    state.style.rotate.get_animation_mut(self.arrow_cw_animation)
                                {
                                    animation.set_delay(std::time::Duration::from_millis(100));
                                }

                                if let Some(animation) =
                                    state.style.rotate.get_animation_mut(self.arrow_ccw_animation)
                                {
                                    animation.set_delay(std::time::Duration::from_millis(0));
                                }

                                state
                                    .style
                                    .rotate
                                    .play_animation(self.arrow, self.arrow_ccw_animation);

                                self.arrow.set_rotate(state, -90.0);


                            }
                        }
                        
                        // state
                        //     .style
                        //     .opacity
                        //     .play_animation(self.container2, self.fade_in_animation);

                            
                        state
                            .style
                            .top
                            .play_animation(self.container2, self.move_down_animation);

                        
                        //self.container2.set_opacity(state, 1.0);

                    } else {
                        self.collapsed = true;

                        entity.set_checked(state, false);

                        // state.insert_event(
                        //     Event::new(PanelEvent::Close).target(entity),
                        // );

                        match entity.get_flex_direction(state) {
                            FlexDirection::Column | FlexDirection::ColumnReverse => {
                                state
                                    .style
                                    .height
                                    .play_animation(self.container1, self.collapse_height_animation);
                                
                                self.container1.set_height(state, Length::Pixels(0.0));

                                state
                                    .style
                                    .rotate
                                    .play_animation(self.arrow, self.arrow_ccw_animation);

                                self.arrow.set_rotate(state, -90.0);
                            }

                            FlexDirection::Row | FlexDirection::RowReverse => {
                                state
                                    .style
                                    .width
                                    .play_animation(self.container1, self.collapse_width_animation);
                                
                                self.container1.set_width(state, Length::Pixels(0.0));

                                state
                                    .style
                                    .rotate
                                    .play_animation(self.arrow, self.arrow_cw_animation);
                                self.arrow.set_rotate(state, 0.0);
                            }
                        }

                        // state
                        //     .style
                        //     .opacity
                        //     .play_animation(self.container2, self.fade_out_animation);

                        state
                            .style
                            .top
                            .play_animation(self.container2, self.move_up_animation);
                        
                        //self.container2.set_opacity(state, 0.0);
                    }
                }
            }
        }

        //if event.target == self.header {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::GeometryChanged(_) => {
                    if event.target == self.container2 {

                        match entity.get_flex_direction(state) {
                            FlexDirection::Column | FlexDirection::ColumnReverse => {
                                if !state.style.height.is_animating(self.container1) {
                                
                                    let container_height = state.data.get_height(self.container1);
                                    if container_height > 0.0 {
                                        //self.container_height = container_height;

                                        if let Some(animation) =
                                            state.style.height.get_animation_mut(self.expand_height_animation)
                                        {
                                            animation.keyframes.last_mut().unwrap().1 =
                                                Length::Pixels(container_height);
                                        }
                                        
                                        if let Some(animation) = state
                                            .style
                                            .height
                                            .get_animation_mut(self.collapse_height_animation)
                                        {
                                            animation.keyframes.first_mut().unwrap().1 =
                                                Length::Pixels(container_height);
                                        }

                                        if let Some(animation) = state
                                            .style
                                            .top
                                            .get_animation_mut(self.move_down_animation)
                                        {
                                            animation.keyframes.first_mut().unwrap().1 =
                                                Length::Pixels(-container_height);
                                        }

                                        if let Some(animation) =
                                            state.style.top.get_animation_mut(self.move_up_animation)
                                        {
                                            animation.keyframes.last_mut().unwrap().1 =
                                                Length::Pixels(-container_height);
                                        }


                                    }
                                }                                
                            }

                            FlexDirection::Row | FlexDirection::RowReverse => {

                                self.arrow.set_rotate(state, -90.0);

                                if !state.style.width.is_animating(self.container1) {
                                
                                    let container_width = state.data.get_width(self.container1);
                                    if container_width > 0.0 {

                                        if let Some(animation) =
                                            state.style.width.get_animation_mut(self.expand_width_animation)
                                        {
                                            animation.keyframes.last_mut().unwrap().1 =
                                                Length::Pixels(container_width);
                                        }
                                        
                                        if let Some(animation) = state
                                            .style
                                            .width
                                            .get_animation_mut(self.collapse_width_animation)
                                        {
                                            animation.keyframes.first_mut().unwrap().1 =
                                                Length::Pixels(container_width);
                                        }
                                    }
                                }
                            }
                        }


                    }
                }

                // WindowEvent::MouseUp(button) => {
                //     if event.target == self.header && state.mouse.left.pressed == self.header {
                //         if *button == MouseButton::Left {
                            
                //         }

                //         event.consume();
                //     }
                // }

                _ => {}
            }
        }
    }
}
