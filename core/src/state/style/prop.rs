use crate::entity::Entity;
use crate::state::style::*;
use crate::State;

use crate::{Event, WindowEvent};

use crate::state::hierarchy::*;

pub trait PropSet {
    //fn get_first_child(self, hierarchy: &Hierarchy) -> Option<Entity>;

    fn class(self, state: &mut State, class_name: &str) -> Self;

    fn get_parent(self, state: &mut State) -> Option<Entity>;

    fn is_enabled(self, state: &mut State) -> bool;
    fn is_disabled(self, state: &mut State) -> bool;
    fn is_checked(self, state: &mut State) -> bool;
    fn is_over(self, state: &mut State) -> bool;
    fn is_active(self, state: &mut State) -> bool;
    fn is_focused(self, state: &mut State) -> bool;

    // Pseudoclass
    fn set_enabled(self, state: &mut State, value: bool) -> Self;
    fn set_disabled(self, state: &mut State, value: bool) -> Self;
    fn set_checked(self, state: &mut State, value: bool) -> Self;
    fn set_over(self, state: &mut State, value: bool) -> Self;
    fn set_active(self, state: &mut State, value: bool) -> Self;
    fn set_hover(self, state: &mut State, value: bool) -> Self;
    fn set_focus(self, state: &mut State, value: bool) -> Self;

    // Style
    fn set_element(self, state: &mut State, value: &str) -> Self;
    fn set_id(self, state: &mut State, value: &str) -> Self;
    fn set_class(self, state: &mut State, value: &str) -> Self;

    // Visibility
    fn set_visibility(self, state: &mut State, value: Visibility) -> Self;

    // Overflow
    fn set_overflow(self, state: &mut State, value: Overflow) -> Self;

    // Display
    fn set_display(self, state: &mut State, value: Display) -> Self;

    //Opacity
    fn set_opacity(self, state: &mut State, value: f32) -> Self;

    // Rotate
    fn set_rotate(self, state: &mut State, value: f32) -> Self;

    // Grid Container
    //fn set_grid_columns(self, state: &mut State, value: Vec<f32>) -> Self;
    //fn set_grid_rows(self, state: &mut State, value: Vec<f32>) -> Self;

    // Grid Item
    //fn set_grid_column_start(self, state: &mut State, value: u32) -> Self;
    //fn set_grid_column_span(self, state: &mut State, value: u32) -> Self;

    // Flex Container
    fn set_flex_direction(self, state: &mut State, value: FlexDirection) -> Self;
    fn set_justify_content(self, state: &mut State, value: JustifyContent) -> Self;
    fn set_align_content(self, state: &mut State, value: AlignContent) -> Self;
    fn set_align_items(self, state: &mut State, value: AlignItems) -> Self;

    // Flex Item
    fn set_flex_grow(self, state: &mut State, value: f32) -> Self;
    fn set_flex_shrink(self, state: &mut State, value: f32) -> Self;
    fn set_flex_basis(self, state: &mut State, value: f32) -> Self;
    fn set_align_self(self, state: &mut State, value: AlignSelf) -> Self;

    // Positioning
    fn set_position(self, state: &mut State, value: Position) -> Self;
    fn set_left(self, state: &mut State, value: Length) -> Self;
    fn set_right(self, state: &mut State, value: Length) -> Self;
    fn set_top(self, state: &mut State, value: Length) -> Self;
    fn set_bottom(self, state: &mut State, value: Length) -> Self;

    // Size
    fn set_width(self, state: &mut State, value: Length) -> Self;
    fn set_height(self, state: &mut State, value: Length) -> Self;

    // Size Constraints
    fn set_min_width(self, state: &mut State, value: Length) -> Self;
    fn set_max_width(self, state: &mut State, value: Length) -> Self;
    fn set_min_height(self, state: &mut State, value: Length) -> Self;
    fn set_max_height(self, state: &mut State, value: Length) -> Self;

    // Text
    fn set_text(self, state: &mut State, text: &str) -> Self;

    // Text Font
    fn set_font(self, state: &mut State, font: String) -> Self;
    fn set_font_size(self, state: &mut State, size: f32) -> Self;
    fn set_font_color(self, state: &mut State, color: Color) -> Self;

    // Text Alignment
    fn set_text_align(self, state: &mut State, align: Align) -> Self;
    fn set_text_justify(self, state: &mut State, justify: Justify) -> Self;

    // Background
    fn set_background_color(self, state: &mut State, value: Color) -> Self;

    // Border
    fn set_border_width(self, state: &mut State, value: Length) -> Self;
    fn set_border_color(self, state: &mut State, value: Color) -> Self;

    // Border Radius
    fn set_border_radius(self, state: &mut State, value: Length) -> Self;
    fn set_border_radius_top_left(self, state: &mut State, value: Length) -> Self;
    fn set_border_radius_top_right(self, state: &mut State, value: Length) -> Self;
    fn set_border_radius_bottom_left(self, state: &mut State, value: Length) -> Self;
    fn set_border_radius_bottom_right(self, state: &mut State, value: Length) -> Self;

    // Margin
    fn set_margin(self, state: &mut State, value: Length) -> Self;
    fn set_margin_left(self, state: &mut State, value: Length) -> Self;
    fn set_margin_right(self, state: &mut State, value: Length) -> Self;
    fn set_margin_top(self, state: &mut State, value: Length) -> Self;
    fn set_margin_bottom(self, state: &mut State, value: Length) -> Self;

    // Padding
    fn set_padding(self, state: &mut State, value: Length) -> Self;
    fn set_padding_left(self, state: &mut State, value: Length) -> Self;
    fn set_padding_right(self, state: &mut State, value: Length) -> Self;
    fn set_padding_top(self, state: &mut State, value: Length) -> Self;
    fn set_padding_bottom(self, state: &mut State, value: Length) -> Self;

    // Clipping
    fn set_clip_widget(self, state: &mut State, value: Entity) -> Self;

    fn set_z_order(self, state: &mut State, vaale: i32) -> Self;

    fn set_next_focus(self, state: &mut State, value: Entity) -> Self;
    fn set_prev_focus(self, state: &mut State, value: Entity) -> Self;
    fn set_focus_order(self, state: &mut State, next: Entity, prev: Entity) -> Self;
}

impl PropSet for Entity {

    fn class(self, state: &mut State, class_name: &str) -> Self
    {
        state.style.insert_class(self, class_name);

        self
    }


    fn get_parent(self, state: &mut State) -> Option<Entity> {
        self.parent(&state.hierarchy)
    }

    fn is_enabled(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.get_enabled()
        } else {
            false
        }
    }
    fn is_disabled(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.get_disabled()
        } else {
            false
        }
    }
    fn is_checked(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.get_checked()
        } else {
            false
        }
    }
    fn is_over(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.get_over()
        } else {
            false
        }
    }
    fn is_active(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.get_active()
        } else {
            false
        }
    }
    fn is_focused(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.get_focus()
        } else {
            false
        }
    }

    // PseudoClass
    fn set_enabled(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_enabled(value);
            pseudo_classes.set_disabled(!value);
        }

        state.insert_event(Event::new(WindowEvent::Restyle).origin(self));
        state.insert_event(Event::new(WindowEvent::Redraw).origin(self));

        self
    }

    fn set_disabled(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_disabled(value);
            pseudo_classes.set_enabled(!value);
        }

        state.insert_event(Event::new(WindowEvent::Restyle).origin(self));
        state.insert_event(Event::new(WindowEvent::Redraw).origin(self));

        self
    }

    fn set_checked(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_checked(value);
        }

        state.insert_event(Event::new(WindowEvent::Restyle).origin(self));
        state.insert_event(Event::new(WindowEvent::Redraw).origin(self));

        self
    }

    fn set_over(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_over(value);
        }

        state.insert_event(Event::new(WindowEvent::Restyle).origin(self));
        state.insert_event(Event::new(WindowEvent::Redraw).origin(self));

        self
    }

    fn set_active(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_active(value);
        }

        state.insert_event(Event::new(WindowEvent::Restyle).origin(self));
        state.insert_event(Event::new(WindowEvent::Redraw).origin(self));

        self
    }

    fn set_hover(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_hover(value);
        }

        state.insert_event(Event::new(WindowEvent::Restyle).origin(self));
        state.insert_event(Event::new(WindowEvent::Redraw).origin(self));

        self
    }

    fn set_focus(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_focus(value);
        }

        state.insert_event(Event::new(WindowEvent::Restyle).origin(self));
        state.insert_event(Event::new(WindowEvent::Redraw).origin(self));

        self
    }

    // Style
    fn set_element(self, state: &mut State, value: &str) -> Self {
        state.style.insert_element(self, value);

        self
    }

    fn set_id(self, state: &mut State, value: &str) -> Self {
        state.style.insert_id(self, value);

        self
    }

    fn set_class(self, state: &mut State, value: &str) -> Self {
        state.style.insert_class(self, value);

        self
    }

    // Visibility
    fn set_visibility(self, state: &mut State, value: Visibility) -> Self {
        state.style.visibility.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    // Overflow
    fn set_overflow(self, state: &mut State, value: Overflow) -> Self {
        state.style.overflow.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    // Display
    fn set_display(self, state: &mut State, value: Display) -> Self {
        state.style.display.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    //Opacity
    fn set_opacity(self, state: &mut State, value: f32) -> Self {
        state.style.opacity.insert(self, Opacity(value));

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    // Rotate
    fn set_rotate(self, state: &mut State, value: f32) -> Self {
        state.style.rotate.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    // Flex Container
    fn set_flex_direction(self, state: &mut State, value: FlexDirection) -> Self {
        state.style.flex_direction.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    //TODO
    // fn set_flex_wrap(self, state: &mut State, value: FlexDirection) -> Self {
    //     if let Some(data) = state.style.grid_container.get_mut(self) {
    //         data.flex_direction = value;
    //     }

    //     self
    // }

    fn set_justify_content(self, state: &mut State, value: JustifyContent) -> Self {
        state.style.justify_content.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_align_content(self, state: &mut State, value: AlignContent) -> Self {
        state.style.align_content.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_align_items(self, state: &mut State, value: AlignItems) -> Self {
        state.style.align_items.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    // Flex Item
    fn set_flex_grow(self, state: &mut State, value: f32) -> Self {
        state.style.flex_grow.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_flex_shrink(self, state: &mut State, value: f32) -> Self {
        state.style.flex_shrink.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_flex_basis(self, state: &mut State, value: f32) -> Self {
        state.style.flex_basis.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_align_self(self, state: &mut State, value: AlignSelf) -> Self {
        state.style.align_self.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    // Positioning
    fn set_position(self, state: &mut State, value: Position) -> Self {
        state.style.position.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_left(self, state: &mut State, value: Length) -> Self {
        state.style.left.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_right(self, state: &mut State, value: Length) -> Self {
        state.style.right.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_top(self, state: &mut State, value: Length) -> Self {
        state.style.top.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_bottom(self, state: &mut State, value: Length) -> Self {
        state.style.bottom.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    // Size
    fn set_width(self, state: &mut State, value: Length) -> Self {
        state.style.width.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_height(self, state: &mut State, value: Length) -> Self {
        state.style.height.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    // Size Constraints
    fn set_min_width(self, state: &mut State, value: Length) -> Self {
        state.style.min_width.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_max_width(self, state: &mut State, value: Length) -> Self {
        state.style.max_width.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_min_height(self, state: &mut State, value: Length) -> Self {
        state.style.min_height.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_max_height(self, state: &mut State, value: Length) -> Self {
        state.style.max_height.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    // Text
    fn set_text(self, state: &mut State, value: &str) -> Self {
        if let Some(data) = state.style.text.get_mut(self) {
            data.text = value.to_string();
        } else {
            state.style.text.insert(
                self,
                Text {
                    text: value.to_string(),
                    ..Default::default()
                },
            );
        }

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    // Text Font
    fn set_font(self, state: &mut State, value: String) -> Self {
        if let Some(data) = state.style.text.get_mut(self) {
            data.font = value;
        } else {
            state.style.text.insert(
                self,
                Text {
                    font: value,
                    ..Default::default()
                },
            );
        }

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_font_size(self, state: &mut State, value: f32) -> Self {

        state.style.font_size.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_font_color(self, state: &mut State, value: Color) -> Self {
        state.style.font_color.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    // Text Alignment
    fn set_text_justify(self, state: &mut State, value: Justify) -> Self {
        state.style.text_justify.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_text_align(self, state: &mut State, value: Align) -> Self {
        state.style.text_align.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    // Background
    fn set_background_color(self, state: &mut State, value: Color) -> Self {
        state.style.background_color.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    // Border
    fn set_border_width(self, state: &mut State, value: Length) -> Self {
        state.style.border_width.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_border_color(self, state: &mut State, value: Color) -> Self {
        state.style.border_color.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    // Border Radius
    fn set_border_radius(self, state: &mut State, value: Length) -> Self {
        state.style.border_radius_top_left.insert(self, value);
        state.style.border_radius_top_right.insert(self, value);
        state.style.border_radius_bottom_left.insert(self, value);
        state.style.border_radius_bottom_right.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_border_radius_top_left(self, state: &mut State, value: Length) -> Self {
        state.style.border_radius_top_left.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_border_radius_top_right(self, state: &mut State, value: Length) -> Self {
        state.style.border_radius_top_right.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_border_radius_bottom_left(self, state: &mut State, value: Length) -> Self {
        state.style.border_radius_bottom_left.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_border_radius_bottom_right(self, state: &mut State, value: Length) -> Self {
        state.style.border_radius_bottom_right.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    // Margin
    fn set_margin(self, state: &mut State, value: Length) -> Self {
        state.style.margin_left.insert(self, value);
        state.style.margin_right.insert(self, value);
        state.style.margin_top.insert(self, value);
        state.style.margin_bottom.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_margin_left(self, state: &mut State, value: Length) -> Self {
        state.style.margin_left.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }
    fn set_margin_right(self, state: &mut State, value: Length) -> Self {
        state.style.margin_right.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }
    fn set_margin_top(self, state: &mut State, value: Length) -> Self {
        state.style.margin_top.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }
    fn set_margin_bottom(self, state: &mut State, value: Length) -> Self {
        state.style.margin_bottom.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    // Padding
    fn set_padding(self, state: &mut State, value: Length) -> Self {
        state.style.padding_left.insert(self, value);
        state.style.padding_right.insert(self, value);
        state.style.padding_top.insert(self, value);
        state.style.padding_bottom.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_padding_left(self, state: &mut State, value: Length) -> Self {
        state.style.padding_left.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }
    fn set_padding_right(self, state: &mut State, value: Length) -> Self {
        state.style.padding_right.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }
    fn set_padding_top(self, state: &mut State, value: Length) -> Self {
        state.style.padding_top.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }
    fn set_padding_bottom(self, state: &mut State, value: Length) -> Self {
        state.style.padding_bottom.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    // Clipping
    fn set_clip_widget(self, state: &mut State, value: Entity) -> Self {
        state.style.clip_widget.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_z_order(self, state: &mut State, value: i32) -> Self {
        state.style.z_order.insert(self, value);

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_next_focus(self, state: &mut State, value: Entity) -> Self {
        if let Some(data) = state.style.focus_order.get_mut(self) {
            data.next = value;
        } else {
            state.style.focus_order.insert(
                self,
                FocusOrder {
                    next: value,
                    ..Default::default()
                },
            );
        }

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_prev_focus(self, state: &mut State, value: Entity) -> Self {
        if let Some(data) = state.style.focus_order.get_mut(self) {
            data.prev = value;
        } else {
            state.style.focus_order.insert(
                self,
                FocusOrder {
                    prev: value,
                    ..Default::default()
                },
            );
        }

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }

    fn set_focus_order(self, state: &mut State, next: Entity, prev: Entity) -> Self {
        if let Some(data) = state.style.focus_order.get_mut(self) {
            data.next = next;
            data.prev = prev;
        } else {
            state
                .style
                .focus_order
                .insert(self, FocusOrder { next, prev });
        }

        state.insert_event(
            Event::new(WindowEvent::Relayout)
                .target(Entity::null())
                .origin(self),
        );
        state.insert_event(Event::new(WindowEvent::Redraw));

        self
    }
}
