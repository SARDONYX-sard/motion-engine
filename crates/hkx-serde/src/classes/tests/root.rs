#[cfg(test)]
mod tests {
    use crate::classes::generated::{
        class_params::ClassParams, HkbBehaviorGraphStringData, HkbVariableValue,
        HkbVariableValueSet,
    };
    use crate::classes::root::{HkSection, Hkx};
    use crate::classes::Class;
    use crate::havok_types::HkArrayClassParam;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let class = Hkx {
            class_version: 8,
            content_version: "hk_2010.2.0-r1".into(),
            top_level_object: "#0056".into(),
            hk_section: HkSection {
                name: "__data__".into(),
                classes: vec![Class {
                    name: "#0057".into(),
                    class: "hkbBehaviorGraphStringData".into(),
                    signature: "0xc713064e".into(),
                    hkparams: ClassParams::HkbBehaviorGraphStringData(vec![
                        HkbBehaviorGraphStringData::CharacterPropertyNames(Default::default()),
                    ]),
                }],
            },
        };
        let result = quick_xml::se::to_string(&class).unwrap();

        let expected_xml = "\
            <hkpackfile classversion=\"8\" contentsversion=\"hk_2010.2.0-r1\" toplevelobject=\"#0056\">\
                <hksection name=\"__data__\">\
                    <hkobject name=\"#0057\" class=\"hkbBehaviorGraphStringData\" signature=\"0xc713064e\">\
                        <hkparam name=\"characterPropertyNames\" numelements=\"0\"/>\
                    </hkobject>\
                </hksection>\
            </hkpackfile>";
        assert_eq!(result, expected_xml);
    }

    #[test]
    fn should_deserialize() -> anyhow::Result<()> {
        let test_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests")
            // .join("test.xml");
            .join("test2.xml");
        // .join("1hm_behavior.xml");

        let xml = std::fs::read_to_string(test_path)?;
        let result: Hkx = quick_xml::de::from_str(&xml)?;

        let expected = Hkx {
            class_version: 8,
            content_version: "hk_2010.2.0-r1".into(),
            top_level_object: "#0056".into(),
            hk_section: HkSection {
                name: "__data__".into(),
                classes: vec![
                    Class {
                        name: "#0057".into(),
                        class: "hkbBehaviorGraphStringData".into(),
                        signature: "0xc713064e".into(),
                        hkparams: ClassParams::HkbBehaviorGraphStringData(vec![
                            HkbBehaviorGraphStringData::EventNames(
                                vec![
                                    "staggerStop",
                                    "syncPoint",
                                    "WeapEquip_OutMoving",
                                    "WeapEquip_Out",
                                    "PitchOverrideEnd",
                                    "PitchOverrideStart",
                                    "moveStart",
                                    "moveStop",
                                    "SprintStop",
                                    "SprintStart",
                                    "FootLeft",
                                    "FootRight",
                                    "staggerStart",
                                    "BeginWeaponDraw",
                                    "FootScuffRight",
                                    "weaponDraw",
                                    "FootScuffLeft",
                                    "SoundPlay.WPNBlade1HandSmallDraw",
                                    "SoundPlay.WPNAxe1HandDraw",
                                    "SoundPlay.WPNMace1HandDraw",
                                    "SoundPlay.WPNStaffHandDraw",
                                    "MagicWeap_ForceEquip",
                                    "SneakStart",
                                    "SneakStop",
                                    "StreamingTest",
                                    "SoundPlay.WPNUnarmedDraw",
                                    "tailSneakIdle",
                                    "tailCombatIdle",
                                    "SoundPlay.WPNLeftHandDraw",
                                    "arrowAttach",
                                ]
                                .into(),
                            ),
                            HkbBehaviorGraphStringData::AttributeNames(Default::default()),
                            HkbBehaviorGraphStringData::VariableNames(
                                vec![
                                    "blendDefault",
                                    "IsEquipping",
                                    "TurnDelta",
                                    "Direction",
                                    "SpeedSampled",
                                    "i1stPerson",
                                    "iSyncSprintState",
                                    "iSyncIdleLocomotion",
                                    "staggerMagnitude",
                                    "IsStaggering",
                                    "PitchManualOverride",
                                    "bEquipUnequip",
                                    "iRightHandType",
                                    "iLeftHandType",
                                    "iIsInSneak",
                                    "IsSneaking",
                                ]
                                .into(),
                            ),
                            HkbBehaviorGraphStringData::CharacterPropertyNames(
                                vec!["LeftArm", "UpperBody", "RightArm"].into(),
                            ),
                        ]),
                    },
                    Class {
                        name: "#0058".into(),
                        class: "hkbVariableValueSet".into(),
                        signature: "0x27812d8d".into(),
                        hkparams: ClassParams::HkbVariableValueSet(vec![
                            HkbVariableValueSet::WordVariableValues(
                                vec![
                                    HkArrayClassParam {
                                        hkparam: vec![HkbVariableValue::Value(1045220557.into())],
                                    },
                                    HkArrayClassParam {
                                        hkparam: vec![HkbVariableValue::Value(0.into())],
                                    },
                                    HkArrayClassParam {
                                        hkparam: vec![HkbVariableValue::Value(0.into())],
                                    },
                                    HkArrayClassParam {
                                        hkparam: vec![HkbVariableValue::Value(0.into())],
                                    },
                                    HkArrayClassParam {
                                        hkparam: vec![HkbVariableValue::Value(0.into())],
                                    },
                                    HkArrayClassParam {
                                        hkparam: vec![HkbVariableValue::Value(0.into())],
                                    },
                                    HkArrayClassParam {
                                        hkparam: vec![HkbVariableValue::Value(0.into())],
                                    },
                                    HkArrayClassParam {
                                        hkparam: vec![HkbVariableValue::Value(0.into())],
                                    },
                                    HkArrayClassParam {
                                        hkparam: vec![HkbVariableValue::Value(0.into())],
                                    },
                                    HkArrayClassParam {
                                        hkparam: vec![HkbVariableValue::Value(0.into())],
                                    },
                                    HkArrayClassParam {
                                        hkparam: vec![HkbVariableValue::Value(0.into())],
                                    },
                                    HkArrayClassParam {
                                        hkparam: vec![HkbVariableValue::Value(0.into())],
                                    },
                                    HkArrayClassParam {
                                        hkparam: vec![HkbVariableValue::Value(0.into())],
                                    },
                                    HkArrayClassParam {
                                        hkparam: vec![HkbVariableValue::Value(0.into())],
                                    },
                                    HkArrayClassParam {
                                        hkparam: vec![HkbVariableValue::Value(0.into())],
                                    },
                                    HkArrayClassParam {
                                        hkparam: vec![HkbVariableValue::Value(0.into())],
                                    },
                                ]
                                .into(),
                            ),
                            HkbVariableValueSet::QuadVariableValues(Default::default()),
                            HkbVariableValueSet::VariantVariableValues(Default::default()),
                        ]),
                    },
                ],
            },
        };
        assert_eq!(result, expected);
        Ok(())
    }
}
