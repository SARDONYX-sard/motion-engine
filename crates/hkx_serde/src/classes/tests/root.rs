#[cfg(test)]
mod tests {
    use crate::havok_types::{
        HkArrayClass, HkArrayClassParam, HkArrayStringPtr, Primitive, Vector4,
    };
    use crate::{
        classes::{
            class_params::ClassParams,
            root::{HkSection, Hkx},
            Class, EventMode, HkRootLevelContainer, HkRootLevelContainerNamedVariant,
            HkbProjectData, HkbProjectStringData,
        },
        helpers::serde::to_string_pretty_xml,
    };
    use ordered_float::OrderedFloat;
    use pretty_assertions::assert_eq;
    use std::borrow::Cow;

    #[test]
    fn should_serialize() {
        let class = Hkx {
            class_version: 8,
            content_version: "hk_2010.2.0-r1".into(),
            top_level_object: "#0050".into(),
            hk_section: HkSection {
                name: "__data__".into(),
                classes: vec![
                    Class {
                        name: "#0050".into(),
                        class: "hkRootLevelContainer".into(),
                        signature: "0x2772c11e".into(),
                        hkparams: ClassParams::HkRootLevelContainer(vec![
                            HkRootLevelContainer::NamedVariants(HkArrayClass {
                                numelements: 1,
                                classes: Some(vec![HkArrayClassParam {
                                    hkparams: vec![
                                        HkRootLevelContainerNamedVariant::Name(
                                            Cow::from("hkbProjectData").into(),
                                        ),
                                        HkRootLevelContainerNamedVariant::ClassName(
                                            Cow::from("hkbProjectData").into(),
                                        ),
                                        HkRootLevelContainerNamedVariant::Variant(Primitive {
                                            value: Cow::from("#0051"),
                                        }),
                                    ],
                                }]),
                            }),
                        ]),
                    },
                    Class {
                        name: "#0051".into(),
                        class: "hkbProjectData".into(),
                        signature: "0x13a39ba7".into(),
                        hkparams: ClassParams::HkbProjectData(vec![
                            HkbProjectData::WorldUpWs(Primitive {
                                value: Vector4 {
                                    x: OrderedFloat(0.0),
                                    y: OrderedFloat(0.0),
                                    z: OrderedFloat(1.0),
                                    w: OrderedFloat(0.0),
                                },
                            }),
                            HkbProjectData::StringData(Primitive {
                                value: "#0052".into(),
                            }),
                            HkbProjectData::DefaultEventMode(Primitive {
                                value: EventMode::EventModeIgnoreFromGenerator,
                            }),
                        ]),
                    },
                    Class {
                        name: "#0052".into(),
                        class: "hkbProjectStringData".into(),
                        signature: "0x076ad60a".into(),
                        hkparams: ClassParams::HkbProjectStringData(vec![
                            HkbProjectStringData::AnimationFilenames(HkArrayStringPtr {
                                numelements: 0,
                                hkcstrings: vec![],
                            }),
                            HkbProjectStringData::BehaviorFilenames(HkArrayStringPtr {
                                numelements: 0,
                                hkcstrings: vec![],
                            }),
                            HkbProjectStringData::CharacterFilenames(HkArrayStringPtr {
                                numelements: 1,
                                hkcstrings: vec!["Characters\\DefaultMale.hkx".into()],
                            }),
                            HkbProjectStringData::EventNames(HkArrayStringPtr {
                                numelements: 0,
                                hkcstrings: vec![],
                            }),
                            HkbProjectStringData::AnimationPath(Primitive { value: "".into() }),
                            HkbProjectStringData::BehaviorPath(Primitive { value: "".into() }),
                            HkbProjectStringData::CharacterPath(Primitive { value: "".into() }),
                            HkbProjectStringData::FullPathToSource(Primitive { value: "".into() }),
                        ]),
                    },
                ],
            },
        };

        let result = to_string_pretty_xml(&class, ' ', 2).unwrap();
        let expected_xml = r##"<hkpackfile classversion="8" contentsversion="hk_2010.2.0-r1" toplevelobject="#0050">
  <hksection name="__data__">
    <hkobject name="#0050" class="hkRootLevelContainer" signature="0x2772c11e">
      <hkparam name="namedVariants" numelements="1">
        <hkobject>
          <hkparam name="name">
            hkbProjectData
          </hkparam>
          <hkparam name="className">
            hkbProjectData
          </hkparam>
          <hkparam name="variant">
            #0051
          </hkparam>
        </hkobject>
      </hkparam>
    </hkobject>
    <hkobject name="#0051" class="hkbProjectData" signature="0x13a39ba7">
      <hkparam name="worldUpWS">
        (0.000000 0.000000 1.000000 0.000000)
      </hkparam>
      <hkparam name="stringData">
        #0052
      </hkparam>
      <hkparam name="defaultEventMode">
        EVENT_MODE_IGNORE_FROM_GENERATOR
      </hkparam>
    </hkobject>
    <hkobject name="#0052" class="hkbProjectStringData" signature="0x076ad60a">
      <hkparam name="animationFilenames" numelements="0"/>
      <hkparam name="behaviorFilenames" numelements="0"/>
      <hkparam name="characterFilenames" numelements="1">
        <hkcstring>Characters\DefaultMale.hkx</hkcstring>
      </hkparam>
      <hkparam name="eventNames" numelements="0"/>
      <hkparam name="animationPath"/>
      <hkparam name="behaviorPath"/>
      <hkparam name="characterPath"/>
      <hkparam name="fullPathToSource"/>
    </hkobject>
  </hksection>
</hkpackfile>"##;
        assert_eq!(result, expected_xml);
    }

    #[test]
    fn should_deserialize() -> anyhow::Result<()> {
        let test_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests")
            .join("defaultmale.xml");
        // .join("1hm_behavior.xml");

        let xml = std::fs::read_to_string(test_path)?;
        let result: Hkx = quick_xml::de::from_str(&xml)?;

        let expected = Hkx {
            class_version: 8,
            content_version: "hk_2010.2.0-r1".into(),
            top_level_object: "#0050".into(),
            hk_section: HkSection {
                name: "__data__".into(),
                classes: vec![
                    Class {
                        name: "#0050".into(),
                        class: "hkRootLevelContainer".into(),
                        signature: "0x2772c11e".into(),
                        hkparams: ClassParams::HkRootLevelContainer(vec![
                            HkRootLevelContainer::NamedVariants(HkArrayClass {
                                numelements: 1,
                                classes: Some(vec![HkArrayClassParam {
                                    hkparams: vec![
                                        HkRootLevelContainerNamedVariant::Name(
                                            Cow::from("hkbProjectData").into(),
                                        ),
                                        HkRootLevelContainerNamedVariant::ClassName(
                                            Cow::from("hkbProjectData").into(),
                                        ),
                                        HkRootLevelContainerNamedVariant::Variant(Primitive {
                                            value: Cow::from("#0051"),
                                        }),
                                    ],
                                }]),
                            }),
                        ]),
                    },
                    Class {
                        name: "#0051".into(),
                        class: "hkbProjectData".into(),
                        signature: "0x13a39ba7".into(),
                        hkparams: ClassParams::HkbProjectData(vec![
                            HkbProjectData::WorldUpWs(Primitive {
                                value: Vector4 {
                                    x: OrderedFloat(0.0),
                                    y: OrderedFloat(0.0),
                                    z: OrderedFloat(1.0),
                                    w: OrderedFloat(0.0),
                                },
                            }),
                            HkbProjectData::StringData(Primitive {
                                value: "#0052".into(),
                            }),
                            HkbProjectData::DefaultEventMode(Primitive {
                                value: EventMode::EventModeIgnoreFromGenerator,
                            }),
                        ]),
                    },
                    Class {
                        name: "#0052".into(),
                        class: "hkbProjectStringData".into(),
                        signature: "0x076ad60a".into(),
                        hkparams: ClassParams::HkbProjectStringData(vec![
                            HkbProjectStringData::AnimationFilenames(HkArrayStringPtr {
                                numelements: 0,
                                hkcstrings: vec![],
                            }),
                            HkbProjectStringData::BehaviorFilenames(HkArrayStringPtr {
                                numelements: 0,
                                hkcstrings: vec![],
                            }),
                            HkbProjectStringData::CharacterFilenames(HkArrayStringPtr {
                                numelements: 1,
                                hkcstrings: vec!["Characters\\DefaultMale.hkx".into()],
                            }),
                            HkbProjectStringData::EventNames(HkArrayStringPtr {
                                numelements: 0,
                                hkcstrings: vec![],
                            }),
                            HkbProjectStringData::AnimationPath(Primitive { value: "".into() }),
                            HkbProjectStringData::BehaviorPath(Primitive { value: "".into() }),
                            HkbProjectStringData::CharacterPath(Primitive { value: "".into() }),
                            HkbProjectStringData::FullPathToSource(Primitive { value: "".into() }),
                        ]),
                    },
                ],
            },
        };
        assert_eq!(result, expected);
        Ok(())
    }
}
