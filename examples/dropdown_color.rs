

// use tuix::*;
// use std::marker::PhantomData;

// // trait ListTrait {
// //     fn 
// // }

// #[derive(Debug, Clone, PartialEq)]
// pub enum ListEvent<T> {
//     Construct,
//     Insert(T),
// }


// // Data with a list
// pub struct ColorList {
//     pub colors: Vec<Color>,
// }

// impl Node for ColorList {
//     fn on_mutate(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
//         println!("Mutate Event");
//         if let Some(list_event) = event.message.downcast() {
//             match list_event {
//                 ListEvent::<Color>::Insert(item) => {
//                     self.colors.push(*item);
//                 }

//                 _=> {}
//             }
//         }
//     }
// }

// struct ListWidget<T> {
//     a: PhantomData<*const T>,
//     children: Vec<Entity>,

//     selected: Entity,

//     on_update: Option<Box<dyn Fn(&mut State, Entity, &T)>>,
// }

// impl<T> ListWidget<T> {
//     pub fn new() -> Self {
//         Self {
//             a: PhantomData::default(),
//             children: Vec::new(),

//             selected: Entity::null(),

//             on_update: None,
//         }
//     }

//     pub fn on_update<F>(mut self, update: F) -> Self 
//     where F: 'static + Fn(&mut State, Entity, &T)
//     {
//         self.on_update = Some(Box::new(update));
//         self
//     }
// }

// impl<T: 'static> Widget for ListWidget<T> {
//     type Ret = Entity;
//     type Data = T;
//     fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
//         entity
//     }

//     fn on_update(&mut self, state: &mut State, entity: Entity, node: &Self::Data, nodes: &NodeMap) {
//         if let Some(update) = self.on_update.take() {
//             (update)(state, entity, node);

//             self.on_update = Some(update);
//         }
//     }

//     fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        
//     }
// }

// struct ColorWidget {
//     data: Entity,
// }

// impl ColorWidget {
//     pub fn new(data: Entity) -> Self {
//         Self {
//             data,
//         }
//     }
// }

// impl Widget for ColorWidget {
//     type Ret = Entity;
//     type Data = Color;

//     fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        

//         Element::new().build(state, entity, |builder| 
//             builder
//                 .set_width(Percentage(0.2))
//                 .set_background_color(Color::red())
//         );

//         Label::<Color>::new("Color")
//             .with_converter(|data| data.to_string())
//             .build(state, parent, |builder| builder)
//             .bind(state, self.data);

//         entity.set_layout_type(state, LayoutType::Row)
//     }

//     fn on_update(&mut self, state: &mut State, entity: Entity, node: &Self::Data, nodes: &NodeMap) {
        
//     }
// }

// pub fn main() {
//     let window_description = WindowDescription::new();
//     let app = Application::new(window_description, |state, window| {
//         // Dropdown::new().build(state, window, |builder| 
//         //     builder
//         // );

//         let mut color_data = ColorList {
//             colors: vec![Color::red(), Color::green(), Color::blue()],
//         };

//         let data = color_data.build(state, window);

//         let list = ListWidget::<ColorList>::new()
//             .on_update(|state, list, data| {

//                 // This is a terrible idea because you lose all the saved state of a child!!!

//                 // Remove previous children
//                 for child in list.child_iter(&state.tree.clone()) {
//                     state.remove(child);
//                 }

//                 // Update with new children
//                 for color in &data.colors {
//                     Label::<String>::new(&color.to_string()).build(state, list, |builder| 
//                         builder
//                             .set_width(Pixels(100.0))
//                             .set_height(Pixels(30.0))
//                             .set_background_color(Color::rgb(50,50,100))
//                     );
//                 }
//             })
//             .build(state, window, |builder| builder)
//             .bind(state, data);
        
//         Button::with_label("Add Item")
//             .on_press(move |button, state, entity| {
//                 state.insert_update(Event::new(ListEvent::Insert(Color::red())).origin(list));
//             })
//             .build(state, window, |builder| 
//                 builder
//                     .set_width(Pixels(100.0))
//                     .set_height(Pixels(30.0))
//                     .set_background_color(Color::rgb(50, 100, 50))
//             );

//         state.insert_update(Event::new(ListEvent::<Color>::Construct).origin(list));
//     });

//     app.run();
// }

fn main() {
    
}