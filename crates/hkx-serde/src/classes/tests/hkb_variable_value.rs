#[cfg(test)]
mod tests {
    use crate::classes::generated::{class_params::ClassParams, HkbVariableValue};
    use crate::classes::Class;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let class = Class {
            name: "#0060".into(),
            class: "hkbVariableValue".into(),
            signature: "0x27812d8d".into(),
            hkparams: ClassParams::HkbVariableValue(vec![HkbVariableValue::Value(
                1045220557.into(),
            )]),
        };

        let result = quick_xml::se::to_string(&class).unwrap();

        let expected_xml = "\
<hkobject name=\"#0060\" class=\"hkbVariableValue\" signature=\"0x27812d8d\">\
    <hkparam name=\"value\">1045220557</hkparam>\
</hkobject>";

        assert_eq!(result, expected_xml);
        dbg!(result);
    }

    #[test]
    fn should_deserialize() {
        let xml = "\
<hkobject name=\"#0060\" class=\"hkbVariableValue\" signature=\"0x27812d8d\">\
    <hkparam name=\"value\">1045220557</hkparam>\
</hkobject>";

        let result: Class = quick_xml::de::from_str(xml).unwrap();
        let expected = Class {
            name: "#0060".into(),
            class: "hkbVariableValue".into(),
            signature: "0x27812d8d".into(),
            hkparams: ClassParams::HkbVariableValue(vec![HkbVariableValue::Value(
                1045220557.into(),
            )]),
        };
        assert_eq!(result, expected);
    }
}
