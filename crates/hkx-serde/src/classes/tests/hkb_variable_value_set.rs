#[cfg(test)]
mod tests {
    use crate::classes::generated::{
        class_params::ClassParams, HkbVariableValue, HkbVariableValueSet,
    };
    use crate::classes::Class;
    use crate::havok_types::{HkArrayClass, HkArrayClassParam, HkArrayRef, HkArrayVector};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let result = quick_xml::se::to_string(&Class {
            name: "#0064".into(),
            class: "hkbVariableValueSet".into(),
            signature: "0x27812d8d".into(),
            hkparams: ClassParams::HkbVariableValueSet(vec![
                HkbVariableValueSet::WordVariableValues(HkArrayClass {
                    numelements: 3,
                    classes: vec![
                        HkArrayClassParam {
                            hkparam: vec![HkbVariableValue::Value(1045220557.into())],
                        },
                        HkArrayClassParam {
                            hkparam: vec![HkbVariableValue::Value(0.into())],
                        },
                        HkArrayClassParam {
                            hkparam: vec![HkbVariableValue::Value(0.into())],
                        },
                    ],
                }),
                HkbVariableValueSet::QuadVariableValues(HkArrayVector {
                    numelements: 2,
                    value: vec![
                        (63.0, 64.0, 65.0, 66.0).into(),
                        (63.0, 64.0, 65.0, 66.0).into(),
                    ],
                }),
                HkbVariableValueSet::VariantVariableValues(HkArrayRef {
                    numelements: 2,
                    value: vec!["#0063".into(), "#0064".into()],
                }),
            ]),
        })
        .unwrap();

        dbg!(&result);
    }

    #[test]
    fn should_deserialize() {
        let xml = r###"
<hkobject name="#0064" class="hkbVariableValueSet" signature="0x27812d8d">
    <hkparam name="wordVariableValues" numelements="3">
        <hkobject>
            <hkparam name="value">1045220557</hkparam>
        </hkobject>
        <hkobject>
            <hkparam name="value">0</hkparam>
        </hkobject>
        <hkobject>
            <hkparam name="value">0</hkparam>
        </hkobject>
    </hkparam>
    <hkparam name="quadVariableValues" numelements="2">
        (0.000000 1.000000 0.000000 0.000000)
        (0.000000 0.000000 -1.000000 0.000000)
    </hkparam>
    <hkparam name="variantVariableValues" numelements="2">
        #0063 #0064
    </hkparam>
</hkobject>
"###;

        let result: Class = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            result,
            Class {
                name: "#0064".into(),
                class: "hkbVariableValueSet".into(),
                signature: "0x27812d8d".into(),
                hkparams: ClassParams::HkbVariableValueSet(vec![
                    HkbVariableValueSet::WordVariableValues(HkArrayClass {
                        numelements: 3,
                        classes: vec![
                            HkArrayClassParam {
                                hkparam: vec![HkbVariableValue::Value(1045220557.into())],
                            },
                            HkArrayClassParam {
                                hkparam: vec![HkbVariableValue::Value(0.into())]
                            },
                            HkArrayClassParam {
                                hkparam: vec![HkbVariableValue::Value(0.into())]
                            },
                        ]
                    },),
                    HkbVariableValueSet::QuadVariableValues(HkArrayVector {
                        numelements: 2,
                        value: vec![
                            (0.000000, 1.000000, 0.000000, 0.000000).into(),
                            (0.000000, 0.000000, -1.000000, 0.000000).into()
                        ],
                    },),
                    HkbVariableValueSet::VariantVariableValues(HkArrayRef {
                        numelements: 2,
                        value: vec!["#0063".into(), "#0064".into()],
                    },),
                ]),
            }
        );
    }
}
