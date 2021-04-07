#![allow(dead_code)]

use crate::state::style::*;

use crate::{MouseButton, State};

use crate::widgets::*;
use crate::widgets::{Button, Element, HBox, VBox};
use crate::AnimationState;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ScrollEvent {
    ScrollV(f32),
    ScrollH(f32),
}

pub struct ScrollContainerH {
    container: Entity,
    horizontal_scroll: Entity,
    //vertical_scroll: Entity,
    scrolly: f32,

    pressedx: f32,
    pressedy: f32,
    moving: bool,
    position: f32,

    vertical_scroll_animation: Animation,
    vertical_container_animation: Animation,

    scrollbar: bool,
    scroll_wheel: bool,
}

impl ScrollContainerH {
    pub fn new() -> Self {
        ScrollContainerH {
            container: Entity::null(),
            horizontal_scroll: Entity::null(),
            //vertical_scroll: Entity::null(),
            scrolly: 0.0,

            pressedx: 0.0,
            pressedy: 0.0,
            moving: false,
            position: 0.0,

            vertical_scroll_animation: Animation::default(),
            vertical_container_animation: Animation::default(),

            scrollbar: true,
            scroll_wheel: true,
        }
    }

    pub fn disable_scrollbar(mut self) -> Self {
        self.scrollbar = false;

        self
    }

    pub fn disable_scroll_wheel(mut self) -> Self {
        self.scroll_wheel = false;

        self
    }
}

impl Widget for ScrollContainerH {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_flex_direction(state, FlexDirection::Column);

        self.container = Button::new().build(state, entity, |builder| {
            builder
                .set_position(Position::Absolute)
                // .set_left(Units::Percentage(0.0))
                // .set_align_self(AlignSelf::FlexStart)
                //.set_background_color(Color::rgb(200, 70, 70))
                .class("container")
            //.set_hoverability(false)
        });

        state.style.clip_widget.insert(self.container, entity);

        if self.scrollbar {
            self.horizontal_scroll = Element::new().build(state, entity, |builder| {
                builder
                    .set_position(Position::Absolute)
                    // .set_left(Units::Percentage(0.0))
                    //.set_height(Units::Pixels(10.0))
                    // .set_width(Units::Percentage(0.0))
                    // .set_align_self(AlignSelf::FlexStart)
                    //.set_background_color(Color::rgb(70, 70, 200))
                    //.set_right(Units::Pixels(0.0))
                    .class("scrollbar")

                //
            });
        }

        self.horizontal_scroll.set_disabled(state, true);
        self.horizontal_scroll.set_enabled(state, false);

        state.style.insert_element(entity, "scroll_containerh");

        let vertical_scroll_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Units::Percentage(0.0)))
            .with_keyframe((1.0, Units::Percentage(0.2)));

        self.vertical_scroll_animation =
            state.style.left.insert_animation(vertical_scroll_animation);

        let vertical_container_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Units::Percentage(0.0)))
            .with_keyframe((1.0, Units::Percentage(-0.2)));

        self.vertical_container_animation = state
            .style
            .left
            .insert_animation(vertical_container_animation);

        self.container
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::GeometryChanged(geometry_changed) => {
                    if event.target == self.container || event.target == entity {
                        if geometry_changed.width || geometry_changed.height {
                            let mut scrollh =
                                state.data.get_width(entity) / state.data.get_width(self.container);

                            if scrollh >= 1.0 {
                                scrollh = 1.0;
                                self.horizontal_scroll.set_disabled(state, true);
                            }

                            if scrollh < 1.0 {
                                self.horizontal_scroll.set_enabled(state, true);
                            }

                            if !state.style.left.is_animating(self.horizontal_scroll) {
                                let dist = state.data.get_posx(self.horizontal_scroll)
                                    - state.data.get_posx(entity);
                                let space = state.data.get_width(entity)
                                    - (scrollh * state.data.get_width(entity));
                                self.scrolly = dist / space;
                            }

                            if self.scrolly.is_nan() {
                                self.scrolly = 0.0;
                            }

                            if self.scrolly < 0.0 {
                                self.scrolly = 0.0;
                            }

                            if self.scrolly >= 1.0 {
                                self.scrolly = 1.0;
                            }

                            // Setting it this way avoid calling Restyle automatically
                            state
                                .style
                                .width
                                .insert(self.horizontal_scroll, Units::Percentage(scrollh));

                            let overflow = 1.0
                                - (state.data.get_width(self.container)
                                    / state.data.get_width(entity));
                            let overflow2 = 1.0
                                - (state.data.get_width(entity)
                                    / state.data.get_width(self.container));

                            state.style.left.insert(
                                self.container,
                                Units::Percentage(self.scrolly * overflow),
                            );

                            state.style.left.insert(
                                self.horizontal_scroll,
                                Units::Percentage(self.scrolly * overflow2),
                            );

                            state.insert_event(Event::new(WindowEvent::Relayout).origin(entity));
                        }
                    }
                }

                WindowEvent::MouseScroll(_, y) => {
                    if self.scroll_wheel {
                        let overflow = state.data.get_height(entity)
                            - state.data.get_width(self.horizontal_scroll);

                        if overflow == 0.0 {
                            return;
                        }

                        // Need better names for these
                        let overflow = 1.0
                            - (state.data.get_width(self.container) / state.data.get_width(entity));
                        let overflow2 = 1.0
                            - (state.data.get_width(entity) / state.data.get_width(self.container));

                        self.scrolly += (30.0 * *y) / (state.data.get_width(entity) * overflow);

                        if self.scrolly < 0.0 {
                            self.scrolly = 0.0;
                        }

                        if self.scrolly > 1.0 {
                            self.scrolly = 1.0;
                        }

                        let _current_scroll_top = state
                            .style
                            .left
                            .get(self.horizontal_scroll)
                            .cloned()
                            .unwrap_or_default();
                        let _current_container_top = state
                            .style
                            .left
                            .get(self.container)
                            .cloned()
                            .unwrap_or_default();

                        self.container
                            .set_left(state, Units::Percentage(self.scrolly * overflow));
                        self.horizontal_scroll
                            .set_left(state, Units::Percentage(self.scrolly * overflow2));

                        state.insert_event(
                            Event::new(ScrollEvent::ScrollV(self.scrolly * overflow))
                                .target(entity),
                        );

                        // Capture the event to stop it triggering twice
                        event.consume();
                    }
                }

                WindowEvent::WindowResize(_, _) => {
                    // let scroll = state
                    //     .style
                    //     .scroll
                    //     .get(self.container)
                    //     .cloned()
                    //     .unwrap_or_default();

                    // event_manager.insert_event(
                    //     Event::new(StyleEvent::Restyle).target(state.root),
                    // );
                }

                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        if state.hovered == self.horizontal_scroll {
                            //println!("Clicked on scrollbar");
                            self.pressedx = state.mouse.cursorx;
                            self.pressedy = state.mouse.cursory;
                            self.moving = true;
                            // let scroll = state
                            //     .style
                            //     .scroll
                            //     .get(self.entity)
                            //     .cloned()
                            //     .unwrap_or_default();
                            //self.position = state.data.get_posy(self.vertical_scroll);
                            self.position = self.scrolly;
                            state.capture(entity);
                        }
                    }
                    _ => {}
                },

                WindowEvent::MouseUp(button) => match button {
                    MouseButton::Left => {
                        self.moving = false;
                        state.release(entity);
                    }

                    _ => {}
                },

                WindowEvent::MouseMove(x, _) => {
                    if self.moving {
                        let dist_x = *x - self.pressedx;
                        let overflow = state.data.get_width(entity)
                            - state.data.get_width(self.horizontal_scroll);

                        if overflow == 0.0 {
                            return;
                        }

                        let ratio = dist_x / overflow;
                        let r = self.position + ratio;

                        // let mut scrollh = state.data.get_height(entity) / state.data.get_height(self.container);
                        // if scrollh > 1.0 {
                        //     scrollh = 1.0;
                        // }

                        self.scrolly = r;

                        if self.scrolly < 0.0 {
                            self.scrolly = 0.0;
                        }

                        if self.scrolly > 1.0 {
                            self.scrolly = 1.0;
                        }

                        // let scroll = state
                        //     .style
                        //     .scroll
                        //     .get(self.entity)
                        //     .cloned()
                        //     .unwrap_or_default();
                        //self.vertical_scroll
                        //    .set_top(state, Units::Pixels(self.position + dist_y));

                        let overflow = 1.0
                            - (state.data.get_width(self.container) / state.data.get_width(entity));
                        let overflow2 = 1.0
                            - (state.data.get_width(entity) / state.data.get_width(self.container));

                        self.container
                            .set_left(state, Units::Percentage(self.scrolly * overflow));
                        self.horizontal_scroll
                            .set_left(state, Units::Percentage(self.scrolly * overflow2));

                        state.insert_event(
                            Event::new(ScrollEvent::ScrollH(self.scrolly * overflow))
                                .target(entity),
                        );

                        state.insert_event(Event::new(WindowEvent::Restyle));
                        state
                            .insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
                        state.insert_event(Event::new(WindowEvent::Redraw));
                        //println!("overflow: {}, dist: {}, ratio: {}, scrolly: {}", overflow, dist_y, r, self.scrolly);
                    }
                }

                _ => {}
            }
        }
    }
}

pub struct ScrollContainer {
    container: Entity,
    horizontal_scroll: Entity,
    vertical_scroll: Entity,
    scrolly: f32,

    pressedx: f32,
    pressedy: f32,
    moving: bool,
    position: f32,

    vertical_scroll_animation: Animation,
    vertical_container_animation: Animation,
}

impl ScrollContainer {
    pub fn new() -> Self {
        ScrollContainer {
            container: Entity::null(),
            horizontal_scroll: Entity::null(),
            vertical_scroll: Entity::null(),
            scrolly: 0.0,

            pressedx: 0.0,
            pressedy: 0.0,
            moving: false,
            position: 0.0,

            vertical_scroll_animation: Animation::default(),
            vertical_container_animation: Animation::default(),
        }
    }
}

impl Widget for ScrollContainer {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_flex_direction(state, FlexDirection::Row);

        //println!("Container: {}", self.container);

        self.container = Element::new().build(state, entity, |builder| {
            builder
                .set_position(Position::Absolute)
                //.set_top(Units::Percentage(0.0))
                //.set_flex_grow(1.0)
                //.set_align_self(AlignSelf::FlexStart)
                .class("container")
        });

        state.style.clip_widget.insert(self.container, entity);

        self.vertical_scroll = Element::new().build(state, entity, |builder| {
            builder
                .set_position(Position::Absolute)
                //.set_top(Units::Percentage(0.0))
                // .set_width(Units::Pixels(10.0))
                //.set_height(Units::Percentage(0.0))
                //.set_align_self(AlignSelf::FlexStart)
                //.set_background_color(Color::rgb(70, 200, 70))
                //.set_right(Units::Pixels(0.0))
                .class("scrollbar")

            //
        });

        self.vertical_scroll.set_disabled(state, true);
        self.vertical_scroll.set_enabled(state, false);

        // self.vertical_scroll =
        //     Scrollbar::new(self.container, Direction::Vertical).build(state, entity, |builder| {
        //         builder
        //             .set_width(Units::Pixels(10.0))
        //             .set_height(Units::Percentage(1.0))
        //             .set_background_color(Color::rgb(50, 50, 100))
        //     });

        state.style.insert_element(entity, "scroll_container");

        let vertical_scroll_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Units::Percentage(0.0)))
            .with_keyframe((1.0, Units::Percentage(0.2)));

        self.vertical_scroll_animation =
            state.style.top.insert_animation(vertical_scroll_animation);

        let vertical_container_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Units::Percentage(0.0)))
            .with_keyframe((1.0, Units::Percentage(-0.2)));

        self.vertical_container_animation = state
            .style
            .top
            .insert_animation(vertical_container_animation);

        self.container
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::GeometryChanged(geometry_changed) => {
                    //println!("Geometry Changed");

                    if event.target == self.container || event.target == entity {
                        if geometry_changed.width || geometry_changed.height {
                            //println!("Geometry Changed");
                            let mut scrollh = state.data.get_height(entity)
                                / state.data.get_height(self.container);

                            if scrollh >= 1.0 {
                                scrollh = 1.0;
                                self.vertical_scroll.set_disabled(state, true);
                            }

                            if scrollh < 1.0 {
                                self.vertical_scroll.set_enabled(state, true);
                            }

                            if !state.style.top.is_animating(self.vertical_scroll) {
                                let dist = state.data.get_posy(self.vertical_scroll)
                                    - state.data.get_posy(entity);
                                let space = state.data.get_height(entity)
                                    - (scrollh * state.data.get_height(entity));
                                self.scrolly = dist / space;
                            }

                            if self.scrolly.is_nan() {
                                self.scrolly = 0.0;
                            }

                            if self.scrolly < 0.0 {
                                self.scrolly = 0.0;
                            }

                            if self.scrolly >= 1.0 {
                                self.scrolly = 1.0;
                            }

                            // Setting it this way avoid calling Restyle automatically
                            state
                                .style
                                .height
                                .insert(self.vertical_scroll, Units::Percentage(scrollh));

                            let overflow = 1.0
                                - (state.data.get_height(self.container)
                                    / state.data.get_height(entity));
                            let overflow2 = 1.0
                                - (state.data.get_height(entity)
                                    / state.data.get_height(self.container));

                            state.style.top.insert(
                                self.container,
                                Units::Percentage(self.scrolly * overflow),
                            );

                            state.style.top.insert(
                                self.vertical_scroll,
                                Units::Percentage(self.scrolly * overflow2),
                            );

                            state.insert_event(
                                Event::new(WindowEvent::Relayout)
                                    .target(Entity::root())
                                    .origin(entity),
                            );
                        }
                    }
                }

                WindowEvent::MouseScroll(_, y) => {
                    //println!("Mouse Scroll Event");
                    // Forward mouse scroll event to the scrollbar
                    // state.insert_event(
                    //     Event::new(WindowEvent::MouseScroll(*x, *y))
                    //         .target(self.vertical_scroll)
                    //         .propagate(Propagation::None),
                    // );

                    //if event.target == entity {

                    //println!("Height: {}", state.data.get_height(entity));

                    let overflow =
                        state.data.get_height(entity) - state.data.get_height(self.vertical_scroll);

                    if overflow == 0.0 {
                        return;
                    }

                    // Need better names for these
                    let overflow = 1.0
                        - (state.data.get_height(self.container) / state.data.get_height(entity));
                    let overflow2 = 1.0
                        - (state.data.get_height(entity) / state.data.get_height(self.container));

                    // TODO - Need a way to configure this
                    self.scrolly += (30.0 * *y) / (state.data.get_height(entity) * overflow);

                    if self.scrolly < 0.0 {
                        self.scrolly = 0.0;
                    }

                    if self.scrolly > 1.0 {
                        self.scrolly = 1.0;
                    }

                    //println!("Scroll: {}", self.scrolly);

                    // let mut scrollh = state.data.get_height(entity) / state.data.get_height(self.container);
                    // if scrollh > 1.0 {
                    //     scrollh = 1.0;
                    // }

                    let _current_scroll_top = state
                        .style
                        .top
                        .get(self.vertical_scroll)
                        .cloned()
                        .unwrap_or_default();
                    let _current_container_top = state
                        .style
                        .top
                        .get(self.container)
                        .cloned()
                        .unwrap_or_default();

                    self.container
                        .set_top(state, Units::Percentage(self.scrolly * overflow));
                    self.vertical_scroll
                        .set_top(state, Units::Percentage(self.scrolly * overflow2));

                    state.insert_event(
                        Event::new(ScrollEvent::ScrollV(self.scrolly * overflow)).target(entity),
                    );

                    /*
                    if let Some(animation) = state.style.top.get_animation_mut(self.vertical_scroll_animation) {
                        *animation.keyframes.first_mut().unwrap() = (0.0, current_scroll_top);
                        *animation.keyframes.last_mut().unwrap() = (1.0, Units::Percentage(self.scrolly * overflow2));
                    }

                    state.style.top.play_animation(self.vertical_scroll, self.vertical_scroll_animation);





                    if let Some(animation) = state.style.top.get_animation_mut(self.vertical_container_animation) {
                        *animation.keyframes.first_mut().unwrap() = (0.0, current_container_top);
                        *animation.keyframes.last_mut().unwrap() = (1.0, Units::Percentage(self.scrolly * overflow));
                    }

                    state.style.top.play_animation(self.container, self.vertical_container_animation);
                    */

                    // Capture the event to stop it triggering twice
                    event.consume();
                }

                WindowEvent::WindowResize(_, _) => {
                    // let scroll = state
                    //     .style
                    //     .scroll
                    //     .get(self.container)
                    //     .cloned()
                    //     .unwrap_or_default();

                    // event_manager.insert_event(
                    //     Event::new(StyleEvent::Restyle).target(state.root),
                    // );
                }

                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        if state.hovered == self.vertical_scroll {
                            //println!("Clicked on scrollbar");
                            self.pressedx = state.mouse.cursorx;
                            self.pressedy = state.mouse.cursory;
                            self.moving = true;
                            // let scroll = state
                            //     .style
                            //     .scroll
                            //     .get(self.entity)
                            //     .cloned()
                            //     .unwrap_or_default();
                            //self.position = state.data.get_posy(self.vertical_scroll);
                            self.position = self.scrolly;
                            state.capture(entity);
                        }
                    }
                    _ => {}
                },

                WindowEvent::MouseUp(button) => match button {
                    MouseButton::Left => {
                        self.moving = false;
                        state.release(entity);
                    }

                    _ => {}
                },

                WindowEvent::MouseMove(_, y) => {
                    if self.moving {
                        let dist_y = *y - self.pressedy;
                        let overflow = state.data.get_height(entity)
                            - state.data.get_height(self.vertical_scroll);

                        if overflow == 0.0 {
                            return;
                        }

                        let ratio = dist_y / overflow;
                        let r = self.position + ratio;

                        // let mut scrollh = state.data.get_height(entity) / state.data.get_height(self.container);
                        // if scrollh > 1.0 {
                        //     scrollh = 1.0;
                        // }

                        self.scrolly = r;

                        if self.scrolly < 0.0 {
                            self.scrolly = 0.0;
                        }

                        if self.scrolly > 1.0 {
                            self.scrolly = 1.0;
                        }

                        // let scroll = state
                        //     .style
                        //     .scroll
                        //     .get(self.entity)
                        //     .cloned()
                        //     .unwrap_or_default();
                        //self.vertical_scroll
                        //    .set_top(state, Units::Pixels(self.position + dist_y));

                        let overflow = 1.0
                            - (state.data.get_height(self.container)
                                / state.data.get_height(entity));
                        let overflow2 = 1.0
                            - (state.data.get_height(entity)
                                / state.data.get_height(self.container));

                        self.container
                            .set_top(state, Units::Percentage(self.scrolly * overflow));
                        self.vertical_scroll
                            .set_top(state, Units::Percentage(self.scrolly * overflow2));

                        state.insert_event(
                            Event::new(ScrollEvent::ScrollV(self.scrolly * overflow))
                                .target(entity),
                        );

                        state.insert_event(Event::new(WindowEvent::Restyle));
                        state
                            .insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
                        state.insert_event(Event::new(WindowEvent::Redraw));
                        //println!("overflow: {}, dist: {}, ratio: {}, scrolly: {}", overflow, dist_y, r, self.scrolly);
                    }
                }

                _ => {}
            }
        }
    }
}

//

pub struct ScrollContainerHV {
    container: Entity,
    horizontal_scroll: Entity,
    vertical_scroll: Entity,
    scrollx: f32,
    scrolly: f32,

    pressedx: f32,
    pressedy: f32,
    moving: bool,
    position: f32,
    //vertical_scroll_animation: usize,
    //vertical_container_animation: usize,
}

impl ScrollContainerHV {
    pub fn new() -> Self {
        ScrollContainerHV {
            container: Entity::null(),
            horizontal_scroll: Entity::null(),
            vertical_scroll: Entity::null(),
            scrollx: 0.0,
            scrolly: 0.0,

            pressedx: 0.0,
            pressedy: 0.0,
            moving: false,
            position: 0.0,
            //vertical_scroll_animation: std::usize::MAX,
            //vertical_container_animation: std::usize::MAX,
        }
    }
}

impl Widget for ScrollContainerHV {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_flex_direction(state, FlexDirection::Row)
            .set_flex_grow(state, 1.0)
            .set_flex_shrink(state, 1.0);

        let hbox = HBox::new().build(state, entity, |builder| {
            builder.set_flex_grow(1.0).set_flex_shrink(1.0)
        });

        let vbox = VBox::new().build(state, hbox, |builder| {
            builder.set_flex_grow(1.0).set_flex_shrink(1.0)
        });

        self.container = Button::new().build(state, vbox, |builder| {
            builder
                .set_top(Units::Percentage(0.0))
                .set_align_self(AlignSelf::FlexStart)
                .class("container")
        });

        state.style.clip_widget.insert(self.container, entity);

        //println!("Container: {}", self.container);

        self.vertical_scroll = Element::new().build(state, hbox, |builder| {
            builder
                //.set_position(Position::Absolute)
                .set_top(Units::Percentage(0.0))
                .set_width(Units::Pixels(10.0))
                .set_height(Units::Percentage(1.0))
                .set_align_self(AlignSelf::FlexStart)
            //.set_background_color(Color::rgb(70, 200, 70))
            //.set_right(Units::Pixels(0.0))
            //.class("scrollbar")

            //
        });

        self.vertical_scroll = Element::new().build(state, hbox, |builder| {
            builder
                //.set_position(Position::Absolute)
                .set_left(Units::Percentage(0.0))
                .set_height(Units::Pixels(10.0))
                .set_width(Units::Percentage(1.0))
                .set_align_self(AlignSelf::FlexStart)
            //.set_background_color(Color::rgb(20, 70, 200))
            //.set_right(Units::Pixels(0.0))
            //.class("scrollbar")

            //
        });

        //self.vertical_scroll.set_disabled(state, true);

        // self.vertical_scroll =
        //     Scrollbar::new(self.container, Direction::Vertical).build(state, entity, |builder| {
        //         builder
        //             .set_width(Units::Pixels(10.0))
        //             .set_height(Units::Percentage(1.0))
        //             .set_background_color(Color::rgb(50, 50, 100))
        //     });

        state.style.insert_element(entity, "scroll_containerhv");

        self.container
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::Relayout => {
                    // // To prevent recursive loop when layout event is triggered inside here
                    if event.origin != entity
                        && event.origin != self.container
                        && event.origin != self.vertical_scroll
                    {
                        let mut scrollv =
                            state.data.get_height(entity) / state.data.get_height(self.container);

                        if scrollv >= 1.0 {
                            scrollv = 1.0;
                            self.vertical_scroll.set_disabled(state, true);
                        }

                        if scrollv < 1.0 {
                            self.vertical_scroll.set_enabled(state, true);
                        }

                        let mut scrollh =
                            state.data.get_width(entity) / state.data.get_width(self.container);

                        if scrollh >= 1.0 {
                            scrollh = 1.0;
                            self.horizontal_scroll.set_disabled(state, true);
                        }

                        if scrollh < 1.0 {
                            self.horizontal_scroll.set_enabled(state, true);
                        }

                        // BUG: fast scrolling causes smaller scroll because the animation hasn't finished when this function is called again
                        // One way to fix this might be to check whether the value is currently being animated before setting here
                        // Possibly not the best solution but it works
                        // if !state.style.top.is_animating(self.vertical_scroll) {
                        //     let dist = state.data.get_posy(self.vertical_scroll)
                        //         - state.data.get_posy(entity);
                        //     let space = state.data.get_height(entity)
                        //         - (scrollh * state.data.get_height(entity));
                        //     self.scrolly = dist / space;
                        // }

                        if self.scrolly.is_nan() {
                            self.scrolly = 0.0;
                        }

                        if self.scrolly < 0.0 {
                            self.scrolly = 0.0;
                        }

                        if self.scrolly >= 1.0 {
                            self.scrolly = 1.0;
                        }

                        // Setting it this way avoid calling Restyle automatically
                        state
                            .style
                            .height
                            .insert(self.vertical_scroll, Units::Percentage(scrollv));

                        let overflow = 1.0
                            - (state.data.get_height(self.container)
                                / state.data.get_height(entity));
                        let overflow2 = 1.0
                            - (state.data.get_height(entity)
                                / state.data.get_height(self.container));

                        state
                            .style
                            .top
                            .insert(self.container, Units::Percentage(self.scrolly * overflow));

                        state.style.top.insert(
                            self.vertical_scroll,
                            Units::Percentage(self.scrolly * overflow2),
                        );

                        if self.scrollx.is_nan() {
                            self.scrollx = 0.0;
                        }

                        if self.scrollx < 0.0 {
                            self.scrollx = 0.0;
                        }

                        if self.scrollx >= 1.0 {
                            self.scrollx = 1.0;
                        }

                        // Setting it this way avoid calling Restyle automatically
                        state
                            .style
                            .width
                            .insert(self.vertical_scroll, Units::Percentage(scrollh));

                        let overflow = 1.0
                            - (state.data.get_width(self.container) / state.data.get_width(entity));
                        let overflow2 = 1.0
                            - (state.data.get_width(entity) / state.data.get_width(self.container));

                        state
                            .style
                            .left
                            .insert(self.container, Units::Percentage(self.scrollx * overflow));

                        state.style.top.insert(
                            self.vertical_scroll,
                            Units::Percentage(self.scrolly * overflow2),
                        );

                        // Relayout and Redraw wont get called automatically so need to manually trigger them
                        state.insert_event(Event::new(WindowEvent::Relayout).origin(entity));
                        //state.insert_event(Event::new(WindowEvent::Redraw));
                        //return true;
                    }
                }

                /*
                WindowEvent::MouseScroll(_, y) => {
                    println!("Mouse Scroll Event");
                    // Forward mouse scroll event to the scrollbar
                    // state.insert_event(
                    //     Event::new(WindowEvent::MouseScroll(*x, *y))
                    //         .target(self.vertical_scroll)
                    //         .propagate(Propagation::None),
                    // );

                    //if event.target == entity {

                    println!("Height: {}", state.data.get_height(entity));

                    let overflow = state.data.get_height(entity)
                        - state.data.get_height(self.vertical_scroll);

                    if overflow == 0.0 {
                        return false;
                    }

                    // Need better names for these
                    let overflow = 1.0
                        - (state.data.get_height(self.container)
                            / state.data.get_height(entity));
                    let overflow2 = 1.0
                        - (state.data.get_height(entity)
                            / state.data.get_height(self.container));

                    self.scrolly += (40.0 * *y) / (state.data.get_height(entity) * overflow);

                    if self.scrolly < 0.0 {
                        self.scrolly = 0.0;
                    }

                    if self.scrolly > 1.0 {
                        self.scrolly = 1.0;
                    }

                    //println!("Scroll: {}", self.scrolly);

                    // let mut scrollh = state.data.get_height(entity) / state.data.get_height(self.container);
                    // if scrollh > 1.0 {
                    //     scrollh = 1.0;
                    // }

                    let current_scroll_top = state
                        .style
                        .top
                        .get(self.vertical_scroll)
                        .cloned()
                        .unwrap_or_default();
                    let current_container_top = state
                        .style
                        .top
                        .get(self.container)
                        .cloned()
                        .unwrap_or_default();

                    self.container
                        .set_top(state, Units::Percentage(self.scrolly * overflow));
                    self.vertical_scroll
                        .set_top(state, Units::Percentage(self.scrolly * overflow2));

                    /*
                    if let Some(animation) = state.style.top.get_animation_mut(self.vertical_scroll_animation) {
                        *animation.keyframes.first_mut().unwrap() = (0.0, current_scroll_top);
                        *animation.keyframes.last_mut().unwrap() = (1.0, Units::Percentage(self.scrolly * overflow2));
                    }

                    state.style.top.play_animation(self.vertical_scroll, self.vertical_scroll_animation);





                    if let Some(animation) = state.style.top.get_animation_mut(self.vertical_container_animation) {
                        *animation.keyframes.first_mut().unwrap() = (0.0, current_container_top);
                        *animation.keyframes.last_mut().unwrap() = (1.0, Units::Percentage(self.scrolly * overflow));
                    }

                    state.style.top.play_animation(self.container, self.vertical_container_animation);
                    */

                    //println!("A: {:?}  B: {:?}", current_container_top, self.scrolly * overflow);

                    //state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
                    //state.insert_event(Event::new(WindowEvent::Redraw));
                    //}

                    // Capture the event to stop it triggering twice
                    return true;
                }
                */
                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        if state.hovered == self.vertical_scroll {
                            //println!("Clicked on scrollbar");
                            self.pressedx = state.mouse.cursorx;
                            self.pressedy = state.mouse.cursory;
                            self.moving = true;
                            // let scroll = state
                            //     .style
                            //     .scroll
                            //     .get(self.entity)
                            //     .cloned()
                            //     .unwrap_or_default();
                            //self.position = state.data.get_posy(self.vertical_scroll);
                            self.position = self.scrolly;
                            state.capture(entity);
                        }

                        if state.hovered == self.horizontal_scroll {
                            //println!("Clicked on scrollbar");
                            self.pressedx = state.mouse.cursorx;
                            self.pressedy = state.mouse.cursory;
                            self.moving = true;
                            // let scroll = state
                            //     .style
                            //     .scroll
                            //     .get(self.entity)
                            //     .cloned()
                            //     .unwrap_or_default();
                            //self.position = state.data.get_posy(self.vertical_scroll);
                            self.position = self.scrollx;
                            state.capture(entity);
                        }
                    }
                    _ => {}
                },

                WindowEvent::MouseUp(button) => match button {
                    MouseButton::Left => {
                        self.moving = false;
                        //println!("Scroll release");
                        state.release(entity);
                    }

                    _ => {}
                },

                WindowEvent::MouseMove(x, y) => {
                    if self.moving && state.captured == self.vertical_scroll {
                        let dist_y = *y - self.pressedy;
                        let overflow = state.data.get_height(entity)
                            - state.data.get_height(self.vertical_scroll);

                        if overflow == 0.0 {
                            return;
                        }

                        let ratio = dist_y / overflow;
                        let r = self.position + ratio;

                        // let mut scrollh = state.data.get_height(entity) / state.data.get_height(self.container);
                        // if scrollh > 1.0 {
                        //     scrollh = 1.0;
                        // }

                        self.scrolly = r;

                        if self.scrolly < 0.0 {
                            self.scrolly = 0.0;
                        }

                        if self.scrolly > 1.0 {
                            self.scrolly = 1.0;
                        }

                        // let scroll = state
                        //     .style
                        //     .scroll
                        //     .get(self.entity)
                        //     .cloned()
                        //     .unwrap_or_default();
                        //self.vertical_scroll
                        //    .set_top(state, Units::Pixels(self.position + dist_y));

                        let overflow = 1.0
                            - (state.data.get_height(self.container)
                                / state.data.get_height(entity));
                        let overflow2 = 1.0
                            - (state.data.get_height(entity)
                                / state.data.get_height(self.container));

                        self.container
                            .set_top(state, Units::Percentage(self.scrolly * overflow));
                        self.vertical_scroll
                            .set_top(state, Units::Percentage(self.scrolly * overflow2));

                        state.insert_event(Event::new(WindowEvent::Restyle));
                        state
                            .insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
                        state.insert_event(Event::new(WindowEvent::Redraw));
                        //println!("overflow: {}, dist: {}, ratio: {}, scrolly: {}", overflow, dist_y, r, self.scrolly);
                    }

                    if self.moving && state.captured == self.horizontal_scroll {
                        let dist_x = *x - self.pressedx;
                        let overflow = state.data.get_width(entity)
                            - state.data.get_width(self.vertical_scroll);

                        if overflow == 0.0 {
                            return;
                        }

                        let ratio = dist_x / overflow;
                        let r = self.position + ratio;

                        // let mut scrollh = state.data.get_height(entity) / state.data.get_height(self.container);
                        // if scrollh > 1.0 {
                        //     scrollh = 1.0;
                        // }

                        self.scrollx = r;

                        if self.scrollx < 0.0 {
                            self.scrollx = 0.0;
                        }

                        if self.scrollx > 1.0 {
                            self.scrollx = 1.0;
                        }

                        // let scroll = state
                        //     .style
                        //     .scroll
                        //     .get(self.entity)
                        //     .cloned()
                        //     .unwrap_or_default();
                        //self.vertical_scroll
                        //    .set_top(state, Units::Pixels(self.position + dist_y));

                        let overflow = 1.0
                            - (state.data.get_width(self.container) / state.data.get_width(entity));
                        let overflow2 = 1.0
                            - (state.data.get_width(entity) / state.data.get_width(self.container));

                        self.container
                            .set_left(state, Units::Percentage(self.scrollx * overflow));
                        self.vertical_scroll
                            .set_left(state, Units::Percentage(self.scrollx * overflow2));

                        state.insert_event(Event::new(WindowEvent::Restyle));
                        state
                            .insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
                        state.insert_event(Event::new(WindowEvent::Redraw));
                        //println!("overflow: {}, dist: {}, ratio: {}, scrolly: {}", overflow, dist_y, r, self.scrolly);
                    }
                }

                _ => {}
            }
        }
    }
}
