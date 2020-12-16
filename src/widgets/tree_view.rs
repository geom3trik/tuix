use crate::entity::Entity;
use crate::State;
use crate::{BuildHandler, EventHandler};

use crate::widgets::{Button, Panel};

pub struct TreeView {}

impl TreeView {
    pub fn new() -> Self {
        TreeView {}
    }
}

impl BuildHandler for TreeView {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        let root = Panel::new("ROOT").build(state, entity, |builder| builder);
        //let level1 = Panel::new("Folder1").build(state, root, |builder| builder.class("level1"));
        //Button::with_label("File").build(state, level1, |builder| builder.set_flex_basis(30.0).class("item2"));
        //     Button::with_label("File").build(state, level1, |builder| builder.set_flex_basis(30.0).class("item2"));
        //     Button::with_label("File").build(state, level1, |builder| builder.set_flex_basis(30.0).class("item2"));
        //     Button::with_label("File").build(state, level1, |builder| builder.set_flex_basis(30.0).class("item2"));
        // let level2 = Panel::new("Folder2").build(state, root, |builder| builder.class("level1"));
        //     Button::with_label("File").build(state, level2, |builder| builder.set_flex_basis(30.0).class("item2"));
        //       Button::with_label("File").build(state, level2, |builder| builder.set_flex_basis(30.0).class("item2"));
        // let level3 = Panel::new("Folder").build(state, root, |builder| builder.class("level1"));
        // let level4 = Panel::new("Folder").build(state, level3, |builder| builder.class("level2"));
        //     Button::with_label("File").build(state, level4, |builder| builder.set_flex_basis(30.0).class("item2"));
        //     Button::with_label("File").build(state, level4, |builder| builder.set_flex_basis(30.0).class("item2"));
        //     Button::with_label("File").build(state, level4, |builder| builder.set_flex_basis(30.0).class("item2"));
        //     Button::with_label("File").build(state, level4, |builder| builder.set_flex_basis(30.0).class("item2"));
        //     Button::with_label("File").build(state, level4, |builder| builder.set_flex_basis(30.0).class("item2"));

        state.style.insert_element(entity, "tree_view");

        root
    }
}

impl EventHandler for TreeView {}
