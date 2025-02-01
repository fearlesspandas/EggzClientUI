
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{CreateSignal,Instanced,InstancedDefault,Defaulted};
use crate::field_abilities::{OpType};
use tokio::sync::mpsc;

pub trait ToMesh{
    fn to_mesh(&self,length:f32,radius:f32) -> Ref<MeshInstance>;
}
impl ToMesh for OpType{
    fn to_mesh(&self,length:f32,radius:f32) -> Ref<MeshInstance> {
        match self{
            OpType::empty => {
                let mesh = MeshInstance::new().into_shared();
                let mesh_obj = unsafe{mesh.assume_safe()};
                let box_mesh = CubeMesh::new().into_shared();
                let box_mesh = unsafe{box_mesh.assume_safe()};
                box_mesh.set_size(Vector3{x:length,y:radius,z:radius});
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
            OpType::globular_teleport => {
                let mesh_anchor = MeshInstance::new().into_shared();
                let mesh_0 = MeshInstance::new().into_shared();
                let mesh_1 = MeshInstance::new().into_shared();
                let mesh_2 = MeshInstance::new().into_shared();
                let mesh_3 = MeshInstance::new().into_shared();
                let mesh_obj_anchor = unsafe{mesh_anchor.assume_safe()};
                let mesh_obj_0 = unsafe{mesh_0.assume_safe()};
                let mesh_obj_1 = unsafe{mesh_1.assume_safe()};
                let mesh_obj_2 = unsafe{mesh_2.assume_safe()};
                let mesh_obj_3 = unsafe{mesh_3.assume_safe()};

                let anchor_mesh = SphereMesh::new().into_shared();
                let anchor_mesh = unsafe{anchor_mesh.assume_safe()};
                anchor_mesh.set_radius((radius/10.0).into());
                anchor_mesh.set_height((radius/10.0).into());

                let vertex_mesh = SphereMesh::new().into_shared();
                let vertex_mesh = unsafe{vertex_mesh.assume_safe()};
                vertex_mesh.set_radius((radius/10.0).into());
                vertex_mesh.set_height((radius/10.0).into());

                let anchor_material = SpatialMaterial::new();
                anchor_material.set_albedo(Color{r:255.0,g:0.0,b:0.0,a:1.0});
                anchor_mesh.set_material(anchor_material);
                let vertex_material = SpatialMaterial::new();
                vertex_material.set_albedo(Color{r:0.0,g:100.0,b:100.0,a:1.0});
                vertex_mesh.set_material(vertex_material);

                mesh_obj_anchor.set_mesh(anchor_mesh);
                mesh_obj_0.set_mesh(vertex_mesh);
                mesh_obj_1.set_mesh(vertex_mesh);
                mesh_obj_2.set_mesh(vertex_mesh);
                mesh_obj_3.set_mesh(vertex_mesh);
                let mut transform = mesh_obj_anchor.transform();
                transform.origin = Vector3{x:length/2.0,y:radius/2.0,z:radius/2.0};
                mesh_obj_0.set_transform(transform);
                transform.origin = Vector3{x:0.0,y:radius/2.0,z:radius/2.0};
                mesh_obj_1.set_transform(transform);
                transform.origin = Vector3{x:-1.0 * length/2.0,y:radius/2.0,z:-1.0 * radius/2.0};
                mesh_obj_2.set_transform(transform);
                transform.origin = Vector3{x:0.0,y:radius/2.0,z:-1.0 * radius/2.0};
                mesh_obj_3.set_transform(transform);

                mesh_obj_anchor.add_child(mesh_obj_0,true);
                mesh_obj_anchor.add_child(mesh_obj_1,true);
                mesh_obj_anchor.add_child(mesh_obj_2,true);
                mesh_obj_anchor.add_child(mesh_obj_3,true);
                
                mesh_anchor
            }
            
        }
    }
}
#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct FieldAbilityMesh{
    mesh:Ref<MeshInstance>,
    length:f32,
    radius:f32,
}
impl InstancedDefault<Spatial,OpType> for FieldAbilityMesh{
    fn make(args:&OpType) -> Self{
        FieldAbilityMesh{
            mesh:args.to_mesh(25.0,5.0),
            length:25.0,
            radius:5.0,
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
