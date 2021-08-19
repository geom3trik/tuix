
use crate::common::*;
use crate::Button;
use crate::scroll_container::Scroll;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollDirection {
    Horizontal,
    Vertical,
}

pub struct Scrollbar {
    front: Entity,
    pub scroll: Scroll,

    direction: ScrollDirection,

    position: f32,
    pub pos_ratio: f32,

    pressed_x: f32,
    pressed_y: f32,
    moving: bool,
    on_scroll: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
}

impl Scrollbar {
    pub fn new(direction: ScrollDirection) -> Self {
        Scrollbar {
            front: Entity::null(),
            // scroll_pos: 0.0,
            // scroll_size: 0.0,
            // overflow: 0.0,

            scroll: Scroll::default(),

            direction,
            position: 0.0,
            pos_ratio: 0.2,

            pressed_x: 0.0,
            pressed_y: 0.0,
            moving: false,
            on_scroll: None,
        }
    }

    pub fn on_scroll<F>(mut self, callback: F) -> Self 
    where F: 'static + Fn(&mut Self, &mut State, Entity)
    {
        self.on_scroll = Some(Box::new(callback));

        self
    }

    // pub fn set_posx(&self, state: &mut State, value: f32) {
    //     //self.back.set_left(state, value);
    //     self.front.set_left(state, Units::Pixels(value));
    // }

    // pub fn on_scroll<F>(mut self, pos: F) -> Self
    // where
    //     F: 'static + Fn(f32) -> Message,
    // {
    //     self.on_scroll = Some(Box::new(pos));
    //     self
    // }
}

impl Widget for Scrollbar {
    type Ret = Entity;
    type Data = Scroll;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.front = Element::new().build(state, entity, |builder| 
            builder
                //.set_background_color(Color::rgb(50, 50, 100))
                .class("front")
        );


        self.front
            .set_width(state, Units::Percentage(100.0))
            .set_height(state, Units::Percentage(100.0));
    


        entity.set_element(state, "scrollbar")
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        // self.scroll_pos = data.scroll_pos;
        // self.scroll_size = data.scroll_size;
        // self.overflow = data.overflow;

        self.scroll = *data;
        let overflow2 = 1.0 - (1.0 / (1.0 - self.scroll.overflow));
        if self.direction == ScrollDirection::Vertical {
            self.front.set_top(state, Percentage(self.scroll.scroll_pos * overflow2 * 100.0));
            state
                .style
                .height
                .insert(self.front, Percentage(self.scroll.scroll_size * 100.0));
        } else {
            self.front.set_left(state, Percentage(self.scroll.scroll_pos * overflow2 * 100.0));
            state
                .style
                .width
                .insert(self.front, Percentage(self.scroll.scroll_size * 100.0));
        }

    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
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
                            .set_top(state, Units::Percentage(scroll.y * (1.0 - scroll.h)));
                        self.front.set_height(state, Units::Percentage(scroll.h));

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

                WindowEvent::MouseScroll(x, y) => {

                    let overflow2 = 1.0 - (1.0 / (1.0 - self.scroll.overflow));

                    // TODO - Need a way to configure this
                    if self.direction == ScrollDirection::Vertical {
                        self.scroll.scroll_pos += (30.0 * *y) / (state.data.get_height(entity) * self.scroll.overflow);
                    } else {
                        self.scroll.scroll_pos += (30.0 * -*x) / (state.data.get_width(entity) * self.scroll.overflow);
                    }
                        

                    if self.scroll.scroll_pos < 0.0 {
                        self.scroll.scroll_pos = 0.0;
                    }

                    if self.scroll.scroll_pos > 1.0 {
                        self.scroll.scroll_pos = 1.0;
                    }

                    if self.direction == ScrollDirection::Vertical {
                        self.front.set_top(state, Units::Percentage(self.scroll.scroll_pos * overflow2 * 100.0));
                    } else {
                        self.front.set_left(state, Units::Percentage(self.scroll.scroll_pos * overflow2 * 100.0));
                    }

                    if let Some(callback) = self.on_scroll.take() {
                        (callback)(self, state, entity);

                        self.on_scroll = Some(callback);
                    }

                    // state.insert_event(
                    //     Event::new(ScrollEvent::Scroll(self.scroll_pos, self.scroll_size, self.overflow)).target(entity).origin(entity),
                    // );

                    

                    //event.consume();
                }

                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        if event.target == self.front {
                            println!("Do This");
                            self.pressed_x = state.mouse.cursorx;
                            self.pressed_y = state.mouse.cursory;
                            self.moving = true;
                            self.position = self.scroll.scroll_pos;
                            state.capture(entity);
                        }
                    }
                    _ => {}
                },

                WindowEvent::MouseUp(button) => match button {
                    MouseButton::Left => {
                        if event.target == entity {
                            self.moving = false;
                            state.release(entity);
                        }
                    }

                    _ => {}
                },

                WindowEvent::MouseMove(x, y) => {
                    if self.moving {
                        
                        let (dist, scroll_bar_overflow) = if self.direction == ScrollDirection::Vertical {
                            (*y - self.pressed_y, state.data.get_height(entity)
                            - state.data.get_height(self.front))
                        } else {
                            (*x - self.pressed_x, state.data.get_width(entity)
                            - state.data.get_width(self.front))
                        };

                        if scroll_bar_overflow == 0.0 {
                            return;
                        }


                        let ratio = dist / scroll_bar_overflow;
                        let r = self.position + ratio;

                        self.scroll.scroll_pos = r;

                        if self.scroll.scroll_pos < 0.0 {
                            self.scroll.scroll_pos = 0.0;
                        }

                        if self.scroll.scroll_pos > 1.0 {
                            self.scroll.scroll_pos = 1.0;
                        }

                        let overflow2 = 1.0 - (1.0 / (1.0 - self.scroll.overflow));

                        if self.direction == ScrollDirection::Vertical {
                            self.front.set_top(state, Units::Percentage(self.scroll.scroll_pos * overflow2 * 100.0));
                        } else {
                            self.front.set_left(state, Units::Percentage(self.scroll.scroll_pos * overflow2 * 100.0));
                        }

                        if let Some(callback) = self.on_scroll.take() {
                            (callback)(self, state, entity);

                            self.on_scroll = Some(callback);
                        }

                        state.insert_event(Event::new(WindowEvent::Restyle));
                        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
                        state.insert_event(Event::new(WindowEvent::Redraw));
                    }
                }

                _ => {}
            }
        }
        
    }
}
