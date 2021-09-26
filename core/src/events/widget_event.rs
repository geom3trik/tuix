use crate::Entity;



#[derive(Debug, Clone, PartialEq)]
pub enum WidgetEvent {
    Drag(Entity),
    Drop(Entity),
    DragEnter(Entity),
    DragLeave(Entity),
}