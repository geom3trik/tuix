
//! Style Data
//!
//! The [Style] struct is responsible for storing all of the style properties for all of the entities,
//! as well as storing style rule definitions created by the user or parsed from stylesheets.

use cssparser::{Parser, ParserInput};

use std::collections::HashSet;

use crate::{Animation, IdManager};
use crate::{Entity, Transition};

use crate::Interpolator;

use crate::AnimationState;

mod themes;

mod theme;
pub use theme::StyleParseError;

mod prop;
pub use prop::{PropGet, PropSet};

mod prop_set;
pub use prop_set::PropSet2;

mod layout;
pub use layout::*;

pub use morphorm::{LayoutType, PositionType, Units};
pub use Units::*;

mod units;

mod gradient;
pub use gradient::*;

mod shadow;
pub(crate) use shadow::*;

mod display;
pub use display::*;

mod property;
pub use property::*;

mod selector;
pub use selector::*;

mod specificity;
pub(crate) use specificity::*;

mod style_rule;
pub(crate) use style_rule::*;

mod rule;
pub use rule::Rule;

mod color;
pub use color::Color;

mod transform;
pub use transform::*;

use std::rc::Rc;

use super::storage::animatable_set::AnimatableSet;
use super::storage::sparse_set::SparseSet;
use super::storage::style_set::StyleSet;

// use bimap::BiMap;


/// Stores the style properties of all entities in the application. To set properties on entities see the [PropSet] trait.
#[derive(Default)]
pub struct Style {

    pub(crate) rule_manager: IdManager<Rule>, 

    /// Creates and destroys animation ids
    pub(crate) animation_manager: IdManager<Animation>,

    pub(crate) rules: Vec<StyleRule>,

    pub default_font: String,

    pub elements: SparseSet<String>,
    pub classes: SparseSet<HashSet<String>>,
    pub pseudo_classes: SparseSet<PseudoClass>,

    // Display
    pub display: AnimatableSet<Display>,
    
    // Visibility
    pub visibility: AnimatableSet<Visibility>,

    // Opacity
    pub opacity: AnimatableSet<Opacity>,    

    // Z Order
    pub z_order: StyleSet<i32>,

    // Clipping
    pub clip_widget: SparseSet<Entity>,

    // Transform
    pub rotate: AnimatableSet<f32>,   
    pub translate: StyleSet<(f32, f32)>,
    pub scale: AnimatableSet<f32>,

    pub overflow: StyleSet<Overflow>, // TODO
    //pub scroll: DenseStorage<Scroll>,     // TODO


    // Border
    pub border_width: AnimatableSet<Units>,
    pub border_color: AnimatableSet<Color>,

    // Border Shape
    pub border_shape_top_left: StyleSet<BorderCornerShape>,
    pub border_shape_top_right: StyleSet<BorderCornerShape>,
    pub border_shape_bottom_left: StyleSet<BorderCornerShape>,
    pub border_shape_bottom_right: StyleSet<BorderCornerShape>,

    // Border Radius
    pub border_radius_top_left: AnimatableSet<Units>,
    pub border_radius_top_right: AnimatableSet<Units>,
    pub border_radius_bottom_left: AnimatableSet<Units>,
    pub border_radius_bottom_right: AnimatableSet<Units>,



    // Focus Order
    pub focus_order: SparseSet<FocusOrder>,

    // Background
    pub background_color: AnimatableSet<Color>,
    pub background_image: StyleSet<Rc<()>>,
    pub background_gradient: StyleSet<LinearGradient>,

    // Outer Shadow
    pub outer_shadow_h_offset: AnimatableSet<Units>,
    pub outer_shadow_v_offset: AnimatableSet<Units>,
    pub outer_shadow_blur: AnimatableSet<Units>,
    pub outer_shadow_color: AnimatableSet<Color>,

    // Inner Shadow (TODO)
    pub inner_shadow_h_offset: AnimatableSet<Units>,
    pub inner_shadow_v_offset: AnimatableSet<Units>,
    pub inner_shadow_blur: AnimatableSet<Units>,
    pub inner_shadow_color: AnimatableSet<Color>,

    //Text & Font
    pub text: StyleSet<String>,
    pub font: StyleSet<String>,
    pub font_color: AnimatableSet<Color>,
    pub font_size: AnimatableSet<f32>,

    pub tooltip: SparseSet<String>,

    // LAYOUT

    // Layout Type
    pub layout_type: StyleSet<LayoutType>,

    // Positioning Type
    pub positioning_type: StyleSet<PositionType>,

    // Spacing
    pub left: AnimatableSet<Units>,
    pub right: AnimatableSet<Units>,
    pub top: AnimatableSet<Units>,
    pub bottom: AnimatableSet<Units>,

    // Size
    pub width: AnimatableSet<Units>,
    pub height: AnimatableSet<Units>,

    // Size Constraints
    pub max_width: AnimatableSet<Units>,
    pub max_height: AnimatableSet<Units>,
    pub min_width: AnimatableSet<Units>,
    pub min_height: AnimatableSet<Units>,

    // Spacing Constraints
    pub min_left: AnimatableSet<Units>,    
    pub max_left: AnimatableSet<Units>,
    pub min_right: AnimatableSet<Units>,
    pub max_right: AnimatableSet<Units>,
    pub min_top: AnimatableSet<Units>,
    pub max_top: AnimatableSet<Units>,
    pub min_bottom: AnimatableSet<Units>,
    pub max_bottom: AnimatableSet<Units>,

    // Grid
    pub grid_rows: StyleSet<Vec<Units>>,
    pub row_between: AnimatableSet<Units>,
    pub grid_cols: StyleSet<Vec<Units>>,
    pub col_between: AnimatableSet<Units>,

    pub row_index: StyleSet<usize>,
    pub col_index: StyleSet<usize>,
    pub row_span: StyleSet<usize>,
    pub col_span: StyleSet<usize>,

    // Child Spacing
    pub child_left: AnimatableSet<Units>,
    pub child_right: AnimatableSet<Units>,
    pub child_top: AnimatableSet<Units>,
    pub child_bottom: AnimatableSet<Units>,


    pub name: StyleSet<String>,
}

impl Style {
    // pub(crate) fn add_rule(&mut self, style_rule: StyleRule) {
    //     if !self.rules.contains(&style_rule) {
    //         self.rules.push(style_rule);
    //         self.rules.sort_by_key(|rule| rule.specificity());
    //         self.rules.reverse();
    //     }

    //     self.set_style_properties();
    // }

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
                    Ok(mut style_rule) => {
                        style_rule.id = self.rule_manager.create();
                        Some(style_rule)
                    },
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
        for rule in self.rules.iter() {
            let rule_id = rule.id;

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

                    Property::BackgroundImage(_value) => {
                        todo!();
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
                        match value {
                            Percentage(val) => {
                                println!("{:?} {}", rule, val);
                            }

                            _=> {}
                        }
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
                                    let animation = self.animation_manager.create();
                                    self.background_color.insert_animation(animation, self.add_transition(transition));
                                    self.background_color.insert_transition(rule_id, animation);
                                }

                                "left" => {
                                    let animation = self.animation_manager.create();
                                    self.left.insert_animation(animation, self.add_transition(transition));
                                    self.left.insert_transition(rule_id, animation);
                                }

                                "top" => {
                                    let animation = self.animation_manager.create();
                                    self.top.insert_animation(animation, self.add_transition(transition));
                                    self.top.insert_transition(rule_id, animation);
                                }

                                "right" => {
                                    let animation = self.animation_manager.create();
                                    self.right.insert_animation(animation, self.add_transition(transition));
                                    self.right.insert_transition(rule_id, animation);
                                }

                                "bottom" => {
                                    let animation = self.animation_manager.create();
                                    self.bottom.insert_animation(animation, self.add_transition(transition));
                                    self.bottom.insert_transition(rule_id, animation);
                                }

                                "min-left" => {
                                    let animation = self.animation_manager.create();
                                    self.min_left.insert_animation(animation, self.add_transition(transition));
                                    self.min_left.insert_transition(rule_id, animation);
                                }

                                "max-left" => {
                                    let animation = self.animation_manager.create();
                                    self.max_left.insert_animation(animation, self.add_transition(transition));
                                    self.max_left.insert_transition(rule_id, animation);
                                }

                                "min-right" => {
                                    let animation = self.animation_manager.create();
                                    self.min_right.insert_animation(animation, self.add_transition(transition));
                                    self.min_right.insert_transition(rule_id, animation);
                                }

                                "max-right" => {
                                    let animation = self.animation_manager.create();
                                    self.max_right.insert_animation(animation, self.add_transition(transition));
                                    self.max_right.insert_transition(rule_id, animation);
                                }

                                "min-top" => {
                                    let animation = self.animation_manager.create();
                                    self.min_top.insert_animation(animation, self.add_transition(transition));
                                    self.min_top.insert_transition(rule_id, animation);
                                }

                                "max-top" => {
                                    let animation = self.animation_manager.create();
                                    self.max_top.insert_animation(animation, self.add_transition(transition));
                                    self.max_top.insert_transition(rule_id, animation);
                                }

                                "min-bottom" => {
                                    let animation = self.animation_manager.create();
                                    self.min_bottom.insert_animation(animation, self.add_transition(transition));
                                    self.min_bottom.insert_transition(rule_id, animation);
                                }

                                "max-bottom" => {
                                    let animation = self.animation_manager.create();
                                    self.max_bottom.insert_animation(animation, self.add_transition(transition));
                                    self.max_bottom.insert_transition(rule_id, animation);
                                }

                                "width" => {
                                    let animation = self.animation_manager.create();
                                    self.width.insert_animation(animation, self.add_transition(transition));
                                    self.width.insert_transition(rule_id, animation);
                                }

                                "height" => {
                                    let animation = self.animation_manager.create();
                                    self.height.insert_animation(animation, self.add_transition(transition));
                                    self.height.insert_transition(rule_id, animation);
                                }

                                "min-width" => {
                                    let animation = self.animation_manager.create();
                                    self.min_width.insert_animation(animation, self.add_transition(transition));
                                    self.min_width.insert_transition(rule_id, animation);
                                }

                                "max-width" => {
                                    let animation = self.animation_manager.create();
                                    self.max_width.insert_animation(animation, self.add_transition(transition));
                                    self.max_width.insert_transition(rule_id, animation);
                                }

                                "min-height" => {
                                    let animation = self.animation_manager.create();
                                    self.min_height.insert_animation(animation, self.add_transition(transition));
                                    self.min_height.insert_transition(rule_id, animation);
                                }

                                "max-height" => {
                                    let animation = self.animation_manager.create();
                                    self.max_height.insert_animation(animation, self.add_transition(transition));
                                    self.max_height.insert_transition(rule_id, animation);
                                }

                                "child-left" => {
                                    let animation = self.animation_manager.create();
                                    self.child_left.insert_animation(animation, self.add_transition(transition));
                                    self.child_left.insert_transition(rule_id, animation);
                                }

                                "child-right" => {
                                    let animation = self.animation_manager.create();
                                    self.child_right.insert_animation(animation, self.add_transition(transition));
                                    self.child_right.insert_transition(rule_id, animation);
                                }

                                "child-top" => {
                                    let animation = self.animation_manager.create();
                                    self.child_top.insert_animation(animation, self.add_transition(transition));
                                    self.child_top.insert_transition(rule_id, animation);
                                }

                                "child-bottom" => {
                                    let animation = self.animation_manager.create();
                                    self.child_bottom.insert_animation(animation, self.add_transition(transition));
                                    self.child_bottom.insert_transition(rule_id, animation);
                                }

                                "opacity" => {
                                    let animation = self.animation_manager.create();
                                    self.opacity.insert_animation(animation, self.add_transition(transition));
                                    self.opacity.insert_transition(rule_id, animation);
                                }

                                "outer-shadow-color" => {
                                    let animation = self.animation_manager.create();
                                    self.outer_shadow_color.insert_animation(animation, self.add_transition(transition));
                                    self.outer_shadow_color.insert_transition(rule_id, animation);
                                }

                                _ => {}
                            }
                        }
                    }

                    _=> {}
                }
            }
        }
        // println!("{:?}", self.rules);
        // println!("{:?}", self.child_left.shared_data);
    }

    fn add_transition<T: Default + Interpolator>(
        &self,
        transition: Transition,
    ) -> AnimationState<T> {
        AnimationState::new(Animation::null())
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
    //pub fn set_property(&mut self, entity: Entity, propert: Property) {}

    // Add style data to an entity
    pub(crate) fn add(&mut self, entity: Entity) {
        self.pseudo_classes.insert(entity, PseudoClass::default()).unwrap();

        //self.z_order.insert(entity, 0);

        self.overflow.insert(entity, Default::default());
        //self.scroll.insert(entity, Default::default());

        self.visibility.insert(entity, Default::default());
        self.focus_order.insert(entity, Default::default()).unwrap();
    }

    pub fn remove(&mut self, entity: Entity) {

        self.elements.remove(entity);
        self.classes.remove(entity);
        self.pseudo_classes.remove(entity);

        // Display
        self.display.remove(entity);
        // Visibility
        self.visibility.remove(entity);
        // Opacity
        self.opacity.remove(entity);
        // Z Order
        self.z_order.remove(entity);
        // Clipping
        self.clip_widget.remove(entity);

        // Transform
        self.translate.remove(entity);
        self.rotate.remove(entity);
        self.scale.remove(entity);

        // Background
        self.background_color.remove(entity);
        self.background_gradient.remove(entity);
        self.background_image.remove(entity);

        // Border
        self.border_color.remove(entity);
        self.border_width.remove(entity);

        // Border Shape
        self.border_shape_bottom_left.remove(entity);
        self.border_shape_bottom_right.remove(entity);
        self.border_shape_top_left.remove(entity);
        self.border_shape_top_right.remove(entity);

        // Border Radius
        self.border_radius_bottom_left.remove(entity);
        self.border_radius_bottom_right.remove(entity);
        self.border_radius_top_left.remove(entity);
        self.border_radius_bottom_right.remove(entity);

        self.layout_type.remove(entity);
        self.positioning_type.remove(entity);

        // Space
        self.left.remove(entity);
        self.right.remove(entity);
        self.top.remove(entity);
        self.bottom.remove(entity);

        // Size
        self.width.remove(entity);
        self.height.remove(entity);

        // Space Constraints
        self.min_left.remove(entity);
        self.max_left.remove(entity);
        self.min_right.remove(entity);
        self.max_right.remove(entity);
        self.min_top.remove(entity);
        self.max_top.remove(entity);
        self.min_bottom.remove(entity);
        self.max_bottom.remove(entity);

        // Size Constraints
        self.min_width.remove(entity);
        self.max_width.remove(entity);
        self.min_height.remove(entity);
        self.max_height.remove(entity);

        // Child Space
        self.child_left.remove(entity);
        self.child_right.remove(entity);
        self.child_top.remove(entity);
        self.child_bottom.remove(entity);
        self.col_between.remove(entity);
        self.row_between.remove(entity);

        // Grid
        self.grid_cols.remove(entity);
        self.grid_rows.remove(entity);
        self.col_index.remove(entity);
        self.col_span.remove(entity);
        self.row_index.remove(entity);
        self.row_span.remove(entity);
        
        // Text and Font
        self.text.remove(entity);
        self.font.remove(entity);
        self.font_color.remove(entity);
        self.font_size.remove(entity);
    }

    pub fn remove_all(&mut self) {
        // Remove all non-inline style data
        self.background_color.clear_rules();
        self.font_color.clear_rules();

        // Position
        self.left.clear_rules();
        self.right.clear_rules();
        self.top.clear_rules();
        self.bottom.clear_rules();

        // Position Constraints
        self.min_left.clear_rules();
        self.max_left.clear_rules();
        self.min_right.clear_rules();
        self.max_right.clear_rules();
        self.min_top.clear_rules();
        self.max_top.clear_rules();
        self.min_bottom.clear_rules();
        self.max_bottom.clear_rules();

        // Size
        self.width.clear_rules();
        self.height.clear_rules();

        // Size Constraints
        self.min_width.clear_rules();
        self.max_width.clear_rules();
        self.min_height.clear_rules();
        self.max_height.clear_rules();

        // Border
        self.border_width.clear_rules();
        self.border_color.clear_rules();

        // Border Radius
        self.border_radius_top_left.clear_rules();
        self.border_radius_top_right.clear_rules();
        self.border_radius_bottom_left.clear_rules();
        self.border_radius_bottom_right.clear_rules();

        // Display
        self.display.clear_rules();
        self.visibility.clear_rules();
        self.opacity.clear_rules();

        // Inner Shadow
        self.inner_shadow_h_offset.clear_rules();
        self.inner_shadow_v_offset.clear_rules();
        self.inner_shadow_blur.clear_rules();
        self.inner_shadow_color.clear_rules();

        // Outer Shadow
        self.outer_shadow_h_offset.clear_rules();
        self.outer_shadow_v_offset.clear_rules();
        self.outer_shadow_blur.clear_rules();
        self.outer_shadow_color.clear_rules();
    }
}
