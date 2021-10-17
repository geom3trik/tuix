
use std::array::IntoIter;
use std::marker::PhantomData;

use tuix::*;
use tuix::widgets::*;

const ICON_DOWN_OPEN_BIG: &str = "\u{e75c}";


fn main() {
    let window_description = WindowDescription::new().with_title("TreeView");
    let app = Application::new(window_description, |state, window| {
    

        // let mut tree_data = TreeData {
        //     name: "root".to_string(),
        //     children: vec![
        //         TreeData {
        //             name: "child item 1".to_string(),
        //             children: vec![
        //                 TreeData {
        //                     name: "child item 1.1".to_string(),
        //                     children: vec![],
        //                 },

        //                 TreeData {
        //                     name: "child item 1.2".to_string(),
        //                     children: vec![],
        //                 },
        //             ]
        //         },

        //         TreeData {
        //             name: "child item 2".to_string(),
        //             children: vec![],
        //         }
        //     ],
        // }.build(state, window);
        
        let tree_view = TreeView::new().build(state, window, |builder| builder);
        let root = TreeViewItem::with_label("root").build(state, tree_view, |builder| builder);
        let child_one = TreeViewItem::with_label("child item 1").build(state, root, |builder| builder);
        let child_one_one = TreeViewItem::with_label("child item 1.1").build(state, child_one, |builder| builder);
        let child_one_two = TreeViewItem::with_label("child item 1.2").build(state, child_one, |builder| builder);
        let child_two = TreeViewItem::with_label("child item 2").build(state, root, |builder| builder);
        
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


#[derive(Debug, Clone, Copy)]
struct NullType;

impl Iterator for NullType {
    type Item = NullType;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// impl IntoIterator for NullType {
//     type Item = NullType;
//     type IntoIter = NullType;

//     fn into_iter(self) -> Self::IntoIter {
//         NullType
//     }
// }

pub struct TreeView<T = NullType> {

    scroll: Entity,

    template: Option<Box<dyn Fn(&mut State, Entity) -> Entity>>,
    p: PhantomData<T>,
}

impl TreeView<NullType> {
    pub fn new() -> Self {
        Self {
            scroll: Entity::null(),
            template: None,
            p: PhantomData::default(),
        }
    }
}

impl<T: Node> TreeView<T> 
where T: Clone + IntoIterator<Item = T> + std::fmt::Debug
{
    pub fn with_template2<F>(template2: F) -> Self 
    where F: 'static + Fn(&mut State, Entity) -> Entity {
        Self {
            scroll: Entity::null(),
            template: Some(Box::new(template2)),
            p: PhantomData::default(),
        }
    }

    fn build_tree(&mut self, state: &mut State, _entity: Entity, data: &T, level: u32) {
        for item in data.clone().into_iter() {
            let tree_item = TreeViewItem::with_header_template(|state, parent|{
                Label::new("Test")
                    .bind_ref(TreeData::name)
                    .build(state, parent, |builder| 
                        builder
                            .set_child_space(Stretch(1.0))
                            .set_child_left(Pixels(0.0))
                    )
            }).build(state, self.scroll, |builder| 
                builder
                    .set_height(Pixels(30.0))
                    .set_left(Pixels(level as f32 * 15.0))
            );

            self.build_tree(state, tree_item, &item, level + 1);
        }
    }

    fn update_tree(&mut self, state: &mut State, _entity: Entity, data: &T) {
        for (item, child) in data.clone().into_iter().zip(self.scroll.child_iter(&state.tree.clone())) {
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

impl<T: Node> Widget for TreeView<T> 
where T: Clone + IntoIterator<Item = T> + std::fmt::Debug
{
    type Ret = Entity;
    type Data = T;
    
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        // let list = List::new().build(state, entity, |builder| 
        //     builder
                
        // );

        let scroll = ScrollContainer::new().build(state, entity, |builder| builder);

        scroll
    }

    fn on_update(&mut self, state: &mut State, _entity: Entity, data: &Self::Data) {
        
        println!("Update: {:?}", data);

        for child in self.scroll.child_iter(&state.tree.clone()) {
            state.remove(child);
        }

        let tree_item = TreeViewItem::with_header_template(|state, parent| {
            Label::new("Root")
                .bind_ref(TreeData::name)
                .build(state, parent, |builder| 
                    builder
                        .set_child_space(Stretch(1.0))
                        .set_child_left(Pixels(0.0))
                )
        }).build(state, self.scroll, |builder| 
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
pub struct TreeViewItem {
    header: Entity,
    arrow: Entity,
    container: Entity,
    item: Entity,

    collapsed: bool,

    // Template for item(s) to be placed after the expand/collapse arrow
    header_template: Option<Box<dyn Fn(&mut State, Entity) -> Entity>>,
}

impl TreeViewItem {
    pub fn new() -> Self {
        Self {
            header: Entity::null(),
            arrow: Entity::null(),
            item: Entity::null(),
            container: Entity::null(),

            header_template: None,

            collapsed: false,
        }
    }

    pub fn with_label(label: &'static str) -> Self {
        Self {
            header: Entity::null(),
            arrow: Entity::null(),
            item: Entity::null(),
            container: Entity::null(),

            header_template: Some(Box::new(move |state, entity| 
                Label::new(&label.to_owned()).build(state, entity, |builder| 
                    builder
                        .set_child_space(Stretch(1.0))
                        .set_child_left(Pixels(0.0))
                )
            )),

            collapsed: false,
        }
    }

    pub fn with_header_template<F>(template: F) -> Self 
    where F: 'static + Fn(&mut State, Entity) -> Entity,
    {
        Self {
            header: Entity::null(),
            arrow: Entity::null(),
            item: Entity::null(),
            container: Entity::null(),

            header_template: Some(Box::new(template)),

            collapsed: false,
        }
    }
}

impl Widget for TreeViewItem {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.header = Element::new().build(state, entity, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_layout_type(LayoutType::Row)
        );

        self.arrow = Element::new().build(state, self.header, |builder| 
            builder
                .set_width(Pixels(30.0))
                .set_text(ICON_DOWN_OPEN_BIG)
                .set_font("icon")
                .set_child_space(Stretch(1.0))
                .class("item")
                .set_visibility(Visibility::Invisible)
                //.set_background_color(Color::red())
        );

        // Label::new("TreeViewItem").build(state, header, |builder| 
        //     builder
        //         .set_child_space(Stretch(1.0))
        //         .set_child_left(Pixels(0.0))
        // );
        // if let Some(widget) = self.widget.take() {
        //     self.item = widget.build(state, self.header, |builder| builder).entity();
        // }

        if let Some(header_template) = self.header_template.take() {
            self.item = (header_template)(state, self.header);
        }

        self.container = Element::new().build(state, entity, |builder| 
            builder
                .set_height(Auto)
                .set_child_left(Pixels(15.0))
        );

        entity.set_height(state, Pixels(30.0));

        self.container
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

        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if event.target == self.arrow {
                        state.capture(entity);
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    if event.target == entity {
                        state.release(entity);
                        if self.collapsed {
                            self.container.set_display(state, Display::Flex);
                            self.arrow.set_rotate(state, 0.0);
                            self.collapsed = false;
                        } else {
                            self.container.set_display(state, Display::None);
                            self.arrow.set_rotate(state, -90.0);
                            self.collapsed = true;
                        }
                    }
                }

                _=> {}
            }
        }
    
        if let Some(widget_event) = event.message.downcast() {
            match widget_event {
                WidgetEvent::ChildAdded(_) => {
                    if event.target == self.container {
                        println!("Child added");
                        self.arrow.set_visibility(state, Visibility::Visible);
                    }
                }

                _=> {}
            }
        }
    }
}

