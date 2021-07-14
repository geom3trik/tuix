use std::any::Any;
use std::collections::HashMap;

use tuix::*;
use tuix::style::themes::DEFAULT_THEME;

// Creating an atom puts the value in a store and returns a constant type


// Store lives at the root of the tree and manages atom
// Need to register an atom and somehow return a unique const type id
// 

pub trait DynamicAtom : Any {}

impl<T: 'static> DynamicAtom for Atom<T> {}


enum RootEvent {
    RegisterAtom(String),
}
pub struct Root {
    atoms: HashMap<String, Box<dyn DynamicAtom>>,
}

impl Widget for Root {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        
    }
}

pub fn use_state<T: std::fmt::Debug + Default>(state: &mut State, atom: Atom<T>) -> (impl Fn(&mut State) -> T, impl Fn(&mut State, T)) {
    return (|state: &mut State|{ return T::default()}, |state: &mut State, value: T|{
        println!("Set value to: {:?}", value);
    });
}


const darkModeAtom: Atom<bool> = Atom::new("darkmode", false);

fn main() {
    let app = Application::new(WindowDescription::new(), |state, window|{
        state.add_theme(DEFAULT_THEME);
         //darkModeAtom = Atom::new(state, "darkmode", false);

        let (dark_mode, set_dark_mode) = use_state(state, darkModeAtom);
        
        Checkbox::new(false)
            //.bind(state, darkModeAtom)
            .on_checked( move |data, state, checkbox| {
                set_dark_mode(state, true);
                println!("Value is: {}", dark_mode);
                //lens.set_property(state, |value|)
                 
            })
            .build(state, window, |builder| builder);


    });

    app.run();
}