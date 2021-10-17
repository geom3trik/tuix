//! # Data Binding
//!
//! Binding provides a way to add reactivity to a tuix application. Rather than sending events back and forth between widgets
//! to update local widget data, widgets can instead `bind` to application data.
//!
//! # Example
//! Fist we declare the data for our application. The [Lens] trait has been derived for the data, which allows us to bind to fields of the struct:
//! ```
//! #[derive(Default, Lens)]
//! sturct AppData {
//!     some_data: bool,
//! }
//! ```
//! Next we'll declare some events which will be sent by widgets to modify the app data. Data binding in tuix is one-way, events are sent up the tree
//! to the app data to muatate it and updated values are sent back down the tree to observer widgets and their children.
//! ```
//! struct AppEvent {
//!     SetTrue,
//!     SetFalse,   
//! }
//! ```
//! Next we implement the [Model] trait on our app data, which allows us to modify the data in response to an [Event]:
//! ```
//! impl Model for AppData {
//!     fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
//!         if let Some(app_event) = event.message.downcast() {
//!             match app_event {
//!                 AppEvent::SetTrue => {
//!                     self.some_data = true;
//!                        entity.update(state);
//!                 }
//!
//!                 AppEvent::SetFalse => {
//!                     self.some_data = false;
//!                        entity.update(state);
//!                 }
//!             }   
//!         }
//!     }
//! }
//! ```
//! Note that the calls to `entity.update(state)` are required to inform tuix that data as been modified.
//! This trait also allows us to build the data into the [Tree]:
//! ```
//! fn main() {
//!     let app = Application::new(WindowDescription::new(), |state, window|{
//!         let app_data = AppData::default().build(state, window);
//!     })   
//! }
//! ```
//! Now we can bind a widget to part of the application data with the `bind` method:
//! ```
//! fn main() {
//!     let app = Application::new(WindowDescription::new(), |state, window|{
//!         let app_data = AppData::default().build(state, window);
//!
//!         Label::new("")
//!             .bind(AppData::some_data, |value|{
//!                 if *value {
//!                     "TRUE".to_string()
//!                 } else {
//!                     "FALSE".to_string()
//!                 }
//!             })
//!             .build(state, app_data, |builder| builder);
//!             
//!     })
//! }
//! ```
//! The first parameter to `bind` is a lens on the application data (see [Lens]), allowing us to bind to some field of the root data. 
//! To bind to the whole of the application data, use `AppData::root`.
//! The second parameter to `bind` is a conversion closure which can be used to convert the data type to the type expected by the widget. 
//! In the code above the `Label` expects a `String` type but our data is `bool`, so we apply a conversion.
//! Now when the data is modified by another widget, the label will update, for example:
//! ```
//! fn main() {
//!     let app = Application::new(WindowDescription::new(), |state, window|{
//!         let app_data = AppData::default().build(state, window);
//!
//!         Label::new("")
//!             .bind(AppData::some_data, |value|{
//!                 if *value {
//!                     "TRUE".to_string()
//!                 } else {
//!                     "FALSE".to_string()
//!                 }
//!             })
//!             .build(state, app_data, |builder| builder);
//!         
//!         Checkbox::new(false)
//!             .on_checked(|_,state,checkbox| checkbox.emit(state, AppEvent::SetTrue))
//!             .on_unchecked(|_,state,checkbox| checkbox.emit(state, AppEvent::SetFalse))
//!             .build(state, app_data, |builder| builder);
//!             
//!     })
//! }
//! ```
//! Note, the checkbox does not need to be bound to the data to send an event to it. By default events will propagate up the tree.
//!
//! # Binding Custom Widgets
//! For custom widgets the type which the widget can bind to must be specified by the `Data` associated type, for example:
//! ```
//! impl Widget for CustomWidget {
//!     type Data = f32;
//!     ...
//! }    
//! ```
//! The `on_update` method of the [Widget] trait is then used to react to changes in the bound data:
//! ```
//! fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
//!     // Do something with the updated value    
//! }
//! ```
mod node;
mod lens;
mod bind_ext;
pub use bind_ext::BindExt;
use std::any::TypeId;

pub use node::*;
pub use lens::{Lens, LensExt};

use crate::{TreeExt};
use crate::{State, Entity, Event, Widget, Propagation};

use crate::Canvas;

mod model;
pub use model::Model;

mod store;
pub use store::Store;

/// Events for data binding.
#[derive(Debug, Clone, PartialEq)]
pub enum BindEvent {
    /// Sent by a widget when bound to some data.
    Bind(Entity, TypeId),
    /// Sent manually when data in [Model] is updated. 
    Update,
}


/// A [LensWrap] without a converter, allowing data to be passed from [Model] to the bound [Widget] as a reference.
pub struct LensWrapRef<L: Lens, W: Widget> {
    widget: W,
    lens: L,
}

impl<L: Lens, W: Widget> LensWrapRef<L,W> {
    pub fn new(widget: W, lens: L) -> Self {
        Self {
            widget,
            lens,
        }
    }
}

impl<L: 'static + Lens, W> Widget for LensWrapRef<L,W>
where W: Widget<Data = <L as Lens>::Target>,
{
    type Ret = <W as Widget>::Ret;
    type Data = <L as Lens>::Source;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        let type_id = TypeId::of::<Self::Data>();
        state.insert_event(Event::new(BindEvent::Bind(entity, type_id)).target(entity).propagate(Propagation::Up));

        self.widget.on_build(state, entity)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {

        self.widget.on_event(state, entity, event)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        
        // Apply the lens
        let value = self.lens.view(data);

        // // Update children
        // for (index, child) in entity.child_iter(&state.tree.clone()).enumerate() {
            
        //     if let Some(mut event_handler) = state.event_handlers.remove(&child) {
        //         event_handler.on_update(state, child, &value);

        //         state.event_handlers.insert(child, event_handler);
        //     }
        // }

        // Update children
        for (_index, child) in entity.child_iter(&state.tree.clone()).enumerate() {

            if let Some(mut event_handler) = state.event_handlers.remove(&child) {
                event_handler.on_update_(state, child, value);

                state.event_handlers.insert(child, event_handler);
            }
        }

        // Update the underlying widget with the lensed and converted data
        self.widget.on_update(state, entity, &value);

        // // Call the on_update callback
        // if let Some(callback) = self.on_update.take() {
        //     (callback)(&mut self.widget, state, entity);

        //     self.on_update = Some(callback);
        // }
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas) {
        self.widget.on_draw(state, entity, canvas)
    }
}


/// A wrapper on a widget which adds the setup for binding as well as the conversion of data + lensing
pub struct LensWrap<L: Lens, W: Widget> {

    widget: W,
    lens: L,
    converter: Box<dyn Fn(&<L as Lens>::Target) -> <W as Widget>::Data>,
    on_update: Option<Box<dyn Fn(&mut W, &mut State, Entity)>>,
}

impl<L: Lens, W: Widget> LensWrap<L,W> {
    pub fn new<F>(widget: W, lens: L, converter: F) -> Self 
    where F: 'static + Fn(&<L as Lens>::Target) -> <W as Widget>::Data
    {
        Self {

            widget,
            lens,
            converter: Box::new(converter),
            on_update: None,
        }
    }

    pub fn on_update<F>(mut self, callback: F) -> Self 
    where F: 'static + Fn(&mut W, &mut State, Entity)
    {
        self.on_update = Some(Box::new(callback));
        self
    }
}

impl<L: 'static + Lens, W: Widget> Widget for LensWrap<L,W> {
    type Ret = <W as Widget>::Ret;
    type Data = <L as Lens>::Source;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        let type_id = TypeId::of::<Self::Data>();
        state.insert_event(Event::new(BindEvent::Bind(entity, type_id)).target(entity).propagate(Propagation::Up));

        self.widget.on_build(state, entity)
        
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.widget.on_event(state, entity, event)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        //println!("LensWrap On Update");
        // Apply the lens
        let view_data = self.lens.view(data);
        // Apply the converter function
        let value = (self.converter)(&view_data);

        //print_type_of(&value);

        // Update children
        for (_index, child) in entity.child_iter(&state.tree.clone()).enumerate() {

            if let Some(mut event_handler) = state.event_handlers.remove(&child) {
                event_handler.on_update_(state, child, &value);

                state.event_handlers.insert(child, event_handler);
            }
        }

        // Update the underlying widget with the lensed and converted data
        self.widget.on_update(state, entity, &value);

        // Call the on_update callback
        if let Some(callback) = self.on_update.take() {
            (callback)(&mut self.widget, state, entity);

            self.on_update = Some(callback);
        }
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas) {
        self.widget.on_draw(state, entity, canvas)
    }
}

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
