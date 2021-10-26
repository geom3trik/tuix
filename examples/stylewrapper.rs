

use tuix::*;
use tuix::widgets::*;


pub struct StyleWrapper<W: Widget> {
    widget: W,

    background_color: Option<Color>,
    width: Option<Units>,
    height: Option<Units>,
    
    child_left: Option<Units>,
    child_right: Option<Units>,
    child_top: Option<Units>,
    child_bottom: Option<Units>,

}

impl<W: Widget> StyleWrapper<W> {
    pub fn new(widget: W) -> Self {
        Self {
            widget,
            background_color: Default::default(),
            width: Default::default(),
            height: Default::default(),
            child_left: Default::default(),
            child_right: Default::default(),
            child_top: Default::default(),
            child_bottom: Default::default(),
        }
    }

    pub fn build(mut self, state: &mut State, parent: Entity) -> <W as Widget>::Ret {
        // Create a new entity
        let entity = state.add(parent.entity());

        //state.insert_event(Event::new(WidgetEvent::ChildAdded(entity)).direct(parent.entity()));
        parent.entity().emit(state, WidgetEvent::ChildAdded(entity));

        // Call the on_build function of the widget
        let ret = self.widget.on_build(state, entity);

        if let Some(color) = self.background_color {
            state.style.background_color.insert(entity, color);
        }

        if let Some(width) = self.width {
            state.style.width.insert(entity, width);
        }

        if let Some(height) = self.height {
            state.style.height.insert(entity, height);
        }

        if let Some(child_left) = self.child_left {
            state.style.child_left.insert(entity, child_left);
        }

        if let Some(child_right) = self.child_right {
            state.style.child_right.insert(entity, child_right);
        }

        if let Some(child_top) = self.child_top {
            state.style.child_top.insert(entity, child_top);
        }

        if let Some(child_bottom) = self.child_bottom {
            state.style.child_bottom.insert(entity, child_bottom);
        }

        state
            .event_handlers
            .insert(entity, Box::new(self.widget));

        // Return the entity or entities returned by the on_build method
        ret    
    }
}

pub trait Stylable {
    type Ret;
    fn background_color(self, color: Color) -> Self::Ret;
    fn width(self, units: Units) -> Self::Ret;
    fn height(self, units: Units) -> Self::Ret;

    fn child_space(self, value: Units) -> Self::Ret;
    fn child_left(self, value: Units) -> Self::Ret;
    fn child_right(self, value: Units) -> Self::Ret;
    fn child_top(self, value: Units) -> Self::Ret;
    fn child_bottom(self, value: Units) -> Self::Ret;
    
}

impl<W: Widget> Stylable for W {
    type Ret = StyleWrapper<W>;
    fn background_color(self, color: Color) -> StyleWrapper<W> {
        StyleWrapper::new(self).background_color(color)
    }

    fn width(self, width: Units) -> StyleWrapper<W> {
        StyleWrapper::new(self).width(width)
    }

    fn height(self, height: Units) -> StyleWrapper<W> {
        StyleWrapper::new(self).height(height)
    }

    fn child_space(self, value: Units) -> StyleWrapper<W> {
        StyleWrapper::new(self).child_space(value)
    }

    fn child_left(self, value: Units) -> StyleWrapper<W> {
        StyleWrapper::new(self).child_left(value)
    }

    fn child_right(self, value: Units) -> StyleWrapper<W> {
        StyleWrapper::new(self).child_right(value)
    }

    fn child_top(self, value: Units) -> StyleWrapper<W> {
        StyleWrapper::new(self).child_top(value)
    }

    fn child_bottom(self, value: Units) -> StyleWrapper<W> {
        StyleWrapper::new(self).child_bottom(value)
    }
}

impl<W: Widget> Stylable for StyleWrapper<W> {
    type Ret = Self;

    fn background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);

        self
    }

    fn width(mut self, width: Units) -> Self {
        self.width = Some(width);

        self
    }

    fn height(mut self, height: Units) -> Self {
        self.height = Some(height);

        self
    }

    fn child_space(mut self, value: Units) -> Self {
        self.child_left = Some(value);
        self.child_right = Some(value);
        self.child_top = Some(value);
        self.child_bottom = Some(value);

        self
    }

    fn child_left(mut self, value: Units) -> Self {
        self.child_left = Some(value);

        self
    }

    fn child_right(mut self, value: Units) -> Self {
        self.child_right = Some(value);

        self
    }

    fn child_top(mut self, value: Units) -> Self {
        self.child_top = Some(value);

        self
    }

    fn child_bottom(mut self, value: Units) -> Self {
        self.child_bottom = Some(value);

        self
    }
}

fn main() {
    Application::new(WindowDescription::new(), |state, window|{
        
        Button::with_label("Press Me")
            .child_space(Stretch(1.0))
            .background_color(Color::red())
            .width(Pixels(100.0))
            .height(Pixels(50.0))
            .build(state, window);
    
    }).run();
}