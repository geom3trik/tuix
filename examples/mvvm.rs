
use std::collections::HashSet;

use tuix::*;
use tuix::style::themes::DEFAULT_THEME;

#[derive(Debug, Clone, PartialEq)]
enum BindEvent<T> {
    AddBinding(Entity, String),
    // This needs to be generic somehow
    NotifyChange(T),
}

// A trait so that a binding can be set up on an entity, using the property name
pub trait Bind {
    fn bind<T: 'static + Clone + std::fmt::Debug + PartialEq>(self, state: &mut State, property: &str) -> Self;
}

impl Bind for Entity {
    fn bind<T: 'static + Clone + std::fmt::Debug + PartialEq>(self, state: &mut State, property: &str) -> Self {
        state.insert_event(Event::new(BindEvent::<T>::AddBinding(self, property.to_owned())).propagate(Propagation::Up).target(self).origin(self));
        self
    }
}

// A property is a widget which contains the data
// It can be bound to (via an event) and can be modified (could be by event but in this case by casting and modifying direectly)
pub struct Prop<T> {
    observers: HashSet<Entity>,
    name: String,
    value: T,
}

impl<T> Prop<T> {
    pub fn new(name: String, init: T) -> Self {
        Self {
            observers: HashSet::new(),
            name,
            value: init,
        }
    }
}

impl<T: 'static + Clone + std::fmt::Debug + PartialEq> Widget for Prop<T> {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_display(state, Display::None)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(bind_event) = event.message.downcast() {
            match bind_event {
                BindEvent::<T>::AddBinding(id, prop) => {
                    println!("Add binding: {} {}", id, prop);
                    if *prop == self.name {
                        self.observers.insert(*id);
                    }
                }

                _=> {}
            }
        }
    }
}


// An observable property is an interface to a property which is stored within a widget
// This allows for the property to be 'get' and 'set' in a more natural way an abstracts the casting away
pub struct ObservableProperty<T> {
    name: String,
    property: Entity,
    temp: T,
}

impl<T: 'static + Clone + std::fmt::Debug + PartialEq> ObservableProperty<T> {
    pub fn new(name: &str, init: T) -> Self {
        Self {
            name: name.to_owned(),
            property: Entity::null(),
            temp: init,
        }
    }

    pub fn add(&mut self, state: &mut State, entity: Entity) {
        self.property = Prop::new(self.name.clone(), self.temp.clone()).build(state, entity, |builder| builder);
    }

    pub fn get(&self, state: &mut State) -> Option<T> {
        if let Some(prop) = state.query::<Prop<T>>(self.property) {
            Some(prop.value.clone())
        } else {
            None
        }
    }

    pub fn set(&self, state: &mut State, value: T) {
        println!("Set Property");
        // Temporary workaround for borrow error
        let mut observers = HashSet::new();

        if let Some(prop) = state.query::<Prop<T>>(self.property) {
            prop.value = value.clone();
            observers = prop.observers.clone();
            //prop.notify(state);
        }

        for observer in observers {
            println!("Observer: {}", observer);
            state.insert_event(Event::new(BindEvent::NotifyChange(value.clone())).target(observer));
        }
    }
}

// A store type which can be 'inherited' from and forwards requests to bind to its properties (children)
pub struct Store {

}

impl Store {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Widget for Store {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        //entity.set_display(state, Display::None)
        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        // Forward event to all children of the store
        state.insert_event(event.clone().propagate(Propagation::Fall).target(entity));
    }
}

// Everything above here is hidden boilerplate. Everything below is user code.



// No MODEL for this particular example

pub fn convert_text(input: String) -> String {
    input.to_uppercase()
}

// VIEWMODEL
pub struct Presenter {

    store: Store,

    // An observable property which acts as an interface for a widget which contains the actual data
    some_text: ObservableProperty<String>,

}

impl Presenter {
    pub fn new() -> Self {
        Self {
            store: Store::new(),
            some_text: ObservableProperty::new("some_text", "".to_string()),
        }
    }
}

impl Widget for Presenter {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        // Initialize this widget as a store
        self.store.on_build(state, entity);

        // Add the some_text property to the store
        self.some_text.add(state, entity);

        entity
    }

    // Receive app events to update the data in the store
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.store.on_event(state, entity, event);

        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::ConvertText(value) => {
                    let new_text = convert_text(value.clone());
                    self.some_text.set(state, new_text);
                }
            }
        }
    }
}

// VIEW

// Event sent for updating the some_text property (this stage could potentially be done with a 2-way binding)
#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    ConvertText(String),
}


#[derive(Debug, Default)]
pub struct TextConverterApp {
    // Temp
    label: Entity,
}

impl Widget for TextConverterApp {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, container: Entity) -> Self::Ret {
        Label::new("Text To Convert").build(state, container, |builder| 
            builder
                .set_width(Pixels(100.0))
                .set_height(Pixels(30.0))
        );
        Textbox::new("text")
            .on_submit(|textbox_data, state, textbox| {
                textbox.emit(state, AppEvent::ConvertText(textbox_data.text.clone()));
            })
            .build(state, container, |builder| 
                builder
                    .set_width(Pixels(100.0))
                    .set_height(Pixels(30.0))
            );
        self.label = Label::new("output")
            .build(state, container, |builder|
                builder
                    .set_width(Pixels(100.0))
                    .set_height(Pixels(30.0))
            ).bind::<String>(state, "some_text");
        container
    }

    // This would be part of the label rather than here
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(bind_event) = event.message.downcast() {
            match bind_event {
                BindEvent::<String>::NotifyChange(val) => {
                    self.label.set_text(state, &val.clone());
                }

                _=> {}
            }
        }
    }
}

fn main() {
    let window_description = WindowDescription::new();
    let app = Application::new(window_description, |state, window|{
        state.add_theme(DEFAULT_THEME);

        let data = Presenter::new().build(state, window, |builder| builder);

        TextConverterApp::default().build(state, data, |builder| 
            builder
                .set_child_space(Stretch(1.0))
        );
    });

    app.run();
}