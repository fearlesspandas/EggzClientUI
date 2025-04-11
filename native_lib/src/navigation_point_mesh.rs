use gdnative::prelude::*;
use gdnative::api::*;
use gdnative::export::StaticallyNamed;
use crate::traits::{Instanced};
use crate::collision_layer;
use crate::assets::{Assets};
use std::collections::{HashSet,HashMap};

type TerrainKey = i64;
type ChunkId = String;
type Octant = (i32,i32,i32);
const OCTANTS:[Octant;8] = [
    (1, 1, 1),
    (1, 1, -1),
    (1, -1, 1),
    (-1, 1, 1),
    (1, -1, -1),
    (-1, -1, 1),
    (-1, 1, -1),
    (-1, -1, -1)
];

#[derive(NativeClass)]
#[inherit(MultiMesh)]
pub struct NavigationPoints{
    terrain_type:Option<TerrainKey>,
    locations:Vec<Vector3>,
    mesh:Option<Ref<Mesh>>,
}
const MINIMUM_TERRAIN_IN_CHUNKS:usize = 512;
impl Instanced<MultiMesh> for NavigationPoints{
    fn make() -> Self{
        NavigationPoints{
            terrain_type:None,
            locations:Vec::with_capacity(MINIMUM_TERRAIN_IN_CHUNKS),
            mesh:None,
        }
    }
}
#[methods]
impl NavigationPoints{
    #[method]
    fn add_terrain(&mut self,location:Vector3){
        self.locations.push(location);
    }
    #[method]
    fn set_terrain_type(&mut self,terrain_type:TerrainKey){
        self.terrain_type = Some(terrain_type.clone());
        let terrain_type = Assets::from(terrain_type);
        let mesh = terrain_type.to_point_mesh_resource().expect("NavigationMeshErr:Resource not found for type");
        self.mesh = Some(mesh);
    }
    #[method]
    fn move_mesh_locations(&self,#[base] owner:TRef<MultiMesh>){
        let terrain_locations = &self.locations;
        for i in 0..owner.instance_count(){
            let idx:usize = i.try_into().expect("NavigationMeshErr:Failed to Index locations array");
            let loc = terrain_locations[idx];
            let (x,y,z) = (loc.x,loc.y,loc.z);
            let mut transform = owner.get_instance_transform(i);
            transform.origin = Vector3::new(x,y,z);
            owner.set_instance_transform(i,transform);
        }
    }
    #[method]
    fn bake_at(&self,#[base] owner:TRef<MultiMesh>){
        let mesh = self.mesh.clone().expect("NavigationMeshErr:bake called without mesh set; set terrain_type using set_terrain_type");
        owner.set_instance_count(0);
        owner.set_transform_format(MultiMesh::TRANSFORM_3D);
        owner.set_instance_count(self.locations.len().try_into().expect("NavigationMeshErr:Inappropriate length for terrain"));
        self.move_mesh_locations(owner);
        owner.set_mesh(mesh);
    }
}
#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct Waypoints{
    mesh_map:HashMap<ChunkId,HashMap<TerrainKey,Instance<NavigationPoints>>>,
}
impl Instanced<Spatial> for Waypoints{
    fn make() -> Self{
        Waypoints{
            mesh_map:HashMap::with_capacity(128),
        }
    }
}
const MAX_DISTANCE:f32 = 1000.0;
#[methods]
impl Waypoints{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Spatial>){ }

    #[method]
    fn add_region_point(&mut self,#[base] owner:TRef<Spatial>,region_id:ChunkId,terrain_type:TerrainKey,location:Vector3){
        let location = owner.to_local(location);
        let is_too_long:f32 = (location.length() > MAX_DISTANCE).into();
        let is_not_too_long:f32 = (location.length() <= MAX_DISTANCE).into();
        let distance = MAX_DISTANCE * is_too_long + location.length() * is_not_too_long;
        let location = location.normalized() * distance;

        let mut chunk_map = self.mesh_map.get(&region_id).map_or_else(||{
            HashMap::with_capacity(64)
        },|x| x.clone());
        let mesh_for_type = chunk_map.get(&terrain_type).map_or_else(||{
            let nav_points = NavigationPoints::make_instance().into_shared();
            let nav_points = unsafe{nav_points.assume_safe()};
            let _ = nav_points.map_mut(|obj,_| obj.set_terrain_type(terrain_type));
            let mesh_instance = MultiMeshInstance::new().into_shared();
            let mesh_instance = unsafe{mesh_instance.assume_safe()};
            mesh_instance.set_multimesh(nav_points.clone());
            owner.add_child(mesh_instance,true);
            nav_points.claim()
        },|x| x.clone());
        let mesh_for_type = unsafe{mesh_for_type.assume_safe()};
        let _ = mesh_for_type.map_mut(|obj,_|obj.add_terrain(location));
        chunk_map.insert(terrain_type,mesh_for_type.claim());
        self.mesh_map.insert(region_id,chunk_map);
    }
    #[method]
    fn bake(&self,region_id:ChunkId){
        let chunk_map = self.mesh_map.get(&region_id).expect("WaypointsErr:Did not find map for region_id during bake call");
        for mesh in chunk_map.values(){
            let mesh = unsafe{mesh.assume_safe()};
            let _ = mesh.map(|obj,msh| obj.bake_at(msh));
        }
    }
}
