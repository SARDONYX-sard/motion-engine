use serde::{Deserialize, Serialize};

/// # Vector of Class in Class field
/// - In C++, this is the case when the type of the in-class field is a class.
/// - In XML, A type to hold another class of `hkparam` as an array within `hkparam`.
///
/// e.g. `wordVariableValues` field of `hkbVariableValueSet` class
///
/// # XML Example
/// ```xml
/// <hkparam name="variantVariableValues" numelements="2">
///     <hkobject>
///         <hkparam name="class_field">#0063</hkparam>
///     </hkobject>
///     <hkobject>
///         <hkparam name="another_class_field">#0064</hkparam>
///     </hkobject>
/// </hkparam>
/// ```
///
/// # Note
/// The `name` attribute is required for `hkparam` but is not included in this structure.
/// This is because the value of the `name` attribute corresponds to a C++ field name,
/// and the processing must be changed according to the value.
/// And to do that, we need the parent enum that wraps this structure.
///
/// In summary, the parent enum determines and retrieves the `name` attribute, so it is not included in this structure.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkparam")]
pub struct HkArrayClass<T> {
    /// Length of the class
    #[serde(rename = "@numelements")]
    pub numelements: usize,
    /// An array that receives `hkparam` of a certain class.
    ///
    /// The XML for this class does not require `name` and `signature`, but only encloses `hkobject` tags without attributes.
    ///
    /// # Note
    /// [document](https://docs.rs/quick-xml/latest/quick_xml/de/index.html#sequences-xsall-and-xssequence-xml-schema-types) in quick_xml.
    /// If we try to use `#[serde(default)` for a enum(Represent C++ class), we will get stuck with the Default implementation,
    /// so you can get away with not serializing [`Option`].
    ///
    /// If there is no skip, an extra one is created like `<hkparam><hkobject /></hkparam>`.
    #[serde(rename = "hkobject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classes: Option<Vec<HkArrayClassParam<T>>>,
}

impl<T> From<Vec<HkArrayClassParam<T>>> for HkArrayClass<T> {
    fn from(classes: Vec<HkArrayClassParam<T>>) -> Self {
        Self {
            numelements: classes.len(),
            classes: Some(classes),
        }
    }
}

impl<T> From<Vec<Vec<T>>> for HkArrayClass<T> {
    fn from(classes: Vec<Vec<T>>) -> Self {
        Self {
            numelements: classes.len(),
            classes: Some(classes.into_iter().map(HkArrayClassParam::from).collect()),
        }
    }
}

/// Fields of a class in `HkArray`
///
/// ```xml
/// <hkobject>
///     <!--     This lines     -->
///     <hkparam>#0063</hkparam>
///     <hkparam>#0064</hkparam>
///     <!--     This lines     -->
/// </hkobject>
/// ````
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HkArrayClassParam<T> {
    /// Fields of the class.
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<T>,
}

impl<T> From<Vec<T>> for HkArrayClassParam<T> {
    fn from(value: Vec<T>) -> Self {
        Self { hkparams: value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let data: HkArrayClass<i32> = vec![vec![1045220557, 0]].into();
        let serialized = quick_xml::se::to_string(&data).unwrap();

        let expected_xml = "\
            <hkparam numelements=\"2\">\
                <hkobject>\
                    <hkparam>\
                        1045220557\
                    </hkparam>\
                </hkobject>\
                <hkobject>\
                    <hkparam>\
                        0\
                    </hkparam>\
                </hkobject>\
            </hkparam>\
        ";

        assert_eq!(serialized, expected_xml);
    }

    #[test]
    fn should_deserialize() {
        let xml = r###"
            <hkparam name="variantVariableValues" numelements="2">
                <hkobject>
                    <hkparam>#0063</hkparam>
                    <hkparam>#0063</hkparam>
                </hkobject>
                <hkobject>
                    <hkparam>#0064</hkparam>
                </hkobject>
            </hkparam>
        "###;
        let deserialized: HkArrayClass<&str> = quick_xml::de::from_str(xml).unwrap();

        let expected = HkArrayClass {
            numelements: 2,
            classes: Some(vec![
                HkArrayClassParam {
                    hkparams: vec!["#0063", "#0063"],
                },
                HkArrayClassParam {
                    hkparams: vec!["#0064"],
                },
            ]),
        };

        assert_eq!(deserialized, expected);
    }
}
