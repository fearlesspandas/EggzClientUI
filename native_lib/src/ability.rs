use gdnative::prelude::*;
use gdnative::api::*;
use serde::{Serialize,Deserialize}; 
use serde_json::{Value};
use tokio::sync::mpsc;
use std::collections::HashMap;
use crate::traits::{Defaulted,Instanced};
use crate::field::{Location,FieldCommand,FieldZone};
use crate::field_abilities::{AbilityType};
use crate::field_ability_colliders::{ToCollider};
use crate::field_ability_mesh::{ToMeshToo};
use crate::field_ability_actions::{ToAction};

type Sender<T> = mpsc::UnboundedSender<T>;

trait Signals<T>{
    fn command_tx(&self) -> &Sender<T>;
    fn set_command_tx(&mut self,tx:Sender<T>);
}
//SERIALIZATION///////
#[derive(Serialize,Deserialize)]
pub struct AbilityData{
    typ:AbilityType,
    location:(i64,i64),
    data:Value,
}
////FIELD ABILITIES////
////ability representations
////while on the field;
////activation logic,
////state representation
trait FieldAbility:Sized +  Into<AbilityType>{}
trait ClientFieldAbility:FieldAbility + ToMeshToo + ToAction{
    fn radius(&self) -> f32;
    fn modify_data(&self,data:AbilityData);
}
trait ServerFieldAbility:FieldAbility + ToAction + ToCollider{
    fn radius(&self) -> f32;
    fn modify_data(&self,data:AbilityData);
}
////ABILITIES////
////ability activations and affects on the world
////collider and mesh handling during ability affects

trait Ability:Sized +  Into<AbilityType>{}
trait ClientAbility:Ability + ToMeshToo + ToAction{
    fn mesh(&self) -> Ref<Spatial>;
    fn duration(&self) -> f32;
    fn set_duration(&self , value:f32);

    fn default_mesh(&self) -> Ref<Spatial>{
        Self::to_mesh(25.0,25.0)
    }
}
trait ServerAbility:Ability + ToCollider{
    fn radius(&self) -> f32;
    fn duration(&self) -> f32;
    fn set_duration(&self , value:f32);
}
trait TimedAbility:Ability{
    fn duration(&self) -> f32;
    fn destroy(&self,owner:TRef<Node>);
    fn ready(&self,owner:TRef<Node>){
        assert!(owner.has_method("destroy"),"owner does not have destroy method");
        let timer = Timer::new().into_shared();
        let timer = unsafe{timer.assume_safe()};
        owner.add_child(timer,true);
        let _ = timer.connect("timeout",owner,"destroy",VariantArray::new_shared(),0);
        timer.start(self.duration().into());
    }
}

////ABILITY IMPLEMENTATIONS////
//////////////////////////////
////Smack
#[derive(NativeClass)]
#[inherit(Node)]
pub struct Smack{
    radius:f32,
    cmd_tx:Option<Sender<FieldCommand>>,
}
impl Into<AbilityType> for Smack{
    fn into(self) -> AbilityType{
        AbilityType::smack
    }
}
impl Instanced<Node> for Smack{
    fn make() -> Self{
        Smack{
            radius:0.0,
            cmd_tx:None,
        }
    }
}
#[methods]
impl Smack{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Node>){
        let mesh = Self::to_mesh(25.0,25.0);
        let collider = self.to_collider(Vector3::new(25.0,25.0,25.0)).expect("ability:No collider found for smack client");
        owner.add_child(mesh,true);
        owner.add_child(collider,true);
        <Self as TimedAbility>::ready(self,owner);
        
    }
    #[method]
    fn destroy(&self,#[base] owner:TRef<Node>){
        <Self as TimedAbility>::destroy(self,owner);
    }
}
impl Signals<FieldCommand> for Smack{
    fn command_tx(&self) -> &Sender<FieldCommand>{&self.cmd_tx.as_ref().expect("SmackErr:No outbound tx found")}
    fn set_command_tx(&mut self,tx:Sender<FieldCommand>){self.cmd_tx = Some(tx)}
}
////We can have multiple signal channels like so
////(uncommenting the following will still compile)
//impl Signals<i64> for Smack{
//    fn command_tx(&self) -> &Sender<i64>{todo!()}
//    fn set_command_tx(&mut self,tx:Sender<i64>){}
//}
impl ClientFieldAbility for Smack{
    fn radius(&self) -> f32{self.radius}
    fn modify_data(&self,data:AbilityData){}
}
impl ToMeshToo for Smack{
    fn to_mesh(length:f32,radius:f32) -> Ref<Spatial>{
        let spatial = Spatial::new();
        let mesh = MeshInstance::new().into_shared();
        let mesh_obj = unsafe{mesh.assume_safe()};
        let sphere_mesh = SphereMesh::new().into_shared();
        let sphere_mesh = unsafe{sphere_mesh.assume_safe()};
        sphere_mesh.set_radius(radius.into());
        sphere_mesh.set_height(radius.into());
        let box_material = SpatialMaterial::new();
        box_material.set_albedo(Color{r:100.0,g:100.0,b:0.0,a:1.0});
        sphere_mesh.set_material(box_material);
        mesh_obj.set_mesh(sphere_mesh);
        spatial.add_child(mesh.clone(),true);
        //mesh
        spatial.into_shared()
    }
}
impl ToAction for Smack{
    fn to_action(&self,tx:Sender<FieldCommand>,location:&Location,field_state:&HashMap<Location,Instance<FieldZone>>){

    }
}
impl ToCollider for Smack{
    fn to_collider(&self,extents:Vector3) -> Option<Ref<Area>> {
        let sphere_shape = SphereShape::new().into_shared();
        let sphere_shape = unsafe{sphere_shape.assume_safe()};
        sphere_shape.set_radius(35.0);
        let collider = CollisionShape::new().into_shared();
        let collider = unsafe{collider.assume_safe()};
        collider.set_shape(sphere_shape);
        let area = Area::new().into_shared();
        let area = unsafe{area.assume_safe()};
        area.add_child(collider,true);
        Some(area.claim())
    }
}
impl TimedAbility for Smack{
    fn duration(&self) -> f32{
        3.0
    }
    fn destroy(&self,owner:TRef<Node>){

    }
}
impl FieldAbility for Smack{ }
impl Ability for Smack{}
////Globular Teleport////
pub struct GlobularTeleport{
    radius:f32,
    cmd_tx:Sender<FieldCommand>,
}
impl Signals<FieldCommand> for GlobularTeleport{
    fn command_tx(&self) -> &Sender<FieldCommand>{&self.cmd_tx}
    fn set_command_tx(&mut self,tx:Sender<FieldCommand>){self.cmd_tx = tx}
}
impl ClientFieldAbility for GlobularTeleport{
    fn radius(&self) -> f32{self.radius}
    fn modify_data(&self,data:AbilityData){}
}

impl Into<AbilityType> for GlobularTeleport{
    fn into(self) -> AbilityType{
        AbilityType::smack
    }
}
impl ToMeshToo for GlobularTeleport{
    fn to_mesh(length:f32,radius:f32) -> Ref<Spatial>{
        todo!()
    }
}
impl ToAction for GlobularTeleport{
    fn to_action(&self,tx:Sender<FieldCommand>,location:&Location,field_state:&HashMap<Location,Instance<FieldZone>>){

    }
}
impl ToCollider for GlobularTeleport{
    fn to_collider(&self,extents:Vector3) -> Option<Ref<Area>> {
        None
    }
}
impl FieldAbility for GlobularTeleport{ }


