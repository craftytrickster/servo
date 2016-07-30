/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub use cssparser::ToCss;
pub use std::sync::Arc;
pub use style::computed_values::display::T::inline_block;
pub use style::properties::{DeclaredValue, PropertyDeclaration, PropertyDeclarationBlock};
pub use style::values::specified::{Length, LengthOrPercentageOrAuto, LengthOrPercentage};

#[test]
fn property_declaration_block_should_serialize_correctly() {
    use style::properties::longhands::overflow_x::computed_value::T as OverflowXValue;
    use style::properties::longhands::overflow_y::computed_value::T as OverflowYContainer;

    let mut normal = Vec::new();
    let mut important = Vec::new();

    let length = DeclaredValue::Value(LengthOrPercentageOrAuto::Length(Length::from_px(70f32)));
    normal.push(PropertyDeclaration::Width(length));

    let min_height = DeclaredValue::Value(LengthOrPercentage::Length(Length::from_px(20f32)));
    normal.push(PropertyDeclaration::MinHeight(min_height));

    let value = DeclaredValue::Value(inline_block);
    normal.push(PropertyDeclaration::Display(value));

    let overflow_x = DeclaredValue::Value(OverflowXValue::auto);
    normal.push(PropertyDeclaration::OverflowX(overflow_x));

    let overflow_y = DeclaredValue::Value(OverflowYContainer(OverflowXValue::auto));
    normal.push(PropertyDeclaration::OverflowY(overflow_y));

    let height = DeclaredValue::Value(LengthOrPercentageOrAuto::Length(Length::from_px(20f32)));
    important.push(PropertyDeclaration::Height(height));

    normal.reverse();
    important.reverse();
    let block = PropertyDeclarationBlock {
        normal: Arc::new(normal),
        important: Arc::new(important)
    };

    let css_string = block.to_css_string();

    assert_eq!(
        css_string,
        "width: 70px; min-height: 20px; display: inline-block; overflow: auto; height: 20px !important;"
    );
}

mod shorthand_serialization {
    pub use super::*;

    pub fn shorthand_properties_to_string(properties: Vec<PropertyDeclaration>) -> String {
        let block = PropertyDeclarationBlock {
            normal: Arc::new(properties),
            important: Arc::new(Vec::new())
        };

        block.to_css_string()
    }

    mod overflow {
        pub use super::*;
        use style::properties::longhands::overflow_x::computed_value::T as OverflowXValue;
        use style::properties::longhands::overflow_y::computed_value::T as OverflowYContainer;

        #[test]
        fn equal_overflow_properties_should_serialize_to_single_value() {
            let mut properties = Vec::new();

            let overflow_x = DeclaredValue::Value(OverflowXValue::auto);
            properties.push(PropertyDeclaration::OverflowX(overflow_x));

            let overflow_y = DeclaredValue::Value(OverflowYContainer(OverflowXValue::auto));
            properties.push(PropertyDeclaration::OverflowY(overflow_y));

            let serialization = shorthand_properties_to_string(properties);
            assert_eq!(serialization, "overflow: auto;");
        }

        #[test]
        fn different_overflow_properties_should_serialize_to_two_values() {
            let mut properties = Vec::new();

            let overflow_x = DeclaredValue::Value(OverflowXValue::scroll);
            properties.push(PropertyDeclaration::OverflowX(overflow_x));

            let overflow_y = DeclaredValue::Value(OverflowYContainer(OverflowXValue::auto));
            properties.push(PropertyDeclaration::OverflowY(overflow_y));

            let serialization = shorthand_properties_to_string(properties);
            assert_eq!(serialization, "overflow-x: scroll; overflow-y: auto;");
        }
    }

    mod four_sides_shorthands {
        pub use super::*;

        // we can use margin as a base to test out the different combinations
        // but afterwards, we only need to to one test per "directional shorthand"
        #[test]
        fn all_equal_properties_should_serialize_to_one_value() {
            let mut properties = Vec::new();

            let px_70 = DeclaredValue::Value(LengthOrPercentageOrAuto::Length(Length::from_px(70f32)));
            properties.push(PropertyDeclaration::MarginTop(px_70.clone()));
            properties.push(PropertyDeclaration::MarginRight(px_70.clone()));
            properties.push(PropertyDeclaration::MarginBottom(px_70.clone()));
            properties.push(PropertyDeclaration::MarginLeft(px_70));

            let serialization = shorthand_properties_to_string(properties);
            assert_eq!(serialization, "margin: 70px;");
        }

        #[test]
        fn equal_vertical_and_equal_horizontal_properties_should_serialize_to_two_value() {
            let mut properties = Vec::new();

            let vertical_px = DeclaredValue::Value(LengthOrPercentageOrAuto::Length(Length::from_px(10f32)));
            let horizontal_px = DeclaredValue::Value(LengthOrPercentageOrAuto::Length(Length::from_px(5f32)));

            properties.push(PropertyDeclaration::MarginTop(vertical_px.clone()));
            properties.push(PropertyDeclaration::MarginRight(horizontal_px.clone()));
            properties.push(PropertyDeclaration::MarginBottom(vertical_px));
            properties.push(PropertyDeclaration::MarginLeft(horizontal_px));

            let serialization = shorthand_properties_to_string(properties);
            assert_eq!(serialization, "margin: 10px 5px;");
        }

        #[test]
        fn different_vertical_and_equal_horizontal_properties_should_serialize_to_three_values() {
            let mut properties = Vec::new();

            let top_px = DeclaredValue::Value(LengthOrPercentageOrAuto::Length(Length::from_px(8f32)));
            let bottom_px = DeclaredValue::Value(LengthOrPercentageOrAuto::Length(Length::from_px(10f32)));
            let horizontal_px = DeclaredValue::Value(LengthOrPercentageOrAuto::Length(Length::from_px(5f32)));

            properties.push(PropertyDeclaration::MarginTop(top_px));
            properties.push(PropertyDeclaration::MarginRight(horizontal_px.clone()));
            properties.push(PropertyDeclaration::MarginBottom(bottom_px));
            properties.push(PropertyDeclaration::MarginLeft(horizontal_px));

            let serialization = shorthand_properties_to_string(properties);
            assert_eq!(serialization, "margin: 8px 5px 10px;");
        }

        #[test]
        fn different_properties_should_serialize_to_four_values() {
            let mut properties = Vec::new();

            let top_px = DeclaredValue::Value(LengthOrPercentageOrAuto::Length(Length::from_px(8f32)));
            let right_px = DeclaredValue::Value(LengthOrPercentageOrAuto::Length(Length::from_px(12f32)));
            let bottom_px = DeclaredValue::Value(LengthOrPercentageOrAuto::Length(Length::from_px(10f32)));
            let left_px = DeclaredValue::Value(LengthOrPercentageOrAuto::Length(Length::from_px(14f32)));

            properties.push(PropertyDeclaration::MarginTop(top_px));
            properties.push(PropertyDeclaration::MarginRight(right_px));
            properties.push(PropertyDeclaration::MarginBottom(bottom_px));
            properties.push(PropertyDeclaration::MarginLeft(left_px));

            let serialization = shorthand_properties_to_string(properties);
            assert_eq!(serialization, "margin: 8px 12px 10px 14px;");
        }

        #[test]
        fn padding_should_serialize_correctly() {
            let mut properties = Vec::new();

            let px_10 = DeclaredValue::Value(LengthOrPercentage::Length(Length::from_px(10f32)));
            let px_15 = DeclaredValue::Value(LengthOrPercentage::Length(Length::from_px(15f32)));
            properties.push(PropertyDeclaration::PaddingTop(px_10.clone()));
            properties.push(PropertyDeclaration::PaddingRight(px_15.clone()));
            properties.push(PropertyDeclaration::PaddingBottom(px_10));
            properties.push(PropertyDeclaration::PaddingLeft(px_15));

            let serialization = shorthand_properties_to_string(properties);
            assert_eq!(serialization, "padding: 10px 15px;");
        }

        #[test]
        fn border_width_should_serialize_correctly() {
            let mut properties = Vec::new();

            use style::properties::longhands::border_top_width::SpecifiedValue as TopContainer;
            use style::properties::longhands::border_right_width::SpecifiedValue as RightContainer;
            use style::properties::longhands::border_bottom_width::SpecifiedValue as BottomContainer;
            use style::properties::longhands::border_left_width::SpecifiedValue as LeftContainer;

            let top_px = DeclaredValue::Value(TopContainer(Length::from_px(10f32)));
            let bottom_px = DeclaredValue::Value(BottomContainer(Length::from_px(10f32)));

            let right_px = DeclaredValue::Value(RightContainer(Length::from_px(15f32)));
            let left_px = DeclaredValue::Value(LeftContainer(Length::from_px(15f32)));

            properties.push(PropertyDeclaration::BorderTopWidth(top_px));
            properties.push(PropertyDeclaration::BorderRightWidth(right_px));
            properties.push(PropertyDeclaration::BorderBottomWidth(bottom_px));
            properties.push(PropertyDeclaration::BorderLeftWidth(left_px));

            let serialization = shorthand_properties_to_string(properties);
            assert_eq!(serialization, "border-width: 10px 15px;");
        }

        #[test]
        fn border_color_should_serialize_correctly() {
            use style::values::specified::CSSColor;
            use style::properties::longhands::outline_color::computed_value::T as ComputedColor;
            use style::values::RGBA;

            let mut properties = Vec::new();

            let red = DeclaredValue::Value(CSSColor {
                parsed: ComputedColor::RGBA(RGBA { red: 1f32, green: 0f32, blue: 0f32, alpha: 1f32 }),
                authored: None
            });

            let blue = DeclaredValue::Value(CSSColor {
                parsed: ComputedColor::RGBA(RGBA { red: 0f32, green: 0f32, blue: 1f32, alpha: 1f32 }),
                authored: None
            });

            properties.push(PropertyDeclaration::BorderTopColor(blue.clone()));
            properties.push(PropertyDeclaration::BorderRightColor(red.clone()));
            properties.push(PropertyDeclaration::BorderBottomColor(blue));
            properties.push(PropertyDeclaration::BorderLeftColor(red));

            let serialization = shorthand_properties_to_string(properties);

            // TODO: Make the rgb test show border-color as blue red instead of below tuples
            assert_eq!(serialization, "border-color: rgb(0, 0, 255) rgb(255, 0, 0);");
        }

        #[test]
        fn border_style_should_serialize_correctly() {
            use style::values::specified::BorderStyle;

            let mut properties = Vec::new();

            let solid = DeclaredValue::Value(BorderStyle::solid);
            let dotted = DeclaredValue::Value(BorderStyle::dotted);
            properties.push(PropertyDeclaration::BorderTopStyle(solid.clone()));
            properties.push(PropertyDeclaration::BorderRightStyle(dotted.clone()));
            properties.push(PropertyDeclaration::BorderBottomStyle(solid));
            properties.push(PropertyDeclaration::BorderLeftStyle(dotted));

            let serialization = shorthand_properties_to_string(properties);
            assert_eq!(serialization, "border-style: solid dotted;");
        }
    }
}