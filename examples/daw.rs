

extern crate tuix;

use tuix::*;

use femtovg::{
    renderer::OpenGl, Baseline, Canvas, FillRule, FontId, ImageFlags, ImageId, LineCap,
    LineJoin, Paint, Path, Renderer, Solidity,
};


static THEME: &'static str = include_str!("themes/widget_theme.css");

fn main() {
    // Create the app
    let mut app = Application::new(|win_desc, state, window| {
        state.insert_theme(THEME);

        let piano_roll = PianoRoll::new().build(state, window, |builder| 
            builder
                //.set_flex_grow(1.0)
                //.set_flex_shrink(1.0)
                .set_width(Length::Percentage(1.0))
                .set_height(Length::Percentage(1.0))
                .set_background_color(Color::rgb(100,50,50))
        );


        win_desc.with_title("basic").with_inner_size(800, 600)
    });

    app.run();
}


pub struct PianoRoll {
    midi_grid_scroll_container: Entity,
}

impl PianoRoll {
    pub fn new() -> Self {
        PianoRoll {
            midi_grid_scroll_container: Entity::null(),
        }
    }
}

impl BuildHandler for PianoRoll {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        let header = HBox::new().build(state, entity, |builder| builder.class("header"));


        

        let body = VBox::new().build(state, entity, |builder| builder.class("body"));

        
        
        let container = HBox::new().build(state, body, |builder| 
            builder
                //.set_width(Length::Pixels(600.0))
                //.set_height(Length::Pixels(1200.0))
                .set_background_color(Color::rgb(100,100,50))
                .set_flex_grow(1.0)
                .set_flex_shrink(1.0)
        );
        let left = Element::new().build(state, container, |builder| builder.set_background_color(Color::rgb(200,200,255)));
        let scroll = ScrollContainer::new().build(state, left, |builder| builder.set_width(Length::Pixels(210.0)).set_flex_grow(0.0));

        let keys_container = VBox::new().build(state, scroll, |builder| 
            builder
            .set_width(Length::Pixels(200.0))
            .set_height(Length::Pixels(1200.0))
            //.set_flex_grow(1.0)
            .set_background_color(Color::rgb(80,80,80))
        );

        for j in 0..4 {
            for i in 0..12 {
                if i == 1 || i == 3 || i == 5 || i == 8 || i == 10 {
                    Element::new().build(state, keys_container, |builder| 
                        builder
                            .set_flex_grow(1.0)
                            .set_margin_bottom(Length::Pixels(1.0))
                            .set_background_color(Color::rgb(0,0,0))
                    );
                } else {
                    Element::new().build(state, keys_container, |builder| 
                        builder
                            .set_flex_grow(1.0)
                            .set_margin_bottom(Length::Pixels(1.0))
                            .set_background_color(Color::rgb(255,255,255))
                    );
                }
            }            
        }

        

        // let scroll2 = ScrollContainerH::new().build(state, container, |builder| 
        //     builder
        //         // .set_width(Length::Pixels(200.0))
        //         // .set_height(Length::Pixels(200.0))
        
        // );

        
        let right = Element::new().build(state, container, |builder| 
            builder
            .set_flex_grow(1.0)
            // .set_flex_shrink(1.0)
            .set_background_color(Color::rgb(200,200,200))
        );

        self.midi_grid_scroll_container = ScrollContainerH::new().build(state, right, |builder| builder);


        let midi_grid = MidiGrid::new().build(state, self.midi_grid_scroll_container, |builder|
            builder
                .set_flex_grow(1.0)
                .set_width(Length::Pixels(2000.0))
                .set_height(Length::Pixels(2000.0))
                .set_background_color(Color::rgb(80,80,80))
                //.set_clip_widget(self.midi_grid_scroll_container)
        );

        // let midi_note = MidiNote::new().build(state, midi_grid, |builder|
        //     builder
        //         .set_top(Length::Pixels(0.0))
        //         .set_width(Length::Pixels(40.0))
        //         .set_height(Length::Pixels(24.0))
        //         .set_background_color(Color::rgb(255,20,20))
        // );
        
        entity
    }
}

impl EventHandler for PianoRoll {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {

        if let Some(scroll_event) = event.message.downcast::<ScrollEvent>() {
            match scroll_event {
                ScrollEvent::ScrollV(val) => {
                    // Currently a hacky way to do it that doesn't currently generalise
                    self.midi_grid_scroll_container.set_top(state, Length::Percentage(*val));
                }
            }
        }

        false
    }
}

const zoom_levels: [f32; 11] = [0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5];

pub struct MidiGrid {
    zoom_index: usize,
}

impl MidiGrid {
    pub fn new() -> Self {
        MidiGrid {
            zoom_index: 5,
        }
    }
}

impl BuildHandler for MidiGrid {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }
}

impl EventHandler for MidiGrid {

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseScroll(x,y) => {
                
                    if state.modifiers.ctrl {
                        
                        let width = state.transform.get_width(entity);
                        let posx = state.transform.get_posx(entity);

                        self.zoom_index = (self.zoom_index as f32 + *y) as usize;

                        if self.zoom_index >= 10 {
                            self.zoom_index = 10;
                        }

                        if self.zoom_index <= 0 {
                            self.zoom_index = 0;
                        }

                        let new_width = 2000.0 * zoom_levels[self.zoom_index];

                        // Distance between centre and mouse position 
                        let distx = state.mouse.cursorx - posx;

                        let new_posx = distx * (zoom_levels[self.zoom_index] - 1.0);

                        entity.set_width(state, Length::Pixels(new_width));
                        //entity.set_left(state, Length::Pixels(-new_posx));
                        //state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()).origin(entity));

                    }
                }

                _=> {}
            }
        }
        
        false
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
        // Skip window
        if entity == Entity::new(0, 0) {
            return;
        }

        // Skip invisible widgets
        if state.transform.get_visibility(entity) == Visibility::Invisible {
            return;
        }

        if state.transform.get_opacity(entity) == 0.0 {
            return;
        }

        let posx = state.transform.get_posx(entity);
        let posy = state.transform.get_posy(entity);
        let width = state.transform.get_width(entity);
        let height = state.transform.get_height(entity);

        let background_color = state
        .style
        .background_color
        .get(entity)
        .cloned()
        .unwrap_or_default();

        let font_color = state
            .style
            .font_color
            .get(entity)
            .cloned()
            .unwrap_or(tuix::Color::rgb(255, 255, 255));

        let border_color = state
            .style
            .border_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let shadow_color = state
            .style
            .shadow_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let parent = state
            .hierarchy
            .get_parent(entity)
            .expect("Failed to find parent somehow");

        let parent_width = state.transform.get_width(parent);

        let border_radius_top_left = match state.style.border_radius_top_left.get(entity).cloned().unwrap_or_default() {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_top_right = match state.style.border_radius_top_right.get(entity).cloned().unwrap_or_default() {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_left = match state.style.border_radius_bottom_left.get(entity).cloned().unwrap_or_default() {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_right = match state.style.border_radius_bottom_right.get(entity).cloned().unwrap_or_default() {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let opacity = state.transform.get_opacity(entity);

        let mut background_color: femtovg::Color = background_color.into();
        background_color.set_alphaf(background_color.a * opacity);

        let mut border_color: femtovg::Color = border_color.into();
        border_color.set_alphaf(border_color.a * opacity);

        let mut shadow_color: femtovg::Color = shadow_color.into();
        shadow_color.set_alphaf(shadow_color.a * opacity);

        let border_width = match state
            .style
            .border_width
            .get(entity)
            .cloned()
            .unwrap_or_default() 
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        // Apply Scissor
        let clip_entity = state.transform.get_clip_widget(entity);

        let clip_posx = state.transform.get_posx(clip_entity);
        let clip_posy = state.transform.get_posy(clip_entity);
        let clip_width = state.transform.get_width(clip_entity);
        let clip_height = state.transform.get_height(clip_entity);

        canvas.scissor(clip_posx, clip_posy, clip_width, clip_height);

        // Draw rounded rect
        let mut path = Path::new();
        path.rounded_rect_varying(
            posx + (border_width / 2.0),
            posy + (border_width / 2.0),
            width - border_width,
            height - border_width,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );
        let mut paint = Paint::color(background_color);
        canvas.fill_path(&mut path, paint);

        // Draw border
        let mut paint = Paint::color(border_color);
        paint.set_line_width(border_width);
        //paint.set_anti_alias(false);
        canvas.stroke_path(&mut path, paint);

        let horizontal_spacing = state.transform.get_width(entity) / 50.0;
        let vertical_spacing = state.transform.get_height(entity) / 50.0;

        for i in 0..50 {
            if i % 2 == 0 {
                let mut path = Path::new();
                path.rect(posx, posy + (i as f32) * 25.0, width, 24.0);
                let mut paint = Paint::color(femtovg::Color::rgb(70,70,70));
                paint.set_line_width(1.0);
                canvas.fill_path(&mut path, paint);
            }
        }


        for i in 0..50 {
            let mut path = Path::new();
            path.move_to(posx + (i as f32)*horizontal_spacing, posy);
            path.line_to(posx + (i as f32)*horizontal_spacing, posy + height);
            let mut paint = Paint::color(femtovg::Color::rgb(100,100,100));
            paint.set_line_width(1.0);
            canvas.stroke_path(&mut path, paint);
        }


    }
}

pub struct MidiNote {
    moving: bool,
    resizing_right: bool,
    resizing_left: bool,
    mouse_down_x: f32,
}

impl MidiNote {
    pub fn new() -> Self {
        MidiNote {
            moving: false,
            resizing_left: false,
            resizing_right: false,
            mouse_down_x: 0.0,
        }
    }
}

impl BuildHandler for MidiNote {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_position(state, Position::Absolute)
    }
}

impl EventHandler for MidiNote {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if event.target == entity && *button == MouseButton::Left {
                        if state.mouse.left.pos_down.0 > state.transform.get_posx(entity) 
                            && state.mouse.left.pos_down.0 < state.transform.get_posx(entity) + 5.0 
                        {
                            self.resizing_left = true;
                            state.capture(entity);

                        } else if state.mouse.left.pos_down.0 > state.transform.get_posx(entity) + state.transform.get_width(entity) - 5.0
                            && state.mouse.left.pos_down.0 < state.transform.get_posx(entity) + state.transform.get_width(entity) 
                        {
                            self.resizing_right = true;
                            state.capture(entity);
                        } else {
                            self.moving = true;
                            self.mouse_down_x = state.mouse.left.pos_down.0;
                            state.capture(entity);
                        }
                        
                      
                        
                    }
                }


                WindowEvent::MouseUp(button) => {
                    if event.target == entity && *button == MouseButton::Left {
                        self.moving = false;
                        self.resizing_left = false;
                        self.resizing_right = false;
                        state.release(entity);
                    }
                }

                WindowEvent::MouseMove(x,y) => {
                    let dx = *x - self.mouse_down_x;

                    let parent = state.hierarchy.get_parent(entity).unwrap();

                    let parent_posy = state.transform.get_posy(parent);
                    let parent_posx = state.transform.get_posx(parent);

                    let posy = state.transform.get_posy(entity) - parent_posy;
                    let posx = state.transform.get_posx(entity) - parent_posx;
                    let height = state.transform.get_height(entity);
                    let width = state.transform.get_width(entity);

                    if self.moving {
                        if *y < state.transform.get_posy(entity) {
                            entity.set_top(state, Length::Pixels(posy - height - 1.0));
                        } else if *y > (state.transform.get_posy(entity) + height) {
                            entity.set_top(state, Length::Pixels(posy + height + 1.0));
                        }

                        if dx < -20.0 {
                            entity.set_left(state, Length::Pixels(posx - 40.0));
                            self.mouse_down_x -= 40.0;
                        } else if dx > 20.0 {
                            entity.set_left(state, Length::Pixels(posx + 40.0));
                            self.mouse_down_x += 40.0;
                        }
                    }

                    if self.resizing_right {
                        if *x > state.transform.get_posx(entity) + state.transform.get_width(entity) + 20.0 {
                            entity.set_width(state, Length::Pixels(width + 40.0));
                        } else if *x < state.transform.get_posx(entity) + state.transform.get_width(entity) - 20.0 {
                            entity.set_width(state, Length::Pixels(width - 40.0));
                        }
                    }

                    if self.resizing_left {
                        if *x > state.transform.get_posx(entity) + 20.0 {
                            entity.set_width(state, Length::Pixels(width - 40.0));
                            entity.set_left(state, Length::Pixels(posx + 40.0));
                        } else if *x < state.transform.get_posx(entity) - 20.0 {
                            entity.set_width(state, Length::Pixels(width + 40.0));
                            entity.set_left(state, Length::Pixels(posx - 40.0));
                        }
                    }
                }

                _=> {}
            }
        }
        
        false
    }
}