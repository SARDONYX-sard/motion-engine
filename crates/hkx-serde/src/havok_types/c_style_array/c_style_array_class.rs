//! If you use the `HkArray` implementation as is, the serialization will include `numelements` attribute.
//! If I set it to [`Option`] and branch the process, I don't know if it is an error or not.
//! Therefore, the code is almost the same, but the code is divided.
use serde::{Deserialize, Serialize};

/// In C++, this is the case when the type of the in-class field is a class.
///
/// In XML, A type to hold another class of `hkparam` as an array within `hkparam`.
///
/// # XML Example
/// ```xml
/// <hkparam name="variantVariableValues">
///     <hkobject>
///         <hkparam name="class_field">#0063</hkparam>
///     </hkobject>
///     <hkobject>
///         <hkparam name="another_class_field">#0064</hkparam>
///     </hkobject>
/// </hkparam>
/// ```
///
/// This is almost the same as for `hkArrayClass`, but with the following differences.
/// - There is no `numelements` attribute.
/// - The number of elements is limited.
///
/// # Note
/// - Extra values are ignored.(e.g. `[i32; 10]` => `[i32; 5]`)
/// - In summary, the parent enum determines and retrieves the `name` attribute, so it is not included in this structure.
///
///   The `name` attribute is required for `hkparam` but is not included in this structure.
///   This is because the value of the `name` attribute corresponds to a C++ field name,
///   and the processing must be changed according to the value.
///   And to do that, we need the parent enum that wraps this structure.
///
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkparam")]
pub struct CStyleArrayClass<T> {
    /// An array that receives `hkparam` of a certain class.
    ///
    /// The XML for this class does not require `name` and `signature`, but only encloses `hkobject` tags without attributes.
    #[serde(rename = "hkobject")]
    pub classes: Vec<CStyleArrayClassParam<T>>,
}

impl<T> From<Vec<CStyleArrayClassParam<T>>> for CStyleArrayClass<T> {
    fn from(classes: Vec<CStyleArrayClassParam<T>>) -> Self {
        Self { classes }
    }
}

impl<T> From<Vec<T>> for CStyleArrayClass<T> {
    fn from(classes: Vec<T>) -> Self {
        Self {
            classes: classes
                .into_iter()
                .map(CStyleArrayClassParam::from)
                .collect(),
        }
    }
}

/// Field(`hkparam`) of C++ Class(`hkobject`)
///
/// ```xml
/// <hkobject>
///     <hkparam>#0063</hkparam> <!-- This line -->
/// </hkobject>
/// ````
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CStyleArrayClassParam<T> {
    #[serde(rename = "hkparam")]
    pub hkparam: T,
}

impl<T> From<T> for CStyleArrayClassParam<T> {
    fn from(value: T) -> Self {
        Self { hkparam: value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize() {
        let data: CStyleArrayClass<i32> = vec![1045220557, 0].into();
        let serialized = quick_xml::se::to_string(&data).unwrap();

        let expected_xml = "\
            <hkparam>\
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
            <hkparam name="variantVariableValues">
                <hkobject>
                    <hkparam>#0063</hkparam>
                </hkobject>
                <hkobject>
                    <hkparam>#0064</hkparam>
                </hkobject>
            </hkparam>
        "###;
        let deserialized: CStyleArrayClass<&str> = quick_xml::de::from_str(xml).unwrap();

        let expected = vec!["#0063", "#0064"].into();

        assert_eq!(deserialized, expected);
    }
}
