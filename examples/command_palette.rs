extern crate tuix;

use tuix::*;

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

fn main() {
    let app = Application::new(|win_desc, state, window| {

        state.insert_stylesheet("examples/themes/cmd_palette_theme.css");

        CommandPalette::new().build(state, window, |builder| 
            builder
            .set_box_shadow_blur(Length::Pixels(10.0))
            .set_box_shadow_v_offset(Length::Pixels(5.0))
            .set_box_shadow_color(Color::rgba(0, 0, 0, 128))
        );

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
    current_selection: Entity,
    matcher: SkimMatcherV2,
    commands: Vec<(String, String)>,
}

impl CommandPalette {
    pub fn new() -> Self {

        let mut commands: Vec<(String, String)> = Default::default();
        commands.push(("Toggle Full Screen Mode".to_string(),"Shortcut".to_string()));
        commands.push(("Toggle Second Window".to_string(),"Shortcut".to_string()));
        commands.push(("Toggle Session/Arrangement View".to_string(),"Shortcut".to_string()));
        commands.push(("Toggle Device/Clip View".to_string(),"Shortcut".to_string()));
        commands.push(("Hide/Show Detail View".to_string(),"Shortcut".to_string()));
        commands.push(("Toggle Hot-Swap Mode".to_string(),"Shortcut".to_string()));
        commands.push(("Toggle Drum Rack/last-selected Pad".to_string(),"Shortcut".to_string()));
        commands.push(("Hide/Show Info View".to_string(),"Shortcut".to_string()));
        commands.push(("Hide/Show Video Window".to_string(),"Shortcut".to_string()));
        commands.push(("Hide/Show Browser".to_string(),"Shortcut".to_string()));
        commands.push(("Hide/Show Overview".to_string(),"Shortcut".to_string()));
        commands.push(("Hide/Show In/Out".to_string(),"Shortcut".to_string()));
        commands.push(("Hide/Show Sends".to_string(),"Shortcut".to_string()));
        commands.push(("Hide/Show Mixer".to_string(),"Shortcut".to_string()));
        commands.push(("Open the Preferences".to_string(),"Shortcut".to_string()));
        commands.push(("Close Window/Dialog".to_string(),"Shortcut".to_string()));

        Self {
            search_box: Entity::null(),
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
            .on_change(move |val| Event::new(SearchboxEvent::Changed(val.to_string())).target(entity))
            .build(state, entity, |builder| builder.class("search"));

        self.current_selection = self.search_box;

        Label::new("").build(state, entity, |builder| builder.class("command"));
        Label::new("").build(state, entity, |builder| builder.class("command"));
        Label::new("").build(state, entity, |builder| builder.class("command"));
        Label::new("").build(state, entity, |builder| builder.class("command"));
        Label::new("").build(state, entity, |builder| builder.class("command"));
        Label::new("").build(state, entity, |builder| builder.class("command"));
        Label::new("").build(state, entity, |builder| builder.class("command"));
        Label::new("").build(state, entity, |builder| builder.class("command"));
        Label::new("").build(state, entity, |builder| builder.class("command"));
        Label::new("").build(state, entity, |builder| builder.class("command"));
        Label::new("").build(state, entity, |builder| builder.class("command"));
        Label::new("").build(state, entity, |builder| builder.class("command"));
        Label::new("").build(state, entity, |builder| builder.class("command"));
        Label::new("").build(state, entity, |builder| builder.class("command"));
        Label::new("").build(state, entity, |builder| builder.class("command"));
        Label::new("").build(state, entity, |builder| builder.class("command"));

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
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {

        if let Some(searchbox_event) = event.message.downcast::<SearchboxEvent>() {
            match searchbox_event {
                SearchboxEvent::Changed(val) => {
                    //println!("Value: {}", val);
                    let matcher =  &self.matcher;
                    self.commands.sort_by_cached_key(|(cmd, _)| matcher.fuzzy_match(cmd, val).unwrap_or(0));
                    
                    println!("");

                    for (index,cmd) in self.commands.iter().rev().enumerate() {
                        
                        let score = matcher.fuzzy_match(&cmd.0, val).unwrap_or(0);

                        if let Some(command_widget) = state.hierarchy.get_child(entity, index + 1) {
                            if score > 0 {
                                command_widget.set_text(state, &cmd.0);
                            } else {
                                command_widget.set_text(state, "");
                            }
                            
                        }
                        
                        state.insert_event(Event::new(WindowEvent::Redraw));
                    }

                    println!("");
                    // if let Some((score, indices)) = self.matcher.fuzzy_indices("some kind of command", val) {
                    //     println!("Score: {}  Indices: {:?}", score, indices);
                    // }
                    
                }
            }
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::KeyDown(code, key) => {
                    if *key == Some(Key::ArrowDown) {
                        if let Some(next_item) = state.hierarchy.get_next_sibling(self.current_selection) {
                            self.current_selection.set_checked(state, false);
                            self.current_selection = next_item;
                            self.current_selection.set_checked(state, true);
                            return true;
                        }
                    }

                    if *key == Some(Key::ArrowUp) {
                        if let Some(prev_item) = state.hierarchy.get_prev_sibling(self.current_selection) {
                            if prev_item != self.search_box {
                                self.current_selection.set_checked(state, false);
                                self.current_selection = prev_item;
                                self.current_selection.set_checked(state, true);   
                                return true;                             
                            }
                        }
                    }
                }

                _=> {}
            }
        }

        false
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
        let shortcut = Label::new(&self.shortcut).build(state, entity, |builder| builder.class("shortcut"));

        entity.set_element(state, "command");

        entity
    }
}

impl EventHandler for Command {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        false
    }
}