use crate::Entity;




pub enum WidgetEvent {
    ChildAdded(Entity),
    ChildRemoved(Entity),
}