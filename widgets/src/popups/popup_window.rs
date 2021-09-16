use tuix_core::{Position, TreeExt};

use crate::{Button, Label, PopupEvent, common::*};




pub struct PopupWindow {
    title: String,
    moving: bool,
    down_x: f32,
    down_y: f32,

    header: Entity,
}

impl PopupWindow {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_owned(),
            moving: false,
            down_x: 0.0,
            down_y: 0.0,

            header: Entity::null(),
        }
    }
}

impl Widget for PopupWindow {
    /// Returns an id to the conatiner and to the header as a tuple
    type Ret = (Entity, Entity);
    type Data = ();
    
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.header = Element::new().build(state, entity, |builder|
            builder
                .set_layout_type(LayoutType::Row)
                .set_height(Pixels(30.0))
                .class("header")
        );

        let label = Label::new(&self.title).build(state, self.header, |builder |
            builder
                .set_child_space(Stretch(1.0))
                .set_hoverable(false)
                .class("label")
        );

        Button::with_label("X")
        .on_release(|data, state, button|{
            button.emit(state, PopupEvent::Close);
        })
        .build(state, self.header, |builder|
            builder
                .set_width(Pixels(30.0))
        );

        let container = Element::new().build(state, entity, |builder|
            builder
                //.set_background_color(Color::red())
                .class("container")
        );

        entity.set_element(state, "window").set_position_type(state, PositionType::SelfDirected);


        (container, self.header)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(popup_event) = event.message.downcast() {

            match popup_event {

                PopupEvent::Open => {
                    entity.set_display(state, Display::Flex);
                    event.consume();
                }

                PopupEvent::Close => {
                    entity.set_display(state, Display::None);
                    event.consume();
                }

                _=> {}
            }
        }

        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if event.target == self.header {
                        self.moving = true;
                        state.capture(entity);
                        self.down_x = state.data.get_posx(entity) - state.mouse.cursorx;
                        self.down_y = state.data.get_posy(entity) - state.mouse.cursory;
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    if event.target == entity {
                        self.moving = false;
                        state.release(entity);
                    }
                }

                WindowEvent::MouseMove(x, y) => {
                    if self.moving {

                        let parent = entity.parent(&state.tree).unwrap();

                        let parent_posx = state.data.get_posx(parent);
                        let parent_posy = state.data.get_posy(parent);

                        let dx = *x - parent_posx + self.down_x;
                        let dy = *y - parent_posy + self.down_y;
                        
                        entity.set_left(state, Units::Pixels(dx));
                        entity.set_top(state, Units::Pixels(dy));
                    }
                }

                _=> {}
            }
        }
    }
}