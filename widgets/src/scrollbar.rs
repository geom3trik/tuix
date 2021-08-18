use crate::ScrollEvent;
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
    scroll_pos: f32,
    scroll_size: f32,
    overflow: f32,

    direction: ScrollDirection,

    pub position: f32,
    pub pos_ratio: f32,

    pressed_x: f32,
    pressed_y: f32,
    moving: bool,
    //on_scroll: Option<Box<dyn Fn(f32) -> Message>>,
}

impl Scrollbar {
    pub fn new(direction: ScrollDirection) -> Self {
        Scrollbar {
            front: Entity::null(),
            scroll_pos: 0.0,
            scroll_size: 0.0,
            overflow: 0.0,

            direction,

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
        self.front.set_left(state, Units::Pixels(value));
    }

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
        self.front = Button::new().build(state, entity, |builder| 
            builder
                .set_background_color(Color::rgb(50, 50, 100))
                .class("front")
        );


        self.front
            .set_width(state, Units::Percentage(100.0))
            .set_height(state, Units::Percentage(100.0));
    


        entity.set_element(state, "scrollbar")
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        self.scroll_pos = data.scroll_pos;
        self.scroll_size = data.scroll_size;
        self.overflow = data.overflow;
        let overflow2 = 1.0 - (1.0 / (1.0 - self.overflow));
        if self.direction == ScrollDirection::Vertical {
            self.front.set_top(state, Percentage(self.scroll_pos * overflow2 * 100.0));
            state
                .style
                .height
                .insert(self.front, Percentage(self.scroll_size * 100.0));
        } else {
            self.front.set_left(state, Percentage(self.scroll_pos * overflow2 * 100.0));
            state
                .style
                .width
                .insert(self.front, Percentage(self.scroll_size * 100.0));
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

                WindowEvent::MouseScroll(_, y) => {

                    let overflow2 = 1.0 - (1.0 / (1.0 - self.overflow));

                    // TODO - Need a way to configure this
                    self.scroll_pos += (30.0 * *y) / (state.data.get_height(entity) * self.overflow);

                    if self.scroll_pos < 0.0 {
                        self.scroll_pos = 0.0;
                    }

                    if self.scroll_pos > 1.0 {
                        self.scroll_pos = 1.0;
                    }

                    if self.direction == ScrollDirection::Vertical {
                        self.front.set_top(state, Units::Percentage(self.scroll_pos * overflow2 * 100.0));
                    } else {
                        self.front.set_left(state, Units::Percentage(self.scroll_pos * overflow2 * 100.0));
                    }

                    state.insert_event(
                        Event::new(ScrollEvent::Scroll(self.scroll_pos, self.scroll_size, self.overflow)).target(entity),
                    );

                    

                    event.consume();
                }

                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        
                    }
                    _ => {}
                },

                WindowEvent::MouseUp(button) => match button {
                   

                    _ => {}
                },

                WindowEvent::MouseMove(_, y) => {

                }

                _ => {}
            }
        }
        
    }
}
