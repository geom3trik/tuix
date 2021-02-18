extern crate tuix;

use image::GenericImageView;

use tuix::*;

static LIGHT_THEME: &'static str = include_str!("themes/calculator_light_theme.css");

#[derive(Debug, Clone, PartialEq)]
pub enum CalculatorEvent {
    Digit(char),
    Operator(char),
}

// impl Message for CalculatorEvent {}

pub struct Calculator {
    display: Entity,

    zero: Entity,
    one: Entity,
    two: Entity,
    three: Entity,
    four: Entity,
    five: Entity,
    six: Entity,
    seven: Entity,
    eight: Entity,
    nine: Entity,

    clear: Entity,
    multiply: Entity,
    divide: Entity,
    subtract: Entity,
    add: Entity,
    percent: Entity,
    plus_minus: Entity,
    decimal_point: Entity,
    equals: Entity,

    input: String,
    left_side: f64,
    right_side: Option<f64>,
    operator: Option<char>,
}

impl Default for Calculator {
    fn default() -> Self {
        Calculator::new()
    }
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            display: Entity::null(),

            zero: Entity::null(),
            one: Entity::null(),
            two: Entity::null(),
            three: Entity::null(),
            four: Entity::null(),
            five: Entity::null(),
            six: Entity::null(),
            seven: Entity::null(),
            eight: Entity::null(),
            nine: Entity::null(),

            clear: Entity::null(),
            multiply: Entity::null(),
            divide: Entity::null(),
            subtract: Entity::null(),
            add: Entity::null(),
            percent: Entity::null(),
            plus_minus: Entity::null(),
            decimal_point: Entity::null(),
            equals: Entity::null(),

            input: "".to_string(),
            left_side: 0.0,
            right_side: None,
            operator: None,
        }
    }

    pub fn update_display(&self, state: &mut State) {
        self.display.set_text(state, &self.left_side.to_string());

        // if self.right_side > 0.0 {
        //     self.display.set_text(state, &self.right_side.to_string());
        // } else if self.left_side > 0.0 {
        //     self.display.set_text(state, &self.left_side.to_string());
        // } else {
        //     self.display.set_text(state, "0");
        // }
    }

    pub fn clear_all(&mut self, state: &mut State) {
        self.input.clear();
        self.left_side = 0.0;
        self.right_side = None;
        self.operator = None;
        self.update_display(state);
    }
}

impl BuildHandler for Calculator {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        let container = Element::new().build(state, entity, |builder| builder.class("container"));

        // Change to label that can be copied but not edited at some point
        self.display = Label::new("0").build(state, container, |builder| builder.class("display"));

        // Currently using flexbox to create the layout but would be good to use grid when working

        let row1 = HBox::new().build(state, container, |builder| builder);

        self.clear = Button::new()
            .on_press(Event::new(CalculatorEvent::Operator('C')))
            .build(state, row1, |builder| builder.set_text("AC").class("digit"));

        self.plus_minus = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('¬')))
            .build(state, row1, |builder| {
                builder.set_text("\u{00B1}").class("digit")
            });

        // Percentage
        self.percent = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('%')))
            .build(state, row1, |builder| {
                builder.set_text("\u{0025}").class("digit")
            });

        // Divide
        self.divide = Button::new()
            .on_press(Event::new(CalculatorEvent::Operator('/')))
            .build(state, row1, |builder| {
                builder.set_text("\u{00F7}").class("operator")
            });

        // Second Row
        let row2 = HBox::new().build(state, container, |builder| builder);

        // Digit Seven
        self.seven = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('7')))
            .build(state, row2, |builder| builder.set_text("7").class("digit"));

        // Digit Eight
        self.eight = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('8')))
            .build(state, row2, |builder| builder.set_text("8").class("digit"));

        // Digit Nine
        self.nine = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('9')))
            .build(state, row2, |builder| builder.set_text("9").class("digit"));

        // Multiply
        self.multiply = Button::new()
            .on_press(Event::new(CalculatorEvent::Operator('*')))
            .build(state, row2, |builder| {
                builder.set_text("\u{00D7}").class("operator")
            });

        // Third Row
        let row3 = HBox::new().build(state, container, |builder| builder);

        // Digit Four
        self.four = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('4')))
            .build(state, row3, |builder| builder.set_text("4").class("digit"));

        // Digit Five
        self.five = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('5')))
            .build(state, row3, |builder| builder.set_text("5").class("digit"));

        // Digit Six
        self.six = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('6')))
            .build(state, row3, |builder| builder.set_text("6").class("digit"));

        // Subtract
        self.subtract = Button::new()
            .on_press(Event::new(CalculatorEvent::Operator('-')))
            .build(state, row3, |builder| {
                builder.set_text("\u{002D}").class("operator")
            });

        // Fourth Row
        let row4 = HBox::new().build(state, container, |builder| builder);

        // Digit One
        self.one = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('1')))
            .build(state, row4, |builder| builder.set_text("1").class("digit"));

        // Digit Two
        self.two = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('2')))
            .build(state, row4, |builder| builder.set_text("2").class("digit"));

        // Digit Three
        self.three = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('3')))
            .build(state, row4, |builder| builder.set_text("3").class("digit"));

        // Add
        self.add = Button::new()
            .on_press(Event::new(CalculatorEvent::Operator('+')))
            .build(state, row4, |builder| {
                builder.set_text("\u{002B}").class("operator")
            });

        // Fifth Row
        let row5 = HBox::new().build(state, container, |builder| builder.class("last"));

        // Digit Zero
        self.zero = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('0')))
            .build(state, row5, |builder| {
                builder.set_text("0").class("digit").class("zero")
            });

        // Decimal Point
        self.decimal_point = Button::new()
            .on_press(Event::new(CalculatorEvent::Digit('.')))
            .build(state, row5, |builder| builder.set_text(".").class("digit"));

        // Equals
        self.equals = Button::new()
            .on_press(Event::new(CalculatorEvent::Operator('=')))
            .build(state, row5, |builder| {
                builder.set_text("\u{003D}").class("operator")
            });

        state.focused = self.display;

        // Define the focus order for the buttons
        self.display
            .set_focus_order(state, self.clear, self.decimal_point);
        self.clear
            .set_focus_order(state, self.plus_minus, self.display);
        self.plus_minus
            .set_focus_order(state, self.percent, self.clear);
        self.percent
            .set_focus_order(state, self.divide, self.plus_minus);
        self.divide
            .set_focus_order(state, self.multiply, self.percent);
        self.multiply
            .set_focus_order(state, self.subtract, self.divide);
        self.subtract
            .set_focus_order(state, self.add, self.multiply);
        self.add.set_focus_order(state, self.equals, self.subtract);
        self.equals.set_focus_order(state, self.zero, self.add);
        self.zero.set_focus_order(state, self.one, self.equals);
        self.one.set_focus_order(state, self.two, self.zero);
        self.two.set_focus_order(state, self.three, self.one);
        self.three.set_focus_order(state, self.four, self.two);
        self.four.set_focus_order(state, self.five, self.three);
        self.five.set_focus_order(state, self.six, self.four);
        self.six.set_focus_order(state, self.seven, self.five);
        self.seven.set_focus_order(state, self.eight, self.six);
        self.eight.set_focus_order(state, self.nine, self.seven);
        self.nine
            .set_focus_order(state, self.decimal_point, self.eight);
        self.decimal_point
            .set_focus_order(state, self.display, self.nine);

        entity
    }
}

impl EventHandler for Calculator {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(calculator_event) = event.message.downcast::<CalculatorEvent>() {
            match calculator_event {
                CalculatorEvent::Digit(num) => {
                    if *num == '¬' {
                        if self.input.len() > 0 {
                            if self.input.contains("-") {
                                self.input.remove(0);
                            } else {
                                self.input.insert(0, '-');
                            }
                        } else {
                            self.input = (self.left_side * -1.0).to_string();
                        }
                    } else if *num == '%' {
                        if let Some(right_side) = self.right_side {
                            if let Some(operator) = self.operator {
                                self.right_side = match operator {
                                    '+' | '-' => Some(self.left_side * 0.01 * right_side),
                                    '*' | '/' => Some(0.01 * right_side),
                                    _ => Some(right_side),
                                }
                            }

                            self.input = self.right_side.unwrap().to_string();
                        }
                    } else if *num == '.' {
                        if self.input.len() == 0 {
                            self.input.push('0');
                            self.input.push('.');
                        } else {
                            self.input.push('.');
                        }
                    } else {
                        if self.input.len() < 15 {
                            self.input.push(*num);
                        }
                    }

                    println!("input: {}", self.input);

                    self.right_side = match self.input.parse::<f64>() {
                        Ok(val) => Some(val),
                        Err(_) => {
                            self.input.pop();
                            self.right_side
                        }
                    };

                    if !self.input.is_empty() {
                        self.display.set_text(state, &self.input);
                    } else {
                        self.display.set_text(state, "0");
                    }
                }

                CalculatorEvent::Operator(op) => {
                    if let Some(right_side) = self.right_side {
                        match self.operator {
                            Some(operator) => {
                                self.left_side = match operator {
                                    '+' => self.left_side + right_side,
                                    '-' => self.left_side - right_side,
                                    '*' => self.left_side * right_side,
                                    '/' => self.left_side / right_side,
                                    '%' => self.left_side,
                                    _ => right_side,
                                };
                            }

                            None => self.left_side = right_side,
                        }

                        self.right_side = None;
                    }

                    self.input.clear();
                    self.update_display(state);

                    match op {
                        '+' => {
                            self.operator = Some('+');
                        }

                        '-' => {
                            self.operator = Some('-');
                        }

                        '*' => {
                            self.operator = Some('*');
                        }

                        '/' => {
                            self.operator = Some('/');
                        }

                        '=' => {
                            self.operator = Some('=');
                        }

                        'C' => {
                            self.clear_all(state);
                        }

                        _ => {}
                    }
                }
            }
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::KeyDown(code, key) => match code {
                    Code::Numpad0 | Code::Digit0 => {
                        state.insert_event(Event::new(ButtonEvent::Pressed).target(self.zero));
                    }

                    Code::Numpad1 | Code::Digit1 => {
                        state.insert_event(Event::new(ButtonEvent::Pressed).target(self.one));
                    }

                    Code::Numpad2 | Code::Digit2 => {
                        state.insert_event(Event::new(ButtonEvent::Pressed).target(self.two));
                    }

                    Code::Numpad3 | Code::Digit3 => {
                        state.insert_event(Event::new(ButtonEvent::Pressed).target(self.three));
                    }

                    Code::Numpad4 | Code::Digit4 => {
                        state.insert_event(Event::new(ButtonEvent::Pressed).target(self.four));
                    }

                    Code::Numpad5 | Code::Digit5 => {
                        if state.modifiers.shift {
                            state.insert_event(
                                Event::new(ButtonEvent::Pressed).target(self.percent),
                            );
                        } else {
                            state.insert_event(Event::new(ButtonEvent::Pressed).target(self.five));
                        }
                    }

                    Code::Numpad6 | Code::Digit6 => {
                        state.insert_event(Event::new(ButtonEvent::Pressed).target(self.six));
                    }

                    Code::Numpad7 | Code::Digit7 => {
                        state.insert_event(Event::new(ButtonEvent::Pressed).target(self.seven));
                    }

                    Code::Numpad8 | Code::Digit8 => {
                        if state.modifiers.shift {
                            state.insert_event(
                                Event::new(ButtonEvent::Pressed).target(self.multiply),
                            );
                        } else {
                            state.insert_event(Event::new(ButtonEvent::Pressed).target(self.eight));
                        }
                    }

                    Code::Numpad9 | Code::Digit9 => {
                        state.insert_event(Event::new(ButtonEvent::Pressed).target(self.nine));
                    }

                    Code::Escape => {
                        state.active = self.clear;
                        self.clear_all(state);
                    }

                    Code::NumpadMultiply => {
                        state.insert_event(Event::new(ButtonEvent::Pressed).target(self.multiply));
                    }

                    Code::NumpadSubtract => {
                        state.insert_event(Event::new(ButtonEvent::Pressed).target(self.subtract));
                    }

                    Code::NumpadAdd => {
                        state.insert_event(Event::new(ButtonEvent::Pressed).target(self.add));
                    }

                    Code::NumpadDivide => {
                        state.insert_event(Event::new(ButtonEvent::Pressed).target(self.divide));
                    }

                    Code::NumpadDecimal => {
                        state.insert_event(
                            Event::new(ButtonEvent::Pressed).target(self.decimal_point),
                        );
                    }

                    Code::Equal => {
                        if state.modifiers.shift {
                            state.insert_event(Event::new(ButtonEvent::Pressed).target(self.add));
                        } else {
                            state
                                .insert_event(Event::new(ButtonEvent::Pressed).target(self.equals));
                        }
                    }

                    Code::NumpadEnter | Code::Enter => {
                        state.insert_event(Event::new(ButtonEvent::Pressed).target(self.equals));
                    }

                    _ => {}
                },

                WindowEvent::KeyUp(code, key) => {
                    match code {
                        Code::Digit0 => {
                            state.insert_event(Event::new(ButtonEvent::Released).target(self.zero));
                        }

                        Code::Digit1 => {
                            state.insert_event(Event::new(ButtonEvent::Released).target(self.one));
                        }

                        Code::Digit2 => {
                            state.insert_event(Event::new(ButtonEvent::Released).target(self.two));
                        }

                        Code::Digit3 => {
                            state
                                .insert_event(Event::new(ButtonEvent::Released).target(self.three));
                        }

                        Code::Digit4 => {
                            state.insert_event(Event::new(ButtonEvent::Released).target(self.four));
                        }

                        Code::Digit5 => {
                            if state.modifiers.shift {
                                state.insert_event(
                                    Event::new(ButtonEvent::Released).target(self.percent),
                                );
                            } else {
                                state.insert_event(
                                    Event::new(ButtonEvent::Released).target(self.five),
                                );
                            }
                        }

                        Code::Digit6 => {
                            state.insert_event(Event::new(ButtonEvent::Released).target(self.six));
                        }

                        Code::Digit7 => {
                            state
                                .insert_event(Event::new(ButtonEvent::Released).target(self.seven));
                        }

                        Code::Digit8 => {
                            if state.modifiers.shift {
                                state.insert_event(
                                    Event::new(ButtonEvent::Released).target(self.multiply),
                                );
                            } else {
                                state.insert_event(
                                    Event::new(ButtonEvent::Released).target(self.eight),
                                );
                            }
                        }

                        Code::Digit9 => {
                            state.insert_event(Event::new(ButtonEvent::Released).target(self.nine));
                        }

                        Code::Escape => {
                            state.active = self.clear;
                            self.clear_all(state);
                        }

                        Code::NumpadMultiply => {
                            state.insert_event(
                                Event::new(ButtonEvent::Released).target(self.multiply),
                            );
                        }

                        Code::NumpadSubtract => {
                            state.insert_event(
                                Event::new(ButtonEvent::Released).target(self.subtract),
                            );
                        }

                        Code::NumpadAdd => {
                            state.insert_event(Event::new(ButtonEvent::Released).target(self.add));
                        }

                        Code::NumpadDivide => {
                            state.insert_event(
                                Event::new(ButtonEvent::Released).target(self.divide),
                            );
                        }

                        Code::NumpadDecimal => {
                            state.insert_event(
                                Event::new(ButtonEvent::Released).target(self.decimal_point),
                            );
                        }

                        Code::Equal => {
                            if state.modifiers.shift {
                                state.insert_event(
                                    Event::new(ButtonEvent::Released).target(self.add),
                                );
                            } else {
                                state.insert_event(
                                    Event::new(ButtonEvent::Released).target(self.equals),
                                );
                            }
                        }

                        Code::NumpadEnter | Code::Enter => {
                            state.insert_event(
                                Event::new(ButtonEvent::Released).target(self.equals),
                            );
                        }

                        _ => {}
                    }

                    //state.insert_event(Event::new(WindowEvent::Restyle).target(state.root));
                }

                _ => {}
            }
        }
    }
}

pub fn main() {
    // Replace this with icon loading using resource manager when working
    let icon = image::open("resources/icons/calculator_dark-128.png").unwrap();

    let mut app = Application::new(|win_desc, state, window| {
        state.add_theme(LIGHT_THEME);

        Calculator::default().build(state, window, |builder| builder.class("calculator"));

        win_desc
            .with_title("Calculator")
            .with_inner_size(300, 400)
            .with_min_inner_size(200, 300)
            .with_icon(icon.to_bytes(), icon.width(), icon.height())
    });

    app.run();
}
