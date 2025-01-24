
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{CreateSignal,Instanced,InstancedDefault,Defaulted};
use tokio::sync::mpsc;

const zone_width:f32 = 20.0;
const zone_height:f32 = 1.0;
#[derive(Copy,Clone)]
pub struct Location{
    x:i64,
    y:i64,
}
impl Defaulted for Location{
    fn default() -> Self{
        Location{x:0,y:0}
    }
}
#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct FieldZone{
    location:Location,
    mesh:Ref<MeshInstance>,
}
impl InstancedDefault<Spatial,Location> for FieldZone{
    fn make(args:&Location) -> Self{
        FieldZone{
            location:*args,
            mesh:MeshInstance::new().into_shared(),
        }
    }
}
#[methods]
impl FieldZone{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Spatial>){
        let mesh = unsafe{self.mesh.assume_safe()};
        let cube = CubeMesh::new().into_shared();
        let cube = unsafe{cube.assume_safe()};
        cube.set_size(Vector3{x:zone_width,y:zone_height,z:zone_width});
        mesh.set_mesh(cube);
        owner.add_child(mesh,true);
    }
}
#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct Field{
    zones:Vec<Instance<FieldZone>>,
}
impl Instanced<Spatial> for Field{
    fn make() -> Self{
        Field{
            zones:Vec::new(),
        }
    }
}
#[methods]
impl Field{
    #[method]
    fn _ready(&self, #[base] owner:TRef<Spatial>){
    }

    #[method]
    fn _process(&self,#[base] owner:TRef<Spatial>,delta:f64){

    }
    #[method]
    fn add_zone(&self,#[base] owner:TRef<Spatial>,location:(i64,i64)){
        let (x,y) = location;
        let location = Location{x:x,y:y};
        let zone = FieldZone::make_instance(&location).into_shared();
        let zone = unsafe{zone.assume_safe()};
        let owner_transform = owner.transform();
        owner.add_child(zone.clone(),true);
        zone.map(|obj,spatial| {
            let mut transform = spatial.transform();
            transform.origin = Vector3{x:zone_width * (location.x as f32),y:0.0,z:zone_width * (location.y as f32)};
            spatial.set_transform(transform);
        });
    }
}

