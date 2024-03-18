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
    fn should_deserialize() {
        let xml_str = r###"
<Class name="#0087" class="hkbBehaviorGraphData" signature="0x95aca5d">
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
</Class>
"###;
        let deserialized: Class = quick_xml::de::from_str(xml_str).unwrap();

        let expected = Class {
            name: "#0087".into(),
            class: "hkbBehaviorGraphData".into(),
            signature: "0x95aca5d".into(),
            hkparams: ClassParams::HkbBehaviorGraphData(vec![
                HkbBehaviorGraphData::AttributeDefaults(HkArrayRef {
                    numelements: 0,
                    value: vec![],
                }),
                HkbBehaviorGraphData::VariableInfos(HkArrayClass {
                    numelements: 101,
                    classes: vec![
                        vec![HkbVariableInfo::Role(SingleClass {
                            class: vec![
                                HkbRoleAttribute::Role(Role::RoleDefault.into()),
                                HkbRoleAttribute::Flags(RoleFlags::NULL.into()),
                            ]
                            .into(),
                        })],
                        vec![HkbVariableInfo::Type(VariableType::VariableTypeReal.into())],
                        //
                        //
                        vec![HkbVariableInfo::Role(SingleClass {
                            class: vec![
                                HkbRoleAttribute::Role(Role::RoleDefault.into()),
                                HkbRoleAttribute::Flags(RoleFlags::NULL.into()),
                            ]
                            .into(),
                        })],
                        vec![HkbVariableInfo::Type(
                            VariableType::VariableTypeInt32.into(),
                        )],
                        //
                        //
                        vec![HkbVariableInfo::Role(SingleClass {
                            class: vec![
                                HkbRoleAttribute::Role(Role::RoleDefault.into()),
                                HkbRoleAttribute::Flags(RoleFlags::NULL.into()),
                            ]
                            .into(),
                        })],
                        vec![HkbVariableInfo::Type(VariableType::VariableTypeBool.into())],
                    ]
                    .into_iter()
                    .map(HkArrayClassParam::from)
                    .collect(),
                }),
            ]),
        };

        println!("{:#?}", deserialized);
        assert_eq!(deserialized, expected);
    }
}
