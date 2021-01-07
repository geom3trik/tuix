#![allow(dead_code)]

use crate::entity::Entity;
use crate::events::{BuildHandler, Event, EventHandler};
use crate::{MouseButton, WindowEvent};
use crate::{PropSet, State};

use crate::state::style::*;

use crate::widgets::Button;

pub enum Direction {
    Horizontal,
    Vertical,
}

pub struct Scrollbar {
    entity: Entity,

    front: Entity,
    direction: Direction,

    pub position: f32,
    pub pos_ratio: f32,

    pressed_x: f32,
    pressed_y: f32,
    moving: bool,
    //on_scroll: Option<Box<dyn Fn(f32) -> Message>>,
}

impl Scrollbar {
    pub fn new(entity: Entity, direction: Direction) -> Self {
        Scrollbar {
            entity,
            front: Entity::null(),

            direction: direction,

            position: 0.0,
            pos_ratio: 0.2,

            pressed_x: 0.0,
            pressed_y: 0.0,
            moving: false,
            //on_scroll: None,
        }
    }

    pub fn set_posx(&self, state: &mut State, value: f32) {
        //self.back.set_left(state, value);
        self.front.set_left(state, Length::Pixels(value));
    }

    // pub fn on_scroll<F>(mut self, pos: F) -> Self
    // where
    //     F: 'static + Fn(f32) -> Message,
    // {
    //     self.on_scroll = Some(Box::new(pos));
    //     self
    // }
}

impl BuildHandler for Scrollbar {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.front = Button::new().build(state, entity, |builder| builder.class("front"));
        match self.direction {
            Direction::Horizontal => {
                // entity
                //     .set_width(state, Length::Pixels(100.0))
                //     .set_height(state, Length::Pixels(10.0));

                //self.front.set_height(state, 1.0);
                //.set_background_color(state, Color::rgb(80, 50, 50));
            }

            Direction::Vertical => {
                //entity
                //    .set_height(state, 1.0);
                //.set_flex_basis(state, 10.0)
                //.set_flex_grow(state, 0.0);
                //.set_background_color(state, Color::rgb(38, 38, 38));

                self.front
                    .set_width(state, Length::Percentage(1.0))
                    .set_height(state, Length::Percentage(1.0));
                //.set_background_color(state, Color::rgb(100, 100, 100));
                //.set_margin_left(state, 1.0)
                //.set_margin_right(state, 1.0);
            }
        }

        state.style.insert_element(entity, "scrollbar");

        entity
    }
}

impl EventHandler for Scrollbar {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        /*
        if let Some(layout_event) = event.message.downcast::<LayoutEvent>() {
            match layout_event {
                LayoutEvent::Relayout => {
                    if *id != entity {
                        println!("LAYOUT EVENT");
                        let scroll = state
                        .style
                        .scroll
                        .get(self.entity)
                        .cloned()
                        .unwrap_or_default();
                        self.front
                            .set_top(state, Length::Percentage(scroll.y * (1.0 - scroll.h)));
                        self.front.set_height(state, Length::Percentage(scroll.h));

                        if scroll.h == 1.0 {
                            state.style.enabled.set(entity, false);
                        } else {
                            state.style.enabled.set(entity, true);
                        }
                        state.insert_event(
                            Event::new(StyleEvent::Restyle).target(state.root),
                        );
                        state.insert_event(
                            Event::new(LayoutEvent::Relayout).target(entity),
                        );

                    }
                }
            }
        }
        */

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                // When a relayout occurs, determine the new height of the scroll bar
                WindowEvent::Relayout => {}

                WindowEvent::WindowResize(_, _) => {
                    let scroll = state
                        .style
                        .scroll
                        .get(self.entity)
                        .cloned()
                        .unwrap_or_default();
                    self.front
                        .set_top(state, Length::Percentage(scroll.y * (1.0 - scroll.h)));
                    self.front.set_height(state, Length::Percentage(scroll.h));

                    if scroll.h == 1.0 {
                        //state.style.enabled.set(entity, false);
                        entity.set_disabled(state, true);
                    } else {
                        //state.style.enabled.set(entity, true);
                        entity.set_enabled(state, true);
                    }
                    state.insert_event(Event::new(WindowEvent::Restyle).target(state.root));
                }

                WindowEvent::MouseScroll(_, y) => {
                    //scroll.y += (10.0 * y);
                    if event.target == entity || event.target == self.front {
                        if let Some(scroll) = state.style.scroll.get_mut(self.entity) {
                            scroll.y -= 0.1 * *y;

                            if scroll.y < 0.0 {
                                scroll.y = 0.0;
                            }

                            if scroll.y > 1.0 {
                                scroll.y = 1.0;
                            }
                        }

                        let scroll = state
                            .style
                            .scroll
                            .get(self.entity)
                            .cloned()
                            .unwrap_or_default();
                        self.front
                            .set_top(state, Length::Percentage(scroll.y * (1.0 - scroll.h)));
                        self.front.set_height(state, Length::Percentage(scroll.h));

                        if scroll.h == 1.0 {
                            //state.style.enabled.set(entity, false);
                            entity.set_disabled(state, true);
                        } else {
                            //state.style.enabled.set(entity, true);
                            entity.set_enabled(state, true);
                        }

                        state.insert_event(Event::new(WindowEvent::Restyle).target(state.root));
                        state.insert_event(Event::new(WindowEvent::Relayout));
                        println!(
                            "Scroll: {}",
                            state
                                .style
                                .scroll
                                .get(self.entity)
                                .cloned()
                                .unwrap_or_default()
                                .y
                        );
                    }
                    //println!("y: {}", y);

                    //println!("Size: {}", state.transform.get_height(self.front));
                }

                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        self.pressed_x = state.mouse.cursorx;
                        self.pressed_y = state.mouse.cursory;
                        self.moving = true;
                        let scroll = state
                            .style
                            .scroll
                            .get(self.entity)
                            .cloned()
                            .unwrap_or_default();
                        self.position = scroll.y;
                        state.capture(entity);
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
                        let dist_y = *y - self.pressed_y;
                        let overflow = state.transform.get_height(entity)
                            - state.transform.get_height(self.front);
                        let ratio = dist_y / overflow;
                        let r = self.position + ratio;
                        if let Some(scroll) = state.style.scroll.get_mut(self.entity) {
                            scroll.y = r;

                            if scroll.y < 0.0 {
                                scroll.y = 0.0;
                            }

                            if scroll.y > 1.0 {
                                scroll.y = 1.0;
                            }
                        }

                        let scroll = state
                            .style
                            .scroll
                            .get(self.entity)
                            .cloned()
                            .unwrap_or_default();
                        self.front
                            .set_top(state, Length::Percentage(scroll.y * (1.0 - scroll.h)));

                        state.insert_event(Event::new(WindowEvent::Restyle).target(state.root));
                        state.insert_event(Event::new(WindowEvent::Relayout).target(state.root));
                        //println!("overflow: {}, dist: {}, ratio: {}", overflow, dist_y, r);
                    }
                }

                _ => {}
            }
        }

        false
    }
    // fn handle_event(
    //     &mut self,
    //     state: &mut State,
    //     event: &WentitygetEvent,
    //     event_handlers: &mut Vec<Box<EventHandler>>,
    //     message_queue: &mut EventQueue<Message>,
    // ) {
    //     // let parent_wentityth = state.transform.get_global_wentityth(self.back);
    //     // state
    //     //     .transform
    //     //     .set_local_x(self.front, self.pos_ratio * parent_wentityth);
    //     // state
    //     //     .transform
    //     //     .set_local_wentityth(self.front, self.dim_ratio * parent_wentityth);

    //     match event {
    //         WentitygetEvent::MouseButton(button, action, mods) => match button {
    //             MouseButton::Left => match action {
    //                 MouseButtonState::Pressed => {
    //                     if state.hovered == self.front || state.hovered == self.back {
    //                         self.pressed_x =
    //                             state.mouse.cursorx - state.transform.get_posx(self.front);
    //                         self.pressed_y =
    //                             state.mouse.cursory - state.transform.get_posy(self.front);
    //                         self.moving = true;
    //                     }
    //                 }

    //                 MouseButtonState::Released => {
    //                     self.moving = false;
    //                 }
    //             },

    //             _ => {}
    //         },

    //         WentitygetEvent::MouseMotion(x, y) => {
    //             match self.direction {
    //                 Direction::Vertical => {
    //                     self.front.set_height(
    //                         state,
    //                         self.dim_ratio * state.transform.get_height(self.back),
    //                     );
    //                 }

    //                 Direction::Horizontal => {
    //                     self.front.set_wentityth(
    //                         state,
    //                         self.dim_ratio * state.transform.get_height(self.back),
    //                     );
    //                 }
    //             }

    //             if self.moving {
    //                 let dx = x - self.pressed_x;
    //                 let dy = y - self.pressed_y;

    //                 match self.direction {
    //                     Direction::Horizontal => {
    //                         self.front.set_left(state, dx);
    //                         self.pos_ratio = dx / state.transform.get_wentityth(self.back);

    //                         let positioning = state
    //                             .style
    //                             .positioning
    //                             .get(self.front)
    //                             .cloned()
    //                             .unwrap_or_default();
    //                         let size = state
    //                             .style
    //                             .size
    //                             .get(self.front)
    //                             .cloned()
    //                             .unwrap_or_default();

    //                         if positioning.left <= 0.0 {
    //                             self.front.set_left(state, 0.0);
    //                         }

    //                         if positioning.left + size.wentityth >= state.transform.get_wentityth(self.back)
    //                         {
    //                             self.front.set_left(
    //                                 state,
    //                                 state.transform.get_wentityth(self.back)
    //                                     - state.transform.get_wentityth(self.front),
    //                             );
    //                         }

    //                         self.pos_ratio = state.transform.get_posx(self.front)
    //                             / state.transform.get_wentityth(self.back);

    //                         // event_queue.push(self.signal_moved.clone(), WentitygetEvent::WentitygetValueChanged(
    //                         //     self.back,
    //                         //     "pos".to_string(),
    //                         //     self.pos_ratio,
    //                         // ))

    //                         // let event = WentitygetEvent::WentitygetValueChanged(
    //                         //     self.back,
    //                         //     "pos".to_string(),
    //                         //     self.pos_ratio,
    //                         // );
    //                         // event_queue.push(event);
    //                     }

    //                     Direction::Vertical => {
    //                         self.front.set_top(state, dy);

    //                         let space = state.transform.get_height(self.back)
    //                             - state.transform.get_height(self.front);
    //                         //self.pos_ratio = dy / space;

    //                         let positioning = state
    //                             .style
    //                             .positioning
    //                             .get(self.front)
    //                             .cloned()
    //                             .unwrap_or_default();
    //                         let size = state
    //                             .style
    //                             .size
    //                             .get(self.front)
    //                             .cloned()
    //                             .unwrap_or_default();

    //                         if positioning.top <= 0.0 {
    //                             self.front.set_top(state, 0.0);
    //                         }

    //                         if positioning.top + size.height
    //                             >= state.transform.get_height(self.back)
    //                         {
    //                             self.front.set_top(
    //                                 state,
    //                                 state.transform.get_height(self.back)
    //                                     - state.transform.get_height(self.front),
    //                             );
    //                         }

    //                         //if space == 0.0 {
    //                         //    self.pos_ratio = 0.0;
    //                         //} else {
    //                         self.pos_ratio = state.transform.get_posy(self.front)
    //                             / state.transform.get_height(self.back);
    //                         if let Some(on_scroll) = &self.on_scroll {
    //                             message_queue.push((on_scroll)(self.pos_ratio));
    //                         }

    //                         //}

    //                         // println!(
    //                         //     "y: {}, h: {}, ratio: {}",
    //                         //     state.transform.get_local_y(self.front),
    //                         //     state.transform.get_global_height(self.back),
    //                         //     self.pos_ratio
    //                         // );

    //                         // let event = WentitygetEvent::WentitygetValueChanged(
    //                         //     self.back,
    //                         //     "pos".to_string(),
    //                         //     self.pos_ratio,
    //                         // );
    //                         //event_queue.push(event);
    //                     }
    //                 }
    //             }
    //         }

    //         WentitygetEvent::MouseScroll(x, y) => {}

    //         WentitygetEvent::WentitygetValueChanged(entity, name, value) => {
    //             if *entity != self.back {
    //                 if *name == "pos".to_string() {
    //                     self.pos_ratio = *value;
    //                 }
    //                 if *name == "wentityth".to_string() {
    //                     self.dim_ratio = *value;

    //                     println!("Scrollbar Value Changed: {}", value);

    //                     match self.direction {
    //                         Direction::Horizontal => {
    //                             let parent_wentityth = state.transform.get_wentityth(self.back);
    //                             // state
    //                             //     .transform
    //                             //     .set_local_x(self.front, self.pos_ratio * parent_wentityth);
    //                             state
    //                                 .transform
    //                                 .set_wentityth(self.front, self.dim_ratio * parent_wentityth);
    //                         }

    //                         Direction::Vertical => {
    //                             let parent_height = state.transform.get_height(self.back);
    //                             state
    //                                 .transform
    //                                 .set_posy(self.front, self.pos_ratio * parent_height);
    //                             state
    //                                 .transform
    //                                 .set_height(self.front, self.dim_ratio * parent_height);

    //                             if state.transform.get_posy(self.front) <= 0.0 {
    //                                 state.transform.set_posy(self.front, 0.0);
    //                             }

    //                             if state.transform.get_posy(self.front)
    //                                 + state.transform.get_height(self.front)
    //                                 >= state.transform.get_height(self.back)
    //                             {
    //                                 state.transform.set_posy(
    //                                     self.front,
    //                                     state.transform.get_height(self.back)
    //                                         - state.transform.get_height(self.front),
    //                                 );
    //                             }

    //                             self.pos_ratio = state.transform.get_posy(self.front)
    //                                 / state.transform.get_height(self.back);
    //                         }
    //                     }
    //                 }
    //             }
    //         }

    //         WentitygetEvent::WentitygetSizeChanged(entity, wentityth, height) => {
    //             if *entity == self.back {
    //                 //let front_height =
    //                 //    self.dim_ratio * state.transform.get_global_height(self.back);

    //                 //let space = state.transform.get_global_height(self.back) - front_height;

    //                 // if state.transform.get_local_y(self.front)
    //                 //     + state.transform.get_local_height(self.front)
    //                 //     >= state.transform.get_global_height(self.back)
    //                 // {
    //                 //     state.transform.set_local_y(
    //                 //         self.front,
    //                 //         state.transform.get_global_height(self.back)
    //                 //             - state.transform.get_local_height(self.front),
    //                 //     );

    //                 //     self.pos_ratio = state.transform.get_local_y(self.front)
    //                 //         / state.transform.get_global_height(self.back);

    //                 // }

    //                 // state.transform.set_local_y(
    //                 //     self.front,
    //                 //     self.pos_ratio * state.transform.get_global_height(self.back),
    //                 // );

    //                 //println!("new_pos: {}", front_height);

    //                 // println!(
    //                 //     "pos: {}, size: {}",
    //                 //     state.transform.get_global_y(self.front),
    //                 //     state.transform.get_global_height(self.front)
    //                 // );

    //                 // if state.transform.get_local_y(self.front) <= 0.0 {
    //                 //     state.transform.set_local_y(self.front, 0.0);
    //                 // }

    //                 // if state.transform.get_local_y(self.front)
    //                 //     + state.transform.get_local_height(self.front)
    //                 //     >= state.transform.get_global_height(self.back)
    //                 // {
    //                 //     state.transform.set_local_y(
    //                 //         self.front,
    //                 //         state.transform.get_global_height(self.back)
    //                 //             - state.transform.get_local_height(self.front),
    //                 //     );
    //                 // }

    //                 // self.pos_ratio = state.transform.get_local_y(self.front)
    //                 //     / state.transform.get_global_height(self.back);

    //                 // if self.pos_ratio < 0.005 {
    //                 //     self.pos_ratio = 0.0;
    //                 // }

    //                 //println!("pos_rat: {}", state.transform.get_local_y(self.front));

    //                 // event_queue.push(
    //                 //     self.signal_moved.clone(),
    //                 //     WentitygetEvent::WentitygetValueChanged(
    //                 //         self.back,
    //                 //         "pos".to_string(),
    //                 //         self.pos_ratio,
    //                 //     ),
    //                 // )
    //             }
    //         }

    //         _ => {}
    //     }
    // }
}
