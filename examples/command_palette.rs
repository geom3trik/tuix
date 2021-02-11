extern crate tuix;

use tuix::*;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use femtovg::{
    renderer::OpenGl, Align, Baseline, Canvas, FillRule, FontId, ImageFlags, ImageId, LineCap,
    LineJoin, Paint, Path, Renderer, Solidity,
};

fn main() {
    let app = Application::new(|win_desc, state, window| {
        state.insert_stylesheet("examples/themes/cmd_palette_theme.css");

        CommandPalette::new().build(state, window, |builder| {
            builder
                .set_box_shadow_blur(Length::Pixels(10.0))
                .set_box_shadow_v_offset(Length::Pixels(5.0))
                .set_box_shadow_color(Color::rgba(0, 0, 0, 128))
        });

        win_desc.with_title("Command Palette")
    });

    app.run();
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchboxEvent {
    Changed(String),
}

pub struct CommandPalette {
    search_box: Entity,
    scroll_container: Entity,
    current_selection: Entity,
    matcher: SkimMatcherV2,
    commands: Vec<(String, String, i64)>,
}

impl CommandPalette {
    pub fn new() -> Self {
        let mut commands: Vec<(String, String, i64)> = Default::default();
        commands.push((
            "Toggle Full Screen Mode".to_string(),
            "Shortcut".to_string(),
            0,
        ));
        commands.push((
            "Toggle Second Window".to_string(),
            "Shortcut".to_string(),
            0,
        ));
        commands.push((
            "Toggle Session/Arrangement View".to_string(),
            "Shortcut".to_string(),
            0,
        ));
        commands.push((
            "Toggle Device/Clip View".to_string(),
            "Shortcut".to_string(),
            0,
        ));
        commands.push((
            "Hide/Show Detail View".to_string(),
            "Shortcut".to_string(),
            0,
        ));
        commands.push((
            "Toggle Hot-Swap Mode".to_string(),
            "Shortcut".to_string(),
            0,
        ));
        commands.push((
            "Toggle Drum Rack/last-selected Pad".to_string(),
            "Shortcut".to_string(),
            0,
        ));
        commands.push(("Hide/Show Info View".to_string(), "Shortcut".to_string(), 0));
        commands.push((
            "Hide/Show Video Window".to_string(),
            "Shortcut".to_string(),
            0,
        ));
        commands.push(("Hide/Show Browser".to_string(), "Shortcut".to_string(), 0));
        commands.push(("Hide/Show Overview".to_string(), "Shortcut".to_string(), 0));
        commands.push(("Hide/Show In/Out".to_string(), "Shortcut".to_string(), 0));
        commands.push(("Hide/Show Sends".to_string(), "Shortcut".to_string(), 0));
        commands.push(("Hide/Show Mixer".to_string(), "Shortcut".to_string(), 0));
        commands.push((
            "Open the Preferences".to_string(),
            "Shortcut".to_string(),
            0,
        ));
        commands.push(("Close Window/Dialog".to_string(), "Shortcut".to_string(), 0));

        Self {
            search_box: Entity::null(),
            scroll_container: Entity::null(),
            current_selection: Entity::null(),
            matcher: Default::default(),
            commands,
        }
    }
}

impl BuildHandler for CommandPalette {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        state.focused = entity;

        // Textbox for command searching
        self.search_box = Textbox::new("Type a command")
            .on_change(move |val| {
                Event::new(SearchboxEvent::Changed(val.to_string())).target(entity)
            })
            .build(state, entity, |builder| builder.class("search"));

        //self.current_selection = self.search_box;

        self.scroll_container = ScrollContainer::new().build(state, entity, |builder| builder);

        self.current_selection = SearchLabel::new("Toggle Full Screen Mode").build(
            state,
            self.scroll_container,
            |builder| builder.class("command").set_checked(true),
        );
        SearchLabel::new("Toggle Second Window").build(state, self.scroll_container, |builder| {
            builder.class("command")
        });
        SearchLabel::new("Toggle Session/Arrangement View").build(
            state,
            self.scroll_container,
            |builder| builder.class("command"),
        );
        SearchLabel::new("Toggle Device/Clip View").build(
            state,
            self.scroll_container,
            |builder| builder.class("command"),
        );
        SearchLabel::new("Hide/Show Detail View").build(state, self.scroll_container, |builder| {
            builder.class("command")
        });
        SearchLabel::new("Toggle Hot-Swap Mode").build(state, self.scroll_container, |builder| {
            builder.class("command")
        });
        SearchLabel::new("Toggle Drum Rack/last-selected Pad").build(
            state,
            self.scroll_container,
            |builder| builder.class("command"),
        );
        SearchLabel::new("Hide/Show Info View").build(state, self.scroll_container, |builder| {
            builder.class("command")
        });
        SearchLabel::new("Hide/Show Video Window").build(state, self.scroll_container, |builder| {
            builder.class("command")
        });
        SearchLabel::new("Hide/Show Browser").build(state, self.scroll_container, |builder| {
            builder.class("command")
        });
        SearchLabel::new("Hide/Show Overview").build(state, self.scroll_container, |builder| {
            builder.class("command")
        });
        SearchLabel::new("Hide/Show In/Out").build(state, self.scroll_container, |builder| {
            builder.class("command")
        });
        SearchLabel::new("Hide/Show Sends").build(state, self.scroll_container, |builder| {
            builder.class("command")
        });
        SearchLabel::new("Hide/Show Mixer").build(state, self.scroll_container, |builder| {
            builder.class("command")
        });
        SearchLabel::new("Open the Preferences").build(state, self.scroll_container, |builder| {
            builder.class("command")
        });
        SearchLabel::new("Close Window/Dialog").build(state, self.scroll_container, |builder| {
            builder.class("command")
        });

        // Command::new("Command 1","Shortcut").build(state, entity, |builder| builder);
        // Command::new("Command 2","Shortcut").build(state, entity, |builder| builder);
        // Command::new("Command 3","Shortcut").build(state, entity, |builder| builder);
        // Command::new("Command 4","Shortcut").build(state, entity, |builder| builder);
        // Command::new("Command 5","Shortcut").build(state, entity, |builder| builder);
        // Command::new("Command 6","Shortcut").build(state, entity, |builder| builder);
        // Command::new("Command 7","Shortcut").build(state, entity, |builder| builder);
        // Command::new("Command 8","Shortcut").build(state, entity, |builder| builder);
        // Command::new("Command 9","Shortcut").build(state, entity, |builder| builder);
        // Command::new("Command 10","Shortcut").build(state, entity, |builder| builder);
        // Command::new("Command 11","Shortcut").build(state, entity, |builder| builder);
        // Command::new("Command 12","Shortcut").build(state, entity, |builder| builder);
        // Command::new("Command 13","Shortcut").build(state, entity, |builder| builder);
        // Command::new("Command 14","Shortcut").build(state, entity, |builder| builder);
        // Command::new("Command 15","Shortcut").build(state, entity, |builder| builder);
        // Command::new("Command 16","Shortcut").build(state, entity, |builder| builder);

        //Command::new("Play","Spacebar").build(state, entity, |builder| builder);

        entity.set_element(state, "command_palette");

        entity
    }
}

impl EventHandler for CommandPalette {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(searchbox_event) = event.message.downcast::<SearchboxEvent>() {
            match searchbox_event {
                SearchboxEvent::Changed(val) => {
                    //println!("Value: {}", val);
                    let matcher = &self.matcher;

                    self.commands.iter_mut().for_each(|(cmd, _, score)| {
                        *score = matcher.fuzzy_match(cmd, val).unwrap_or(0)
                    });

                    self.commands.sort_by_cached_key(|(_, _, score)| *score);

                    for (index, cmd) in self.commands.iter().rev().enumerate() {
                        let score = cmd.2;

                        let (_, indices) =
                            matcher.fuzzy_indices(&cmd.0, val).unwrap_or((0, vec![]));

                        if let Some(command_widget) =
                            state.hierarchy.get_child(self.scroll_container, index + 1)
                        {
                            if val.is_empty() {
                                command_widget.set_text(state, &cmd.0);
                                command_widget.set_display(state, Display::Flexbox);
                                state.insert_event(Event::new(SearchLabelEvent::Highlight(vec![])));
                            } else {
                                if score > 0 {
                                    command_widget.set_text(state, &cmd.0);
                                    command_widget.set_display(state, Display::Flexbox);
                                    state.insert_event(Event::new(SearchLabelEvent::Highlight(
                                        indices,
                                    )));
                                } else {
                                    command_widget.set_text(state, "");
                                    command_widget.set_display(state, Display::None);
                                }
                            }
                        }

                        state.insert_event(Event::new(WindowEvent::Redraw));
                    }

                    // if let Some((score, indices)) = self.matcher.fuzzy_indices("some kind of command", val) {
                    //     println!("Score: {}  Indices: {:?}", score, indices);
                    // }
                }
            }
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::KeyDown(code, key) => {
                    match key {
                        Some(Key::ArrowDown) => {
                            if let Some(next_item) =
                                state.hierarchy.get_next_sibling(self.current_selection)
                            {
                                if state.data.get_visibility(next_item) != Visibility::Invisible {
                                    self.current_selection.set_checked(state, false);
                                    self.current_selection = next_item;
                                    self.current_selection.set_checked(state, true);
                                }

                                event.consume();
                            }
                        }

                        Some(Key::ArrowUp) => {
                            if let Some(prev_item) =
                                state.hierarchy.get_prev_sibling(self.current_selection)
                            {
                                if prev_item != self.search_box {
                                    self.current_selection.set_checked(state, false);
                                    self.current_selection = prev_item;
                                    self.current_selection.set_checked(state, true);
                                    event.consume();
                                }
                            }
                        }

                        _ => {}
                    }

                    // if *key == Some(Key::ArrowDown) {
                    //     if let Some(next_item) =
                    //         state.hierarchy.get_next_sibling(self.current_selection)
                    //     {
                    //         if state.data.get_visibility(next_item) != Visibility::Invisible {
                    //             self.current_selection.set_checked(state, false);
                    //             self.current_selection = next_item;
                    //             self.current_selection.set_checked(state, true);
                    //         }

                    //         event.consume();
                    //     }
                    // }

                    // if *key == Some(Key::ArrowUp) {
                    //     if let Some(prev_item) =
                    //         state.hierarchy.get_prev_sibling(self.current_selection)
                    //     {
                    //         if prev_item != self.search_box {
                    //             self.current_selection.set_checked(state, false);
                    //             self.current_selection = prev_item;
                    //             self.current_selection.set_checked(state, true);
                    //             event.consume();
                    //         }
                    //     }
                    // }
                }

                _ => {}
            }
        }
    }
}

pub struct Command {
    label: String,
    shortcut: String,
}

impl Command {
    pub fn new(label: &str, shortcut: &str) -> Self {
        Self {
            label: label.to_string(),
            shortcut: shortcut.to_string(),
        }
    }
}

impl BuildHandler for Command {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_flex_direction(state, FlexDirection::Row);

        let label = Label::new(&self.label).build(state, entity, |builder| builder.class("search"));
        let shortcut =
            Label::new(&self.shortcut).build(state, entity, |builder| builder.class("shortcut"));

        entity.set_element(state, "command");

        entity
    }
}

impl EventHandler for Command {}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchLabelEvent {
    Highlight(Vec<usize>),
}

pub struct SearchLabel {
    text: String,
    indices: Vec<usize>,
}

impl SearchLabel {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            indices: Default::default(),
        }
    }
}

impl BuildHandler for SearchLabel {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_text(state, &self.text);
        //entity.set_element(state, "command");
        entity.class(state, "search_label");

        entity
    }
}

impl EventHandler for SearchLabel {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(search_label_event) = event.message.downcast::<SearchLabelEvent>() {
            match search_label_event {
                SearchLabelEvent::Highlight(indices) => {
                    self.indices = indices.clone();

                    state.insert_event(Event::new(WindowEvent::Redraw));
                }
            }
        }
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
        // Skip invisible widgets
        if state.data.get_visibility(entity) == Visibility::Invisible {
            return;
        }

        let posx = state.data.get_posx(entity);
        let posy = state.data.get_posy(entity);
        let width = state.data.get_width(entity);
        let height = state.data.get_height(entity);

        let padding_left = match state
            .style
            .padding_left
            .get(entity)
            .unwrap_or(&Length::Auto)
        {
            Length::Pixels(val) => val,
            _ => &0.0,
        };

        let padding_right = match state
            .style
            .padding_right
            .get(entity)
            .unwrap_or(&Length::Auto)
        {
            Length::Pixels(val) => val,
            _ => &0.0,
        };

        let padding_top = match state.style.padding_top.get(entity).unwrap_or(&Length::Auto) {
            Length::Pixels(val) => val,
            _ => &0.0,
        };

        let padding_bottom = match state
            .style
            .padding_bottom
            .get(entity)
            .unwrap_or(&Length::Auto)
        {
            Length::Pixels(val) => val,
            _ => &0.0,
        };

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
            .unwrap_or(crate::Color::rgb(255, 255, 255));

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

        let parent_width = state.data.get_width(parent);
        let parent_height = state.data.get_height(parent);

        let border_radius_top_left = match state
            .style
            .border_radius_top_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_top_right = match state
            .style
            .border_radius_top_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_left = match state
            .style
            .border_radius_bottom_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_right = match state
            .style
            .border_radius_bottom_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

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

        let opacity = state.data.get_opacity(entity);

        let mut background_color: femtovg::Color = background_color.into();
        background_color.set_alphaf(background_color.a * opacity);

        let mut border_color: femtovg::Color = border_color.into();
        border_color.set_alphaf(border_color.a * opacity);

        let mut shadow_color: femtovg::Color = shadow_color.into();
        shadow_color.set_alphaf(shadow_color.a * opacity);

        canvas.save();

        // Apply Scissor
        let clip_entity = state.data.get_clip_widget(entity);

        let clip_posx = state.data.get_posx(clip_entity);
        let clip_posy = state.data.get_posy(clip_entity);
        let clip_width = state.data.get_width(clip_entity);
        let clip_height = state.data.get_height(clip_entity);

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

        // Stupid way, draw each glyph seperately
        if let Some(text) = state.style.text.get_mut(entity) {
            let font_id = match text.font.as_ref() {
                "Sans" => state.fonts.regular.unwrap(),
                "Icons" => state.fonts.icons.unwrap(),
                _ => state.fonts.regular.unwrap(),
            };

            let mut x = posx + (border_width / 2.0);
            let mut y = posy + (border_width / 2.0);

            let text_string = text.text.to_owned();

            let text_align = state
                .style
                .text_align
                .get(entity)
                .cloned()
                .unwrap_or_default();
            let text_justify = state
                .style
                .text_justify
                .get(entity)
                .cloned()
                .unwrap_or_default();

            let align = match text_justify {
                Justify::Start => {
                    x += padding_left;
                    Align::Left
                }
                Justify::Center => {
                    x += 0.5 * width;
                    Align::Center
                }
                Justify::End => {
                    x += width - padding_right;
                    Align::Right
                }
            };

            let baseline = match text_align {
                tuix::Align::Start => {
                    y += padding_top;
                    Baseline::Top
                }
                tuix::Align::Center => {
                    y += 0.5 * height;
                    Baseline::Middle
                }
                tuix::Align::End => {
                    y += height - padding_bottom;
                    Baseline::Bottom
                }
            };

            let mut font_color: femtovg::Color = font_color.into();
            font_color.set_alphaf(font_color.a * opacity);

            let font_size = state.style.font_size.get(entity).cloned().unwrap_or(16.0);

            let mut paint = Paint::color(font_color);
            paint.set_font_size(font_size);
            paint.set_font(&[font_id]);
            paint.set_text_align(align);
            paint.set_text_baseline(baseline);
            paint.set_anti_alias(false);

            let mut paint_highlight = Paint::color(femtovg::Color::rgb(0, 148, 252));
            paint_highlight.set_font_size(font_size);
            paint_highlight.set_font(&[font_id]);
            paint_highlight.set_text_align(align);
            paint_highlight.set_text_baseline(baseline);
            paint_highlight.set_anti_alias(false);

            let text_metrics = canvas.measure_text(x, y, &text_string, paint).unwrap();

            let mut temp = [0; 4];

            for ((index, c), glyph) in text
                .text
                .chars()
                .enumerate()
                .zip(text_metrics.glyphs.iter())
            {
                let px = glyph.x;
                if self.indices.contains(&index) {
                    canvas.fill_text(px, y, c.encode_utf8(&mut temp), paint_highlight);
                } else {
                    canvas.fill_text(px, y, c.encode_utf8(&mut temp), paint);
                }
            }

            //canvas.fill_text(x, y, &text_string, paint);
        }

        canvas.restore();
    }
}
