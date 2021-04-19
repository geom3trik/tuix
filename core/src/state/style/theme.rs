use std::{fs::File, io::BufReader, io::Read, mem, path::Path, sync::Arc};

use cssparser::{
    self, AtRuleType, BasicParseError, BasicParseErrorKind, CowRcStr, DeclarationListParser,
    ParseError, ParseErrorKind, Parser, ParserInput, RuleListParser, SourceLocation, Token,
};

use crate::layout::{Align, Justify};

use crate::state::style::property::Property;
use crate::state::style::selector::{Relation, Selector};

use crate::state::animation::Transition;
use crate::state::style::StyleRule;

use crate::state::style::*;

use crate::state::style::color::Color;

#[derive(Clone, Debug)]
pub enum CustomParseError {
    InvalidLengthUnits(String),
    InvalidValue(String),
    InvalidColorName(String),
    InvalidColorHex(String),
    InvalidStringName(String),
}

impl<'t> From<CustomParseError> for ParseError<'t, CustomParseError> {
    fn from(e: CustomParseError) -> Self {
        ParseError {
            kind: ParseErrorKind::Custom(e),
            location: SourceLocation { line: 0, column: 0 },
        }
    }
}

pub struct RuleParser;

impl RuleParser {
    pub fn new() -> Self {
        RuleParser {}
    }
}

impl<'i> cssparser::QualifiedRuleParser<'i> for RuleParser {
    type Prelude = Vec<Selector>;
    type QualifiedRule = StyleRule;
    type Error = CustomParseError;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let res = parse_selectors(input)?;
        Ok(res)
    }

    fn parse_block<'t>(
        &mut self,
        selectors: Self::Prelude,
        location: SourceLocation,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let decl_parser = DeclarationParser {};

        let properties = DeclarationListParser::new(input, decl_parser)
            .filter_map(|property| property.ok())
            .collect::<Vec<_>>();

        Ok(StyleRule {
            selectors,
            properties,
        })
    }
}

impl<'i> cssparser::AtRuleParser<'i> for RuleParser {
    type PreludeBlock = ();
    type PreludeNoBlock = ();
    type AtRule = StyleRule;
    type Error = CustomParseError;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<AtRuleType<Self::PreludeNoBlock, Self::PreludeBlock>, ParseError<'i, Self::Error>>
    {
        match &*name {
            "keyframes" => {
                while let Ok(t) = input.next() {
                    match t {
                        Token::Ident(animation_name) => {}

                        t => {
                            let basic_error = BasicParseError {
                                kind: BasicParseErrorKind::UnexpectedToken(t.clone()),
                                location: input.current_source_location(),
                            };
                            return Err(basic_error.into());
                        }
                    }
                }
            }

            _ => {
                let token = input.next()?.to_owned();
                return Err(input.new_basic_unexpected_token_error(token).into());
            }
        }

        Ok(AtRuleType::WithBlock(()))
    }

    // TODO
    /*
    fn parse_block<'t>(
        &mut self,
        prelude: Self::PreludeBlock,
        location: SourceLocation,
        input: &mut Parser<'i, 't>
    ) -> Result<Self::AtRule, ParseError<'i, Self::Error>> {
        let rule_parser = RuleParser::new();
        let rule_list_parser = RuleListParser::new_for_nested_rule(input, rule_parser).collect::<Vec<_>>();

        // Keyframe rules
        for rule in &rule_list_parser {

        }
    }
    */
}

fn parse_selectors<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Vec<Selector>, ParseError<'i, CustomParseError>> {
    let mut selectors: Vec<Selector> = Vec::new();

    let mut selector = Selector::default();

    let mut first_token_in_selector = true;
    let mut whitespace = false;
    while let Ok(t) = input.next_including_whitespace() {
        match t {
            // Element
            Token::Ident(ref element_name) => {
                if whitespace {
                    selector.relation = Relation::Ancestor;
                    selectors.push(selector);
                    selector = Selector::default();
                    selector.set_element(&element_name.to_string());
                } else {
                    selector.set_element(&element_name.to_string());
                }

                whitespace = false;
            }

            Token::Delim('>') => {
                //let mut old_selector = Selector::from(&input.expect_ident()?.to_string());
                //mem::swap(&mut old_selector, &mut selector);
                selector.relation = Relation::Parent;
                selectors.push(selector);
                //selector = Selector::from(&input.expect_ident()?.to_string());
                selector = Selector::default();
                // if let Some(selec) = selectors.last_mut() {
                //     selec.relation = Relation::Parent;
                // }
                //selector.relation = Some(Box::new(SelectorRelation::Parent(old_selector)));
            }

            // Id
            Token::IDHash(ref id_name) => {
                selector.set_id(&id_name.to_string());
                whitespace = false;
            }

            // Any element
            Token::Delim('*') => {
                if whitespace {
                    selector.relation = Relation::Ancestor;
                    selectors.push(selector);
                    selector = Selector::default();
                    selector.asterisk = true;
                } else {
                    selector.asterisk = true;
                }

                whitespace = false;
            }

            // Class
            Token::Delim('.') => {
                if whitespace {
                    selector.relation = Relation::Ancestor;
                    selectors.push(selector);
                    selector = Selector::default();
                    selector
                        .classes
                        .insert(input.expect_ident()?.to_owned().to_string());
                } else {
                    selector
                        .classes
                        .insert(input.expect_ident()?.to_owned().to_string());
                }

                whitespace = false;
            }

            Token::WhiteSpace(ref ws) => {
                whitespace = true;
            }

            // Pseudo-class
            Token::Colon => {
                let pseudo_class_str = input.expect_ident()?.to_owned();

                match pseudo_class_str.as_ref() {
                    "hover" => selector.pseudo_classes.set_hover(true),
                    "active" => selector.pseudo_classes.set_active(true),
                    "focus" => selector.pseudo_classes.set_focus(true),
                    "enabled" => selector.pseudo_classes.set_enabled(true),
                    "disabled" => selector.pseudo_classes.set_disabled(true),
                    "checked" => selector.pseudo_classes.set_checked(true),

                    _ => {}
                }

                // let pseudo_class = match pseudo_class_str.as_ref() {
                //     "hover" => PseudoClass::Hover,
                //     "active" => PseudoClass::Active,
                //     "focus" => PseudoClass::Focus,
                //     "enabled" => PseudoClass::Enabled,
                //     "disabled" => PseudoClass::Disabled,
                //     "checked" => PseudoClass::Checked,
                //     "over" => PseudoClass::Over,
                //     _ => PseudoClass::None,
                // };

                // selector.pseudo_classes.insert(pseudo_class);
            }

            // This selector is done, on to the next one
            Token::Comma => {
                selectors.push(selector);
                selector = Selector::default();
                first_token_in_selector = true;
                continue; // need to continue to avoid `first_token_in_selector` being set to false
            }

            t => {
                let basic_error = BasicParseErrorKind::UnexpectedToken(t.to_owned());
                let parse_error = ParseError {
                    kind: ParseErrorKind::Basic(basic_error),
                    location: SourceLocation { line: 0, column: 0 },
                };
                return Err(parse_error);
            }
        }

        first_token_in_selector = false;
    }

    selectors.push(selector);

    // for selec in selectors.iter() {
    //     println!("{:?}", selec);
    // }

    // if selectors.iter().any(|sel| sel.relation.is_some()) {
    //     eprintln!("WARNING: Complex selector relations not implemented");
    // }

    Ok(selectors)
}

// fn parse_selector<'i,'t>(input: &mut Parser<'i,'t>) -> Result<Selector, ParseError<'i, CustomParseError>> {
//     let mut selector = Selector::default();

//     let token = input.next();

//     match token {
//         Token::Ident(ref element_name) => {
//             selector.set_element(&element_name.to_string());

//         }
//     }
// }

struct RGBDeclaration;

// impl<'i> cssparser::DeclarationParser<'i> for RGBDeclaration {
//     type Declaration = Color;
//     type Error = CustomParseError;

//     fn parse_block
// }

struct DeclarationParser;

impl<'i> cssparser::DeclarationParser<'i> for DeclarationParser {
    type Declaration = Property;
    type Error = CustomParseError;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        Ok(match &*name {
            // Colors
            "background-color" => Property::BackgroundColor(parse_color(input)?),
            "color" => Property::FontColor(parse_color(input)?),
            "background-image" => Property::BackgroundImage(parse_string(input)?),

            // Positioning
            "position" => Property::PositionType(parse_positioning_type(input)?),

            "left" => Property::Left(parse_units(input)?),
            "right" => Property::Right(parse_units(input)?),
            "top" => Property::Top(parse_units(input)?),
            "bottom" => Property::Bottom(parse_units(input)?),

            "min-left" => Property::MinLeft(parse_units(input)?),
            "max-left" => Property::MaxLeft(parse_units(input)?),
            "min-right" => Property::MinRight(parse_units(input)?),
            "max-right" => Property::MaxRight(parse_units(input)?),
            "min-top" => Property::MinTop(parse_units(input)?),
            "max-top" => Property::MaxTop(parse_units(input)?),
            "min-bottom" => Property::MinBottom(parse_units(input)?),
            "max-bottom" => Property::MaxBottom(parse_units(input)?),

            // Size
            "width" => Property::Width(parse_units(input)?),
            "height" => Property::Height(parse_units(input)?),

            // Size Constraints
            //TODO - Are percentages supported?
            "min-width" => Property::MinWidth(parse_units(input)?),
            "min-height" => Property::MinHeight(parse_units(input)?),
            "max-width" => Property::MaxWidth(parse_units(input)?),
            "max-height" => Property::MaxHeight(parse_units(input)?),

            "child-space" => Property::ChildSpace(parse_units(input)?),
            "child-left" => Property::ChildLeft(parse_units(input)?),
            "child-right" => Property::ChildRight(parse_units(input)?),
            "child-top" => Property::ChildTop(parse_units(input)?),
            "child-bottom" => Property::ChildBottom(parse_units(input)?),
            "child-between" => Property::ChildBetween(parse_units(input)?),

            "font-size" => Property::FontSize(parse_font_size(input)?),

            // Border
            "border-width" => Property::BorderWidth(parse_units(input)?),
            "border-color" => Property::BorderColor(parse_color(input)?),
            // TODO - Support array for specifying each corner
            "border-radius" => Property::BorderRadius(parse_units(input)?),

            "border-top-left-radius" => Property::BorderTopLeftRadius(parse_units(input)?),
            "border-top-right-radius" => Property::BorderTopRightRadius(parse_units(input)?),
            "border-bottom-left-radius" => Property::BorderBottomLeftRadius(parse_units(input)?),
            "border-bottom-right-radius" => Property::BorderBottomRightRadius(parse_units(input)?),

            "opacity" => Property::Opacity(parse_length_or_percentage(input)?),

            "display" => Property::Display(parse_display(input)?),
            "visibility" => Property::Visibility(parse_visibility(input)?),

            "overflow" => Property::Overflow(parse_overflow(input)?),

            "outer-shadow" => Property::OuterShadow(parse_box_shadow(input)?),
            "inner-shadow" => Property::InnerShadow(parse_box_shadow(input)?),

            "transition" => {
                Property::Transition(input.parse_comma_separated(|F| parse_transition2(F))?)
            }

            "z-index" => Property::ZIndex(parse_z_index(input)?),

            _ => {
                let basic_error = BasicParseError {
                    kind: BasicParseErrorKind::UnexpectedToken(input.next()?.to_owned()),
                    location: SourceLocation { line: 0, column: 0 },
                };
                return Err(basic_error.into());
            }
        })
    }
}

impl<'i> cssparser::AtRuleParser<'i> for DeclarationParser {
    type PreludeNoBlock = ();
    type PreludeBlock = ();
    type AtRule = Property;
    type Error = CustomParseError;
}

fn css_color(name: &str) -> Option<Color> {
    Some(match name {
        "transparent" => Color::from(name),

        "black" => Color::from("#000000"),
        "silver" => Color::from("#C0C0C0"),
        "gray" | "grey" => Color::from("#808080"),
        "white" => Color::from("#FFFFFF"),
        "maroon" => Color::from("#800000"),
        "red" => Color::from("#FF0000"),
        "purple" => Color::from("#800080"),
        "fuchsia" => Color::from("#FF00FF"),
        "green" => Color::from("#008000"),
        "lime" => Color::from("#00FF00"),
        "olive" => Color::from("#808000"),
        "yellow" => Color::from("#FFFF00"),
        "navy" => Color::from("#000080"),
        "blue" => Color::from("#0000FF"),
        "teal" => Color::from("#008080"),
        "aqua" => Color::from("#00FFFF"),
        _ => return None,
    })
}

fn css_string(name: &str) -> Option<String> {
    Some(String::from(name))
}

fn parse_string<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<String, ParseError<'i, CustomParseError>> {
    Ok(match input.next()? {
        Token::QuotedString(s) => match css_string(&s) {
            Some(string) => string,
            None => {
                return Err(CustomParseError::InvalidStringName(s.to_owned().to_string()).into())
            }
        },

        t => {
            let basic_error = BasicParseErrorKind::UnexpectedToken(t.to_owned());
            let parse_error = ParseError {
                kind: ParseErrorKind::Basic(basic_error),
                location: SourceLocation { line: 0, column: 0 },
            };
            return Err(parse_error);
        }
    })
}

fn parse_basic_color<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Color, ParseError<'i, CustomParseError>> {
    Ok(match input.next()? {
        Token::Ident(s) => match css_color(&s) {
            Some(color) => color,
            None => {
                return Err(CustomParseError::InvalidColorName(s.to_owned().to_string()).into());
            }
        },

        Token::IDHash(hash) | Token::Hash(hash) => Color::from(hash.to_owned().to_string()),

        t => {
            let basic_error = BasicParseErrorKind::UnexpectedToken(t.to_owned());
            let parse_error = ParseError {
                kind: ParseErrorKind::Basic(basic_error),
                location: SourceLocation { line: 0, column: 0 },
            };
            return Err(parse_error);
        }
    })
}

fn parse_length_or_percentage<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<f32, ParseError<'i, CustomParseError>> {
    Ok(match input.next()? {
        Token::Number { value: x, .. } => *x as f32,
        Token::Percentage { unit_value: x, .. } => *x as f32,

        Token::Dimension { value: x, .. } => *x as f32,
        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location: SourceLocation { line: 0, column: 0 },
            };
            return Err(basic_error.into());
        }
    })
}

fn parse_z_index<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<i32, ParseError<'i, CustomParseError>> {
    Ok(match input.next()? {
        Token::Number { value: x, .. } => *x as i32,
        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location: SourceLocation { line: 0, column: 0 },
            };
            return Err(basic_error.into());
        }
    })
}

//TODO
// fn parse_transition<'i, 't>(
//     input: &mut Parser<'i, 't>,
//     mut transition: Transition,
// ) -> Result<Transition, ParseError<'i, CustomParseError>> {
//     //let transition = Transition::default();

//     Ok(match input.next()? {
//         Token::Ident(s) => {
//             println!("Transition: {}", s);
//             transition.property = s.to_string();

//             match input.next()? {
//                 Token::Number { value: x, .. } => {
//                     println!("With duration: {}", x);
//                 }

//                 t => {
//                     let basic_error = BasicParseError {
//                         kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
//                         location: SourceLocation { line: 0, column: 0 },
//                     };
//                     return Err(basic_error.into());
//                 }
//             }

//             transition
//         }

//         t => {
//             let basic_error = BasicParseError {
//                 kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
//                 location: SourceLocation { line: 0, column: 0 },
//             };
//             return Err(basic_error.into());
//         }
//     })
// }

//TODO
fn parse_box_shadow<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<BoxShadow, ParseError<'i, CustomParseError>> {
    let mut box_shadow = BoxShadow::default();

    match parse_length2(input.next()?) {
        Ok(units) => {
            box_shadow.horizontal_offset = units;
            match parse_length2(input.next()?) {
                Ok(units) => {
                    box_shadow.vertical_offset = units;
                    let next_token = input.next()?;
                    match parse_length2(next_token) {
                        Ok(units) => {
                            box_shadow.blur_radius = units;

                            let next_token = input.next()?;
                            match parse_color2(next_token) {
                                Ok(color) => box_shadow.color = color,

                                _ => {}
                            }
                        }
                        _ => {
                            // Parse a color
                            match parse_color2(next_token) {
                                Ok(color) => box_shadow.color = color,

                                _ => {}
                            }
                        }
                    }
                }
                Err(error) => return Err(error),
            }
        }

        Err(error) => return Err(error),
    }

    Ok(box_shadow)

    // match token {
    //     Token::Number { value: x, .. } => {
    //         box_shadow.horizontal_offset = Units::Pixels(*x);

    //         match input.next()? {
    //             Token::Number { value: x, .. } => {
    //                 box_shadow.vertical_offset = Units::Pixels(*x);
    //             }

    //             t => {}
    //         }
    //     }

    //     Token::Percentage { unit_value: x, .. } => Units::Percentage(*x as f32),

    //     Token::Dimension { value: x, .. } => Units::Pixels(*x as f32),

    //     t => {
    //         let basic_error = BasicParseError {
    //             kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
    //             location: SourceLocation { line: 0, column: 0 },
    //         };
    //         return Err(basic_error.into());
    //     }
    // }

    // Ok(box_shadow)
}

fn parse_length2<'i>(token: &Token<'i>) -> Result<Units, ParseError<'i, CustomParseError>> {
    match token {
        Token::Number { value: x, .. } => Ok(Units::Pixels(*x as f32)),
        Token::Percentage { unit_value: x, .. } => Ok(Units::Percentage(*x as f32)),

        Token::Dimension { value: x, .. } => Ok(Units::Pixels(*x as f32)),
        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location: SourceLocation { line: 0, column: 0 },
            };
            return Err(basic_error.into());
        }
    }
}

fn parse_transition2<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Transition, ParseError<'i, CustomParseError>> {
    let mut transition = Transition::new();

    Ok(match input.next()? {
        Token::Ident(s) => {
            transition.property = s.to_string();

            match input.next()? {
                Token::Number { value: x, .. } => {
                    transition.duration = *x;

                    match input.next()? {
                        Token::Number { value: x, .. } => {
                            transition.delay = *x;
                        }

                        t => {
                            // let basic_error = BasicParseError {
                            //     kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                            //     location: SourceLocation { line: 0, column: 0 },
                            // };
                            // return Err(basic_error.into());
                        }
                    }
                }

                t => {
                    let basic_error = BasicParseError {
                        kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                        location: SourceLocation { line: 0, column: 0 },
                    };
                    return Err(basic_error.into());
                }
            }

            transition
        }

        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location: SourceLocation { line: 0, column: 0 },
            };
            return Err(basic_error.into());
        }
    })
}

fn parse_units<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Units, ParseError<'i, CustomParseError>> {
    Ok(match input.next()? {
        Token::Number { value: x, .. } => Units::Pixels(*x as f32),
        Token::Percentage { unit_value: x, .. } => Units::Percentage(*x as f32),


        Token::Dimension {has_sign: _, value: v, int_value: _, unit: u} if u == &"px" => {
            Units::Pixels(*v as f32)
        }

        Token::Dimension {has_sign: _, value: v, int_value: _, unit: u} if u == &"s" => {
            Units::Stretch(*v as f32)
        }

        Token::Ident(name) if name == &"auto" => {
            Units::Auto
        }
            
        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location: SourceLocation { line: 0, column: 0 },
            };
            return Err(basic_error.into());
        }
    })
}

fn parse_positioning_type<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<PositionType, ParseError<'i, CustomParseError>> {
    let location = input.current_source_location();

    Ok(match input.next()? {
        Token::Ident(name) => match name.as_ref() {
            "self-directed" => PositionType::SelfDirected,
            "parent-directed" => PositionType::ParentDirected,

            t => {
                return Err(
                    CustomParseError::InvalidStringName(name.to_owned().to_string()).into(),
                );
            }
        },

        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location,
            };
            return Err(basic_error.into());
        }
    })
}

fn parse_alignment<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Align, ParseError<'i, CustomParseError>> {
    let location = input.current_source_location();

    Ok(match input.next()? {
        Token::Ident(name) => match name.as_ref() {
            "start" | "top" => Align::Start,
            "center" | "centre" => Align::Center,
            "end" | "bottom" => Align::End,

            _ => {
                return Err(
                    CustomParseError::InvalidStringName(name.to_owned().to_string()).into(),
                );
            }
        },

        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location,
            };
            return Err(basic_error.into());
        }
    })
}

fn parse_justification<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Justify, ParseError<'i, CustomParseError>> {
    let location = input.current_source_location();

    Ok(match input.next()? {
        Token::Ident(name) => match name.as_ref() {
            "start" | "left" => Justify::Start,
            "center" | "centre" => Justify::Center,
            "end" | "right" => Justify::End,

            _ => {
                return Err(
                    CustomParseError::InvalidStringName(name.to_owned().to_string()).into(),
                );
            }
        },

        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location,
            };
            return Err(basic_error.into());
        }
    })
}

fn parse_display<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Display, ParseError<'i, CustomParseError>> {
    let location = input.current_source_location();

    Ok(match input.next()? {
        Token::Ident(name) => match name.as_ref() {
            "none" => Display::None,
            "flex" => Display::Flexbox,

            _ => {
                return Err(
                    CustomParseError::InvalidStringName(name.to_owned().to_string()).into(),
                );
            }
        },

        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location,
            };
            return Err(basic_error.into());
        }
    })
}

fn parse_visibility<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Visibility, ParseError<'i, CustomParseError>> {
    let location = input.current_source_location();

    Ok(match input.next()? {
        Token::Ident(name) => match name.as_ref() {
            "visible" => Visibility::Visible,
            "hidden" => Visibility::Invisible,

            _ => {
                return Err(
                    CustomParseError::InvalidStringName(name.to_owned().to_string()).into(),
                );
            }
        },

        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location,
            };
            return Err(basic_error.into());
        }
    })
}

fn parse_overflow<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Overflow, ParseError<'i, CustomParseError>> {
    let location = input.current_source_location();

    Ok(match input.next()? {
        Token::Ident(name) => match name.as_ref() {
            "visible" => Overflow::Visible,
            "hidden" => Overflow::Hidden,

            _ => {
                return Err(
                    CustomParseError::InvalidStringName(name.to_owned().to_string()).into(),
                );
            }
        },

        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location,
            };
            return Err(basic_error.into());
        }
    })
}

fn parse_flex_direction<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<FlexDirection, ParseError<'i, CustomParseError>> {
    let location = input.current_source_location();

    Ok(match input.next()? {
        Token::Ident(name) => match name.as_ref() {
            "row" => FlexDirection::Row,
            "column" => FlexDirection::Column,
            "row-reverse" => FlexDirection::RowReverse,
            "column-reverse" => FlexDirection::ColumnReverse,

            _ => {
                return Err(
                    CustomParseError::InvalidStringName(name.to_owned().to_string()).into(),
                );
            }
        },

        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location,
            };
            return Err(basic_error.into());
        }
    })
}

fn parse_justify_content<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<JustifyContent, ParseError<'i, CustomParseError>> {
    let location = input.current_source_location();

    Ok(match input.next()? {
        Token::Ident(name) => match name.as_ref() {
            "flex-start" => JustifyContent::FlexStart,
            "flex-end" => JustifyContent::FlexEnd,
            "center" => JustifyContent::Center,
            "space-between" => JustifyContent::SpaceBetween,
            "space-around" => JustifyContent::SpaceAround,
            "space-evenly" => JustifyContent::SpaceEvenly,

            _ => {
                return Err(
                    CustomParseError::InvalidStringName(name.to_owned().to_string()).into(),
                );
            }
        },

        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location,
            };
            return Err(basic_error.into());
        }
    })
}

fn parse_align_content<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<AlignContent, ParseError<'i, CustomParseError>> {
    let location = input.current_source_location();

    Ok(match input.next()? {
        Token::Ident(name) => match name.as_ref() {
            "flex-start" => AlignContent::FlexStart,
            "flex-end" => AlignContent::FlexEnd,
            "center" => AlignContent::Center,
            "space-between" => AlignContent::SpaceBetween,
            "space-around" => AlignContent::SpaceAround,
            "stretch" => AlignContent::Stretch,

            _ => {
                return Err(
                    CustomParseError::InvalidStringName(name.to_owned().to_string()).into(),
                );
            }
        },

        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location,
            };
            return Err(basic_error.into());
        }
    })
}

fn parse_align_items<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<AlignItems, ParseError<'i, CustomParseError>> {
    let location = input.current_source_location();

    Ok(match input.next()? {
        Token::Ident(name) => match name.as_ref() {
            "flex-start" => AlignItems::FlexStart,
            "flex-end" => AlignItems::FlexEnd,
            "center" => AlignItems::Center,
            "stretch" => AlignItems::Stretch,
            //"baseline" => AlignItems::Baseline, //TODO
            _ => {
                return Err(
                    CustomParseError::InvalidStringName(name.to_owned().to_string()).into(),
                );
            }
        },

        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location,
            };
            return Err(basic_error.into());
        }
    })
}

fn parse_align_self<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<AlignSelf, ParseError<'i, CustomParseError>> {
    let location = input.current_source_location();

    Ok(match input.next()? {
        Token::Ident(name) => match name.as_ref() {
            "flex-start" => AlignSelf::FlexStart,
            "flex-end" => AlignSelf::FlexEnd,
            "center" => AlignSelf::Center,
            "stretch" => AlignSelf::Stretch,

            _ => {
                return Err(
                    CustomParseError::InvalidStringName(name.to_owned().to_string()).into(),
                );
            }
        },

        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location,
            };
            return Err(basic_error.into());
        }
    })
}

// TODO
// fn parse_transform<'i,'t>(
//     input: &mut Parser<'i,'t>
// ) -> Result<Scale, ParseError<'i, CustomParseError>> {

// }

fn parse_color<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Color, ParseError<'i, CustomParseError>> {
    let location = input.current_source_location();

    Ok(match input.next()? {
        Token::Ident(name) => {
            // if input.try_parse(|input| input.expect_ident_matching("rgb")).is_ok() {
            //     if input.expect_parenthesis_block().is_ok() {
            //         input.parse_nested_block(parse: F)
            //     }
            // }

            match css_color(&name) {
                Some(color) => color,
                None => {
                    return Err(
                        CustomParseError::InvalidColorName(name.to_owned().to_string()).into(),
                    );
                }
            }
        }

        Token::IDHash(hash) | Token::Hash(hash) => Color::from(hash.to_owned().to_string()),

        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location,
            };
            return Err(basic_error.into());
        }
    })
}

fn parse_color2<'i>(token: &Token<'i>) -> Result<Color, ParseError<'i, CustomParseError>> {
    match token {
        Token::Ident(name) => {
            // if input.try_parse(|input| input.expect_ident_matching("rgb")).is_ok() {
            //     if input.expect_parenthesis_block().is_ok() {
            //         input.parse_nested_block(parse: F)
            //     }
            // }

            match css_color(&name) {
                Some(color) => Ok(color),
                None => {
                    return Err(
                        CustomParseError::InvalidColorName(name.to_owned().to_string()).into(),
                    );
                }
            }
        }

        Token::IDHash(hash) | Token::Hash(hash) => Ok(Color::from(hash.to_owned().to_string())),

        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location: SourceLocation { line: 0, column: 0 },
            };
            return Err(basic_error.into());
        }
    }
}

fn parse_font_size<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<f32, ParseError<'i, CustomParseError>> {
    let location = input.current_source_location();

    Ok(match input.next()? {
        Token::Ident(name) => match name.as_ref() {
            "medium" => 14.0,
            "xx-small" => 8.0,
            "x-small" => 10.0,
            "small" => 12.0,
            "large" => 16.0,
            "x-large" => 18.0,
            "xx-large" => 20.0,

            _ => {
                return Err(
                    CustomParseError::InvalidStringName(name.to_owned().to_string()).into(),
                );
            }
        },

        Token::Number { value: x, .. } => *x,
        Token::Percentage { unit_value: x, .. } => *x,

        Token::Dimension { value: x, .. } => *x,

        t => {
            let basic_error = BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.to_owned()),
                location,
            };
            return Err(basic_error.into());
        }
    })
}

pub fn parse(s: &str) -> Vec<StyleRule> {
    let mut input = ParserInput::new(s);
    let mut parser = Parser::new(&mut input);
    let rule_parser = RuleParser::new();

    let rules = {
        let rule_list_parser =
            cssparser::RuleListParser::new_for_stylesheet(&mut parser, rule_parser);
        rule_list_parser.collect::<Vec<_>>()
    };

    rules.into_iter().filter_map(|rule| rule.ok()).collect()
}
