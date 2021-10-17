

use std::collections::HashMap;

use tuix::*;
use tuix::widgets::*;

use crate::{AppData, AppEvent};

pub struct TreeView {
    scroll: Entity,
    container: Entity,
    pub levels: HashMap<Entity, (u32, Entity)>,
}

impl TreeView {
    pub fn new() -> Self {
        Self {
            scroll: Entity::null(),
            container: Entity::null(),
            levels: HashMap::new(),
        }
    }
}

impl Widget for TreeView {
    type Ret = Entity;
    type Data = AppData;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.scroll = ScrollContainer::new()
            .build(state, entity, |builder| 
                builder 
            );
        
        self.container = Element::new().build(state, self.scroll, |builder| builder.set_background_color(Color::red()));

        entity
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        
        if !data.canvas.is_null() {
            println!("BRANCH");
            state.remove(self.container);
            self.levels.clear();
            self.container = Element::new().build(state, self.scroll, |builder| builder.set_height(Auto));
            
            let root = Panel::new("CANVAS").build(state, self.container, |builder| builder);
            self.levels.insert(data.canvas, (0, root.0));
            println!("{}", data.canvas);

            for (index, child) in data.canvas.branch_iter(&state.tree.clone()).enumerate() {
                if index == 0 {
                    continue;
                }

                let parent = child.parent(&state.tree).unwrap();
                println!("{}", parent);

                println!("Entity: {}", child);
                if state.tree.has_children(child) {
                    
                    if let Some(level) = self.levels.get(&parent) {
                        let (panel, header) = Panel::new("panel").build(state, level.1, |builder| builder);
                        header.set_left(state, Pixels((level.0 + 1) as f32 * 15.0));
                        self.levels.insert(child, (level.0 + 1, panel));
                    } 
                } else {
                    if let Some(level) = self.levels.get(&parent) {
                        let label = CheckButton::with_label("label")
                            .on_checked(move |data, state, checkbutton|{
                                checkbutton.emit(state, AppEvent::SelectWidget(child));
                            })
                            .build(state, level.1, |builder| builder.set_height(Pixels(30.0)));
                        label.set_left(state, Pixels((level.0 + 1) as f32 * 15.0));
                        self.levels.insert(child, (level.0 + 1, label));
                    }
                }
                
            }            
        }

    }
}