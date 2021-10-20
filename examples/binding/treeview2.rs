
use tuix::*;
use tuix::widgets::*;

const ICON_DOWN_OPEN_BIG: &str = "\u{e75c}";

const STYLE: &str = r#"

    label {
        background-color: white;
    }

    label:hover {
        background-color: #CCCCCC;
    }
"#;

fn main() {
    let window_description = WindowDescription::new().with_title("TreeView");
    let app = Application::new(window_description, |state, window| {
        
        state.add_theme(STYLE);

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

        let treeview = TreeView::with_template(|state, parent| {
            Label::new("default")
                .bind_ref(TreeData::name)
                .build(state, parent, |builder| 
                    builder
                        .set_child_space(Stretch(1.0))
                        .set_child_left(Pixels(0.0))
                )
        })
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

impl TreeIter for TreeData {
    type Item = TreeData;
    type IntoIter = std::vec::IntoIter<TreeData>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter()
    }
}

impl Model for TreeData {
    
}
