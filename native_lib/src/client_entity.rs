
use gdnative::prelude::*;



trait ClientEntity{
    fn body(&self) -> &KinematicBody;
    fn default_physics_process(_delta:f64){
    }
}
