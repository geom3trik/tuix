use crate::common::*;
use crate::{Row, Column};

use tuix_derive::Lens;

// #[derive(Debug, Copy, Clone, PartialEq)]
// pub enum ScrollEvent {
//     Scroll(f32, f32, f32),
// }


#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Scroll {
    pub scroll_pos: f32,
    pub scroll_size: f32,
    pub overflow: f32,
}

// #[derive(Debug, Default, Lens, Clone, Copy)]
// pub struct ScrollData {
//     scroll: Scroll,
// }

// impl Model for ScrollData {
//     fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
//         if let Some(scroll_event) = event.message.downcast() {
//             match scroll_event {

//                 ScrollEvent::Scroll(pos, size, overflow) => {
//                     self.scroll.scroll_pos = *pos;
//                     self.scroll.scroll_size = *size;
//                     self.scroll.overflow = *overflow;
//                     entity.emit(state, BindEvent::Update);
//                     event.consume();
//                 }
//             }
//         }
//     }
// }

pub struct ScrollContainerH {
    container: Entity,
    horizontal_scroll: Entity,
    //vertical_scroll: Entity,
    //scrolly: f32,
    //scrollh: f32,

    pub scroll: Scroll,

    pressedx: f32,
    pressedy: f32,
    moving: bool,
    position: f32,

    vertical_scroll_animation: Animation,
    vertical_container_animation: Animation,

    scrollbar: bool,
    scroll_wheel: bool,

    on_scroll: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
}

impl ScrollContainerH {
    pub fn new() -> Self {
        ScrollContainerH {
            container: Entity::null(),
            horizontal_scroll: Entity::null(),
            //vertical_scroll: Entity::null(),
            //scrolly: 0.0,
            //scrollh: 0.0,

            scroll: Scroll::default(),

            pressedx: 0.0,
            pressedy: 0.0,
            moving: false,
            position: 0.0,

            vertical_scroll_animation: Animation::default(),
            vertical_container_animation: Animation::default(),

            scrollbar: true,
            scroll_wheel: true,

            on_scroll: None,
        }
    }

    // TODO
    pub fn disable_scrollbar(mut self) -> Self {
        self.scrollbar = false;

        self
    }

    pub fn disable_scroll_wheel(mut self) -> Self {
        self.scroll_wheel = false;

        self
    }

    pub fn on_scroll<F>(mut self, callback: F) -> Self 
    where F: 'static + Fn(&mut Self, &mut State, Entity)
    {
        self.on_scroll = Some(Box::new(callback));

        self
    }
}

impl Widget for ScrollContainerH {
    type Ret = Entity;
    type Data = Scroll;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_layout_type(state, LayoutType::Column)
            .set_min_width(state, Pixels(0.0));

        self.container = Element::new().build(state, entity, |builder| {
            builder
                //.set_position_type(PositionType::SelfDirected)
                .set_width(Auto)
                // .set_left(Units::Percentage(0.0))
                // .set_align_self(AlignSelf::FlexStart)
                //.set_background_color(Color::rgb(200, 70, 70))
                .class("container")
            //.set_hoverable(false)
        });

        state.style.clip_widget.insert(self.container, entity);

        //if self.scrollbar {
            self.horizontal_scroll = Element::new().build(state, entity, |builder| {
                builder
                    //.set_position_type(PositionType::SelfDirected)
                    .set_min_width(Pixels(0.0))
                    // .set_left(Units::Percentage(0.0))
                    //.set_height(Units::Pixels(10.0))
                    // .set_width(Units::Percentage(0.0))
                    // .set_align_self(AlignSelf::FlexStart)
                    //.set_background_color(Color::rgb(70, 70, 200))
                    //.set_right(Units::Pixels(0.0))
                    .class("scrollbar")

                //
            });
        //}

        entity.set_disabled(state, true);

        entity.set_element(state, "scroll_containerh");

        let vertical_scroll_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Units::Percentage(0.0)))
            .with_keyframe((1.0, Units::Percentage(20.0)));

        self.vertical_scroll_animation =
            state.style.left.insert_animation(vertical_scroll_animation);

        let vertical_container_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Units::Percentage(0.0)))
            .with_keyframe((1.0, Units::Percentage(-20.0)));

        self.vertical_container_animation = state
            .style
            .left
            .insert_animation(vertical_container_animation);

        self.container
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        //self.scroll.scroll_pos = data.scroll_pos;
        //self.scroll.scroll_size = data.scroll_size;

        self.scroll = *data;

        // let overflow = 1.0
        //     - (state.data.get_width(self.container)
        //         / state.data.get_width(entity));
        // let overflow2 = 1.0
        //     - (state.data.get_width(entity)
        //         / state.data.get_width(self.container));

        let overflow2 = 1.0 - (1.0 / (1.0 - self.scroll.overflow));

        self.container
            .set_left(state, Units::Percentage(self.scroll.scroll_pos * self.scroll.overflow * 100.0));
        self.horizontal_scroll
            .set_left(state, Units::Percentage(self.scroll.scroll_pos * overflow2 * 100.0));
    }


    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::GeometryChanged(geometry_changed) => {
                    if event.target == self.container || event.target == entity {
                        if geometry_changed.width || geometry_changed.height {
                            self.scroll.scroll_size =
                                state.data.get_width(entity) / state.data.get_width(self.container);

                            if self.scroll.scroll_size >= 1.0 {
                                self.scroll.scroll_size = 1.0;
                                entity.set_disabled(state, true);
                            }

                            if self.scroll.scroll_size < 1.0 {
                                entity.set_disabled(state, false);
                            }

                            if !state.style.left.is_animating(self.horizontal_scroll) {
                                let dist = state.data.get_posx(self.horizontal_scroll)
                                    - state.data.get_posx(entity);
                                let space = state.data.get_width(entity)
                                    - (self.scroll.scroll_size * state.data.get_width(entity));
                                self.scroll.scroll_pos = dist / space;
                            }

                            if self.scroll.scroll_pos.is_nan() {
                                self.scroll.scroll_pos = 0.0;
                            }

                            if self.scroll.scroll_pos < 0.0 {
                                self.scroll.scroll_pos = 0.0;
                            }

                            if self.scroll.scroll_pos >= 1.0 {
                                self.scroll.scroll_pos = 1.0;
                            }

                            // Setting it this way avoid calling Restyle automatically
                            state
                                .style
                                .width
                                .insert(self.horizontal_scroll, Units::Percentage(self.scroll.scroll_size * 100.0));

                            self.scroll.overflow = 1.0
                                - (state.data.get_width(self.container)
                                    / state.data.get_width(entity));
                            let overflow2 = 1.0 - (1.0 / (1.0 - self.scroll.overflow));
                            // let overflow2 = 1.0
                            //     - (state.data.get_width(entity)
                            //         / state.data.get_width(self.container));

                            state
                                .style
                                .left
                                .insert(self.container, Units::Percentage(self.scroll.scroll_pos * self.scroll.overflow * 100.0));

                            state.style.left.insert(
                                self.horizontal_scroll,
                                Units::Percentage(self.scroll.scroll_pos * overflow2 * 100.0),
                            );

                            state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()).origin(entity));
                        

                            if let Some(callback) = self.on_scroll.take() {
                                (callback)(self, state, entity);

                                self.on_scroll = Some(callback);
                            }

                            // state.insert_event(
                            //     Event::new(ScrollEvent::Scroll(self.scroll.scroll_pos, self.scroll.scroll_size, overflow)).target(entity).origin(entity),
                            // );  
                        }
                    }
                }

                WindowEvent::MouseScroll(_, y) => {
                    if self.scroll_wheel {
                        self.scroll.overflow = state.data.get_width(entity)
                            - state.data.get_width(self.horizontal_scroll);

                        if self.scroll.overflow == 0.0 {
                            return;
                        }

                        // Need better names for these
                        self.scroll.overflow = 1.0
                            - (state.data.get_width(self.container) / state.data.get_width(entity));
                        let overflow2 = 1.0
                            - (state.data.get_width(entity) / state.data.get_width(self.container));

                        self.scroll.scroll_pos += (30.0 * *y) / (state.data.get_width(entity) * self.scroll.overflow);

                        if self.scroll.scroll_pos < 0.0 {
                            self.scroll.scroll_pos = 0.0;
                        }

                        if self.scroll.scroll_pos > 1.0 {
                            self.scroll.scroll_pos = 1.0;
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
                            .set_left(state, Units::Percentage(self.scroll.scroll_pos * self.scroll.overflow * 100.0));
                        self.horizontal_scroll
                            .set_left(state, Units::Percentage(self.scroll.scroll_pos * overflow2 * 100.0));


                        if let Some(callback) = self.on_scroll.take() {
                            (callback)(self, state, entity);

                            self.on_scroll = Some(callback);
                        }
                        
                        // state.insert_event(
                        //     Event::new(ScrollEvent::Scroll(self.scroll.scroll_pos, self.scroll.scroll_size, overflow))
                        //         .target(entity).origin(entity),
                        // );

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
                            self.position = self.scroll.scroll_pos;
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
                        self.scroll.overflow = state.data.get_width(entity)
                            - state.data.get_width(self.horizontal_scroll);

                        if self.scroll.overflow == 0.0 {
                            return;
                        }

                        let ratio = dist_x / self.scroll.overflow;
                        let r = self.position + ratio;

                        // let mut scrollh = state.data.get_height(entity) / state.data.get_height(self.container);
                        // if scrollh > 1.0 {
                        //     scrollh = 1.0;
                        // }

                        self.scroll.scroll_pos = r;

                        if self.scroll.scroll_pos < 0.0 {
                            self.scroll.scroll_pos = 0.0;
                        }

                        if self.scroll.scroll_pos > 1.0 {
                            self.scroll.scroll_pos = 1.0;
                        }

                        // let scroll = state
                        //     .style
                        //     .scroll
                        //     .get(self.entity)
                        //     .cloned()
                        //     .unwrap_or_default();
                        //self.vertical_scroll
                        //    .set_top(state, Units::Pixels(self.position + dist_y));

                        self.scroll.overflow = 1.0
                            - (state.data.get_width(self.container) / state.data.get_width(entity));
                        let overflow2 = 1.0
                            - (state.data.get_width(entity) / state.data.get_width(self.container));

                        self.container
                            .set_left(state, Units::Percentage(self.scroll.scroll_pos * self.scroll.overflow * 100.0));
                        self.horizontal_scroll
                            .set_left(state, Units::Percentage(self.scroll.scroll_pos * overflow2 * 100.0));


                        if let Some(callback) = self.on_scroll.take() {
                            (callback)(self, state, entity);

                            self.on_scroll = Some(callback);
                        }

                        // state.insert_event(
                        //     Event::new(ScrollEvent::Scroll(self.scroll.scroll_pos, self.scroll.scroll_size, overflow))
                        //         .target(entity).origin(entity),
                        // );

                        state.insert_event(Event::new(WindowEvent::Restyle));
                        state
                            .insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
                        state.insert_event(Event::new(WindowEvent::Redraw));
                        //println!("overflow: {}, dist: {}, ratio: {}, scrolly: {}", overflow, dist_y, r, self.scroll.scroll_pos);
                    }
                }

                _ => {}
            }
        }
    }
}

pub struct ScrollContainer {
    container: Entity,
    vertical_scroll: Entity,
    pub scroll: Scroll,

    pressedx: f32,
    pressedy: f32,
    moving: bool,
    position: f32,

    vertical_scroll_animation: Animation,
    vertical_container_animation: Animation,

    on_scroll: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
}

impl ScrollContainer {
    /// Create a new ScrollContainer widget
    pub fn new() -> Self {
        ScrollContainer {
            container: Entity::null(),
            vertical_scroll: Entity::null(),
            scroll: Scroll::default(),

            pressedx: 0.0,
            pressedy: 0.0,
            moving: false,
            position: 0.0,

            vertical_scroll_animation: Animation::default(),
            vertical_container_animation: Animation::default(),

            on_scroll: None,
        }
    }

    pub fn on_scroll<F>(mut self, callback: F) -> Self 
    where F: 'static + Fn(&mut Self, &mut State, Entity)
    {
        self.on_scroll = Some(Box::new(callback));

        self
    }
}

impl Widget for ScrollContainer {
    type Ret = Entity;
    type Data = Scroll;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_layout_type(state, LayoutType::Row)
            .set_min_height(state, Pixels(0.0));

        self.container = Element::new().build(state, entity, |builder| {
            builder
                //.set_position_type(PositionType::SelfDirected)
                .set_height(Auto)
                //.set_top(Units::Percentage(0.0))
                //.set_flex_grow(1.0)
                //.set_align_self(AlignSelf::FlexStart)
                .class("container")
        });

        state.style.clip_widget.insert(self.container, entity);

        self.vertical_scroll = Element::new().build(state, entity, |builder| {
            builder
                //.set_position_type(PositionType::SelfDirected)
                .set_min_height(Pixels(0.0))
                //.set_top(Units::Percentage(0.0))
                // .set_width(Units::Pixels(10.0))
                //.set_height(Units::Percentage(0.0))
                //.set_align_self(AlignSelf::FlexStart)
                //.set_background_color(Color::rgb(70, 200, 70))
                //.set_right(Units::Pixels(0.0))
                .class("scrollbar")

            //
        });

        entity.set_disabled(state, true);
        // self.vertical_scroll =
        //     Scrollbar::new(self.container, Direction::Vertical).build(state, entity, |builder| {
        //         builder
        //             .set_width(Units::Pixels(10.0))
        //             .set_height(Units::Percentage(1.0))
        //             .set_background_color(Color::rgb(50, 50, 100))
        //     });

        entity.set_element(state, "scroll_container");

        let vertical_scroll_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Units::Percentage(0.0)))
            .with_keyframe((1.0, Units::Percentage(20.0)));

        self.vertical_scroll_animation =
            state.style.top.insert_animation(vertical_scroll_animation);

        let vertical_container_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Units::Percentage(0.0)))
            .with_keyframe((1.0, Units::Percentage(-20.0)));

        self.vertical_container_animation = state
            .style
            .top
            .insert_animation(vertical_container_animation);

        self.container
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        self.scroll = *data;

        
        let overflow2 = 1.0
            - (state.data.get_height(entity)
                / state.data.get_height(self.container));

        self.container
            .set_top(state, Units::Percentage(self.scroll.scroll_pos * self.scroll.overflow * 100.0));
        self.vertical_scroll
            .set_top(state, Units::Percentage(self.scroll.scroll_pos * overflow2 * 100.0));
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::GeometryChanged(geometry_changed) => {
                    if event.target == self.container || event.target == entity {
                        if geometry_changed.width || geometry_changed.height {
                            self.scroll.scroll_size = state.data.get_height(entity)
                                / state.data.get_height(self.container);

                            if self.scroll.scroll_size >= 1.0 {
                                self.scroll.scroll_size = 1.0;
                                entity.set_disabled(state, true);
                            }

                            if self.scroll.scroll_size < 1.0 {
                                entity.set_disabled(state, false);
                            }

                            if !state.style.top.is_animating(self.vertical_scroll) {
                                let dist = state.data.get_posy(self.vertical_scroll)
                                    - state.data.get_posy(entity);
                                let space = state.data.get_height(entity)
                                    - (self.scroll.scroll_size * state.data.get_height(entity));
                                self.scroll.scroll_pos = dist / space;
                            }

                            if self.scroll.scroll_pos.is_nan() {
                                self.scroll.scroll_pos = 0.0;
                            }

                            if self.scroll.scroll_pos < 0.0 {
                                self.scroll.scroll_pos = 0.0;
                            }

                            if self.scroll.scroll_pos >= 1.0 {
                                self.scroll.scroll_pos = 1.0;
                            }

                            // Setting it this way avoid calling Restyle automatically
                            state
                                .style
                                .height
                                .insert(self.vertical_scroll, Units::Percentage(self.scroll.scroll_size * 100.0));

                            self.scroll.overflow = 1.0
                                - (state.data.get_height(self.container)
                                    / state.data.get_height(entity));
                            let overflow2 = 1.0
                                - (state.data.get_height(entity)
                                    / state.data.get_height(self.container));

                            state
                                .style
                                .top
                                .insert(self.container, Units::Percentage(self.scroll.scroll_pos * self.scroll.overflow * 100.0));

                            state.style.top.insert(
                                self.vertical_scroll,
                                Units::Percentage(self.scroll.scroll_pos * overflow2 * 100.0),
                            );

                            state.insert_event(
                                Event::new(WindowEvent::Relayout)
                                    .target(Entity::root())
                                    .origin(entity),
                            );

                            if let Some(callback) = self.on_scroll.take() {
                                (callback)(self, state, entity);

                                self.on_scroll = Some(callback);
                            }

                            // state.insert_event(
                            //     Event::new(ScrollEvent::Scroll(self.scroll.scroll_pos, self.scroll.scroll_size, overflow)).target(entity).origin(entity),
                            // );        
                        }
                    }
                }

                WindowEvent::MouseScroll(_, y) => {

                    self.scroll.overflow =
                        state.data.get_height(entity) - state.data.get_height(self.vertical_scroll);

                    if self.scroll.overflow == 0.0 {
                        return;
                    }

                    // Need better names for these
                    self.scroll.overflow = 1.0
                        - (state.data.get_height(self.container) / state.data.get_height(entity));
                    let overflow2 = 1.0
                        - (state.data.get_height(entity) / state.data.get_height(self.container));

                    // TODO - Need a way to configure this
                    self.scroll.scroll_pos += (30.0 * *y) / (state.data.get_height(entity) * self.scroll.overflow);

                    if self.scroll.scroll_pos < 0.0 {
                        self.scroll.scroll_pos = 0.0;
                    }

                    if self.scroll.scroll_pos > 1.0 {
                        self.scroll.scroll_pos = 1.0;
                    }

                    self.container
                        .set_top(state, Units::Percentage(self.scroll.scroll_pos * self.scroll.overflow * 100.0));
                    self.vertical_scroll
                        .set_top(state, Units::Percentage(self.scroll.scroll_pos * overflow2 * 100.0));

                    if let Some(callback) = self.on_scroll.take() {
                        (callback)(self, state, entity);

                        self.on_scroll = Some(callback);
                    }

                    // state.insert_event(
                    //     Event::new(ScrollEvent::Scroll(self.scroll.scroll_pos, self.scroll.scroll_size, overflow)).target(entity).origin(entity),
                    // );

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
                            self.position = self.scroll.scroll_pos;
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
                        self.scroll.overflow = state.data.get_height(entity)
                            - state.data.get_height(self.vertical_scroll);

                        if self.scroll.overflow == 0.0 {
                            return;
                        }

                        let ratio = dist_y / self.scroll.overflow;
                        let r = self.position + ratio;

                        // let mut scrollh = state.data.get_height(entity) / state.data.get_height(self.container);
                        // if scrollh > 1.0 {
                        //     scrollh = 1.0;
                        // }

                        self.scroll.scroll_pos = r;

                        if self.scroll.scroll_pos < 0.0 {
                            self.scroll.scroll_pos = 0.0;
                        }

                        if self.scroll.scroll_pos > 1.0 {
                            self.scroll.scroll_pos = 1.0;
                        }

                        // let scroll = state
                        //     .style
                        //     .scroll
                        //     .get(self.entity)
                        //     .cloned()
                        //     .unwrap_or_default();
                        //self.vertical_scroll
                        //    .set_top(state, Units::Pixels(self.position + dist_y));

                        self.scroll.overflow = 1.0
                            - (state.data.get_height(self.container)
                                / state.data.get_height(entity));
                        let overflow2 = 1.0
                            - (state.data.get_height(entity)
                                / state.data.get_height(self.container));

                        self.container
                            .set_top(state, Units::Percentage(self.scroll.scroll_pos * self.scroll.overflow * 100.0));
                        self.vertical_scroll
                            .set_top(state, Units::Percentage(self.scroll.scroll_pos * overflow2 * 100.0));

                        if let Some(callback) = self.on_scroll.take() {
                            (callback)(self, state, entity);

                            self.on_scroll = Some(callback);
                        }

                        // state.insert_event(
                        //     Event::new(ScrollEvent::Scroll(self.scroll.scroll_pos, self.scroll.scroll_size, overflow))
                        //         .target(entity).origin(entity),
                        // );

                        state.insert_event(Event::new(WindowEvent::Restyle));
                        state
                            .insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
                        state.insert_event(Event::new(WindowEvent::Redraw));
                        //println!("overflow: {}, dist: {}, ratio: {}, scrolly: {}", overflow, dist_y, r, self.scroll.scroll_pos);
                    }
                }

                _ => {}
            }
        }
    }
}

//
/*
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
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_layout_type(state, LayoutType::Row);

        let row = Row::new().build(state, entity, |builder| builder);

        let column = Column::new().build(state, row, |builder| builder);

        self.container = Element::new().build(state, column, |builder| {
            builder
                .set_top(Units::Percentage(0.0))
                //.set_align_self(AlignSelf::FlexStart)
                .class("container")
        });

        state.style.clip_widget.insert(self.container, entity);

        //println!("Container: {}", self.container);

        self.vertical_scroll = Element::new().build(state, row, |builder| {
            builder
                //.set_position(Position::Absolute)
                .set_top(Units::Percentage(0.0))
                .set_width(Units::Pixels(10.0))
                .set_height(Units::Percentage(100.0))
            //.set_align_self(AlignSelf::FlexStart)
            //.set_background_color(Color::rgb(70, 200, 70))
            //.set_right(Units::Pixels(0.0))
            //.class("scrollbar")

            //
        });

        self.vertical_scroll = Element::new().build(state, row, |builder| {
            builder
                //.set_position(Position::Absolute)
                .set_left(Units::Percentage(0.0))
                .set_height(Units::Pixels(10.0))
                .set_width(Units::Percentage(100.0))
            //.set_align_self(AlignSelf::FlexStart)
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

        entity.set_element(state, "scroll_containerhv");

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
                            self.vertical_scroll.set_disabled(state, false);
                        }

                        let mut scrollh =
                            state.data.get_width(entity) / state.data.get_width(self.container);

                        if scrollh >= 1.0 {
                            scrollh = 1.0;
                            self.horizontal_scroll.set_disabled(state, true);
                        }

                        if scrollh < 1.0 {
                            self.horizontal_scroll.set_disabled(state, false);
                        }

                        // BUG: fast scrolling causes smaller scroll because the animation hasn't finished when this function is called again
                        // One way to fix this might be to check whether the value is currently being animated before setting here
                        // Possibly not the best solution but it works
                        // if !state.style.top.is_animating(self.vertical_scroll) {
                        //     let dist = state.data.get_posy(self.vertical_scroll)
                        //         - state.data.get_posy(entity);
                        //     let space = state.data.get_height(entity)
                        //         - (scrollh * state.data.get_height(entity));
                        //     self.scroll.scroll_pos = dist / space;
                        // }

                        if self.scroll.scroll_pos.is_nan() {
                            self.scroll.scroll_pos = 0.0;
                        }

                        if self.scroll.scroll_pos < 0.0 {
                            self.scroll.scroll_pos = 0.0;
                        }

                        if self.scroll.scroll_pos >= 1.0 {
                            self.scroll.scroll_pos = 1.0;
                        }

                        // Setting it this way avoid calling Restyle automatically
                        state
                            .style
                            .height
                            .insert(self.vertical_scroll, Units::Percentage(scrollv * 100.0));

                        let overflow = 1.0
                            - (state.data.get_height(self.container)
                                / state.data.get_height(entity));
                        let overflow2 = 1.0
                            - (state.data.get_height(entity)
                                / state.data.get_height(self.container));

                        state
                            .style
                            .top
                            .insert(self.container, Units::Percentage(self.scroll.scroll_pos * overflow * 100.0));

                        state.style.top.insert(
                            self.vertical_scroll,
                            Units::Percentage(self.scroll.scroll_pos * overflow2 * 100.0),
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
                            .insert(self.vertical_scroll, Units::Percentage(scrollh * 100.0));

                        let overflow = 1.0
                            - (state.data.get_width(self.container) / state.data.get_width(entity));
                        let overflow2 = 1.0
                            - (state.data.get_width(entity) / state.data.get_width(self.container));

                        state
                            .style
                            .left
                            .insert(self.container, Units::Percentage(self.scrollx * overflow * 100.0));

                        state.style.top.insert(
                            self.vertical_scroll,
                            Units::Percentage(self.scroll.scroll_pos * overflow2 * 100.0),
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

                    self.scroll.scroll_pos += (40.0 * *y) / (state.data.get_height(entity) * overflow);

                    if self.scroll.scroll_pos < 0.0 {
                        self.scroll.scroll_pos = 0.0;
                    }

                    if self.scroll.scroll_pos > 1.0 {
                        self.scroll.scroll_pos = 1.0;
                    }

                    //println!("Scroll: {}", self.scroll.scroll_pos);

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
                        .set_top(state, Units::Percentage(self.scroll.scroll_pos * overflow));
                    self.vertical_scroll
                        .set_top(state, Units::Percentage(self.scroll.scroll_pos * overflow2));

                    /*
                    if let Some(animation) = state.style.top.get_animation_mut(self.vertical_scroll_animation) {
                        *animation.keyframes.first_mut().unwrap() = (0.0, current_scroll_top);
                        *animation.keyframes.last_mut().unwrap() = (1.0, Units::Percentage(self.scroll.scroll_pos * overflow2));
                    }

                    state.style.top.play_animation(self.vertical_scroll, self.vertical_scroll_animation);





                    if let Some(animation) = state.style.top.get_animation_mut(self.vertical_container_animation) {
                        *animation.keyframes.first_mut().unwrap() = (0.0, current_container_top);
                        *animation.keyframes.last_mut().unwrap() = (1.0, Units::Percentage(self.scroll.scroll_pos * overflow));
                    }

                    state.style.top.play_animation(self.container, self.vertical_container_animation);
                    */

                    //println!("A: {:?}  B: {:?}", current_container_top, self.scroll.scroll_pos * overflow);

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
                            self.position = self.scroll.scroll_pos;
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

                        self.scroll.scroll_pos = r;

                        if self.scroll.scroll_pos < 0.0 {
                            self.scroll.scroll_pos = 0.0;
                        }

                        if self.scroll.scroll_pos > 1.0 {
                            self.scroll.scroll_pos = 1.0;
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
                            .set_top(state, Units::Percentage(self.scroll.scroll_pos * overflow * 100.0));
                        self.vertical_scroll
                            .set_top(state, Units::Percentage(self.scroll.scroll_pos * overflow2 * 100.0));

                        state.insert_event(Event::new(WindowEvent::Restyle));
                        state
                            .insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
                        state.insert_event(Event::new(WindowEvent::Redraw));
                        //println!("overflow: {}, dist: {}, ratio: {}, scrolly: {}", overflow, dist_y, r, self.scroll.scroll_pos);
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
                            .set_left(state, Units::Percentage(self.scrollx * overflow * 100.0));
                        self.vertical_scroll
                            .set_left(state, Units::Percentage(self.scrollx * overflow2 * 100.0));

                        state.insert_event(Event::new(WindowEvent::Restyle));
                        state
                            .insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
                        state.insert_event(Event::new(WindowEvent::Redraw));
                        //println!("overflow: {}, dist: {}, ratio: {}, scrolly: {}", overflow, dist_y, r, self.scroll.scroll_pos);
                    }
                }

                _ => {}
            }
        }
    }
}
*/