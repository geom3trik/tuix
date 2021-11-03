use std::cell::RefCell;
use std::rc::Rc;

use tuix::widgets::*;
use tuix::*;

fn main() {

    let app = Application::new(WindowDescription::new(), move |state, window| {
        
    })
    .should_poll()
    .on_idle(move |state| {

    });
    app.run();
}