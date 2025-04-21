use gdnative::prelude::*;
use gdnative::api::*;
use serde::{Serialize,Deserialize}; 
use serde_json::{Value};
use tokio::sync::mpsc;
use std::collections::HashMap;
use crate::traits::{Defaulted,Instanced};
use crate::field::{Location,FieldCommand,FieldZone};
use crate::field_abilities::{AbilityType};
use crate::field_ability_colliders::{ToCollider,FieldCollider,AbilityCollider};
use crate::field_ability_mesh::{ToMesh,FieldMesh,AbilityMesh};
use crate::field_ability_actions::{ToAction};
use crate::assets::{array_mesh,point_material,point_mesh};
use crate::ability::{
    AbilityId,
    Signals,
    ToAbilityType,
    AbilityData,
    Ability,
    FieldAbility,
    ClientFieldAbility,
    ServerFieldAbility,
    ClientAbility,
    ServerAbility,
    Smack,
    GlobularTeleport,
};

type Sender<T> = mpsc::UnboundedSender<T>;
impl From<AbilityType> for Abilities{
    fn from(item:AbilityType) -> Self{
        match item{
            AbilityType::smack => {
                let smack = Smack::make_instance().into_shared();
                Abilities::smack(smack)
            }
            AbilityType::globular_teleport => {
                let gt = GlobularTeleport::make_instance().into_shared();
                Abilities::globular_teleport(gt)
            }
            _ => todo!()
        }
    }
}
pub enum Abilities{
    smack(Instance<Smack>),
    globular_teleport(Instance<GlobularTeleport>),
}
impl Abilities{ }
impl ClientAbility for Abilities{ }
impl ServerAbility for Abilities{}
impl Ability for Abilities{
    fn id(&self) -> AbilityId{ todo!()}
}
impl ClientFieldAbility for Abilities{
    fn radius(&self) -> f32{
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| ClientFieldAbility::radius(obj))
                    .expect("Abilities:Could not get Smack radius")
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| ClientFieldAbility::radius(obj))
                    .expect("Abilities:Could not get GlobularTeleport radius")
            }
        }
    }
    fn modify_data(&self,data:AbilityData){}
}
impl ServerFieldAbility for Abilities{
    fn radius(&self) -> f32{
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| ServerFieldAbility::radius(obj))
                    .expect("Abilities:Could not get Smack radius")
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| ServerFieldAbility::radius(obj))
                    .expect("Abilities:Could not get GlobularTeleport radius")
            }
        }
    }
    fn modify_data(&self,data:AbilityData){}
}
impl ToAbilityType for Abilities{
    const TYPE:AbilityType = AbilityType::empty;
}
impl Signals<FieldCommand> for Abilities{
    fn command_tx(&self) -> Sender<FieldCommand>{
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| Signals::<FieldCommand>::command_tx(obj))
                    .expect("Abilities:Could not get Smack tx")
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| Signals::<FieldCommand>::command_tx(obj))
                    .expect("Abilities:Could not get GlobularTeleport tx")
            }
        }
    }
    fn set_command_tx(&mut self,tx:Sender<FieldCommand>){
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                let _ = smack.map_mut(|obj,_| Signals::<FieldCommand>::set_command_tx(obj,tx));
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                let _ = globular_teleport.map_mut(|obj,_| Signals::<FieldCommand>::set_command_tx(obj,tx));
            }
        }
    }
}
impl FieldMesh for Abilities{
    fn to_mesh(&self,length:f32,radius:f32) -> Ref<Spatial>{
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| FieldMesh::to_mesh(obj,length,radius))
                    .expect("Abilities:Could not get Smack field mesh")
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| FieldMesh::to_mesh(obj,length,radius))
                    .expect("Abilities:Could not get GlobularTeleport field mesh")
            }
        }
    }
}
impl AbilityMesh for Abilities{
    fn to_mesh(&self) -> Ref<Spatial>{
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| AbilityMesh::to_mesh(obj))
                    .expect("Abilities:Could not get Smack ability mesh")
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| AbilityMesh::to_mesh(obj))
                    .expect("Abilities:Could not get GlobularTeleport ability mesh")
            }
        }
    }
}
impl ToAction for Abilities{
    fn to_action(&self,tx:Sender<FieldCommand>,location:&Location,field_state:&HashMap<Location,Instance<FieldZone>>){
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                let _ = smack.map_mut(|obj,_| ToAction::to_action(obj,tx.clone(),location,field_state));
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                let _ = globular_teleport.map_mut(|obj,_| ToAction::to_action(obj,tx.clone(),location,field_state));
            }
        }
    }
}
impl AbilityCollider for Abilities{
    fn to_collider(&self,extents:Vector3) -> Option<Ref<Area>> {
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| AbilityCollider::to_collider(obj,extents))
                    .expect("Abilities:Could not get Smack ability collider")
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| AbilityCollider::to_collider(obj,extents))
                    .expect("Abilities:Could not get GlobularTeleport ability collider")
            }
        }
    }
}
impl FieldCollider for Abilities{
    fn to_collider(&self,extents:Vector3) -> Option<Ref<Area>> {
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| FieldCollider::to_collider(obj,extents))
                    .expect("Abilities:Could not get Smack field collider")
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| FieldCollider::to_collider(obj,extents))
                    .expect("Abilities:Could not get GlobularTeleport field collider")
            }
        }
    }
}
impl FieldAbility for Abilities{
    fn instance_count(&self) -> i64{
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| FieldAbility::instance_count(obj))
                    .expect("Abilities:Could not get Smack instance count")
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| FieldAbility::instance_count(obj))
                    .expect("Abilities:Could not get GlobularTeleport instance count")
            }
        }
    }
    fn set_instance_count(&mut self,value:i64){
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                let _ = smack.map_mut(|obj,_| FieldAbility::set_instance_count(obj,value));
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                let _ = globular_teleport.map_mut(|obj,_| FieldAbility::set_instance_count(obj,value));
            }
        }
    }
    fn proc_count(&self) -> i64{
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                smack.map(|obj,_| FieldAbility::proc_count(obj))
                    .expect("Abilities:Could not get Smack proc count")
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                globular_teleport.map(|obj,_| FieldAbility::proc_count(obj))
                    .expect("Abilities:Could not get GlobularTeleport proc count")
            }
        }
    }
    fn set_proc_count(&mut self,value:i64){
        match self{
            Abilities::smack(smack) => {
                let smack = unsafe{smack.assume_safe()};
                let _ = smack.map_mut(|obj,_| FieldAbility::set_proc_count(obj,value));
            }
            Abilities::globular_teleport(globular_teleport) => {
                let globular_teleport = unsafe{globular_teleport.assume_safe()};
                let _ = globular_teleport.map_mut(|obj,_| FieldAbility::set_proc_count(obj,value));
            }
        }
    }
}
