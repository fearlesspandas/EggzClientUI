
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{CreateSignal,Instanced,InstancedDefault,Defaulted};
use tokio::sync::mpsc;



#[derive(NativeClass)]
#[inherit(Spatial)]
//#[register_with(Self::register_signals)]
pub struct BodyPiece{
    radius:f32,
    color:Color,
    mesh:Ref<MeshInstance>,
}
impl Instanced<Spatial> for BodyPiece{
    fn make() -> Self{
        BodyPiece{
            radius:1.0,
            color:Color{r:0.0,g:0.0,b:0.0,a:1.0},
            mesh:MeshInstance::new().into_shared(),
        }
    }
}
#[methods]
impl BodyPiece{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Spatial>){
        let mesh = unsafe{self.mesh.assume_safe()};
        let sphere_mesh = SphereMesh::new().into_shared();
        let sphere_mesh = unsafe{sphere_mesh.assume_safe()};

        mesh.set_mesh(sphere_mesh);

        mesh.translate(Vector3{x:0.0,y:self.radius,z:0.0});

        owner.add_child(mesh,true);

    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Spatial>,delta:f64){
        owner.rotate_z(delta);
    }

    #[method]
    fn set_offset(&self,#[base] owner:TRef<Spatial>, offset:f32){
        owner.set_rotation(Vector3{x:0.0,y:0.0,z:offset});
    }
}

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct Slizzard{
    length:f32,
    height:f64,
    body_pieces:Vec<Instance<BodyPiece>>
}
impl Instanced<Spatial> for Slizzard{
    fn make() -> Self{
        Slizzard{
            length:30.0,
            height:100.0,
            body_pieces:Vec::new()
        }
    }
}
#[methods]
impl Slizzard{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Spatial>){ }

    #[method]
    fn add_body_piece(&mut self,#[base] owner:TRef<Spatial>){
        let body_piece = BodyPiece::make_instance().into_shared();
        owner.add_child(body_piece.clone(),true);
        self.body_pieces.push(body_piece.clone());
        let body_piece = unsafe{body_piece.assume_safe()};
        body_piece.map(|obj,spatial| obj.set_offset(spatial,3.14 * 0.25 * (self.body_pieces.len() as f32)) );
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Spatial>,delta:f64){
        let num_pieces = self.body_pieces.len() as f32;
        let mut idx = 0.0;
        let distance = self.length/num_pieces;
        let owner_transform = owner.transform();
        for piece in &self.body_pieces{
            let piece = unsafe{piece.assume_safe()};
            piece.map(|obj,mesh| {
                let mut transform = mesh.transform();
                let origin = owner_transform.origin - Vector3{x:0.0,y:0.0,z:self.length/2.0};
                transform.origin = origin + Vector3{x:0.0,y:0.0,z:distance * idx};
                mesh.set_transform(transform);
                idx += 1.0;
            });
        }

    }
    #[method]
    fn set_rotation(&self,#[base] owner:TRef<Spatial>,rotation:Vector3){
        owner.set_rotation(rotation);
    }
}

