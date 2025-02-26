
use gdnative::prelude::*;
use tokio::sync::mpsc;


trait ClientEntity{
    fn body(&self) -> &KinematicBody;
    fn default_physics_process(delta:f64){
    }
}
