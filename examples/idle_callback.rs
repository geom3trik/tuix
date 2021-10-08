
use tuix::*;
use tuix::widgets::*;

fn main() {
    let window_description = WindowDescription::new();
    let app = Application::new(window_description, |state, window|{
        
    })
    .on_idle(|state| {
        println!("Do something");
    })
    .run();
}