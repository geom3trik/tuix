

use tuix::*;
use fnv::FnvHashMap;
use std::any::Any;
use std::rc::Rc;

use tuix::style::themes::DEFAULT_THEME;


#[derive(Debug, Clone)]
struct Contact {
    name: String,
    age: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    SetName(usize, String),
    SetAge(usize, i32),
    SetIndex(usize),
}

struct AppState {
    contacts: Vec<Contact>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            contacts: vec![Contact {name: "John Doe".to_string(), age: 24}, Contact {name: "Jane Doe".to_string(), age: 32}],
        }
    }
}

impl Node for AppState {
    fn get_data(&self) -> Option<&dyn Any> {
        Some(self)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SetName(index, new_name) => {
                    self.contacts[*index].name = new_name.to_owned();
                }

                _=> {}
            }
        }
    }
}

struct ContactFilter {
    contact: Contact,
    index: usize,
}

impl ContactFilter {
    pub fn new(index: usize) -> Self {
        Self {
            contact: Contact {
                name: "None".to_string(),
                age: 0,
            },

            index,
        }
    }
}

impl Node for ContactFilter {
    fn get_data(&self) -> Option<&dyn Any> {
        Some(self)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SetIndex(new_index) => {
                    self.index = *new_index;
                    state.insert_update(Event::new(AppEvent::SetIndex(*new_index)).origin(entity));
                }

                AppEvent::SetName(index, new_name) => {
                    // Alter the mutation event on its way up the graph
                    *index = self.index;
                }

                _=> {}
            }
        }
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, node: &dyn Any, nodes: &FnvHashMap<Entity, Box<dyn Node>>) {
        if let Some(app_state) = node.downcast_ref::<AppState>() {
            self.contact = app_state.contacts[self.index].clone();
        } 
    }
}

struct AppWidget {
    name_label: Entity,
    name_edit: Entity,
}

impl AppWidget {
    pub fn new() -> Self {
        Self {
            name_label: Entity::null(),
            name_edit: Entity::null(),
        }
    }
}

impl Widget for AppWidget {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        entity.set_layout_type(state, LayoutType::Grid);
        entity.set_grid_rows(state, vec![Pixels(30.0), Stretch(1.0), Pixels(30.0)]);
        entity.set_grid_cols(state, vec![Stretch(1.0), Stretch(1.0), Stretch(1.0)]);

        let app_entity = entity;
        self.name_edit = Textbox::new("Name")
            .on_submit(move |textbox, state, entity| {
                let new_name = textbox.text.clone();
                state.insert_update(Event::new(AppEvent::SetName(2, new_name)).origin(app_entity));
            })
            .build(state, entity, |builder| 
                builder
                    .set_row(0)
                    .set_col(0)
                    .set_col_span(3)
            );
        
        self.name_label = Textbox::new("Name")
            .on_submit(move |textbox, state, entity| {
                let new_name = textbox.text.clone();
                state.insert_update(Event::new(AppEvent::SetName(2, new_name)).origin(app_entity));
            })
            .build(state, entity, |builder| 
                builder
                    .set_row(1)
                    .set_col(0)
                    .set_col_span(3)
            );

        Spinbox::new(0)
            .with_min(0)
            .with_max(1)
            .on_change(move |spinbox, state, entity|{
                let new_index = spinbox.value;
                state.insert_update(Event::new(AppEvent::SetIndex(new_index)).origin(app_entity));
            })
            .build(state, entity, |builder| 
                builder
                    .set_row(2)
                    .set_col(0)
                    .set_col_span(3)
                    //.set_height(Pixels(30.0))
            );

        entity
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, node: &dyn Any, nodes: &FnvHashMap<Entity, Box<dyn Node>>) {
        if let Some(filter) = node.downcast_ref::<ContactFilter>() {
            let new_name = filter.contact.name.clone();
            self.name_label.set_text(state, &new_name);
            self.name_edit.set_text(state, &new_name);
        }
    }
}

fn main() {
    let window_description = WindowDescription::new();
    let app = Application::new(window_description, |state, window|{

        state.add_theme(DEFAULT_THEME);

        let app_state = AppState::new().build(state, window);
        let filter1 = ContactFilter::new(0).build(state, app_state);
        let filter2 = ContactFilter::new(1).build(state, app_state);
        AppWidget::new().build(state, window, |builder| builder).bind(state, filter1);
        //AppWidget::new().build(state, window, |builder| builder).bind(state, filter2);
    });
    app.run();

}