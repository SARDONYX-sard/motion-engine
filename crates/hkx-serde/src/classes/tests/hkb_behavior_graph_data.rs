#[cfg(test)]
mod tests {
    use crate::classes::generated::class_params::ClassParams;
    use crate::classes::generated::{
        HkbBehaviorGraphData, HkbRoleAttribute, HkbVariableInfo, Role, RoleFlags, VariableType,
    };
    use crate::classes::Class;
    use crate::havok_types::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let class = Class {
            name: "#0087".into(),
            class: "hkbBehaviorGraphData".into(),
            signature: "0x95aca5d".into(),
            hkparams: ClassParams::HkbBehaviorGraphData(vec![
                HkbBehaviorGraphData::AttributeDefaults(HkArrayNum {
                    numelements: 0,
                    values: vec![],
                }),
                HkbBehaviorGraphData::VariableInfos(HkArrayClass {
                    numelements: 101,
                    classes: Some(
                        vec![
                            vec![
                                HkbVariableInfo::Role(SingleClass {
                                    class: vec![
                                        HkbRoleAttribute::Role(Role::RoleDefault.into()),
                                        HkbRoleAttribute::Flags(RoleFlags::NULL.into()),
                                    ]
                                    .into(),
                                }),
                                HkbVariableInfo::Type(VariableType::VariableTypeReal.into()),
                            ],
                            vec![
                                HkbVariableInfo::Role(SingleClass {
                                    class: vec![
                                        HkbRoleAttribute::Role(Role::RoleDefault.into()),
                                        HkbRoleAttribute::Flags(RoleFlags::NULL.into()),
                                    ]
                                    .into(),
                                }),
                                HkbVariableInfo::Type(VariableType::VariableTypeInt32.into()),
                            ],
                            vec![
                                HkbVariableInfo::Role(SingleClass {
                                    class: vec![
                                        HkbRoleAttribute::Role(Role::RoleDefault.into()),
                                        HkbRoleAttribute::Flags(RoleFlags::NULL.into()),
                                    ]
                                    .into(),
                                }),
                                HkbVariableInfo::Type(VariableType::VariableTypeBool.into()),
                            ],
                        ]
                        .into_iter()
                        .map(HkArrayClassParam::from)
                        .collect(),
                    ),
                }),
                HkbBehaviorGraphData::WordMinVariableValues(HkArrayClass {
                    numelements: 0,
                    classes: None,
                }),
                HkbBehaviorGraphData::WordMaxVariableValues(HkArrayClass {
                    numelements: 0,
                    classes: None,
                }),
                HkbBehaviorGraphData::VariableInitialValues(Cow::from("#0086").into()),
                HkbBehaviorGraphData::StringData(Cow::from("#0085").into()),
            ]),
        };

        let serialized = quick_xml::se::to_string(&class).unwrap();

        let expected = "\
<Class name=\"#0087\" class=\"hkbBehaviorGraphData\" signature=\"0x95aca5d\">\
  <hkparam name=\"attributeDefaults\" numelements=\"0\"/>\
  <hkparam name=\"variableInfos\" numelements=\"101\">\
    <hkobject>\
      <hkparam name=\"role\">\
        <hkobject>\
          <hkparam name=\"role\">ROLE_DEFAULT</hkparam>\
          <hkparam name=\"flags\">0</hkparam>\
        </hkobject>\
      </hkparam>\
      <hkparam name=\"type\">VARIABLE_TYPE_REAL</hkparam>\
    </hkobject>\
    <hkobject>\
      <hkparam name=\"role\">\
        <hkobject>\
          <hkparam name=\"role\">ROLE_DEFAULT</hkparam>\
          <hkparam name=\"flags\">0</hkparam>\
        </hkobject>\
      </hkparam>\
      <hkparam name=\"type\">VARIABLE_TYPE_INT32</hkparam>\
    </hkobject>\
    <hkobject>\
      <hkparam name=\"role\">\
        <hkobject>\
          <hkparam name=\"role\">ROLE_DEFAULT</hkparam>\
          <hkparam name=\"flags\">0</hkparam>\
        </hkobject>\
      </hkparam>\
      <hkparam name=\"type\">VARIABLE_TYPE_BOOL</hkparam>\
    </hkobject>\
  </hkparam>\
  <hkparam name=\"wordMinVariableValues\" numelements=\"0\"/>\
  <hkparam name=\"wordMaxVariableValues\" numelements=\"0\"/>\
  <hkparam name=\"variableInitialValues\">#0086</hkparam>\
  <hkparam name=\"stringData\">#0085</hkparam>\
</Class>";

        assert_eq!(serialized, expected);
    }

    #[test]
    fn should_deserialize() {
        let xml_str = r###"
<hkobject name="#0087" class="hkbBehaviorGraphData" signature="0x95aca5d">
  <hkparam name="attributeDefaults" numelements="0"></hkparam>
  <hkparam name="variableInfos" numelements="101">
    <hkobject>
      <hkparam name="role">
        <hkobject>
          <hkparam name="role">ROLE_DEFAULT</hkparam>
          <hkparam name="flags">0</hkparam>
        </hkobject>
      </hkparam>
      <hkparam name="type">VARIABLE_TYPE_REAL</hkparam>
    </hkobject>
    <hkobject>
      <hkparam name="role">
        <hkobject>
          <hkparam name="role">ROLE_DEFAULT</hkparam>
          <hkparam name="flags">0</hkparam>
        </hkobject>
      </hkparam>
      <hkparam name="type">VARIABLE_TYPE_INT32</hkparam>
    </hkobject>
    <hkobject>
      <hkparam name="role">
        <hkobject>
          <hkparam name="role">ROLE_DEFAULT</hkparam>
          <hkparam name="flags">0</hkparam>
        </hkobject>
      </hkparam>
      <hkparam name="type">VARIABLE_TYPE_BOOL</hkparam>
    </hkobject>
  </hkparam>
  <hkparam name="wordMinVariableValues" numelements="0"></hkparam>
  <hkparam name="wordMaxVariableValues" numelements="0"></hkparam>
  <hkparam name="variableInitialValues">#0086</hkparam>
  <hkparam name="stringData">#0085</hkparam>
</hkobject>
"###;
        let deserialized: Class = quick_xml::de::from_str(xml_str).unwrap();

        let expected = Class {
            name: "#0087".into(),
            class: "hkbBehaviorGraphData".into(),
            signature: "0x95aca5d".into(),
            hkparams: ClassParams::HkbBehaviorGraphData(vec![
                HkbBehaviorGraphData::AttributeDefaults(HkArrayNum {
                    numelements: 0,
                    values: vec![],
                }),
                HkbBehaviorGraphData::VariableInfos(HkArrayClass {
                    numelements: 101,
                    classes: Some(
                        vec![
                            vec![
                                HkbVariableInfo::Role(SingleClass {
                                    class: vec![
                                        HkbRoleAttribute::Role(Role::RoleDefault.into()),
                                        HkbRoleAttribute::Flags(RoleFlags::NULL.into()),
                                    ]
                                    .into(),
                                }),
                                HkbVariableInfo::Type(VariableType::VariableTypeReal.into()),
                            ],
                            vec![
                                HkbVariableInfo::Role(SingleClass {
                                    class: vec![
                                        HkbRoleAttribute::Role(Role::RoleDefault.into()),
                                        HkbRoleAttribute::Flags(RoleFlags::NULL.into()),
                                    ]
                                    .into(),
                                }),
                                HkbVariableInfo::Type(VariableType::VariableTypeInt32.into()),
                            ],
                            vec![
                                HkbVariableInfo::Role(SingleClass {
                                    class: vec![
                                        HkbRoleAttribute::Role(Role::RoleDefault.into()),
                                        HkbRoleAttribute::Flags(RoleFlags::NULL.into()),
                                    ]
                                    .into(),
                                }),
                                HkbVariableInfo::Type(VariableType::VariableTypeBool.into()),
                            ],
                        ]
                        .into_iter()
                        .map(HkArrayClassParam::from)
                        .collect(),
                    ),
                }),
                HkbBehaviorGraphData::WordMinVariableValues(HkArrayClass {
                    numelements: 0,
                    classes: None,
                }),
                HkbBehaviorGraphData::WordMaxVariableValues(HkArrayClass {
                    numelements: 0,
                    classes: None,
                }),
                HkbBehaviorGraphData::VariableInitialValues(Cow::from("#0086").into()),
                HkbBehaviorGraphData::StringData(Cow::from("#0085").into()),
            ]),
        };

        assert_eq!(deserialized, expected);
    }
}
