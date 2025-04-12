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
    terrain_ids:HashSet<TerrainId>,
    mesh:Option<Ref<Mesh>>,
}
const MINIMUM_TERRAIN_IN_CHUNKS:usize = 512;
impl Instanced<MultiMesh> for NavigationPoints{
    fn make() -> Self{
        NavigationPoints{
            terrain_type:None,
            locations:Vec::with_capacity(MINIMUM_TERRAIN_IN_CHUNKS),
            terrain_ids:HashSet::with_capacity(256),
            mesh:None,
        }
    }
}
#[methods]
impl NavigationPoints{
    #[method]
    fn add_terrain(&mut self,terrain_id:TerrainId,location:Vector3){
        self.locations.push(location);
        self.terrain_ids.insert(terrain_id);
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
type TerrainId = String;
#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct Waypoints{
    //NavigationPoints handles coords as localized
    mesh_map:HashMap<ChunkId,HashMap<TerrainKey,Instance<NavigationPoints>>>,
    //in global coords
    terrain_id_to_chunk:HashMap<TerrainId,ChunkId>,
    terrain_id_to_terrain_key:HashMap<TerrainId,TerrainKey>,
    terrain_id_to_location:HashMap<TerrainId,Vector3>,
    chunk_id_to_terrain:HashMap<ChunkId,Vec<TerrainId>>,
}
impl Instanced<Spatial> for Waypoints{
    fn make() -> Self{
        Waypoints{
            mesh_map:HashMap::with_capacity(128),
            terrain_id_to_chunk:HashMap::with_capacity(2048 * 8),
            terrain_id_to_terrain_key:HashMap::with_capacity(2048 * 8),
            terrain_id_to_location:HashMap::with_capacity(2048 * 8),
            chunk_id_to_terrain:HashMap::with_capacity(2048),
        }
    }
}
const MAX_DISTANCE:f32 = 1024.0;
#[methods]
impl Waypoints{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Spatial>){ }

    #[method]
    fn adjust_to_player_location(&self,loc:Vector3){
        for (chunk_id,chunk_mesh) in &self.mesh_map{
            for mesh in chunk_mesh.values(){
                let mesh = unsafe{mesh.assume_safe()};
            }
        }

    }

    #[method]
    fn deep_count(&self) -> usize{
        self.mesh_map.values().into_iter().map(|chunk_map|{
            chunk_map.values().into_iter().map(|mesh|{
                let mesh = unsafe{mesh.assume_safe()};
                mesh.map(|obj,_| obj.locations.len())
                    .expect("WaypointsErr:Could not get locations size in deep_count")
                }).fold(0,|acc_inn,curr_inn| acc_inn + curr_inn)
        }).fold(0,|acc,curr|  acc + curr )
    }

    #[method]
    fn add_region_point(&mut self,#[base] owner:TRef<Spatial>,region_id:ChunkId,terrain_id:TerrainId,terrain_type:TerrainKey,location:Vector3){
        if self.terrain_id_to_chunk.contains_key(&terrain_id){return ;}
        self.terrain_id_to_chunk.insert(terrain_id.clone(),region_id.clone());
        self.terrain_id_to_terrain_key.insert(terrain_id.clone(),terrain_type.clone());
        self.terrain_id_to_location.insert(terrain_id.clone(),location.clone());

        let location = owner.to_local(location);
        let is_too_long:f32 = (location.length() > MAX_DISTANCE).into();
        let is_not_too_long:f32 = (location.length() <= MAX_DISTANCE).into();
        let distance = MAX_DISTANCE * is_too_long + location.length() * is_not_too_long;
        let location = location.normalized() * distance;

        let mut chunk_map = self.mesh_map.get(&region_id).map_or_else(||{
            HashMap::with_capacity(64)
        },|x| x.clone());//this clone should be fine as we only have a few terrain types we're
                         //concerned with
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
        let _ = mesh_for_type.map_mut(|obj,_|obj.add_terrain(terrain_id,location));
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
        let count = self.deep_count();
        godot_print!("{}",format!("Baking NavMesh with count :{count:?}"));
    }
}
