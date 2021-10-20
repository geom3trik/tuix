

use tuix::*;
use tuix::widgets::*;

const ICON_DOWN_OPEN_BIG: &str = "\u{e75c}";

const STYLE: &str = r#"

"#;

fn main() {
    let window_description = WindowDescription::new().with_title("TreeView");
    let app = Application::new(window_description, |state, window| {
        
        state.add_theme(STYLE);

        let mut tree_data = TreeData {
            name: "root".to_string(),
            children: vec![
                TreeItem {
                    name: "child item 1".to_string(),
                    children: vec![
                        TreeSubItem {
                            name: "child item 1.1".to_string(),
                        },
                    ],
                },

                TreeItem {
                    name: "child item 2".to_string(),
                    children: vec![
                        TreeSubItem {
                            name: "child item 2.1".to_string(),
                        },

                        TreeSubItem {
                            name: "child item 2.2".to_string(),
                        }
                    ]
                }
            ],
        }.build(state, window);
        
        let treeview = TreeView::with_template(|state, parent| {
            Label::new("default")
                .bind_ref(TreeItem::name)
                .build(state, parent, |builder| 
                    builder
                        .set_child_space(Stretch(1.0))
                        .set_child_left(Pixels(0.0))
                )
        })
        .bind_ref(TreeData::root)
        .build(state, tree_data, |builder| builder);
    
        // TreeView::with_template(|state, level0|{
        //     /// Level 0
        //     TreeViewItem::with_header_template(|state, header0|{
        //         Label::new("")
        //             .bind_ref(TreeData::name)
        //             .build(state, level0, |builder| builder);
        //     }).set_item_template(|state, level1|{
        //         /// Level 1
        //         TreeViewItem::with_header_template(|state, header1|{
        //             Label::new("")
        //                 .bind_ref(TreeData::name)
        //                 .build(state, header1, |builder| builder);
        //         }).set_item_tenplate(|state, level2|{
        //             /// Level 2
        //             TreeViewItem::with_header_template(|state, header2|{
        //                 Label::new("")
        //                     .bind_ref(TreeData::name)
        //                     .build(state, header2, |builder| builder);
        //             }).build(state, level2, |builder| builder)
        //         }).build(state, level1, |builder| builder)
        //     }).build(state, level0, |builder| builder)
        // });
    
    });

    app.run();
}



#[derive(Clone, Lens, Debug)]
pub struct TreeData {
    pub name: String,
    pub children: Vec<TreeItem>,
}

impl<'a> TreeIter for TreeData {
    type Item = TreeItem;
    type IntoIter = std::vec::IntoIter<TreeItem>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter()
    }
}

#[derive(Debug, Lens, Clone)]
pub struct TreeItem {
    pub name: String,
    children: Vec<TreeSubItem>,
}

impl TreeIter for TreeItem {
    type Item = TreeSubItem;
    type IntoIter = std::vec::IntoIter<TreeSubItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter()
    }

}

#[derive(Debug, Lens, Clone)]
pub struct TreeSubItem {
    pub name: String,
}

impl TreeIter for TreeSubItem {
    type Item = NullType;
    type IntoIter = NullType;

    fn into_iter(self) -> Self::IntoIter {
        NullType
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

impl Model for TreeData {
    
}
