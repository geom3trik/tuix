use crate::entity::Entity;

use super::Property;

use crate::state::storage::dense_storage::DenseStorage;

use crate::style::*;

#[derive(Clone, Debug, PartialEq)]
pub struct StyleRule {
    pub selectors: Vec<Selector>,
    pub properties: Vec<Property>,
}

impl StyleRule {
    pub fn new() -> Self {
        StyleRule {
            selectors: Vec::new(),
            properties: Vec::new(),
        }
    }

    pub fn selector(mut self, selector: Selector) -> Self {
        self.selectors.push(selector);

        self
    }

    pub fn parent_selector(mut self, mut selector: Selector) -> Self {
        selector.relation = Relation::Parent;
        self.selectors.push(selector);

        self
    }

    pub fn property(mut self, property: Property) -> Self {
        self.properties.push(property);

        self
    }

    pub fn specificity(&self) -> Specificity {
        let mut specificity = Specificity([0, 0, 0]);
        for selector in self.selectors.iter() {
            specificity += selector.specificity();
        }

        return specificity;
    }

    // Property Setters

    pub fn set_display(mut self, value: Display) -> Self {
        self.properties.push(Property::Display(value));

        self
    }

    pub fn set_visibility(mut self, value: Visibility) -> Self {
        self.properties.push(Property::Visibility(value));

        self
    }

    pub fn set_overflow(mut self, value: Overflow) -> Self {
        self.properties.push(Property::Overflow(value));

        self
    }

    // Background
    pub fn set_background_color(mut self, value: Color) -> Self {

        self.properties.push(Property::BackgroundColor(value));

        self
    }

    pub fn set_background_gradient(mut self, value: LinearGradient) -> Self {
        self.properties.push(Property::BackgroundGradient(value));

        self
    }

    // Outer Shadow
    pub fn set_outer_shadow_h_offset(mut self, value: Units) -> Self {
        let mut box_shadow = BoxShadow::default();
        box_shadow.horizontal_offset = value;

        self.properties.push(Property::OuterShadow(box_shadow));

        self
    }

    pub fn set_outer_shadow_v_offset(mut self, value: Units) -> Self {
        let mut box_shadow = BoxShadow::default();
        box_shadow.vertical_offset = value;

        self.properties.push(Property::OuterShadow(box_shadow));

        self
    }

    pub fn set_outer_shadow_color(mut self, value: Color) -> Self {
        let mut box_shadow = BoxShadow::default();
        box_shadow.color = value;

        self.properties.push(Property::OuterShadow(box_shadow));

        self
    }

    pub fn set_outer_shadow_blur(mut self, value: Units) -> Self {
        let mut box_shadow = BoxShadow::default();
        box_shadow.blur_radius = value;

        self.properties.push(Property::OuterShadow(box_shadow));

        self
    }

    // Inner Shadow
    pub fn set_inner_shadow_h_offset(mut self, value: Units) -> Self {
        let mut box_shadow = BoxShadow::default();
        box_shadow.horizontal_offset = value;

        self.properties.push(Property::InnerShadow(box_shadow));

        self
    }

    pub fn set_inner_shadow_v_offset(mut self, value: Units) -> Self {
        let mut box_shadow = BoxShadow::default();
        box_shadow.vertical_offset = value;

        self.properties.push(Property::InnerShadow(box_shadow));

        self
    }

    pub fn set_inner_shadow_color(mut self, value: Color) -> Self {
        let mut box_shadow = BoxShadow::default();
        box_shadow.color = value;

        self.properties.push(Property::InnerShadow(box_shadow));

        self
    }

    pub fn set_inner_shadow_blur(mut self, value: Units) -> Self {
        
        let mut box_shadow = BoxShadow::default();
        box_shadow.blur_radius = value;

        self.properties.push(Property::InnerShadow(box_shadow));

        self
    }

    // Positioning

    pub fn set_left(mut self, value: Units) -> Self {
        self.properties.push(Property::Left(value));

        self
    }

    pub fn set_right(mut self, value: Units) -> Self {
        self.properties.push(Property::Right(value));

        self
    }

    pub fn set_top(mut self, value: Units) -> Self {
        self.properties.push(Property::Top(value));
        self
    }

    pub fn set_bottom(mut self, value: Units) -> Self {
        self.properties.push(Property::Bottom(value));
        self
    }

    // Alignment and Justification

    // pub fn set_justification(mut self, val: Justification) -> Self {
    //     self.state.style.justification.set(self.entity, val);
    //     self
    // }

    // pub fn set_alignment(mut self, val: Alignment) -> Self {
    //     self.state.style.alignment.set(self.entity, val);
    //     self
    // }

    // Size

    pub fn set_width(mut self, value: Units) -> Self {
        self.properties.push(Property::Width(value));

        self
    }

    pub fn set_height(mut self, value: Units) -> Self {
        self.properties.push(Property::Height(value));

        self
    }

    // Size Constraints

    pub fn set_min_width(mut self, value: Units) -> Self {
        self.properties.push(Property::MinHeight(value));

        self
    }

    pub fn set_max_width(mut self, value: Units) -> Self {
        self.properties.push(Property::MaxWidth(value));

        self
    }

    pub fn set_min_height(mut self, value: Units) -> Self {
        self.properties.push(Property::MinHeight(value));

        self
    }

    pub fn set_max_height(mut self, value: Units) -> Self {
        self.properties.push(Property::MaxHeight(value));

        self
    }

    // Margins

    pub fn set_margin(mut self, value: Units) -> Self {
        self.properties.push(Property::MarginLeft(value));
        self.properties.push(Property::MarginRight(value));
        self.properties.push(Property::MarginTop(value));
        self.properties.push(Property::MarginBottom(value));


        self
    }

    pub fn set_margin_left(mut self, value: Units) -> Self {
        self.properties.push(Property::MarginLeft(value));

        self
    }

    pub fn set_margin_right(mut self, value: Units) -> Self {
        self.properties.push(Property::MarginRight(value));

        self
    }

    pub fn set_margin_top(mut self, value: Units) -> Self {
        self.properties.push(Property::MarginTop(value));

        self
    }

    pub fn set_margin_bottom(mut self, value: Units) -> Self {
        self.properties.push(Property::MarginBottom(value));

        self
    }

    // Padding

    pub fn set_padding(mut self, value: Units) -> Self {
        self.properties.push(Property::PaddingLeft(value));
        self.properties.push(Property::PaddingRight(value));
        self.properties.push(Property::PaddingTop(value));
        self.properties.push(Property::PaddingBottom(value));


        self
    }

    pub fn set_padding_left(mut self, value: Units) -> Self {
        self.properties.push(Property::PaddingLeft(value));

        self
    }

    pub fn set_padding_right(mut self, value: Units) -> Self {
        self.properties.push(Property::PaddingRight(value));

        self
    }

    pub fn set_padding_top(mut self, value: Units) -> Self {
        self.properties.push(Property::PaddingTop(value));

        self
    }

    pub fn set_padding_bottom(mut self, value: Units) -> Self {
        self.properties.push(Property::PaddingBottom(value));

        self
    }

    // Flex Item

    pub fn set_flex_grow(mut self, value: f32) -> Self {
        self.properties.push(Property::FlexGrow(value));

        self
    }

    pub fn set_flex_shrink(mut self, value: f32) -> Self {
        self.properties.push(Property::FlexShrink(value));

        self
    }

    pub fn set_flex_basis(mut self, value: Units) -> Self {
        self.properties.push(Property::FlexBasis(value));

        self
    }

    pub fn set_align_self(mut self, value: AlignSelf) -> Self {
        self.properties.push(Property::AlignSelf(value));

        self
    }

    // Flex Container

    pub fn set_flex_direction(mut self, value: FlexDirection) -> Self {
        self.properties.push(Property::FlexDirection(value));

        self
    }

    pub fn set_justify_content(mut self, value: JustifyContent) -> Self {
        self.properties.push(Property::JustifyContent(value));

        self
    }

    pub fn set_align_content(mut self, value: AlignContent) -> Self {
        self.properties.push(Property::AlignContent(value));

        self
    }

    pub fn set_align_items(mut self, value: AlignItems) -> Self {
        self.properties.push(Property::AlignItems(value));

        self
    }

    // Border

    pub fn set_border_color(mut self, value: Color) -> Self {
        self.properties.push(Property::BorderColor(value));

        self
    }

    pub fn set_border_width(mut self, value: Units) -> Self {
        self.properties.push(Property::BorderWidth(value));

        self
    }

    pub fn set_border_radius(mut self, value: Units) -> Self {
        self.properties.push(Property::BorderTopLeftRadius(value));

        self
    }

    pub fn set_border_radius_top_left(mut self, value: Units) -> Self {
        self.properties.push(Property::BorderTopLeftRadius(value));

        self
    }

    pub fn set_border_radius_top_right(mut self, value: Units) -> Self {
        self.properties.push(Property::BorderTopRightRadius(value));
        
        self
    }

    pub fn set_border_radius_bottom_left(mut self, value: Units) -> Self {
        self.properties.push(Property::BorderBottomLeftRadius(value));

        self
    }

    pub fn set_border_radius_bottom_right(mut self, value: Units) -> Self {
        self.properties.push(Property::BorderBottomRightRadius(value));

        self
    }

    pub fn set_color(mut self, value: Color) -> Self {
        self.properties.push(Property::FontColor(value));

        self
    }

    pub fn set_font_size(mut self, value: f32) -> Self {
        self.properties.push(Property::FontSize(value));

        self
    }

    // Text Alignment
    pub fn set_text_justify(mut self, value: Justify) -> Self {
        self.properties.push(Property::TextJustify(value));

        self
    }

    pub fn set_text_align(mut self, value: Align) -> Self {

        self.properties.push(Property::TextAlign(value));

        self
    }
}
