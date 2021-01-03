
pub enum ListboxEvent {
    
}

pub struct ListboxItem {

}

impl ListboxItem {
    pub fn new() -> Self {
        ListboxItem {}
    }
}

impl BuildHandler for ListboxItem {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }
}

impl EventHandler for ListboxItem {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(listbox_event)
    }
}

pub struct Listbox {

}

impl Listbox {
    pub fn new() -> Self {
        Listbox {

        }
    }
}

impl BuildHandler for Listbox {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }
}

impl EventHandler for Listbox {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {

    }
}