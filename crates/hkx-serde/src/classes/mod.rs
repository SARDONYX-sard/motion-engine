mod generated;
mod hkb_behavior_graph_string_data;
mod hkb_variable_value;
mod hkb_variable_value_set;
mod root;

pub mod hkx_vertex_description;
pub mod hkx_vertex_description_element_decl;

use generated::class_params::ClassParams;
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class name
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Class<'a> {
    /// e.g. `#0106`
    ///
    /// In XML, these names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    pub name: Cow<'a, str>,

    /// Name of each C++ class.
    ///
    /// e.g. `"hkbBehaviorGraphStringData"`
    pub class: Cow<'a, str>,

    /// Unique value of each class.
    ///
    /// e.g. `0xc713064e`
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ fields) vector
    pub hkparams: ClassParams<'a>,
}
