//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleFrictionStatusAxisStatus`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpVehicleFrictionStatusAxisStatus`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: false
/// - signature: `0xe70e2bb4`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpVehicleFrictionStatusAxisStatus {
    /// # C++ Class Fields Info
    /// -   name:`"forward_slip_velocity"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forward_slip_velocity")]
    ForwardSlipVelocity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"side_slip_velocity"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "side_slip_velocity")]
    SideSlipVelocity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"skid_energy_density"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skid_energy_density")]
    SkidEnergyDensity(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"side_force"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "side_force")]
    SideForce(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"delayed_forward_impulse"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "delayed_forward_impulse")]
    DelayedForwardImpulse(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"sideRhs"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sideRhs")]
    SideRhs(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"forwardRhs"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forwardRhs")]
    ForwardRhs(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"relativeSideForce"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "relativeSideForce")]
    RelativeSideForce(Primitive<f32>),
    /// # C++ Class Fields Info
    /// -   name:`"relativeForwardForce"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "relativeForwardForce")]
    RelativeForwardForce(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleFrictionStatusAxisStatus, "@name",
    ("forward_slip_velocity" => ForwardSlipVelocity(Primitive<f32>)),
    ("side_slip_velocity" => SideSlipVelocity(Primitive<f32>)),
    ("skid_energy_density" => SkidEnergyDensity(Primitive<f32>)),
    ("side_force" => SideForce(Primitive<f32>)),
    ("delayed_forward_impulse" => DelayedForwardImpulse(Primitive<f32>)),
    ("sideRhs" => SideRhs(Primitive<f32>)),
    ("forwardRhs" => ForwardRhs(Primitive<f32>)),
    ("relativeSideForce" => RelativeSideForce(Primitive<f32>)),
    ("relativeForwardForce" => RelativeForwardForce(Primitive<f32>)),
}

impl ByteDeSerialize for HkpVehicleFrictionStatusAxisStatus {
    fn from_bytes<B>(bytes: &[u8]) -> Result<Vec<Self>>
    where
        B: ByteOrder,
        Self: Sized,
    {
        todo!()
    }
}
