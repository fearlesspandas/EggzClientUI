use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Instanced};
use crate::collision_layer;
use crate::field::{ZONE_WIDTH,ZONE_HEIGHT};

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct AntiGravityTank{
    mesh_instance:Ref<MeshInstance>,
    outer_cylinder:Ref<CylinderMesh>,
    outer_material:Ref<SpatialMaterial>,
    inner_cylinder:Ref<CylinderMesh>,
    inner_material:Ref<SpatialMaterial>,
}
impl Instanced<Spatial> for AntiGravityTank{
    fn make() -> Self{
        AntiGravityTank{
            mesh_instance:MeshInstance::new().into_shared(),
            outer_cylinder:CylinderMesh::new().into_shared(),
            outer_material:SpatialMaterial::new().into_shared(),
            inner_cylinder:CylinderMesh::new().into_shared(),
            inner_material:SpatialMaterial::new().into_shared(),
        }
    }
}
#[methods]
impl AntiGravityTank{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Spatial>){
        let mesh_instance = unsafe{self.mesh_instance.assume_safe()};
        let cylinder_mesh = unsafe{self.outer_cylinder.assume_safe()};
        let outer_material      = unsafe{self.outer_material.assume_safe()};
        let inner_material      = unsafe{self.inner_material.assume_safe()};
        outer_material.set_albedo(Color::from_rgba(100.0,0.0,100.0,1.0));
        inner_material.set_albedo(Color::from_rgba(100.0,0.0,100.0,1.0));
        cylinder_mesh.set_material(outer_material);
        cylinder_mesh.set_material(inner_material);
        mesh_instance.set_mesh(cylinder_mesh);
        owner.add_child(mesh_instance,true);
    }
    #[method]
    pub fn set_top_radius(&self,radius:f32){
        let cylinder_mesh = unsafe{self.outer_cylinder.assume_safe()};
        cylinder_mesh.set_top_radius(radius.into());
    }
    #[method]
    pub fn set_bottom_radius(&self,radius:f32){
        let cylinder_mesh = unsafe{self.outer_cylinder.assume_safe()};
        cylinder_mesh.set_bottom_radius(radius.into());
    }
    #[method]
    pub fn set_height(&self,value:f32){
        let cylinder_mesh = unsafe{self.outer_cylinder.assume_safe()};
        cylinder_mesh.set_height(value.into());
    }
}


