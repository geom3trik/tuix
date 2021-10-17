//! # Building Widgets
//!
//! Any custom widget must implement the [Widget] trait. This allows for widgets to be built into the 
//! application with the `build` method, for example:
//! ```
//! CustomWidget::new().build(state, parent, |builder| builder);
//! ```
//! The first parameter to `build` is a mutable reference to [State]. The second parameter is the [Entity] is of the parent,
//! and the third parameter is a closure which provides a [Builder], allowing properties to more easily be set on the widget
//! at build time. For example:
//! ```
//! CustomWidget::new().build(state, parent, |builder| {
//!     builder
//!         .set_left(Pixels(100.0))
//!         .et_width(Pixels(200.0))
//!         .set_height(Pixels(30.0))
//!         .set_background_color(Color::red())
//! });
//! ```
//! 
//! The [Widget] trait has two associated types: `Ret` and `Data`. The `Ret` associated type is used to describe whether the 
//! widget will return an entity or a tuple of entities when built. The `build` method, shown above, returns this `Ret` type.
//! Usually this will be a single entity to describe the entire widget. However, sometimes for widgets which are made of several sub-widgets,
//! each of which has its own [Entity] id, it can be useful to return these sub-widget ids as well.
//! The `Ret` type implements `AsEntity`, which is implemented for an [Entity] and tuples of entities (up to 5). The [AsEntity] trait allows 
//! properties to be set on tuples of entities by setting those properties on the first entity in the tuple. Therefore, the first entity in the
//! tuple should always be the root entity of the widget, provided by the `on_build` method.
//!
//! The `on_build` method in the [Widget] trait is the only method which must be implemented. The method is called once when a widget is
//! built into the application tree and is where sub-widgets should be added. For example, a widget made up of two buttons could look like this:
//! ```
//! impl Widget for CustomWidget {
//!     type Ret = Entity;
//!     type Data = ();
//!     
//!     fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
//!         Button::new().build(state, entity, |builder| builder);
//!         Button::new().build(state, entity, |builder| builder);
//!         // Return the root entity of the widget
//!         entity
//!     }
//! }
//! ```
mod widget;
pub use widget::Widget;

mod builder;
pub use builder::Builder;

mod widget_event;
pub use widget_event::WidgetEvent;