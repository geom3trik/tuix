use crate::{AsEntity, BindEvent, PropSet, State};



/// Trait which provides convenience methods on entities for binding
/// 
pub trait BindExt 
where Self: Sized + AsEntity,
{
    fn update(self, state: &mut State) -> Self {
        self.entity().emit(state, BindEvent::Update);

        self
    } 
}

impl<T: AsEntity> BindExt for T {
    
} 