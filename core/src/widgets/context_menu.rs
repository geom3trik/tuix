use crate::{mouse::MouseButton, HierarchyTree, Visibility};

use crate::style::*;
use crate::{widgets::*, Length};

// Wrap a widget in a context menu to add a right-click menu to a widget
pub struct ContextMenu {
    context_menu: Entity,
}

impl ContextMenu {
    pub fn new() -> Self {
        Self {
            context_menu: Entity::default(),
        }
    }
}

impl BuildHandler for ContextMenu {
    type Ret = (Entity, Entity);
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.context_menu = Element::new().build(state, entity, |builder| {
            builder
                .set_background_color(Color::red())
                .set_visibility(Visibility::Invisible)
        });
        (entity, self.context_menu)
    }
}

impl EventHandler for ContextMenu {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Right {
                        let px = state.mouse.right.pos_down.0
                            - state
                                .data
                                .get_posx(entity.parent(&state.hierarchy).unwrap());
                        let py = state.mouse.right.pos_down.1
                            - state
                                .data
                                .get_posy(entity.parent(&state.hierarchy).unwrap());
                        self.context_menu
                            .set_left(state, Length::Pixels(px))
                            .set_top(state, Length::Pixels(py))
                            .set_visibility(state, Visibility::Visible);
                    }
                }

                _ => {}
            }
        }
    }
}
