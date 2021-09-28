use tuix::*;

const STYLE: &str = r#"
    textbox {
        height: 30px;
        border-width: 2px;
        border-color: black;
        child-space: 1s;
        child-left: 5px;
    }

    button.add {
        background-color: #4040AA;
        color: white;
        height: 30px;
        width: 100px;
        border-radius: 15px;
        child-space: 1s;
    }

    button.add:hover {
        background-color: #4040DD;
    }

    button.delete {
        opacity: 0.5;
    }

    button.delete:hover {
        opacity: 1.0;
    }
"#;

const ICON_TRASH: &str = "\u{e729}";

#[derive(Debug, Clone, Lens)]
pub struct TodoItem {
    description: String,
    completed: bool,
}

#[derive(Debug, Clone, Lens)]
pub struct AppData {
    show_sorted: bool,
    todo_items: Vec<TodoItem>,
}

impl AppData {
    pub fn new() -> Self {

        let mut todo_items = Vec::new();

        todo_items.push(TodoItem {
            description: "Finish making this example".to_string(),
            completed: false,
        });

        todo_items.push(TodoItem {
            description: "Make example editable".to_string(),
            completed: false,
        });

        todo_items.push(TodoItem {
            description: "Restyle counter examples".to_string(),
            completed: false,
        });

        todo_items.push(TodoItem {
            description: "Update images in quick start guide".to_string(),
            completed: false,
        });

        todo_items.push(TodoItem {
            description: "Change default window background to white".to_string(),
            completed: true,
        });

        todo_items.push(TodoItem {
            description: "Clean up the code".to_string(),
            completed: false,
        });

        todo_items.push(TodoItem {
            description: "Change default font color to black".to_string(),
            completed: true,
        });

        todo_items.push(TodoItem {
            description: "Propagate binding data to children".to_string(),
            completed: true,
        });

        todo_items.push(TodoItem {
            description: "Add Drag and Drop functionality".to_string(),
            completed: false,
        });

        todo_items.push(TodoItem {
            description: "Add ability to add Timers".to_string(),
            completed: false,
        });

        Self {
            show_sorted: false,
            todo_items,
        }
    }
}

#[derive(PartialEq)]
pub enum TodoEvent {
    Add(String),
    Remove(usize),
    Sort(bool),
    Mark(usize, bool),
}

impl Model for AppData {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(todo_event) = event.message.downcast() {
            match todo_event {
                TodoEvent::Add(todo) => {
                    self.todo_items.push(TodoItem {
                        description: todo.clone(),
                        completed: false,
                    });
                    entity.emit(state, BindEvent::Update);
                }

                TodoEvent::Remove(index) => {
                    self.todo_items.remove(*index);
                    entity.emit(state, BindEvent::Update);
                }

                TodoEvent::Sort(flag) => {
                    self.show_sorted = *flag;
                    entity.emit(state, BindEvent::Update);
                }

                TodoEvent::Mark(index, flag) => {
                    if let Some(todo_item) = self.todo_items.get_mut(*index) {
                        todo_item.completed = *flag;
                        entity.emit(state, BindEvent::Update);
                    } 
                }

                _=> {}
            }
        }
    }
}

#[derive(Default)]
pub struct TodoItemWidget {

}

impl Widget for TodoItemWidget {
    type Ret = Entity;
    type Data = TodoItem;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        let index = state.tree.get_child_index(entity).unwrap();
        
        // Row for laying out the checkbox and label horizontally and for adding space around and between
        let row = Row::new().build(state, entity, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_child_space(Pixels(5.0))
                .set_col_between(Pixels(5.0))
        );

        // Checkbox to show whether the todo item is done or not
        Checkbox::new(false)
            .on_checked(move |_, state, checkbox|{
                checkbox.emit(state, TodoEvent::Mark(index, true));
            })
            .on_unchecked(move |_, state, checkbox|{
                checkbox.emit(state, TodoEvent::Mark(index, false));
            })
            // Bind the checkbox to the TodoItem's completed flag
            .bind(TodoItem::completed, |completed| *completed)
            .build(state, row, |builder| builder);

        // Label to show the todo description
        Label::new("")
            // Bind the label to the TodoItem's description field
            .bind(TodoItem::description, |desc| desc.clone())
            .build(state, row, |builder| builder);
        
        
        
        Button::with_label(ICON_TRASH)
        .on_press(move |_, state, button| {
            button.emit(state, TodoEvent::Remove(index));
        })
        .build(state, row, |builder| 
            builder
                .set_width(Pixels(30.0))
                .set_right(Pixels(0.0))
                .set_child_space(Stretch(1.0))
                .set_font("icons")
                .set_color(Color::rgb(220, 50, 50))
                .class("delete")
        );

        entity
            .set_border_color(state, Color::black())
            .set_border_width(state, Pixels(2.0))
            .set_height(state, Auto)
    }
}


pub struct SortableList {
    list: ListView<TodoItem, TodoItemWidget>,
}

impl SortableList {
    pub fn new() -> Self {
        Self {
            list: ListView::with_template(|_,_| TodoItemWidget::default()),
        }
    }
}

impl Widget for SortableList {
    type Ret = Entity;
    type Data = (bool, Vec<TodoItem>);

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.list.on_build(state, entity)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        let (sorted, todos) = data;
        let new_todos = if *sorted {
            // Need to clone because we can't sort the original due to only having an immutable ref
            let mut t = todos.clone();
            t.sort_by_cached_key(|item| item.completed);
            t
        } else {
            todos.clone()
        };
        self.list.on_update(state, entity, &new_todos);
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.list.on_event(state, entity, event);
    }
}


fn main() {
    let window_description = WindowDescription::new().with_title("Basic Todos");
    let app = Application::new(window_description, |state, window|{

        state.add_theme(STYLE);

        // Add the app data to the root of the application 
        let app_data = AppData::new().build(state, window);

        let col = Column::new().build(state, app_data, |builder| 
            builder
                .set_height(Auto)
                .set_row_between(Pixels(10.0))
                .set_child_top(Pixels(10.0))
                .set_child_bottom(Pixels(10.0))
                .set_child_left(Pixels(10.0))
                .set_child_right(Pixels(10.0))
        );

        Label::new("TODOS APP").build(state, col, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(0.0))
                //.set_background_color(Color::red())
                .set_font_size(24.0)
        );

        Textbox::new("What do you want todo?")
            .on_submit(|data, state, textbox|{
                textbox.emit(state, TodoEvent::Add(data.text.clone()));
            })
            .build(state, col, |builder| builder);
        
        Element::new().build(state, app_data, |builder| 
            builder
                .set_background_color(Color::black())
                .set_height(Pixels(2.0))
        );

        let row = Row::new().build(state, col, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_col_between(Pixels(10.0))
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_child_left(Stretch(1.0))
        ); 

        Label::new("WARNING: Example not finished.").build(state, row, |builder| 
            builder
                .set_height(Pixels(20.0))
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(0.0))
                .set_width(Pixels(150.0))
                .set_color(Color::red())
        );

        Checkbox::new(false)
            .on_checked(|_, state, checkbox|{
                checkbox.emit(state, TodoEvent::Sort(true));
            })
            .on_unchecked(|_, state, checkbox|{
                checkbox.emit(state, TodoEvent::Sort(false));
            })
            .bind(AppData::show_sorted, |should_sort| *should_sort)
            .build(state, row, |builder| builder);

        Label::new("Sort by completed").build(state, row, |builder| 
            builder
                .set_height(Pixels(20.0))
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(0.0))
                .set_width(Pixels(150.0))
        );

        let scroll = ScrollContainer::new().build(state, app_data, |builder| builder);

        // Create a new list view and specify a widget to use to show the list item
        SortableList::new()
            // Bind the ListView to the list data
            .bind(AppData::show_sorted.and(AppData::todo_items), |items| items.clone())
            // Build the ListView into the app
            .build(state, scroll, |builder| 
                builder
                    .set_child_space(Pixels(10.0))
                    .set_row_between(Pixels(10.0))
                    .set_height(Auto)
            );

        // // Create a new list view and specify a widget to use to show the list item
        // ListView::with_template(|_,_| TodoItemWidget::default())
        //     // Bind the ListView to the list data
        //     .bind(AppData::todo_items, |items| items.clone())
        //     // Build the ListView into the app
        //     .build(state, scroll, |builder| 
        //         builder
        //             .set_child_space(Pixels(10.0))
        //             .set_row_between(Pixels(10.0))
        //             .set_height(Auto)
        //     );
    });

    app.run();
}