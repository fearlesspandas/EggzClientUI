
use gdnative::prelude::*;
use crate::traits::{Defaulted};

#[derive(Copy,Clone,Eq,Hash,PartialEq,Debug)]
pub enum AbilityType{
    empty,
    occupied,
    smack,
    globular_teleport,
    slizzard,
}
impl Defaulted for AbilityType{
    fn default() -> Self{
        AbilityType::empty
    }
}
impl From<u8> for AbilityType{
    fn from(value:u8) -> Self{
        match value{
            255 => AbilityType::empty,
            254 => AbilityType::occupied,
            0 => AbilityType::smack,
            1 => AbilityType::globular_teleport,
            2 => AbilityType::slizzard,
            _ => todo!(),
        }
    }
}
impl Into<u8> for AbilityType{
    fn into(self) -> u8 {
        match self{
            AbilityType::empty => 255,
            AbilityType::occupied => 254,
            AbilityType::smack => 0,
            AbilityType::globular_teleport => 1,
            AbilityType::slizzard => 2,
        }
    }
}
impl ToVariant for AbilityType{
    fn to_variant(&self) -> Variant{
        let u:u8 = (*self).into();
        Variant::new(u)
    }
}
impl FromVariant for AbilityType{
    fn from_variant(item:&Variant) -> Result<AbilityType,FromVariantError>{
        match item.get_type() {
            VariantType::I64 if item.try_to::<i64>().map(|x| x < 0).unwrap_or(false) => Ok(AbilityType::occupied),
            VariantType::I64 => item.try_to::<u8>().map(|id| AbilityType::from(id)),
            VariantType::F64 => item.try_to::<f64>().map(|id_f| id_f.round() as i64 as u8).map(|id| AbilityType::from(id)),
            typ => {Err(FromVariantError::Custom(format!("Could not cast from given type {typ:?}").to_string()))}
        }
    }
}
trait ToLabel{
    fn to_label(&self) -> String;
}
impl ToString for AbilityType{
    fn to_string(&self) -> String{
        match self{
            AbilityType::empty => "Empty".to_string(),
            AbilityType::occupied => {assert!(false,"called to_string for occupied");todo!()}
            AbilityType::smack => "Smack".to_string(),
            AbilityType::globular_teleport => "Globular Teleport".to_string(),
            AbilityType::slizzard => "Slizzard".to_string(),
        }
    }
}
#[derive(Copy,Clone,Eq,Hash,PartialEq)]
pub enum SubAbilityType{
    empty,
    globular_teleport_anchor,
    globular_teleport_vertex,
}
impl From<u8> for SubAbilityType{
    fn from(value:u8) -> Self{
        match value{
            255 => SubAbilityType::empty,
            0 => SubAbilityType::globular_teleport_anchor,
            1 => SubAbilityType::globular_teleport_vertex,
            _ => todo!(),
        }
    }
}
impl Into<u8> for SubAbilityType{
    fn into(self) -> u8 {
        match self{
            SubAbilityType::empty => 255,
            SubAbilityType::globular_teleport_anchor => 0,
            SubAbilityType::globular_teleport_vertex => 1,
        }
    }
}
