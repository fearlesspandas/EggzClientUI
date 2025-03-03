

use gdnative::prelude::*;
use gdnative::api::*;
use crate::field_abilities::{AbilityType};
use crate::collision_layer;

pub trait ToCollider{
    fn to_collider(&self,extents:Vector3) -> Option<Ref<Area>>;
}
impl ToCollider for AbilityType{
    fn to_collider(&self,extents:Vector3) -> Option<Ref<Area>> {
        match self{
            AbilityType::empty => {
                None
            }
            AbilityType::occupied => {
                None
            }
            AbilityType::smack => {
                None
            }
            AbilityType::globular_teleport => {
                None
            }
            AbilityType::slizzard => {
                let area_ref = Area::new().into_shared();
                let area = unsafe{area_ref.assume_safe()};
                let collider_ref = CollisionShape::new().into_shared();
                let collider = unsafe{collider_ref.assume_safe()};
                let shape = BoxShape::new().into_shared();
                let shape = unsafe{shape.assume_safe()};
                shape.set_extents(extents);
                collider.set_shape(shape);
                area.add_child(collider,true);
                area.set_collision_layer_bit(collision_layer::SERVER_TERRAIN_COLLISION_LAYER.into(),false);
                area.set_collision_mask_bit(collision_layer::SERVER_TERRAIN_COLLISION_LAYER.into(),false);
                area.set_collision_layer_bit(collision_layer::SERVER_NPC_COLLISION_LAYER.into(),false);
                area.set_collision_mask_bit(collision_layer::SERVER_NPC_COLLISION_LAYER.into(),true);
                Some(area_ref)
            }
            
        }
    }
}
