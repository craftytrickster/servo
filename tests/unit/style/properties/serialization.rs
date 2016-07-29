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
    let mut normal = Vec::new();
    let mut important = Vec::new();

    let length = LengthOrPercentageOrAuto::Length(Length::from_px(70f32));
    let value = DeclaredValue::Value(length);
    normal.push(PropertyDeclaration::Width(value));

    let min_height = LengthOrPercentage::Length(Length::from_px(20f32));
    let value = DeclaredValue::Value(min_height);
    normal.push(PropertyDeclaration::MinHeight(value));

    let value = DeclaredValue::Value(inline_block);
    normal.push(PropertyDeclaration::Display(value));

    let height = LengthOrPercentageOrAuto::Length(Length::from_px(20f32));
    let value = DeclaredValue::Value(height);
    important.push(PropertyDeclaration::Height(value));

    normal.reverse();
    important.reverse();
    let block = PropertyDeclarationBlock {
        normal: Arc::new(normal),
        important: Arc::new(important)
    };

    let css_string = block.to_css_string();

    assert_eq!(
        css_string,
        "width: 70px; min-height: 20px; display: inline-block; height: 20px !important;"
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

    mod positional_shorthands {
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

    }
}