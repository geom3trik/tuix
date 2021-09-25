#![allow(warnings)]
use cssparser::{Parser, ParserInput, RuleListParser};

use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use crate::tree::Tree;
use crate::state::storage::animatable_storage::AnimatableStorage;
use crate::state::storage::dense_storage::DenseStorage;
use crate::state::storage::style_storage::StyleStorage;
use crate::theme::StyleParseError;
use crate::{entity::Entity, Transition};

use crate::Interpolator;

use crate::state::animation::AnimationState;

pub mod themes;

pub mod theme;

pub mod prop;
pub use prop::{PropGet, PropSet};

pub mod layout;
pub use layout::*;

pub use morphorm::{LayoutType, PositionType, Units};
pub use Units::*;

pub mod units;

pub mod gradient;
pub use gradient::*;

pub mod shadow;
pub use shadow::*;

pub mod display;
pub use display::*;

pub mod property;
pub use property::*;

pub mod selector;
pub use selector::*;

pub mod specificity;
pub use specificity::*;

pub mod rule;
pub use rule::*;

pub mod color;
pub use color::Color;

pub mod transform;
pub use transform::*;

use std::rc::Rc;

// use bimap::BiMap;

#[derive(Clone, Default)]
pub struct Style {
    pub rules: Vec<StyleRule>,

    pub default_font: String,

    pub elements: DenseStorage<String>,
    pub classes: DenseStorage<HashSet<String>>,
    pub pseudo_classes: DenseStorage<PseudoClasses>,

    pub z_order: StyleStorage<i32>,

    // Transform
    pub rotate: AnimatableStorage<f32>,   
    pub translate: StyleStorage<(f32, f32)>,
    pub scale: StyleStorage<f32>,

    // General
    pub display: StyleStorage<Display>,
    pub visibility: StyleStorage<Visibility>,
    // Opacity
    pub opacity: AnimatableStorage<Opacity>,

    pub overflow: StyleStorage<Overflow>, // TODO
    //pub scroll: DenseStorage<Scroll>,     // TODO

    // Spacing
    pub left: AnimatableStorage<Units>,
    pub right: AnimatableStorage<Units>,
    pub top: AnimatableStorage<Units>,
    pub bottom: AnimatableStorage<Units>,

    // Size
    pub width: AnimatableStorage<Units>,
    pub height: AnimatableStorage<Units>,

    // Size Constraints
    pub max_width: AnimatableStorage<Units>,
    pub max_height: AnimatableStorage<Units>,
    pub min_width: AnimatableStorage<Units>,
    pub min_height: AnimatableStorage<Units>,

    // Spacing Constraints
    pub min_left: AnimatableStorage<Units>,    
    pub max_left: AnimatableStorage<Units>,
    pub min_right: AnimatableStorage<Units>,
    pub max_right: AnimatableStorage<Units>,
    pub min_top: AnimatableStorage<Units>,
    pub max_top: AnimatableStorage<Units>,
    pub min_bottom: AnimatableStorage<Units>,
    pub max_bottom: AnimatableStorage<Units>,
    

    // Border
    pub border_width: AnimatableStorage<Units>,
    pub border_color: AnimatableStorage<Color>,

    // Border Radius
    pub border_shape_top_left: StyleStorage<BorderCornerShape>,
    pub border_shape_top_right: StyleStorage<BorderCornerShape>,
    pub border_shape_bottom_left: StyleStorage<BorderCornerShape>,
    pub border_shape_bottom_right: StyleStorage<BorderCornerShape>,


    pub border_radius_top_left: AnimatableStorage<Units>,
    pub border_radius_top_right: AnimatableStorage<Units>,
    pub border_radius_bottom_left: AnimatableStorage<Units>,
    pub border_radius_bottom_right: AnimatableStorage<Units>,

    pub clip_widget: DenseStorage<Entity>,

    pub focus_order: DenseStorage<FocusOrder>,

    // Background
    pub background_color: AnimatableStorage<Color>,
    pub background_image: StyleStorage<Rc<()>>,
    pub background_gradient: StyleStorage<LinearGradient>,

    // Outer Shadow
    pub outer_shadow_h_offset: AnimatableStorage<Units>,
    pub outer_shadow_v_offset: AnimatableStorage<Units>,
    pub outer_shadow_blur: AnimatableStorage<Units>,
    pub outer_shadow_color: AnimatableStorage<Color>,

    // Inner Shadow
    pub inner_shadow_h_offset: AnimatableStorage<Units>,
    pub inner_shadow_v_offset: AnimatableStorage<Units>,
    pub inner_shadow_blur: AnimatableStorage<Units>,
    pub inner_shadow_color: AnimatableStorage<Color>,

    //Text & Font
    pub text: DenseStorage<String>,
    pub font: StyleStorage<String>,
    pub font_color: AnimatableStorage<Color>,
    pub font_size: AnimatableStorage<f32>,

    pub tooltip: DenseStorage<String>,

    // LAYOUT

    // Layout Type
    pub layout_type: StyleStorage<LayoutType>,

    // Positioning Type
    pub positioning_type: StyleStorage<PositionType>,

    // Grid
    pub grid_rows: StyleStorage<Vec<Units>>,
    pub row_between: AnimatableStorage<Units>,
    pub grid_cols: StyleStorage<Vec<Units>>,
    pub col_between: AnimatableStorage<Units>,

    pub row_index: StyleStorage<usize>,
    pub col_index: StyleStorage<usize>,
    pub row_span: StyleStorage<usize>,
    pub col_span: StyleStorage<usize>,

    // Child Spacing
    pub child_left: AnimatableStorage<Units>,
    pub child_right: AnimatableStorage<Units>,
    pub child_top: AnimatableStorage<Units>,
    pub child_bottom: AnimatableStorage<Units>,
    // pub child_wrap: AnimatableStorage<Units>,

    pub name: StyleStorage<String>,
}

impl Style {
    pub fn add_rule(&mut self, style_rule: StyleRule) {
        if !self.rules.contains(&style_rule) {
            self.rules.push(style_rule);
            self.rules.sort_by_key(|rule| rule.specificity());
            self.rules.reverse();
        }

        self.set_style_properties();
    }

    pub fn parse_theme(&mut self, stylesheet: &str) {
        let mut input = ParserInput::new(stylesheet);
        let mut parser = Parser::new(&mut input);
        let rule_parser = theme::RuleParser::new();

        let rules = {
            let rule_list_parser =
                cssparser::RuleListParser::new_for_stylesheet(&mut parser, rule_parser);
            rule_list_parser.collect::<Vec<_>>()
        };

        let mut rule_list: Vec<StyleRule> =
            rules.into_iter().filter_map(|rule| {
                match rule {
                    Ok(style_rule) => Some(style_rule),
                    Err(parse_error) => {
                        let style_parse_error = StyleParseError(parse_error.0);
                        println!("{}", style_parse_error);
                        None
                    }
                }
                //rule.ok()
        }).collect();

        self.rules.append(&mut rule_list);

        self.rules.sort_by_key(|rule| rule.specificity());
        self.rules.reverse();

        // for rule in self.rules.iter() {
        //     print!("{}", rule);
        // }

        self.remove_all();
        self.set_style_properties();
    }

    fn set_style_properties(&mut self) {
        for (rule_id, rule) in self.rules.iter().enumerate() {
            //let rule_id = self.rules.len();

            for property in rule.properties.clone() {
                match property {

                    Property::None => {
                        //
                    }

                    Property::Unknown(ident, prop) => {

                    }

                    Property::Display(value) => {
                        self.display.insert_rule(rule_id, value);
                    }

                    Property::Visibility(value) => {
                        self.visibility.insert_rule(rule_id, value);
                    }

                    Property::Opacity(value) => {
                        self.opacity.insert_rule(rule_id, Opacity(value));
                    }

                    Property::Overflow(value) => {
                        self.overflow.insert_rule(rule_id, value);
                    }

                    Property::BackgroundImage(value) => {
                        //
                    }

                    Property::BackgroundGradient(value) => {
                        self.background_gradient.insert_rule(rule_id, value);
                    }

                    Property::PositionType(value) => {
                        self.positioning_type.insert_rule(rule_id, value);
                    }

                    Property::Space(value) => {
                        self.left.insert_rule(rule_id, value);
                        self.right.insert_rule(rule_id, value);
                        self.top.insert_rule(rule_id, value);
                        self.bottom.insert_rule(rule_id, value);
                    }

                    Property::Left(value) => {
                        self.left.insert_rule(rule_id, value);
                    }

                    Property::Right(value) => {
                        self.right.insert_rule(rule_id, value);
                    }

                    Property::Top(value) => {
                        self.top.insert_rule(rule_id, value);
                    }

                    Property::Bottom(value) => {
                        self.bottom.insert_rule(rule_id, value);
                    }

                    // Position Constraints
                    Property::MinLeft(value) => {
                        self.min_left.insert_rule(rule_id, value);
                    }

                    Property::MaxLeft(value) => {
                        self.max_left.insert_rule(rule_id, value);
                    }

                    Property::MinRight(value) => {
                        self.min_right.insert_rule(rule_id, value);
                    }

                    Property::MaxRight(value) => {
                        self.max_right.insert_rule(rule_id, value);
                    }

                    Property::MinTop(value) => {
                        self.min_top.insert_rule(rule_id, value);
                    }

                    Property::MaxTop(value) => {
                        self.max_top.insert_rule(rule_id, value);
                    }

                    Property::MinBottom(value) => {
                        self.min_left.insert_rule(rule_id, value);
                    }

                    Property::MaxBottom(value) => {
                        self.max_left.insert_rule(rule_id, value);
                    }

                    // Size
                    Property::Width(value) => {
                        self.width.insert_rule(rule_id, value);
                    }

                    Property::Height(value) => {
                        self.height.insert_rule(rule_id, value);
                    }

                    // Size Constraints
                    Property::MaxWidth(value) => {
                        self.max_width.insert_rule(rule_id, value);
                    }

                    Property::MinWidth(value) => {
                        self.min_width.insert_rule(rule_id, value);
                    }

                    Property::MaxHeight(value) => {
                        self.max_height.insert_rule(rule_id, value);
                    }

                    Property::MinHeight(value) => {
                        self.min_height.insert_rule(rule_id, value);
                    }

                    // Border
                    Property::BorderWidth(value) => {
                        self.border_width.insert_rule(rule_id, value);
                    }

                    Property::BorderColor(value) => {
                        self.border_color.insert_rule(rule_id, value);
                    }

                    Property::BorderCornerShape(shape) => {
                        self.border_shape_top_left.insert_rule(rule_id, shape);
                        self.border_shape_top_right.insert_rule(rule_id, shape);
                        self.border_shape_bottom_left.insert_rule(rule_id, shape);
                        self.border_shape_bottom_right.insert_rule(rule_id, shape);
                    }

                    Property::BorderTopLeftShape(shape) => {
                        self.border_shape_top_left.insert_rule(rule_id, shape);
                    }

                    Property::BorderTopRightShape(shape) => {
                        self.border_shape_top_right.insert_rule(rule_id, shape);
                    }

                    Property::BorderBottomLeftShape(shape) => {
                        self.border_shape_bottom_left.insert_rule(rule_id, shape);
                    }

                    Property::BorderBottomRightShape(shape) => {
                        self.border_shape_bottom_right.insert_rule(rule_id, shape);
                    }

                    // Border Radius
                    Property::BorderRadius(value) => {
                        self.border_radius_top_left.insert_rule(rule_id, value);
                        self.border_radius_top_right.insert_rule(rule_id, value);
                        self.border_radius_bottom_left.insert_rule(rule_id, value);
                        self.border_radius_bottom_right.insert_rule(rule_id, value);
                    }

                    Property::BorderTopLeftRadius(value) => {
                        self.border_radius_top_left.insert_rule(rule_id, value);
                    }

                    Property::BorderTopRightRadius(value) => {
                        self.border_radius_top_right.insert_rule(rule_id, value);
                    }

                    Property::BorderBottomLeftRadius(value) => {
                        self.border_radius_bottom_left.insert_rule(rule_id, value);
                    }

                    Property::BorderBottomRightRadius(value) => {
                        self.border_radius_bottom_right.insert_rule(rule_id, value);
                    }

                    // Font
                    Property::FontSize(value) => {
                        self.font_size.insert_rule(rule_id, value);
                    }

                    Property::FontColor(value) => {
                        self.font_color.insert_rule(rule_id, value);
                    }

                    Property::Font(value) => {
                        self.font.insert_rule(rule_id, value);
                    }

                    // Background
                    Property::BackgroundColor(value) => {
                        self.background_color.insert_rule(rule_id, value);
                    }

                    // Property::BackgroundImage(value) => {
                    //     self.background_image.insert_rule(rule_id, value);
                    // }

                    // Layout
                    Property::LayoutType(value) => {
                        self.layout_type.insert_rule(rule_id, value);
                    }

                    Property::ZIndex(value) => {
                        self.z_order.insert_rule(rule_id, value);
                    }

                    // Outer Shadow
                    Property::OuterShadow(box_shadow) => {
                        self.outer_shadow_h_offset
                            .insert_rule(rule_id, box_shadow.horizontal_offset);
                        self.outer_shadow_v_offset
                            .insert_rule(rule_id, box_shadow.vertical_offset);
                        self.outer_shadow_blur
                            .insert_rule(rule_id, box_shadow.blur_radius);
                        self.outer_shadow_color
                            .insert_rule(rule_id, box_shadow.color);
                    }

                    Property::OuterShadowColor(color) => {
                        self.outer_shadow_color.insert_rule(rule_id, color);
                    }

                    // Inner Shadow
                    Property::InnerShadow(box_shadow) => {
                        self.inner_shadow_h_offset
                            .insert_rule(rule_id, box_shadow.horizontal_offset);
                        self.inner_shadow_v_offset
                            .insert_rule(rule_id, box_shadow.vertical_offset);
                        self.inner_shadow_blur
                            .insert_rule(rule_id, box_shadow.blur_radius);
                        self.inner_shadow_color
                            .insert_rule(rule_id, box_shadow.color);
                    }

                    // Child Spacing
                    Property::ChildLeft(value) => {
                        self.child_left.insert_rule(rule_id, value);
                    }

                    Property::ChildRight(value) => {
                        self.child_right.insert_rule(rule_id, value);
                    }

                    Property::ChildTop(value) => {
                        self.child_top.insert_rule(rule_id, value);
                    }

                    Property::ChildBottom(value) => {
                        self.child_bottom.insert_rule(rule_id, value);
                    }

                    Property::ChildSpace(value) => {
                        self.child_left.insert_rule(rule_id, value);
                        self.child_right.insert_rule(rule_id, value);
                        self.child_top.insert_rule(rule_id, value);
                        self.child_bottom.insert_rule(rule_id, value);
                    }

                    Property::RowBetween(value) => {
                        self.row_between.insert_rule(rule_id, value);
                    }

                    Property::ColBetween(value) => {
                        self.col_between.insert_rule(rule_id, value);
                    }

                    // Transitions
                    Property::Transition(transitions) => {
                        for transition in transitions {
                            println!("{:?}", transition);
                            match transition.property.as_ref() {
                                "background-color" => {
                                    self.background_color.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "left" => {
                                    self.left.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "top" => {
                                    self.top.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "right" => {
                                    self.right.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "bottom" => {
                                    self.bottom.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "min-left" => {
                                    self.min_left.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "max-left" => {
                                    self.max_left.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "min-right" => {
                                    self.min_right.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "max-right" => {
                                    self.max_right.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "min-top" => {
                                    self.min_top.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "max-top" => {
                                    self.max_top.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "min-bottom" => {
                                    self.min_bottom.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "max-bottom" => {
                                    self.max_bottom.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "width" => {
                                    self.width.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "height" => {
                                    self.height.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "min-width" => {
                                    self.min_width.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "max-width" => {
                                    self.max_width.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "min-height" => {
                                    self.min_height.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "max-height" => {
                                    self.max_height.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }
                                "opacity" => {
                                    self.opacity.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                "outer-shadow-color" => {
                                    self.outer_shadow_color.insert_transition(
                                        rule_id,
                                        self.add_transition(transition),
                                    );
                                }

                                _ => {}
                            }
                        }
                    }

                    _=> {}
                }
            }
        }
    }

    fn add_transition<T: Default + Interpolator>(
        &self,
        transition: Transition,
    ) -> AnimationState<T> {
        AnimationState::new()
            .with_duration(std::time::Duration::from_secs_f32(transition.duration))
            .with_delay(std::time::Duration::from_secs_f32(transition.delay))
            .with_keyframe((0.0, Default::default()))
            .with_keyframe((1.0, Default::default()))
    }

    /*
    pub fn parse_theme2(&mut self, stylesheet: &str) {
        let mut input = ParserInput::new(stylesheet);
        let mut parser = Parser::new(&mut input);
        let rule_parser = theme::RuleParser::new();

        let rules = {
            let rule_list_parser =
                cssparser::RuleListParser::new_for_stylesheet(&mut parser, rule_parser);
            rule_list_parser.collect::<Vec<_>>()
        };

        let mut rule_list: Vec<StyleRule> =
            rules.into_iter().filter_map(|rule| rule.ok()).collect();
        rule_list.sort_by_key(|rule| rule.specificity());
        rule_list.reverse();

        for rule in rule_list.iter() {
            let rule_id = self.rules.len();
            //println!("Rule: {}, Specificity: {:?}, rule: {:?}", rule_id, rule.specificity(), rule);
            //self.rule_selectors.push(rule.selectors.clone());
            self.rules.push(rule.clone());
            //self.rules.push(rule_id);
            for property in rule.properties.clone() {
                match property {
                    Property::Display(value) => {
                        self.display.insert_rule(rule_id, value);
                    }

                    Property::Visibility(value) => {
                        self.visibility.insert_rule(rule_id, value);
                    }

                    Property::Opacity(value) => {
                        self.opacity.insert_rule(rule_id, Opacity(value));
                    }

                    Property::Overflow(value) => {
                        self.overflow.insert_rule(rule_id, value);
                    }

                    Property::TextAlign(value) => {
                        self.text_align.insert_rule(rule_id, value);
                    }

                    Property::TextJustify(value) => {
                        self.text_justify.insert_rule(rule_id, value);
                    }

                    Property::Position(value) => {
                        self.position.insert_rule(rule_id, value);
                    }

                    Property::Left(value) => {
                        self.left.insert_rule(rule_id, value);
                    }

                    Property::Right(value) => {
                        self.right.insert_rule(rule_id, value);
                    }

                    Property::Top(value) => {
                        self.top.insert_rule(rule_id, value);
                    }

                    Property::Bottom(value) => {
                        self.bottom.insert_rule(rule_id, value);
                    }

                    Property::Width(value) => {
                        self.width.insert_rule(rule_id, value);
                    }

                    Property::Height(value) => {
                        self.height.insert_rule(rule_id, value);
                    }

                    Property::MaxWidth(value) => {
                        self.max_width.insert_rule(rule_id, value);
                    }

                    Property::MinWidth(value) => {
                        self.min_width.insert_rule(rule_id, value);
                    }

                    Property::MaxHeight(value) => {
                        self.max_height.insert_rule(rule_id, value);
                    }

                    Property::MinHeight(value) => {
                        self.min_height.insert_rule(rule_id, value);
                    }

                    Property::Padding(value) => {
                        self.padding_left.insert_rule(rule_id, value);
                        self.padding_right.insert_rule(rule_id, value);
                        self.padding_top.insert_rule(rule_id, value);
                        self.padding_bottom.insert_rule(rule_id, value);
                    }

                    Property::PaddingLeft(value) => {
                        self.padding_left.insert_rule(rule_id, value);
                    }

                    Property::PaddingRight(value) => {
                        self.padding_right.insert_rule(rule_id, value);
                    }

                    Property::PaddingTop(value) => {
                        self.padding_top.insert_rule(rule_id, value);
                    }

                    Property::PaddingBottom(value) => {
                        self.padding_bottom.insert_rule(rule_id, value);
                    }

                    // Border
                    Property::BorderWidth(value) => {
                        self.border_width.insert_rule(rule_id, value);
                    }

                    Property::BorderColor(value) => {
                        self.border_color.insert_rule(rule_id, value);
                    }

                    Property::BorderRadius(value) => {
                        self.border_radius_top_left.insert_rule(rule_id, value);
                        self.border_radius_top_right.insert_rule(rule_id, value);
                        self.border_radius_bottom_left.insert_rule(rule_id, value);
                        self.border_radius_bottom_right.insert_rule(rule_id, value);
                    }

                    Property::BorderTopLeftRadius(value) => {
                        self.border_radius_top_left.insert_rule(rule_id, value);
                    }

                    Property::BorderTopRightRadius(value) => {
                        self.border_radius_top_right.insert_rule(rule_id, value);
                    }

                    Property::BorderBottomLeftRadius(value) => {
                        self.border_radius_bottom_left.insert_rule(rule_id, value);
                    }

                    Property::BorderBottomRightRadius(value) => {
                        self.border_radius_bottom_right.insert_rule(rule_id, value);
                    }

                    Property::FontSize(value) => {
                        self.font_size.insert_rule(rule_id, value);
                    }

                    Property::FontColor(value) => {
                        self.font_color.insert_rule(rule_id, value);
                    }

                    Property::BackgroundColor(value) => {
                        self.background_color.insert_rule(rule_id, value);
                    }

                    // Property::BackgroundImage(value) => {
                    //     self.background_image.insert_rule(rule_id, value);
                    // }

                    // Flex Container
                    Property::FlexDirection(value) => {
                        self.flex_direction.insert_rule(rule_id, value);
                    }
                    Property::JustifyContent(value) => {
                        self.justify_content.insert_rule(rule_id, value);
                    }
                    Property::AlignContent(value) => {
                        self.align_content.insert_rule(rule_id, value);
                    }
                    Property::AlignItems(value) => {
                        self.align_items.insert_rule(rule_id, value);
                    }

                    Property::AlignSelf(value) => {
                        self.align_self.insert_rule(rule_id, value);
                    }

                    // Flex Item
                    Property::FlexGrow(value) => {
                        self.flex_grow.insert_rule(rule_id, value);
                    }

                    Property::FlexShrink(value) => {
                        self.flex_shrink.insert_rule(rule_id, value);
                    }

                    Property::FlexBasis(value) => {
                        self.flex_basis.insert_rule(rule_id, value);
                    }

                    Property::ZIndex(value) => {
                        self.z_order.insert_rule(rule_id, value);
                    }

                    Property::OuterShadow(box_shadow) => {
                        self.outer_shadow_h_offset
                            .insert_rule(rule_id, box_shadow.horizontal_offset);
                        self.outer_shadow_v_offset
                            .insert_rule(rule_id, box_shadow.vertical_offset);
                        self.outer_shadow_blur
                            .insert_rule(rule_id, box_shadow.blur_radius);
                        self.outer_shadow_color
                            .insert_rule(rule_id, box_shadow.color);
                    }

                    Property::InnerShadow(box_shadow) => {
                        self.inner_shadow_h_offset
                            .insert_rule(rule_id, box_shadow.horizontal_offset);
                        self.inner_shadow_v_offset
                            .insert_rule(rule_id, box_shadow.vertical_offset);
                        self.inner_shadow_blur
                            .insert_rule(rule_id, box_shadow.blur_radius);
                        self.inner_shadow_color
                            .insert_rule(rule_id, box_shadow.color);
                    }

                    Property::Transition(transitions) => {
                        for transition in transitions {
                            match transition.property.as_ref() {
                                "background-color" => {
                                    self.background_color.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(
                                                transition.delay,
                                            ))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "flex-basis" => {
                                    self.flex_basis.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(
                                                transition.delay,
                                            ))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "left" => {
                                    self.left.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(
                                                transition.delay,
                                            ))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "top" => {
                                    self.top.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(
                                                transition.delay,
                                            ))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "right" => {
                                    self.right.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(
                                                transition.delay,
                                            ))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "bottom" => {
                                    self.bottom.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(
                                                transition.delay,
                                            ))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "width" => {
                                    self.width.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(
                                                transition.delay,
                                            ))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "height" => {
                                    self.height.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(
                                                transition.delay,
                                            ))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "margin-bottom" => {
                                    self.margin_bottom.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(
                                                transition.delay,
                                            ))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "margin-top" => {
                                    self.margin_top.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(
                                                transition.delay,
                                            ))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "margin-left" => {
                                    self.margin_left.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(
                                                transition.delay,
                                            ))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "margin-right" => {
                                    self.margin_right.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(
                                                transition.delay,
                                            ))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "padding-left" => {
                                    self.padding_left.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(
                                                transition.delay,
                                            ))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "padding-right" => {
                                    self.padding_right.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(
                                                transition.delay,
                                            ))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "padding-top" => {
                                    self.padding_top.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(
                                                transition.delay,
                                            ))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "padding-bottom" => {
                                    self.padding_bottom.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(
                                                transition.delay,
                                            ))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                "opacity" => {
                                    self.opacity.insert_transition(
                                        rule_id,
                                        AnimationState::new()
                                            .with_duration(std::time::Duration::from_secs_f32(
                                                transition.duration,
                                            ))
                                            .with_delay(std::time::Duration::from_secs_f32(
                                                transition.delay,
                                            ))
                                            .with_keyframe((0.0, Default::default()))
                                            .with_keyframe((1.0, Default::default())),
                                    );
                                }

                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    */

    // TODO
    pub fn set_property(&mut self, entity: Entity, propert: Property) {}

    // Add style data to an entity
    pub(crate) fn add(&mut self, entity: Entity) {
        self.pseudo_classes.insert(entity, PseudoClasses::default());

        //self.z_order.insert(entity, 0);

        self.overflow.insert(entity, Default::default());
        //self.scroll.insert(entity, Default::default());

        self.visibility.insert(entity, Default::default());
        self.focus_order.insert(entity, Default::default());
    }

    pub fn remove(&mut self, entity: Entity) {}

    pub fn remove_all(&mut self) {
        // Remove all non-inline style data
        self.background_color.remove_styles();
        self.font_color.remove_styles();

        // Position
        self.left.remove_styles();
        self.right.remove_styles();
        self.top.remove_styles();
        self.bottom.remove_styles();

        // Position Constraints
        self.min_left.remove_styles();
        self.max_left.remove_styles();
        self.min_right.remove_styles();
        self.max_right.remove_styles();
        self.min_top.remove_styles();
        self.max_top.remove_styles();
        self.min_bottom.remove_styles();
        self.max_bottom.remove_styles();

        // Size
        self.width.remove_styles();
        self.height.remove_styles();

        // Size Constraints
        self.min_width.remove_styles();
        self.max_width.remove_styles();
        self.min_height.remove_styles();
        self.max_height.remove_styles();

        // Border
        self.border_width.remove_styles();
        self.border_color.remove_styles();

        // Border Radius
        self.border_radius_top_left.remove_styles();
        self.border_radius_top_right.remove_styles();
        self.border_radius_bottom_left.remove_styles();
        self.border_radius_bottom_right.remove_styles();

        // Display
        self.display.remove_styles();
        self.visibility.remove_styles();
        self.opacity.remove_styles();

        // Inner Shadow
        self.inner_shadow_h_offset.remove_styles();
        self.inner_shadow_v_offset.remove_styles();
        self.inner_shadow_blur.remove_styles();
        self.inner_shadow_color.remove_styles();

        // Outer Shadow
        self.outer_shadow_h_offset.remove_styles();
        self.outer_shadow_v_offset.remove_styles();
        self.outer_shadow_blur.remove_styles();
        self.outer_shadow_color.remove_styles();
    }
}
