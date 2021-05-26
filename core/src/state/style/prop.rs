use crate::State;
use crate::{entity::Entity, Builder, EventHandler, Propagation};
use crate::{state::style::*, AsEntity, Pos};

use crate::{Event, WindowEvent};

use crate::state::hierarchy::*;

pub trait PropSet: AsEntity + Sized {

    // This could fail
    fn bind(&self, state: &mut State, node: Entity) -> Entity {
        state.data_graph.add(self.entity(), node);

        self.entity()
    }

    fn insert_event(&self, state: &mut State, mut event: Event) -> Entity
    where
        Self: 'static,
    {
        state.insert_event(event.target(self.entity()));

        self.entity()
    }

    /// Add a class name to an entity
    fn class(self, state: &mut State, class_name: &str) -> Self;

    // TODO move to PropGet
    fn get_parent(self, state: &mut State) -> Option<Entity>;

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
    fn set_hoverability(self, state: &mut State, value: bool) -> Self;
    fn set_focusability(self, state: &mut State, value: bool) -> Self;

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
    //fn set_grid_column_span(self, state: &mut State, value: u32) -> Self

    // Position
    fn set_position_type(self, state: &mut State, value: PositionType) -> Entity {
        state.style.positioning_type.insert(self.entity(), value);

        self.entity()
    }

    fn set_space(self, state: &mut State, value: Units) -> Entity {
        state.style.left.insert(self.entity(), value);
        state.style.right.insert(self.entity(), value);
        state.style.top.insert(self.entity(), value);
        state.style.bottom.insert(self.entity(), value);

        self.entity()
    } 

    fn set_left(self, state: &mut State, value: Units) -> Self;
    fn set_right(self, state: &mut State, value: Units) -> Self;
    fn set_top(self, state: &mut State, value: Units) -> Self;
    fn set_bottom(self, state: &mut State, value: Units) -> Self;

    // Position Constraints
    fn set_min_left(self, state: &mut State, value: Units) -> Entity {
        state.style.min_left.insert(self.entity(), value);

        self.entity()
    }

    fn set_max_left(self, state: &mut State, value: Units) -> Entity {
        state.style.max_left.insert(self.entity(), value);

        self.entity()
    }

    fn set_min_right(self, state: &mut State, value: Units) -> Entity {
        state.style.min_right.insert(self.entity(), value);

        self.entity()
    }

    fn set_max_right(self, state: &mut State, value: Units) -> Entity {
        state.style.max_right.insert(self.entity(), value);

        self.entity()
    }

    fn set_min_top(self, state: &mut State, value: Units) -> Entity {
        state.style.min_top.insert(self.entity(), value);

        self.entity()
    }

    fn set_max_top(self, state: &mut State, value: Units) -> Entity {
        state.style.max_top.insert(self.entity(), value);

        self.entity()
    }

    fn set_min_bottom(self, state: &mut State, value: Units) -> Entity {
        state.style.min_bottom.insert(self.entity(), value);

        self.entity()
    }

    fn set_max_bottom(self, state: &mut State, value: Units) -> Entity {
        state.style.max_bottom.insert(self.entity(), value);

        self.entity()
    }

    // Size
    fn set_width(self, state: &mut State, value: Units) -> Self;
    fn set_height(self, state: &mut State, value: Units) -> Self;

    // Size Constraints
    fn set_min_width(self, state: &mut State, value: Units) -> Self;
    fn set_max_width(self, state: &mut State, value: Units) -> Self;
    fn set_min_height(self, state: &mut State, value: Units) -> Self;
    fn set_max_height(self, state: &mut State, value: Units) -> Self;

    // Text
    fn set_text(self, state: &mut State, text: &str) -> Self;

    // Text Font
    fn set_font(self, state: &mut State, font: &str) -> Self;
    fn set_font_size(self, state: &mut State, size: f32) -> Self;
    fn set_color(self, state: &mut State, color: Color) -> Self;

    // Tooltip
    fn set_tooltip(self, state: &mut State, text: &str) -> Self;

    // Background
    fn set_background_color(self, state: &mut State, value: Color) -> Self;
    fn set_background_image(self, state: &mut State, value: std::rc::Rc<()>) -> Self;

    // Border
    fn set_border_width(self, state: &mut State, value: Units) -> Self;
    fn set_border_color(self, state: &mut State, value: Color) -> Self;

    // Border Radius
    fn set_border_radius(self, state: &mut State, value: Units) -> Self;
    fn set_border_radius_top_left(self, state: &mut State, value: Units) -> Self;
    fn set_border_radius_top_right(self, state: &mut State, value: Units) -> Self;
    fn set_border_radius_bottom_left(self, state: &mut State, value: Units) -> Self;
    fn set_border_radius_bottom_right(self, state: &mut State, value: Units) -> Self;

    // Clipping
    fn set_clip_widget(self, state: &mut State, value: Entity) -> Self;

    fn set_z_order(self, state: &mut State, vaale: i32) -> Self;

    fn set_next_focus(self, state: &mut State, value: Entity) -> Self;
    fn set_prev_focus(self, state: &mut State, value: Entity) -> Self;
    fn set_focus_order(self, state: &mut State, next: Entity, prev: Entity) -> Self;

    fn mutate<F: FnMut(Builder) -> Builder>(self, state: &mut State, builder: F) -> Self;

    // fn testy<B: EventHandler + 'static>(self, state: &mut State) -> Option<&mut B>;

    // fn testy2<B: EventHandler + 'static, F: FnMut(&mut B)>(
    //     self,
    //     state: &mut State,
    //     mutator: F,
    // ) -> Self;

    // Layout

    fn set_layout_type(&self, state: &mut State, value: LayoutType) -> Entity {
        state.style.layout_type.insert(self.entity(), value);

        self.entity()
    }

    fn set_child_space(&self, state: &mut State, value: Units) -> Entity {
        state.style.child_left.insert(self.entity(), value);
        state.style.child_right.insert(self.entity(), value);
        state.style.child_top.insert(self.entity(), value);
        state.style.child_bottom.insert(self.entity(), value);

        self.entity()
    }

    fn set_child_left(&self, state: &mut State, value: Units) -> Entity {
        state.style.child_left.insert(self.entity(), value);

        self.entity()
    }

    fn set_child_between(&self, state: &mut State, value: Units) -> Entity {
        state.style.child_between.insert(self.entity(), value);

        self.entity()
    }

    fn set_child_right(&self, state: &mut State, value: Units) -> Entity {
        state.style.child_right.insert(self.entity(), value);

        self.entity()
    }

    fn set_child_top(&self, state: &mut State, value: Units) -> Entity {
        state.style.child_top.insert(self.entity(), value);

        self.entity()
    }

    fn set_child_bottom(&self, state: &mut State, value: Units) -> Entity {
        state.style.child_bottom.insert(self.entity(), value);

        self.entity()
    }

    fn set_grid_rows(&self, state: &mut State, value: Vec<Units>) -> Entity {
        state.style.grid_rows.insert(self.entity(), value);

        self.entity()
    }

    fn set_grid_cols(&self, state: &mut State, value: Vec<Units>) -> Entity {
        state.style.grid_cols.insert(self.entity(), value);

        self.entity()
    }

    fn set_row_item(&self, state: &mut State, value: (u32, u32)) -> Entity {
        if let Some(grid_item) = state.style.grid_item.get_mut(self.entity()) {
            grid_item.row_index = value.0;
            grid_item.row_span = value.1;
        }

        self.entity()
    }

    fn set_col_item(&self, state: &mut State, value: (u32, u32)) -> Entity {
        if let Some(grid_item) = state.style.grid_item.get_mut(self.entity()) {
            grid_item.col_index = value.0;
            grid_item.col_span = value.1;
        }

        self.entity()
    }

}

impl PropSet for Entity {
    // fn testy<B: EventHandler + 'static>(self, state: &mut State) -> Option<&mut B>
    // where
    //     Self: std::marker::Sized + 'static,
    // {
    //     let t = state.event_handlers.get_mut(&self).unwrap().borrow_mut();

    //     let t1 = t.downcast::<B>();

    //     t1
    // }

    // fn testy2<B: EventHandler + 'static, F: FnMut(&mut B)>(
    //     self,
    //     state: &mut State,
    //     mut mutator: F,
    // ) -> Self
    // where
    //     Self: std::marker::Sized + 'static,
    // {
    //     let t = state.event_handlers.get_mut(&self).unwrap().borrow_mut();

    //     let t1 = t.downcast::<B>().expect("Failed to cast");

    //     mutator(t1);

    //     self
    // }

    fn mutate<F>(self, state: &mut State, mut builder: F) -> Self
    where
        F: FnMut(Builder) -> Builder,
    {
        builder(Builder::new(state, self));

        self
    }

    fn class(self, state: &mut State, class_name: &str) -> Self {
        state.style.insert_class(self, class_name);

        self
    }

    fn get_parent(self, state: &mut State) -> Option<Entity> {
        self.parent(&state.hierarchy)
    }

    // PseudoClass
    fn set_enabled(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_enabled(value);
            pseudo_classes.set_disabled(!value);
        }

        state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));

        self
    }

    fn set_disabled(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_disabled(value);
            pseudo_classes.set_enabled(!value);
        }

        state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));

        self
    }

    fn set_checked(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_checked(value);
        }

        state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_over(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_over(value);
        }

        state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));

        self
    }

    fn set_active(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_active(value);
        }

        state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));

        self
    }

    fn set_hover(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_hover(value);
        }

        state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));

        self
    }

    fn set_focus(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_focus(value);
        }

        state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));

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

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_hoverability(self, state: &mut State, value: bool) -> Self {
        state.data.set_hoverability(self, value);

        self
    }

    fn set_focusability(self, state: &mut State, value: bool) -> Self {
        state.data.set_focusability(self, value);

        self
    }

    // Overflow
    fn set_overflow(self, state: &mut State, value: Overflow) -> Self {
        state.style.overflow.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    // Display
    fn set_display(self, state: &mut State, value: Display) -> Self {
        state.style.display.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    //Opacity
    fn set_opacity(self, state: &mut State, value: f32) -> Self {
        state.style.opacity.insert(self, Opacity(value));

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    // Rotate
    fn set_rotate(self, state: &mut State, value: f32) -> Self {
        state.style.rotate.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_left(self, state: &mut State, value: Units) -> Self {
        state.style.left.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_right(self, state: &mut State, value: Units) -> Self {
        state.style.right.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_top(self, state: &mut State, value: Units) -> Self {
        state.style.top.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_bottom(self, state: &mut State, value: Units) -> Self {
        state.style.bottom.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    // Size
    fn set_width(self, state: &mut State, value: Units) -> Self {
        state.style.width.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_height(self, state: &mut State, value: Units) -> Self {
        state.style.height.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    // Size Constraints
    fn set_min_width(self, state: &mut State, value: Units) -> Self {
        state.style.min_width.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_max_width(self, state: &mut State, value: Units) -> Self {
        state.style.max_width.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_min_height(self, state: &mut State, value: Units) -> Self {
        state.style.min_height.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_max_height(self, state: &mut State, value: Units) -> Self {
        state.style.max_height.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    // Tooltip
    fn set_tooltip(self, state: &mut State, value: &str) -> Self {
        state.style.tooltip.insert(self, value.to_owned());

        self
    }

    // Text
    fn set_text(self, state: &mut State, value: &str) -> Self {
        state.style.text.insert(self, value.to_owned());

        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    // Text Font
    fn set_font(self, state: &mut State, value: &str) -> Self {
        state.style.font.insert(self, value.to_owned());

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_font_size(self, state: &mut State, value: f32) -> Self {
        state.style.font_size.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_color(self, state: &mut State, value: Color) -> Self {
        state.style.font_color.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    // Background
    fn set_background_color(self, state: &mut State, value: Color) -> Self {
        state.style.background_color.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_background_image(self, state: &mut State, value: std::rc::Rc<()>) -> Self {
        state.style.background_image.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    // Border
    fn set_border_width(self, state: &mut State, value: Units) -> Self {
        state.style.border_width.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_border_color(self, state: &mut State, value: Color) -> Self {
        state.style.border_color.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    // Border Radius
    fn set_border_radius(self, state: &mut State, value: Units) -> Self {
        state.style.border_radius_top_left.insert(self, value);
        state.style.border_radius_top_right.insert(self, value);
        state.style.border_radius_bottom_left.insert(self, value);
        state.style.border_radius_bottom_right.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_border_radius_top_left(self, state: &mut State, value: Units) -> Self {
        state.style.border_radius_top_left.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_border_radius_top_right(self, state: &mut State, value: Units) -> Self {
        state.style.border_radius_top_right.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_border_radius_bottom_left(self, state: &mut State, value: Units) -> Self {
        state.style.border_radius_bottom_left.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_border_radius_bottom_right(self, state: &mut State, value: Units) -> Self {
        state.style.border_radius_bottom_right.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    // Clipping
    fn set_clip_widget(self, state: &mut State, value: Entity) -> Self {
        state.style.clip_widget.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }

    fn set_z_order(self, state: &mut State, value: i32) -> Self {
        state.style.z_order.insert(self, value);

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

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

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

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

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

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

        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));

        self
    }
}

pub trait PropGet: AsEntity {
    fn is_enabled(self, state: &mut State) -> bool;
    fn is_disabled(self, state: &mut State) -> bool;
    fn is_checked(self, state: &mut State) -> bool;
    fn is_over(self, state: &mut State) -> bool;
    fn is_active(self, state: &mut State) -> bool;
    fn is_focused(self, state: &mut State) -> bool;

    //
    fn get_overflow(&self, state: &mut State) -> Overflow;

    // Display
    fn get_display(&self, state: &mut State) -> Display;

    fn get_layout_type(&self, state: &mut State) -> LayoutType {
        state
            .style
            .layout_type
            .get(self.entity())
            .cloned()
            .unwrap_or_default()
    }

    // Background Color
    fn get_background_color(&self, state: &mut State) -> Color {
        state.style.background_color.get(self.entity()).cloned().unwrap_or_default()
    }

    // Position
    fn get_left(&self, state: &mut State) -> Units;
    fn get_right(&self, state: &mut State) -> Units;
    fn get_top(&self, state: &mut State) -> Units;
    fn get_bottom(&self, state: &mut State) -> Units;

    // Size
    fn get_width(&self, state: &mut State) -> Units;
    fn get_height(&self, state: &mut State) -> Units;

    // Size Constraints
    fn get_min_width(&self, state: &mut State) -> Units;
    fn get_max_width(&self, state: &mut State) -> Units;
    fn get_min_height(&self, state: &mut State) -> Units;
    fn get_max_height(&self, state: &mut State) -> Units;

    // Border
    fn get_border_width(&self, state: &mut State) -> Units;

    // Tooltip
    fn get_tooltip(&self, state: &mut State) -> String;

    // Text
    fn get_text(&self, state: &mut State) -> String;
    fn get_font(&self, state: &mut State) -> String;
}

impl PropGet for Entity {
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

    fn get_overflow(&self, state: &mut State) -> Overflow {
        state.style.overflow.get(*self).cloned().unwrap_or_default()
    }

    // Display
    fn get_display(&self, state: &mut State) -> Display {
        state.style.display.get(*self).cloned().unwrap_or_default()
    }

    // Position
    fn get_left(&self, state: &mut State) -> Units {
        state.style.left.get(*self).cloned().unwrap_or_default()
    }
    fn get_right(&self, state: &mut State) -> Units {
        state.style.right.get(*self).cloned().unwrap_or_default()
    }
    fn get_top(&self, state: &mut State) -> Units {
        state.style.top.get(*self).cloned().unwrap_or_default()
    }
    fn get_bottom(&self, state: &mut State) -> Units {
        state.style.bottom.get(*self).cloned().unwrap_or_default()
    }

    // Size
    fn get_width(&self, state: &mut State) -> Units {
        state.style.width.get(*self).cloned().unwrap_or_default()
    }

    fn get_height(&self, state: &mut State) -> Units {
        state.style.height.get(*self).cloned().unwrap_or_default()
    }

    // Size Constraints
    fn get_min_width(&self, state: &mut State) -> Units {
        state
            .style
            .min_width
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_max_width(&self, state: &mut State) -> Units {
        state
            .style
            .max_width
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_min_height(&self, state: &mut State) -> Units {
        state
            .style
            .min_height
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_max_height(&self, state: &mut State) -> Units {
        state
            .style
            .max_height
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    // Border
    fn get_border_width(&self, state: &mut State) -> Units {
        state
            .style
            .border_width
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    // Tooltip
    fn get_tooltip(&self, state: &mut State) -> String {
        state.style.tooltip.get(*self).cloned().unwrap_or_default()
    }

    // Text
    fn get_text(&self, state: &mut State) -> String {
        state.style.text.get(*self).cloned().unwrap_or_default()
    }

    fn get_font(&self, state: &mut State) -> String {
        state.style.font.get(*self).cloned().unwrap_or_default()
    }
}
