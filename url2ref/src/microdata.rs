use crate::parser::AttributeParser;

struct Microdata;

impl AttributeParser for Microdata {
    fn parse_attribute(
        parse_info: &crate::parser::ParseInfo,
        attribute_type: crate::attribute::AttributeType,
    ) -> Option<crate::attribute::Attribute> {
        todo!()
    }
}
