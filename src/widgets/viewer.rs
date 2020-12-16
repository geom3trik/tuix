#![allow(dead_code)]

use crate::component::storage::Storage;
use crate::entity::Entity;
use crate::events::{EventHandler, EventQueue, WidgetEvent, WidgetList};
use crate::mouse::*;
use crate::widget::Widget;
use crate::State;

use crate::widget::button::Button;
use crate::widget::scrollbar::{Direction, ScrollBar, Slot};

use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};

pub struct Viewer<Message> {
    viewer: Entity,
    //nav: Entity,
    image: Entity,

    image_width: f32,
    image_height: f32,

    // nav_posx: f32,
    // nav_width: f32,
    image_id: u32,

    pressed_x: f32,
    pressed_y: f32,
    panning: bool,

    zoom_index: usize,
    zoom_levels: Vec<f32>,
    zoom_level: f32,

    horizontal_scroll: Entity,
    vertical_scroll: Entity,
    scale_to_fit_button: Entity,
    //signal_pos_changed: Vec<Entity>,
    //signal_width_changed: Vec<Box<Slot>>,
}

impl<Message> Viewer<Message>
where
    Message: Copy,
{
    pub fn new(state: &mut State, widget_list: &mut WidgetList<Message>, parent: Entity) -> Self {
        //let state = &mut gui.state;
        let viewer = state.add(parent);
        //let mut nav = state.add(Some(viewer)).unwrap();
        let image = state.add(viewer);

        let viewer_width = 200.0;
        let viewer_height = 200.0;

        // Get the image from the resource manager
        let img = state.resource.images.first().unwrap();
        let w = img.width() as f32;
        let h = img.height() as f32;

        let img_min_width = 0.06 * w;
        let img_min_height = 0.06 * h;

        //let nav_width = (viewer_width / img_min_width) * viewer_width;
        //let nav_height = (viewer_height / img_min_height) * viewer_height;

        viewer
            .set_width(state, viewer_width)
            .set_height(state, viewer_height);
        // nav.set_width(state, nav_width)
        //     .set_height(state, nav_height)
        //     .set_posx(state, 0.0)
        //     .set_posy(state, 0.0);

        let img_width = (viewer_width / (w as f32)) * (w as f32);
        let img_height = (viewer_width / (w as f32)) * (h as f32);

        image
            .set_width(state, img_min_width)
            .set_height(state, img_min_height)
            .set_left(state, (viewer_width - img_min_width) / 2.0)
            .set_top(state, (viewer_height - img_min_height) / 2.0);

        let nav_width = 2.0 * 0.8 * viewer_width + img_min_width;
        let nav_height = 2.0 * 0.8 * viewer_height + img_min_height;
        let image_posx = (viewer_width - img_min_width) / 2.0;
        let image_posy = (viewer_height - img_min_height) / 2.0;
        let prx = -(image_posx - 0.8 * viewer_width) / nav_width;
        let drw = viewer_width / nav_width;
        let pry = -(image_posy - 0.8 * viewer_height) / nav_height;
        let drh = viewer_height / nav_height;

        //println!("pr: {}", prx);

        let mut h_scroll = ScrollBar::new(state, viewer, Direction::Horizontal);
        h_scroll.set_posy(state, 190.0).set_width(state, 190.0);
        h_scroll.pos_ratio = prx;
        h_scroll.dim_ratio = drw;
        let h_s = h_scroll.get_entity();

        //state.event_manager.event_handlers.push(Box::new(h_scroll));
        widget_list.push(h_scroll);

        let mut v_scroll = ScrollBar::new(state, viewer, Direction::Vertical);
        v_scroll.set_posx(state, 190.0).set_height(state, 200.0);
        v_scroll.pos_ratio = pry;
        v_scroll.dim_ratio = drh;
        let v_s = v_scroll.get_entity();

        widget_list.push(v_scroll);
        //state.event_manager.event_handlers.push(Box::new(v_scroll));

        let stf_button = Button::new(state, viewer);
        stf_button
            .id
            .set_left(state, 0.0)
            .set_top(state, 0.0)
            .set_width(state, 20.0)
            .set_height(state, 20.0);
        let stfb = stf_button.get_entity();

        widget_list.push(stf_button);

        Viewer {
            viewer: viewer,
            //nav: nav,
            image: image,
            image_id: 0,
            image_width: w,
            image_height: h,
            pressed_x: 0.0,
            pressed_y: 0.0,
            panning: false,
            zoom_index: 0,
            zoom_levels: vec![
                0.06, 0.07, 0.08, 0.09, 0.1, 0.2, 0.3, 0.4, 0.5, 0.75, 1.0, 2.0,
            ],
            zoom_level: 0.06,
            horizontal_scroll: h_s,
            vertical_scroll: v_s,
            scale_to_fit_button: stfb,
        }
    }

    pub fn get_entity(&self) -> Entity {
        self.viewer
    }

    pub fn scale_to_fit(&mut self, state: &mut State) {
        let viewer_width = state.transform.get_width(self.viewer);
        let viewer_height = state.transform.get_height(self.viewer);

        self.zoom_level = viewer_width / self.image_width;

        let idx = self
            .zoom_levels
            .iter()
            .position(|&x| (x - self.zoom_level) >= 0.0)
            .unwrap();

        self.zoom_index = idx;

        let image_height = self.image_height * self.zoom_level;

        state.transform.set_width(self.image, viewer_width);
        state.transform.set_height(self.image, image_height);

        state.transform.set_posx(self.image, 0.0);
        state
            .transform
            .set_posy(self.image, (viewer_height - image_height) / 2.0);
    }
}

impl<Message> EventHandler<Message> for Viewer<Message> {
    fn handle_event(
        &mut self,
        state: &mut State,
        event: &WidgetEvent,
        event_handlers: &mut Vec<Box<EventHandler<Message>>>,
        event_queue: &mut EventQueue<Message>,
    ) {
        match event {
            WidgetEvent::MouseButton(button, action, mods) => match button {
                MouseButton::Middle => match action {
                    MouseButtonState::Pressed => {
                        if state.hovered == self.viewer || state.hovered == self.image {
                            self.pressed_x =
                                state.mouse.cursorx - state.transform.get_posx(self.image);
                            self.pressed_y =
                                state.mouse.cursory - state.transform.get_posy(self.image);
                            self.panning = true;
                        }
                    }

                    MouseButtonState::Released => {
                        self.panning = false;
                    }
                },

                MouseButton::Right => match action {
                    MouseButtonState::Pressed => {
                        if mods.ctrl {
                            self.pressed_x =
                                state.mouse.cursorx - state.transform.get_posx(self.image);
                            self.pressed_y =
                                state.mouse.cursory - state.transform.get_posy(self.image);
                            self.panning = true;
                        }
                    }

                    MouseButtonState::Released => {
                        self.panning = false;
                    }
                },

                _ => {}
            },

            WidgetEvent::MouseMotion(x, y) => {
                if self.panning {
                    let dx = x - self.pressed_x;
                    let dy = y - self.pressed_y;

                    state.transform.set_local_x(self.image, dx);
                    state.transform.set_local_y(self.image, dy);

                    //let zoom_level = self.zoom_levels[self.zoom_index];
                    let image_width = self.zoom_level * self.image_width;
                    let image_height = self.zoom_level * self.image_height;

                    //let mut nav_width = self.nav_ratio * image_width;
                    let nav_posx = dx;

                    //let mut nav_posx = dx - 0.8 * state.transform.get_global_width(self.viewer);

                    let viewer_width = state.transform.get_global_width(self.viewer);
                    let viewer_height = state.transform.get_global_height(self.viewer);

                    //let min_dx = 0.2 * viewer_width - image_width;
                    //let max_dx = 0.8 * viewer_width;

                    //let nav_width = max_dx - min_dx;
                    let nav_width = 2.0 * 0.8 * viewer_width + image_width;
                    let nav_height = 2.0 * 0.8 * viewer_height + image_height;

                    let mut prx = -(dx - 0.8 * viewer_width) / nav_width;
                    let mut pry = -(dy - 0.8 * viewer_height) / nav_height;

                    //println!("nav_width: {}", nav_width);
                    //println!("nav_posx: {}", nav_posx);

                    if dx >= 0.8 * viewer_width {
                        state.transform.set_local_x(self.image, 0.8 * viewer_width);
                        prx = 0.0;
                    }

                    if dx + image_width <= 0.2 * viewer_width {
                        state
                            .transform
                            .set_local_x(self.image, 0.2 * viewer_width - image_width);
                        prx = (0.6 * viewer_width + image_width) / nav_width;
                    }

                    if dy >= 0.8 * viewer_height {
                        state.transform.set_local_y(self.image, 0.8 * viewer_height);
                        pry = 0.0;
                    }

                    if dy + image_height <= 0.2 * viewer_height {
                        state
                            .transform
                            .set_local_y(self.image, 0.2 * viewer_height - image_height);
                        pry = (0.6 * viewer_height + image_height) / nav_height;
                    }

                    // if state.transform.get_local_x(self.nav)
                    //     + state.transform.get_global_width(self.nav)
                    //     <= state.transform.get_global_width(self.viewer)
                    // {
                    //     state.transform.set_local_x(
                    //         self.nav,
                    //         state.transform.get_global_width(self.viewer)
                    //             - state.transform.get_global_width(self.nav),
                    //     );
                    // }

                    event_queue.push(WidgetEvent::WidgetValueChanged(
                        self.viewer,
                        "pos".to_string(),
                        prx,
                    ));
                    event_queue.push(WidgetEvent::WidgetValueChanged(
                        self.viewer,
                        "pos".to_string(),
                        pry,
                    ));

                    //let event = WidgetEvent::WidgetValueChanged(self.viewer, "pos".to_string(), pr);
                    //event_queue.push(event);
                }
            }

            WidgetEvent::MouseScroll(x, y) => {
                let yy: i32 = if *y > 0.0 { 1 } else { -1 };

                let mut zoom_index: i32 = self.zoom_index as i32 + yy;

                if zoom_index > (self.zoom_levels.len() - 1) as i32 {
                    zoom_index = (self.zoom_levels.len() - 1) as i32;
                }

                if zoom_index < 0 {
                    zoom_index = 0;
                }

                self.zoom_index = zoom_index as usize;
                self.zoom_level = self.zoom_levels[zoom_index as usize];

                println!("zoom_level: {}", self.zoom_level);

                let new_width = self.image_width * self.zoom_level;
                let new_height = self.image_height * self.zoom_level;
                let viewer_width = state.transform.get_global_width(self.viewer);
                let viewer_height = state.transform.get_global_height(self.viewer);

                let nav_width = 2.0 * 0.8 * viewer_width + new_width;
                let nav_height = 2.0 * 0.8 * viewer_height + new_height;

                //let mut nav_posx =
                //    state.transform.get_local_x(self.image) + (new_width - nav_width) / 2.0;

                // let img_width = state.transform.get_global_width(self.image);
                state.transform.set_local_width(self.image, new_width);
                state.transform.set_local_height(self.image, new_height);

                let posx = state.transform.get_local_x(self.image);
                let posy = state.transform.get_local_y(self.image);

                state
                    .transform
                    .set_local_x(self.image, (viewer_width - new_width) / 2.0);
                state
                    .transform
                    .set_local_y(self.image, (viewer_height - new_height) / 2.0);

                let mut prx =
                    -(state.transform.get_local_x(self.image) - 0.8 * viewer_width) / nav_width;

                let mut pry =
                    -(state.transform.get_local_y(self.image) - 0.8 * viewer_height) / nav_height;

                if state.transform.get_local_x(self.image) >= 0.8 * viewer_width {
                    state.transform.set_local_x(self.image, 0.8 * viewer_width);
                    prx = 0.0;
                }

                if state.transform.get_local_x(self.image) + new_width <= 0.2 * viewer_width {
                    state
                        .transform
                        .set_local_x(self.image, 0.2 * viewer_width - new_width);
                    prx = (0.6 * viewer_width + new_width) / nav_width;
                }

                if state.transform.get_local_y(self.image) >= 0.8 * viewer_height {
                    state.transform.set_local_y(self.image, 0.8 * viewer_height);
                    pry = 0.0;
                }

                if state.transform.get_local_y(self.image) + new_height <= 0.2 * viewer_height {
                    state
                        .transform
                        .set_local_y(self.image, 0.2 * viewer_height - new_height);
                    pry = (0.6 * viewer_height + new_height) / nav_height;
                }

                // let width = state.transform.get_global_width(self.nav);
                // state.transform.set_local_width(self.nav, width + 10.0 * y);

                // if state.transform.get_local_width(self.nav)
                //     <= state.transform.get_global_width(self.viewer)
                // {
                //     state
                //         .transform
                //         .set_local_width(self.nav, state.transform.get_global_width(self.viewer));
                // }

                // let posx = state.transform.get_local_x(self.nav);
                // state.transform.set_local_x(self.nav, posx - 5.0 * y);

                // if state.transform.get_local_x(self.nav) >= 0.0 {
                //     state.transform.set_local_x(self.nav, 0.0);
                // }

                // if state.transform.get_local_x(self.nav)
                //     + state.transform.get_global_width(self.nav)
                //     <= state.transform.get_global_width(self.viewer)
                // {
                //     state.transform.set_local_x(
                //         self.nav,
                //         state.transform.get_global_width(self.viewer)
                //             - state.transform.get_global_width(self.nav),
                //     );
                // }

                //let pr = -nav_posx / nav_width;
                // let event = WidgetEvent::WidgetValueChanged(self.viewer, "pos".to_string(), pr);
                // event_queue.push(event);

                event_queue.push(WidgetEvent::WidgetValueChanged(
                    self.viewer,
                    "pos".to_string(),
                    prx,
                ));
                event_queue.push(WidgetEvent::WidgetValueChanged(
                    self.viewer,
                    "pos".to_string(),
                    pry,
                ));

                let drw = state.transform.get_global_width(self.viewer) / nav_width;
                let drh = state.transform.get_global_height(self.viewer) / nav_height;
                // let event = WidgetEvent::WidgetValueChanged(self.viewer, "width".to_string(), wr);
                // event_queue.push(event);

                event_queue.push(WidgetEvent::WidgetValueChanged(
                    self.viewer,
                    "width".to_string(),
                    drw,
                ));
                event_queue.push(WidgetEvent::WidgetValueChanged(
                    self.viewer,
                    "width".to_string(),
                    drh,
                ));
            }

            WidgetEvent::WidgetValueChanged(entity, name, value) => {
                if *entity == self.horizontal_scroll {
                    if *name == "pos".to_string() {
                        let viewer_width = state.transform.get_global_width(self.viewer);
                        let zoom_level = self.zoom_levels[self.zoom_index];
                        let image_width = zoom_level * self.image_width;
                        let nav_width = 2.0 * 0.8 * viewer_width + image_width;
                        let posx = -value * nav_width + 0.8 * viewer_width;
                        //let dx = value * state.transform.get_global_width(self.nav);
                        state.transform.set_local_x(self.image, posx);
                    }
                }

                if *entity == self.vertical_scroll {
                    if *name == "pos".to_string() {
                        let viewer_height = state.transform.get_global_height(self.viewer);
                        let zoom_level = self.zoom_levels[self.zoom_index];
                        let image_height = zoom_level * self.image_height;
                        let nav_height = 2.0 * 0.8 * viewer_height + image_height;
                        let posy = -value * nav_height + 0.8 * viewer_height;
                        //let dx = value * state.transform.get_global_width(self.nav);
                        state.transform.set_local_y(self.image, posy);
                    }
                }
            }

            WidgetEvent::WidgetPressed(entity) => {
                if *entity == self.scale_to_fit_button {
                    self.scale_to_fit(state);
                }
            }

            WidgetEvent::CharInput(input) => {
                if *input == 'g' {
                    self.scale_to_fit(state);
                }
            }

            _ => {}
        }
    }

    fn get_entity(&self) -> Entity {
        self.viewer
    }
}
