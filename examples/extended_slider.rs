use tuix::*;

// Example showing extending the slider widget, adding on_press and on_release callbacks


const STYLE: &str = r#"

    slider {
        height: 20px;
    }

    slider>.track {
        background-color: #dfdfdf;
        border-radius: 2px;
    }

    slider>.track>.active {
        background-color: #f74c00;
        border-radius: 2px;
    }

    slider>.thumb {
        background-color: white;
        width: 20px;
        height: 20px;
        border-radius: 9.5px;
        border-color: #757575;
        border-width: 1px;
    }

"#;

pub struct ExtendedSlider {
    pub slider: Slider,

    on_press: Option<Box<dyn Fn(&mut Slider, &mut State, Entity)>>,
    on_release: Option<Box<dyn Fn(&mut Slider, &mut State, Entity)>>,
}

impl ExtendedSlider {
    pub fn new() -> Self {
        Self {
            slider: Slider::new(),

            on_press: None,
            on_release: None,
        }
    }

    pub fn on_press<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Slider, &mut State, Entity),
    {
        self.on_press = Some(Box::new(callback));

        self
    }

    pub fn on_release<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Slider, &mut State, Entity),
    {
        self.on_release = Some(Box::new(callback));

        self
    }
}

impl Widget for ExtendedSlider {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.slider.on_build(state, entity)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.slider.on_event(state, entity, event);

        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if entity == event.target && !entity.is_disabled(state) {
                        state.capture(entity);
                        if let Some(callback) = self.on_press.take() {
                            (callback)(&mut self.slider, state, entity);

                            self.on_press = Some(callback);
                        }
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    if entity == event.target && state.mouse.left.pressed == entity {
                        state.release(entity);
                        entity.set_active(state, false);
                        if !entity.is_disabled(state) {
                            if state.hovered == entity {
                                if let Some(callback) = self.on_release.take() {
                                    (callback)(&mut self.slider, state, entity);
        
                                    self.on_release = Some(callback);
                                }
                            }
                        }
                    }
                }

                _=> {}
            }
        }
    }
}

fn main() {
    let window_description = WindowDescription::new();
    let app = Application::new(window_description, |state, window| {
        
        state.add_theme(STYLE);
        
        let mut extended_slider = ExtendedSlider::new()
            .on_press(|slider, _, _| {
                println!("Pressed: {}", slider.value);
            })
            .on_release(|slider, _, _| {
                println!("Released: {}", slider.value);
            });
        
        extended_slider.slider = extended_slider.slider
            .on_changing(|slider, _, _| {
                println!("Changing: {}", slider.value);
            });
        
        extended_slider.build(state, window, |builder| 
                builder
                    .set_width(Pixels(210.0))
                    .set_height(Pixels(30.0))
                    .set_space(Stretch(1.0))
            );
    });

    app.run();
}