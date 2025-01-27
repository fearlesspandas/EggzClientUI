
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{CreateSignal,Instanced,InstancedDefault,Defaulted};
use crate::field::{OpType};
use tokio::sync::mpsc;

trait ToMesh{
    fn to_mesh(&self,radius:f32) -> Ref<MeshInstance>;
}
impl ToMesh for OpType{
    fn to_mesh(&self,radius:f32) -> Ref<MeshInstance> {
        match self{
            OpType::empty => {
                let mesh = MeshInstance::new().into_shared();
                let mesh_obj = unsafe{mesh.assume_safe()};
                let box_mesh = CubeMesh::new().into_shared();
                let box_mesh = unsafe{box_mesh.assume_safe()};
                box_mesh.set_size(Vector3{x:25.0,y:radius,z:radius});
                let box_material = SpatialMaterial::new();
                box_material.set_albedo(Color{r:0.0,g:30.0,b:30.0,a:1.0});
                box_mesh.set_material(box_material);
                mesh_obj.set_mesh(box_mesh);
                mesh
            }
            OpType::smack => {
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
                mesh
            }
        
        }
    }
}
#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct FieldAbilityMesh{
    mesh:Ref<MeshInstance>,
}
impl InstancedDefault<Spatial,OpType> for FieldAbilityMesh{
    fn make(args:&OpType) -> Self{
        FieldAbilityMesh{
            mesh:args.to_mesh(5.0),
        }
    }
}
#[methods]
impl FieldAbilityMesh{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Spatial>){
        let mesh = unsafe{self.mesh.assume_safe()};
        owner.add_child(mesh,true);
    }
}
