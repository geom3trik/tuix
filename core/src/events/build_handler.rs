use crate::{Entity, EventHandler, EventManager, Selector, State};

use crate::{Align, Display, FlexDirection, Hierarchy, Justify};

use crate::state::style::flexbox::{AlignContent, AlignItems, AlignSelf};

use crate::style::*;

pub trait BuildHandler: EventHandler {
    type Ret;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret;

    fn build<F>(mut self, state: &mut State, parent: Entity, mut builder: F) -> Self::Ret
    where
        F: FnMut(Builder) -> Builder,
        Self: std::marker::Sized + 'static,
    {
        let id = state.add(parent);

        let entity = self.on_build(state, id);

        builder(Builder::new(state, id)).build(self);

        entity
    }
}

pub struct Builder<'a> {
    pub entity: Entity,
    pub state: &'a mut State,
}

impl<'a> Builder<'a> {
    pub fn new(state: &'a mut State, entity: Entity) -> Self {
        Builder { entity, state }
    }

    pub fn build<T>(mut self, event_handler: T) -> Entity
    where
        T: EventHandler + 'static,
    {
        self.state
            .event_handlers
            .insert(self.entity, Box::new(event_handler));

        self.entity
    }

    pub fn state(&mut self) -> &mut State {
        self.state
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn class(mut self, class: &str) -> Self {
        self.state.style.insert_class(self.entity, class);

        self
    }

    pub fn element(mut self, element: &str) -> Self {
        self.state.style.insert_element(self.entity, element);

        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.state.style.insert_id(self.entity, id);

        self
    }

    pub fn set_hoverability(mut self, val: bool) -> Self {
        self.state.transform.set_hoverability(self.entity, val);

        self
    }

    pub fn set_opacity(mut self, val: f32) -> Self {
        self.state.style.opacity.insert(self.entity, Opacity(val));

        self
    }

    pub fn set_checked(mut self, val: bool) -> Self {
        if let Some(pseudo_classes) = self.state.style.pseudo_classes.get_mut(self.entity) {
            pseudo_classes.set_checked(val);
        }

        self
    }

    pub fn set_z_order(mut self, val: i32) -> Self {
        self.state.style.z_order.insert(self.entity, val);

        self
    }

    pub fn set_clip_widget(mut self, val: Entity) -> Self {
        self.state.style.clip_widget.insert(self.entity, val);

        self
    }

    pub fn set_text(mut self, val: &str) -> Self {
        if let Some(text) = self.state.style.text.get_mut(self.entity) {
            text.text = val.to_string();
        } else {
            self.state.style.text.insert(
                self.entity,
                Text {
                    text: val.to_string(),
                    ..Default::default()
                },
            );
        }

        self
    }

    // Display

    pub fn set_display(mut self, val: Display) -> Self {
        self.state.style.display.insert(self.entity, val);

        self
    }

    pub fn set_visibility(mut self, val: Visibility) -> Self {
        self.state.style.visibility.insert(self.entity, val);

        self
    }

    pub fn set_overflow(mut self, val: Overflow) -> Self {
        self.state.style.overflow.insert(self.entity, val);

        self
    }

    // Background
    pub fn set_background_color(mut self, val: Color) -> Self {
        self.state.style.background_color.insert(self.entity, val);

        self
    }

    // Positioning

    pub fn set_position(mut self, val: Position) -> Self {
        self.state.style.position.insert(self.entity, val);

        self
    }

    pub fn set_left(mut self, val: Length) -> Self {
        self.state.style.left.insert(self.entity, val);

        self
    }

    pub fn set_right(mut self, val: Length) -> Self {
        self.state.style.right.insert(self.entity, val);

        self
    }

    pub fn set_top(mut self, val: Length) -> Self {
        self.state.style.top.insert(self.entity, val);
        self
    }

    pub fn set_bottom(mut self, val: Length) -> Self {
        self.state.style.bottom.insert(self.entity, val);
        self
    }

    // Alignment and Justification

    // pub fn set_justification(mut self, val: Justification) -> Self {
    //     self.state.style.justification.set(self.entity, val);
    //     self
    // }

    // pub fn set_alignment(mut self, val: Alignment) -> Self {
    //     self.state.style.alignment.set(self.entity, val);
    //     self
    // }

    // Size

    pub fn set_width(mut self, val: Length) -> Self {
        self.state.style.width.insert(self.entity, val);

        self
    }

    pub fn set_height(mut self, val: Length) -> Self {
        self.state.style.height.insert(self.entity, val);

        self
    }

    // Size Constraints

    pub fn set_min_width(mut self, val: Length) -> Self {
        self.state.style.min_width.insert(self.entity, val);

        self
    }

    pub fn set_max_width(mut self, val: Length) -> Self {
        self.state.style.max_width.insert(self.entity, val);

        self
    }

    pub fn set_min_height(mut self, val: Length) -> Self {
        self.state.style.min_height.insert(self.entity, val);

        self
    }

    pub fn set_max_height(mut self, val: Length) -> Self {
        self.state.style.max_height.insert(self.entity, val);

        self
    }

    // Margins

    pub fn set_margin(mut self, val: Length) -> Self {
        self.state.style.margin_left.insert(self.entity, val);
        self.state.style.margin_right.insert(self.entity, val);
        self.state.style.margin_top.insert(self.entity, val);
        self.state.style.margin_bottom.insert(self.entity, val);

        self
    }

    pub fn set_margin_left(mut self, val: Length) -> Self {
        self.state.style.margin_left.insert(self.entity, val);

        self
    }

    pub fn set_margin_right(mut self, val: Length) -> Self {
        self.state.style.margin_right.insert(self.entity, val);

        self
    }

    pub fn set_margin_top(mut self, val: Length) -> Self {
        self.state.style.margin_top.insert(self.entity, val);

        self
    }

    pub fn set_margin_bottom(mut self, val: Length) -> Self {
        self.state.style.margin_bottom.insert(self.entity, val);

        self
    }

    // Padding

    pub fn set_padding(mut self, val: Length) -> Self {
        self.state.style.padding_left.insert(self.entity, val);
        self.state.style.padding_right.insert(self.entity, val);
        self.state.style.padding_top.insert(self.entity, val);
        self.state.style.padding_bottom.insert(self.entity, val);

        self
    }

    pub fn set_padding_left(mut self, val: Length) -> Self {
        self.state.style.padding_left.insert(self.entity, val);

        self
    }

    pub fn set_padding_right(mut self, val: Length) -> Self {
        self.state.style.padding_right.insert(self.entity, val);

        self
    }

    pub fn set_padding_top(mut self, val: Length) -> Self {
        self.state.style.padding_top.insert(self.entity, val);

        self
    }

    pub fn set_padding_bottom(mut self, val: Length) -> Self {
        self.state.style.padding_bottom.insert(self.entity, val);

        self
    }

    // Flex Item

    pub fn set_flex_grow(mut self, val: f32) -> Self {
        self.state.style.flex_grow.insert(self.entity, val);

        self
    }

    pub fn set_flex_shrink(mut self, val: f32) -> Self {
        self.state.style.flex_shrink.insert(self.entity, val);

        self
    }

    pub fn set_flex_basis(mut self, val: f32) -> Self {
        self.state.style.flex_basis.insert(self.entity, val);

        self
    }

    pub fn set_align_self(mut self, val: AlignSelf) -> Self {
        self.state.style.align_self.insert(self.entity, val);

        self
    }

    // Flex Container

    pub fn set_flex_direction(mut self, val: FlexDirection) -> Self {
        self.state.style.flex_direction.insert(self.entity, val);

        self
    }

    pub fn set_justify_content(mut self, val: JustifyContent) -> Self {
        self.state.style.justify_content.insert(self.entity, val);

        self
    }

    pub fn set_align_content(mut self, val: AlignContent) -> Self {
        self.state.style.align_content.insert(self.entity, val);

        self
    }

    pub fn set_align_items(mut self, val: AlignItems) -> Self {
        self.state.style.align_items.insert(self.entity, val);

        self
    }

    // Border

    pub fn set_border_color(mut self, val: Color) -> Self {
        self.state.style.border_color.insert(self.entity, val);

        self
    }

    pub fn set_border_width(mut self, val: Length) -> Self {
        self.state.style.border_width.insert(self.entity, val);

        self
    }

    pub fn set_border_radius(mut self, val: Length) -> Self {
        self.state.style.border_radius_top_left.insert(self.entity, val);
        self.state.style.border_radius_top_right.insert(self.entity, val);
        self.state.style.border_radius_bottom_left.insert(self.entity, val);
        self.state.style.border_radius_bottom_right.insert(self.entity, val);

        self
    }

    pub fn set_font(mut self, value: String) -> Self {
        if let Some(data) = self.state.style.text.get_mut(self.entity) {
            data.font = value;
        } else {
            self.state.style.text.insert(
                self.entity,
                Text {
                    font: value,
                    ..Default::default()
                },
            );
        }

        self
    }

    // Text Alignment
    pub fn set_text_justify(mut self, value: Justify) -> Self {
        self.state.style.text_justify.insert(self.entity, value);

        self
    }

    pub fn set_text_align(mut self, value: Align) -> Self {
        self.state.style.text_align.insert(self.entity, value);

        self
    }

    pub fn set_next_focus(mut self, val: Entity) -> Self {
        if let Some(entity) = self.state.style.focus_order.get_mut(self.entity) {
            entity.next = val;
        } else {
            self.state.style.focus_order.insert(
                self.entity,
                FocusOrder {
                    next: val,
                    ..Default::default()
                },
            );
        }

        self
    }

    pub fn set_prev_focus(mut self, val: Entity) -> Self {
        if let Some(entity) = self.state.style.focus_order.get_mut(self.entity) {
            entity.prev = val;
        } else {
            self.state.style.focus_order.insert(
                self.entity,
                FocusOrder {
                    prev: val,
                    ..Default::default()
                },
            );
        }

        self
    }

    pub fn set_focus(mut self, next: Entity, prev: Entity) -> Self {
        if let Some(entity) = self.state.style.focus_order.get_mut(self.entity) {
            entity.next = next;
            entity.prev = prev;
        } else {
            self.state
                .style
                .focus_order
                .insert(self.entity, FocusOrder { next, prev });
        }

        self
    }

    pub fn set_rotate(mut self, rotate: f32) -> Self {
        self.state.style.rotate.insert(self.entity, rotate);

        self
    }

    pub fn set_scaley(mut self, scaley: f32) -> Self {
        self.state.style.scaley.insert(self.entity, Scale::new(scaley));

        self
    }
}
