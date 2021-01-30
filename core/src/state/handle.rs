use crate::{Entity, Handlers, DrawHandler, Event, State, EventData, Message};
use cssparser::stylesheet_encoding;

use crate::style::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::any::{Any, TypeId};

#[derive(Clone)]
pub struct Handle {
    pub entity: Entity,
    pub(crate) style_data: Rc<RefCell<Style>>,
    handlers: Rc<RefCell<Handlers>>,
}

impl Handle {
    pub fn new(entity: Entity, style_data: Rc<RefCell<Style>>, handlers:Rc<RefCell<Handlers>>) -> Self {
        Self {
            entity,
            style_data,
            handlers,
        }
    }
}

impl Handle {
    pub fn entity(self) -> Entity {
        self.entity
    }

    pub fn add_component<C: 'static + Any>(self, component: C) -> Handle {
        self.handlers.borrow_mut().components.insert(
            (self.entity, component.type_id()),
            Rc::new(RefCell::new(component)),
        );

        self
    }

    pub fn add_draw_hander<D: DrawHandler + 'static>(self, draw_handler: D) -> Self {
        self.handlers.borrow_mut().draw_handlers.insert(self.entity, Box::new(draw_handler));

        self
    }

    pub fn add_event_handler<C, E, F>(self, mut handler: F) -> Self
    where
        C: std::any::Any + 'static,
        E: Message,
        F: FnMut(&mut C, &mut State, &Handle, &EventData, &mut E) -> bool + 'static,
    {
        let rc_component: Rc<RefCell<C>> = self
            .handlers
            .borrow_mut()
            .components
            .get(&(self.entity, TypeId::of::<C>()))
            .and_then(|rc| Rc::downcast(rc.clone()).ok())
            .unwrap(); // !!!
        self.add_erased_handler(
            Box::new(move |state, handle, event| {
                if let Some(e) = event.message.downcast::<E>() {
                    (handler)(
                        &mut *rc_component.borrow_mut(),
                        state,
                        handle,
                        &EventData {
                            target: event.target,
                        },
                        e,
                    )
                } else {
                    false
                }
            }),
        )
    }

    pub fn add_event_handler2<E, F>(self, mut handler: F) -> Self
    where
        E: Message,
        F: FnMut(&mut State, &Handle, &EventData, &mut E) -> bool + 'static,
    {
        self.add_erased_handler(
            Box::new(move |state, handle, event| {
                if let Some(e) = event.message.downcast::<E>() {
                    (handler)(
                        state,
                        handle,
                        &EventData {
                            target: event.target,
                        },
                        e,
                    )
                } else {
                    false
                }
            }),
        )
    }

    fn add_erased_handler(
        self,
        handler: Box<dyn FnMut(&mut State, &Handle, &mut Event) -> bool + 'static>,
    ) -> Self {

        {
            let mut test = &mut self.handlers.borrow_mut().event_handlers;

            if let Some(h) = test.get_mut(&self.entity) {
                h.push(handler);
            }  else {
                let mut handlers = Vec::new();
                handlers.push(handler);
                test.insert(self.entity, handlers);
            }            
        }

    
        
        self
    }

    // CSS Selector Properties
    pub fn set_element(self, value: &str) -> Handle {
        self.style_data
            .borrow_mut()
            .insert_element(self.entity, value);
        self
    }

    pub fn add_class(self, value: &str) -> Handle {
        self.style_data
            .borrow_mut()
            .insert_class(self.entity, value);

        self
    }

    pub fn set_id(&self, value: &str) -> &Handle {
        self.style_data.borrow_mut().insert_id(self.entity, value);
        self
    }

    // CSS Pseudoclasses

    pub fn set_enabled(&self, value: bool) -> &Handle {
        if let Some(pseudo_classes) = self
            .style_data
            .borrow_mut()
            .pseudo_classes
            .get_mut(self.entity)
        {
            pseudo_classes.set_enabled(value);
        }

        self
    }

    pub fn set_disabled(&self, value: bool) -> &Handle {
        if let Some(pseudo_classes) = self
            .style_data
            .borrow_mut()
            .pseudo_classes
            .get_mut(self.entity)
        {
            pseudo_classes.set_disabled(value);
        }

        self
    }

    pub fn set_checked(&self, value: bool) -> &Handle {
        if let Some(pseudo_classes) = self
            .style_data
            .borrow_mut()
            .pseudo_classes
            .get_mut(self.entity)
        {
            pseudo_classes.set_checked(value);
        }

        self
    }

    pub fn set_over(&self, value: bool) -> &Handle {
        if let Some(pseudo_classes) = self
            .style_data
            .borrow_mut()
            .pseudo_classes
            .get_mut(self.entity)
        {
            pseudo_classes.set_over(value);
        }

        self
    }

    pub fn set_active(&self, value: bool) -> &Handle {
        if let Some(pseudo_classes) = self
            .style_data
            .borrow_mut()
            .pseudo_classes
            .get_mut(self.entity)
        {
            pseudo_classes.set_active(value);
        }

        self
    }

    pub fn set_hover(&self, value: bool) -> &Handle {
        if let Some(pseudo_classes) = self
            .style_data
            .borrow_mut()
            .pseudo_classes
            .get_mut(self.entity)
        {
            pseudo_classes.set_hover(value);
        }

        self
    }

    pub fn set_focus(&self, value: bool) -> &Handle {
        if let Some(pseudo_classes) = self
            .style_data
            .borrow_mut()
            .pseudo_classes
            .get_mut(self.entity)
        {
            pseudo_classes.set_focus(value);
        }

        self
    }

    // Display, Visibility and Opacity
    pub fn set_display(&self, value: Display) -> &Handle {
        self.style_data
            .borrow_mut()
            .display
            .insert(self.entity, value);

        self
    }

    pub fn set_visibility(&self, value: Visibility) -> &Handle {
        self.style_data
            .borrow_mut()
            .visibility
            .insert(self.entity, value);

        self
    }

    pub fn set_opacity(&self, value: f32) -> &Handle {
        self.style_data
            .borrow_mut()
            .opacity
            .insert(self.entity, Opacity(value));

        self
    }

    // Transform

    // TODO

    // Position

    pub fn set_left(&self, value: Length) -> &Handle {
        self.style_data.borrow_mut().left.insert(self.entity, value);

        self
    }

    pub fn set_right(&self, value: Length) -> &Handle {
        self.style_data
            .borrow_mut()
            .right
            .insert(self.entity, value);

        self
    }

    pub fn set_top(&self, value: Length) -> &Handle {
        self.style_data.borrow_mut().top.insert(self.entity, value);

        self
    }

    pub fn set_bottom(&self, value: Length) -> &Handle {
        self.style_data
            .borrow_mut()
            .bottom
            .insert(self.entity, value);

        self
    }

    // Size

    pub fn set_width(self, value: Length) -> Handle {
        println!("Entity: {}", self.entity);
        self.style_data
            .borrow_mut()
            .width
            .insert(self.entity, value);

        self
    }

    pub fn set_height(self, value: Length) -> Handle {
        self.style_data
            .borrow_mut()
            .height
            .insert(self.entity, value);

        self
    }

    // Size Constraints

    pub fn set_max_width(&self, value: Length) -> &Handle {
        self.style_data
            .borrow_mut()
            .max_width
            .insert(self.entity, value);

        self
    }
    pub fn set_min_width(&self, value: Length) -> &Handle {
        self.style_data
            .borrow_mut()
            .min_width
            .insert(self.entity, value);

        self
    }

    pub fn set_max_height(&self, value: Length) -> &Handle {
        self.style_data
            .borrow_mut()
            .max_height
            .insert(self.entity, value);

        self
    }

    pub fn set_min_height(&self, value: Length) -> &Handle {
        self.style_data
            .borrow_mut()
            .min_height
            .insert(self.entity, value);

        self
    }

    // Background
    pub fn set_background_color(&self, value: Color) -> &Handle {
        self.style_data
            .borrow_mut()
            .background_color
            .insert(self.entity, value);

        self
    }

    //

    // Text

    pub fn set_text(self, value: &str) -> Handle {
        if let Some(text) = self.style_data.borrow_mut().text.get_mut(self.entity) {
            text.text = value.to_string();
        }

        self.style_data.borrow_mut().text.insert(
            self.entity,
            Text {
                text: value.to_string(),
                ..Default::default()
            },
        );

        self
    }
}
