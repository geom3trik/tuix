
use tuix::*;
use tuix::widgets::*;

const ICON_DOWN_OPEN_BIG: &str = "\u{e75c}";

fn main() {
    let window_description = WindowDescription::new().with_title("TreeView");
    let app = Application::new(window_description, |state, window| {
        
        let mut tree_data = TreeData {
            name: "root".to_string(),
            children: vec![
                TreeData {
                    name: "child item 1".to_string(),
                    children: vec![
                        TreeData {
                            name: "child item 1.1".to_string(),
                            children: vec![],
                        },

                        TreeData {
                            name: "child item 1.2".to_string(),
                            children: vec![],
                        },
                    ]
                },

                TreeData {
                    name: "child item 2".to_string(),
                    children: vec![],
                }
            ],
        }.build(state, window);
        
        let treeview = TreeView::with_template(|item| Label::new("default"))
            .bind_ref(TreeData::root)
            .build(state, tree_data, |builder| builder);
    });

    app.run();
}

#[derive(Clone, Lens, Debug)]
pub struct TreeData {
    pub name: String,
    pub children: Vec<TreeData>,
}

impl TreeData {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            children: Vec::new(),
        }
    }
}

// Ideally would like to use an iterator over references rather than cloning the data,
// but this requires GATs to get the lifetimes to work
// impl<'a> IntoIterator for &'a TreeData {
//     type Item = &'a TreeData;
//     type IntoIter = std::slice::Iter<'a, TreeData>;
    
//     fn into_iter(self) -> Self::IntoIter {
//         self.children.iter()
//     }
// }

impl<'a> IntoIterator for TreeData {
    type Item = TreeData;
    type IntoIter = std::vec::IntoIter<TreeData>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter()
    }
}

impl Model for TreeData {
    
}

pub struct TreeView<T, W> {
    template: Option<Box<dyn Fn(&T) -> W>>
}

impl<'a, T: Node, W: Widget> TreeView<T, W> 
where T: Clone + IntoIterator<Item = T> + std::fmt::Debug
{
    pub fn new() -> Self {
        Self {
            template: None,
        }
    }

    pub fn with_template<F>(template: F) -> Self 
    where F: 'static + Fn(&T) -> W,
    {
        Self {
            template: Some(Box::new(template)),
        }
    }

    fn build_tree(&mut self, state: &mut State, entity: Entity, data: &T, level: u32) {
        for item in data.clone().into_iter() {
            let tree_item = TreeViewItem::new(Label::new("Test").bind(TreeData::name, |name| {
                println!("Convert: {:?}", name);
                name.to_owned()
            })).build(state, entity, |builder| 
                builder
                    .set_height(Pixels(30.0))
                    .set_left(Pixels(level as f32 * 15.0))
            );

            self.build_tree(state, tree_item, &item, level + 1);
        }
    }

    fn update_tree(&mut self, state: &mut State, entity: Entity, data: &T) {
        for (item, child) in data.clone().into_iter().zip(entity.child_iter(&state.tree.clone())) {
            println!("Update with: {:?} {}", item, child);
            if let Some(mut event_handler) = state.event_handlers.remove(&child) {
                event_handler.on_update_(state, child, &item);

                state.event_handlers.insert(child, event_handler);
            }

            if let Some(container) = state.tree.get_child(child, 1) {
                self.update_tree(state, container, &item);
            }

        }
    }
}

impl<T: Node, W: Widget> Widget for TreeView<T, W> 
where T: Clone + IntoIterator<Item = T> + std::fmt::Debug
{
    type Ret = Entity;
    type Data = T;
    
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        // let list = List::new().build(state, entity, |builder| 
        //     builder
                
        // );

        entity
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        
        println!("Update: {:?}", data);

        for child in entity.child_iter(&state.tree.clone()) {
            state.remove(child);
        }

        let tree_item = TreeViewItem::new(Label::new("Root").bind_ref(TreeData::name)).build(state, entity, |builder| 
            builder
                .set_height(Pixels(30.0))
        );

        self.build_tree(state, tree_item, data, 1);

        self.update_tree(state, tree_item, data);
        
        // for item in data.clone().into_iter() {
        //     println!("Item: {:?}", item);
        //     TreeViewItem::new(Label::new("Test")).build(state, entity, |builder| 
        //         builder
        //             .set_height(Pixels(30.0))
        //     );

        //     for child in item.into_iter() {
        //         TreeViewItem::new(Label::new("Test")).build(state, entity, |builder| 
        //             builder
        //                 .set_height(Pixels(30.0))
        //                 .set_left(Pixels(15.0))
        //         );
        //     }
        // }
    }
}

pub struct TreeViewItem<W> {
    widget: Option<W>,
    header: Entity,
    item: Entity,
}

impl<W> TreeViewItem<W> {
    pub fn new(widget: W) -> Self {
        Self {
            widget: Some(widget),
            header: Entity::null(),
            item: Entity::null(),
        }
    }
}

impl<W: Widget> Widget for TreeViewItem<W> {
    type Ret = Entity;
    type Data = <W as Widget>::Data;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.header = Element::new().build(state, entity, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_layout_type(LayoutType::Row)
        );

        Element::new().build(state, self.header, |builder| 
            builder
                .set_width(Pixels(30.0))
                .set_text(ICON_DOWN_OPEN_BIG)
                .set_font("icon")
                .set_child_space(Stretch(1.0))
        );

        // Label::new("TreeViewItem").build(state, header, |builder| 
        //     builder
        //         .set_child_space(Stretch(1.0))
        //         .set_child_left(Pixels(0.0))
        // );
        if let Some(widget) = self.widget.take() {
            self.item = widget.build(state, self.header, |builder| builder).entity();
        }

        let container = Element::new().build(state, entity, |builder| 
            builder
                .set_height(Auto)
        );



        container
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        if let Some(mut event_handler) = state.event_handlers.remove(&self.item) {
            event_handler.on_update_(state, self.item, data);

            state.event_handlers.insert(self.item, event_handler);
        }
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        // Intercept bind event so that the tree view widget has to update its items
        // TODO - be more selective about this. It should only apply to treeview items
        if let Some(bind_event) = event.message.downcast() {
            match bind_event {
                BindEvent::Bind(target, _) => {
                    
                    if target.is_child_of(&state.tree, self.header) {
                        if *target != entity {
                            event.consume();
                        }                        
                    }                        
                    

                }

                _=> {}
            }
        }
    }
}