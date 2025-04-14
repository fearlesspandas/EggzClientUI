

use gdnative::prelude::*;
use gdnative::api::*;
use gdnative::api::spatial_material::TextureParam;
use gdnative::export::StaticallyNamed;
use crate::traits::{Instanced};
use crate::collision_layer;
use crate::collision_box::{CollisionBox};
use std::collections::{HashSet,HashMap};

pub enum Assets{
    player,
    server_entity,
    block_terrain,
    health_star,
    spawn_frame,
    prowler_anchor,
    monk_garden,
    planet_a
}
const ASSETS:[Assets;8] = [
            Assets::player,
            Assets::server_entity,
            Assets::block_terrain,
            Assets::spawn_frame,
            Assets::health_star, 
            Assets::prowler_anchor,
            Assets::monk_garden,
            Assets::planet_a,
];
impl From<i64> for Assets{
    fn from(item:i64)->Self{
        match item{
            0 => Assets::player,
            1 => Assets::server_entity,
            6 => Assets::block_terrain,
            9 => Assets::spawn_frame,
            11 => Assets::health_star, 
            16 => Assets::prowler_anchor,
            24 => Assets::monk_garden,
            25 => Assets::planet_a,
            _ => {assert!(false,"Assets::No asset assigned for item");todo!()},
        }
    }
}

impl Into<i64> for Assets{
    fn into(self)->i64{
        match self{
             Assets::player => 0,
             Assets::server_entity => 1,
             Assets::block_terrain => 7,
             Assets::spawn_frame => 9,
             Assets::health_star => 11, 
             Assets::prowler_anchor => 16,
             Assets::monk_garden => 24,
             Assets::planet_a => 25,
        }
    }
}
impl Assets{
    pub fn to_mesh_resource_path(&self)->Option<&str>{
        match self{
             Assets::player => Some(""),
             Assets::server_entity => Some(""),
             Assets::block_terrain => Some("res://world/client/Dust.gd"),
             Assets::spawn_frame => Some(""),
             Assets::health_star => Some(""), 
             Assets::prowler_anchor => Some(""),
             Assets::monk_garden => Some(""),
             Assets::planet_a => Some(""),
        }
    }
    pub fn to_client_collider_resource_path(&self)->Option<&str>{
        match self{
             Assets::player => Some(""),
             Assets::server_entity => Some(""),
             Assets::block_terrain => Some(""),
             Assets::spawn_frame => Some(""),
             Assets::health_star => Some(""), 
             Assets::prowler_anchor => Some(""),
             Assets::monk_garden => Some(""),
             Assets::planet_a => Some(""),
        }
    }
    pub fn to_server_collider_resource_path(&self)->Option<&str>{
        match self{
             Assets::player => Some(""),
             Assets::server_entity => Some(""),
             Assets::block_terrain => Some(""),
             Assets::spawn_frame => Some(""),
             Assets::health_star => Some(""), 
             Assets::prowler_anchor => Some(""),
             Assets::monk_garden => Some(""),
             Assets::planet_a => Some(""),
        }
    }
    pub fn to_server_collider_resource(&self) -> Option<Ref<CollisionObject>>{
        match self{
             Assets::player => None,
             Assets::server_entity => None,
             Assets::block_terrain => {
                let collision_box = CollisionBox::make_instance().into_shared();
                let collision_box = unsafe{collision_box.assume_safe()};
                let _ = collision_box.map_mut(|obj,_| obj.set_radius(10.0));
                Some(collision_box.base().upcast::<CollisionObject>().claim())
             },
             Assets::spawn_frame => None,
             Assets::health_star => {
                 None
             }, 
             Assets::prowler_anchor => None,
             Assets::monk_garden => None,
             Assets::planet_a => None,
        }
    }
    const HEALTH_STAR_RADIUS:f32 = 10.0;
    pub fn to_mesh_resource(&self) -> Option<Ref<Mesh>>{
        let res = match self{
             Assets::player => None,
             Assets::server_entity => None,
             Assets::block_terrain => {
                 let mesh = CubeMesh::new().into_shared();
                 let mesh_obj = unsafe{mesh.assume_safe()};
                 mesh_obj.set_size(Vector3::new(10.0,10.0,10.0));
                 Some(mesh.upcast::<Mesh>())
             },
             Assets::spawn_frame => None,
             Assets::health_star => {
                 let radius = Self::HEALTH_STAR_RADIUS;
                 let mut vertices = PoolArray::<Vector3>::new();
                 vertices.push(Vector3::new(0.0,-radius/2.0,0.0));
                 vertices.push(Vector3::new(radius/2.0,radius,radius/2.0));
                 vertices.push(Vector3::new(0.0,-radius/2.0,radius));

                 vertices.push(Vector3::new(radius,-radius/2.0,0.0));
                 vertices.push(Vector3::new(radius/2.0,radius,radius/2.0));
                 vertices.push(Vector3::new(radius,-radius/2.0,radius));

                 vertices.push(Vector3::new(0.0,-radius/2.0,radius));
                 vertices.push(Vector3::new(radius/2.0,radius,radius/2.0));
                 vertices.push(Vector3::new(radius,-radius/2.0,0.0));

                 let mesh = ArrayMesh::new().into_shared();
                 let mesh = unsafe{mesh.assume_safe()};
                 let arrays = VariantArray::new();
                 arrays.resize(ArrayMesh::ARRAY_MAX as i32);
                 arrays.set(ArrayMesh::ARRAY_VERTEX as i32,vertices);
                 mesh.add_surface_from_arrays(Mesh::PRIMITIVE_TRIANGLES,arrays.into_shared(),VariantArray::new_shared(),2194432);
                 Some(mesh.claim().upcast::<Mesh>())
             }, 
             Assets::prowler_anchor => None,
             Assets::monk_garden => None,
             Assets::planet_a => None,
        };
        res.map(|mesh|{
            let mesh = unsafe{mesh.assume_safe()};
            for i in 0..mesh.get_surface_count(){
                let material = self.to_material_resource(i).expect("ChunkMeshErr:Material Not found for type");
                mesh.surface_set_material(i,material);
            }
            mesh.claim()
        })
    }
    pub fn to_material_resource(&self,surface_idx:i64) -> Option<Ref<Material>>{
        match self{
             Assets::player => None,
             Assets::server_entity => None,
             Assets::block_terrain => {
                 let material = SpatialMaterial::new().into_shared();
                 let material = unsafe{material.assume_safe()};
                 material.set_albedo(Color::from_rgba(0.0,10.0,256.0,1.0));
                 let path = "res://textures/vortex.png";
                 let texture = ResourceLoader::godot_singleton().load(path,"",false).expect("").cast::<StreamTexture>().expect("");
                 //let texture = StreamTexture::new().into_shared();
                 let texture = unsafe{texture.assume_safe()};
                 //let _ = texture.load();
                 texture.set_flags(7);
                 material.set_texture(TextureParam::ALBEDO.into(),texture);
                 Some(material.claim().upcast::<Material>())
             },
             Assets::spawn_frame => None,
             Assets::health_star => {
                 let material = SpatialMaterial::new().into_shared();
                 let material = unsafe{material.assume_safe()};
                 material.set_albedo(Color::from_rgba(0.0,256.0,10.0,1.0));
                 Some(material.claim().upcast::<Material>())
             }, 
             Assets::prowler_anchor => None,
             Assets::monk_garden => None,
             Assets::planet_a => None,
        }
    }
    pub fn to_point_mesh_resource(&self) -> Option<Ref<Mesh>>{
        let res = match self{
             Assets::player => None,
             Assets::server_entity => None,
             Assets::block_terrain => {
                 Some(PointMesh::new().into_shared().upcast::<Mesh>())
             },
             Assets::spawn_frame => None,
             Assets::health_star => {
                 Some(PointMesh::new().into_shared().upcast::<Mesh>())
             }, 
             Assets::prowler_anchor => None,
             Assets::monk_garden => None,
             Assets::planet_a => None,
        };
        res.map(|mesh|{
            let mesh = unsafe{mesh.assume_safe()};
            for i in 0..mesh.get_surface_count(){
                let material = self.to_point_material_resource().expect("ChunkMeshErr:Material Not found for type");
                mesh.surface_set_material(i,material);
            }
            mesh.claim()
        })
    }
    pub fn to_point_material_resource(&self) -> Option<Ref<Material>>{
        match self{
             Assets::player => None,
             Assets::server_entity => None,
             Assets::block_terrain => {
                 Some(point_material(2.0,Color::from_rgba(0.0,10.0,255.0,1.0)))
             },
             Assets::spawn_frame => None,
             Assets::health_star => {
                 Some(point_material(2.0,Color::from_rgba(0.0,255.0,10.0,1.0)))
             }, 
             Assets::prowler_anchor => None,
             Assets::monk_garden => None,
             Assets::planet_a => None,
        }
    }
    pub fn to_client_collider_resource(&self) -> Option<Ref<Resource>>{
        self.to_client_collider_resource_path().and_then(|path|{
            ResourceLoader::godot_singleton().load(path,"",false)
        })
    }
    //pub fn to_server_collider_resource(&self) -> Option<Ref<Resource>>{
    //    self.to_server_collider_resource_path().and_then(|path|{
    //        ResourceLoader::godot_singleton().load(path,"",false)
    //    })
    //}
}

fn point_material(size:f64,color:Color) -> Ref<Material>{
    let material = SpatialMaterial::new().into_shared();
    let material = unsafe{material.assume_safe()};
    let path = "res://textures/vortex.png";
    let texture = ResourceLoader::godot_singleton().load(path,"",false).expect("").cast::<StreamTexture>().expect("");
    //let texture = StreamTexture::new().into_shared();
    let texture = unsafe{texture.assume_safe()};
    //let _ = texture.load();
    texture.set_flags(7);
    material.set_texture(TextureParam::ALBEDO.into(),texture);
    material.set_flag(SpatialMaterial::FLAG_USE_POINT_SIZE,true);
    material.set_point_size(size);
    material.set_billboard_mode(1);
    material.set_albedo(color);
    material.claim().upcast::<Material>()
}
