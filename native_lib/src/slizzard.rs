
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Instanced,InstancedDefault};
use crate::collision_layer;



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
            color:Color{r:0.0,g:70.0,b:100.0,a:1.0},
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

        let material = SpatialMaterial::new().into_shared();
        let material = unsafe{material.assume_safe()};
        material.set_albedo(self.color);
        sphere_mesh.set_material(material);
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
#[inherit(Area)]
#[register_with(Self::register_signals)]
pub struct Slizzard{
    length:f32,
    height:f64,
    body_pieces:Vec<Instance<BodyPiece>>,
}
impl Instanced<Area> for Slizzard{
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
    fn register_signals(builder:&ClassBuilder<Self>){
        builder
            .signal("damage")
            .with_param("id",VariantType::GodotString)
            .with_param_default("amount",Variant::new(0.0))
            .done();
    }
    #[method]
    fn _ready(&self,#[base] owner:TRef<Area>){
        let collider = CollisionShape::new().into_shared();
        let collider = unsafe{collider.assume_safe()};
        let shape = BoxShape::new().into_shared();
        let shape = unsafe{shape.assume_safe()};
        shape.set_extents(Vector3{x:self.length,y:self.height as f32,z:self.length});
        collider.set_shape(shape);
        owner.add_child(collider,true);
        let _ = owner.connect("body_entered",owner,"attack_entity",VariantArray::new_shared(),0);
    }

    #[method]
    fn attack_entity(&self,#[base] owner:TRef<Area>,body:Ref<Node,Shared>){
        let body = unsafe{body.assume_safe()};
        let position = body.cast::<KinematicBody>().map(|kinematic_body| kinematic_body.global_translation());
        let _ = position.map(|pos| owner.look_at(pos,Vector3{x:0.0,y:1.0,z:0.0}));
    }

    #[method]
    pub fn add_body_piece(&mut self,#[base] owner:TRef<Area>){
        let body_piece = BodyPiece::make_instance().into_shared();
        owner.add_child(body_piece.clone(),true);
        self.body_pieces.push(body_piece.clone());
        let body_piece = unsafe{body_piece.assume_safe()};
        let _ = body_piece.map(|obj,spatial| obj.set_offset(spatial,3.14 * 0.25 * (self.body_pieces.len() as f32)) );
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Area>,delta:f64){
        let num_pieces = self.body_pieces.len() as f32;
        let mut idx = 0.0;
        let distance = self.length/num_pieces;
        let owner_transform = owner.transform();
        for piece in &self.body_pieces{
            let piece = unsafe{piece.assume_safe()};
            let _ = piece.map(|obj,mesh| {
                let mut transform = mesh.transform();
                let origin = owner_transform.origin - Vector3{x:0.0,y:0.0,z:self.length/2.0};
                transform.origin = origin + Vector3{x:0.0,y:0.0,z:distance * idx};
                mesh.set_transform(transform);
                idx += 1.0;
            });
        }
    }
    #[method]
    fn set_rotation(&self,#[base] owner:TRef<Area>,rotation:Vector3){
        owner.set_rotation(rotation);
    }
}

#[derive(NativeClass)]
#[inherit(Area)]
#[register_with(Self::register_signals)]
pub struct SlizzardServer{
    length:f32,
    height:f64,
}
impl Instanced<Area> for SlizzardServer{
    fn make() -> Self{
        SlizzardServer{
            length:30.0,
            height:100.0,
        }
    }
}
#[methods]
impl SlizzardServer{
    fn register_signals(builder:&ClassBuilder<Self>){
        builder
            .signal("damage")
            .with_param("id",VariantType::GodotString)
            .with_param_default("amount",Variant::new(0.0))
            .done();
    }
    #[method]
    fn _ready(&self,#[base] owner:TRef<Area>){
        let collider = CollisionShape::new().into_shared();
        let collider = unsafe{collider.assume_safe()};
        let shape = BoxShape::new().into_shared();
        let shape = unsafe{shape.assume_safe()};
        shape.set_extents(Vector3{x:self.length,y:self.height as f32,z:self.length});
        collider.set_shape(shape);
        owner.add_child(collider,true);
        let _ = owner.connect("body_entered",owner,"attack_entity",VariantArray::new_shared(),0);
        owner.set_collision_layer_bit(collision_layer::SERVER_TERRAIN_COLLISION_LAYER.into(),false);
        owner.set_collision_mask_bit(collision_layer::SERVER_TERRAIN_COLLISION_LAYER.into(),false);
        owner.set_collision_layer_bit(collision_layer::CLIENT_NPC_COLLISION_LAYER.into(),false);
        owner.set_collision_mask_bit(collision_layer::CLIENT_NPC_COLLISION_LAYER.into(),true);
    }

    #[method]
    fn attack_entity(&self,#[base] owner:TRef<Area>,body:Ref<Node,Shared>){
        let body = unsafe{body.assume_safe()};
        let entity_id = body.get("id");
        if entity_id.is_nil(){
            assert!(false,"Body id is null");
        }else{
            let _ = entity_id.try_to::<String>()
                .map(|id| owner.emit_signal("damage",&[Variant::new(id),Variant::new(100.0)]))
                .map_err(|err| assert!(false,"Incorrect type for id"));
        }
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Area>,delta:f64){
    }
}
