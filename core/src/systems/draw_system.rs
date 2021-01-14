

use femtovg::{
    renderer::OpenGl, Align, Baseline, Canvas, FillRule, FontId, ImageFlags, ImageId, LineCap,
    LineJoin, Paint, Path, Renderer, Solidity,
};

// Returns true if the entity is visible and should be drawn
pub fn is_visible(state: &mut State, entity: Entity) -> bool {

}

// Draws a shadow based on the shadow style properties of the entity
pub fn draw_shadow(state: &mut State, entity: Entity, canvas: Canvas<OpenGl>) {

}

// Draws the widget with specified background and border
pub fn draw_widget(state: &mut State, entity: Entity, canvas: Canvas<OpenGl>) {

}

// Draws the text based on style properties
pub fn draw_text(state: &mut State, entity: Entity, canvas: Canvas<OpenGl>) {

}

// Applies a clipping scissor to the widget
pub fn apply_scissor(state: &mut State, entity: Entity, canvas: Canvas<OpenGl>) {

}

// Converts from widget coordinates to screen coordinates
pub fn widget_to_screen(widget_coordinates: (f32, f32)) -> (f32,f32) {
    
}