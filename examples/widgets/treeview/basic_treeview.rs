
use std::array::IntoIter;
use std::marker::PhantomData;

use tuix::*;
use tuix::widgets::*;




fn main() {
    let window_description = WindowDescription::new().with_title("TreeView");
    let app = Application::new(window_description, |state, window| {
    
        let tree_view = TreeView::new().build(state, window, |builder| builder);
        let root = TreeViewItem::with_label("root").build(state, tree_view, |builder| builder);
        let child_one = TreeViewItem::with_label("child item 1").build(state, root, |builder| builder);
        let child_one_one = TreeViewItem::with_label("child item 1.1").build(state, child_one, |builder| builder);
        let child_one_two = TreeViewItem::with_label("child item 1.2").build(state, child_one, |builder| builder);
        let child_two = TreeViewItem::with_label("child item 2").build(state, root, |builder| builder);
        
    });

    app.run();
}





