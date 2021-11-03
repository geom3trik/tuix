use std::cell::RefCell;
use std::rc::Rc;

use tuix::widgets::*;
use tuix::*;

fn main() {
    let button = Rc::new(RefCell::new(None));
    let button_ = button.clone();
    let app = Application::new(WindowDescription::new(), move |state, window| {
        *button.borrow_mut() = Some(Button::new().build(state, window, |builder| builder));
    })
    .should_poll()
    .on_idle(move |state| {
        println!("Tree");
        // for entity in state.tree.into_iter() {
        //     println!("Entity: {} Parent: {:?} posx: {} posy: {} width: {} height: {}", entity, entity.parent(&state.tree), state.data.get_posx(entity), state.data.get_posy(entity), state.data.get_width(entity), state.data.get_height(entity));
        // }
        // if let Some(button) = button_.borrow_mut().take() {
        //     state.remove(button);
        // }
    });
    app.run();
}