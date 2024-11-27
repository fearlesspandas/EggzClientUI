use gdnative::prelude::*;
use gdnative::api::*;
use std::collections::HashMap;
use serde_json::{Result as JResult, Value};
use serde::{Deserialize,Serialize};

use crate::traits::{Instanced};

#[derive(Deserialize,Serialize)]
pub struct BarGraphSnapshot{
    pub name:String,
    pub data:HashMap<String,f32>,
}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct DataSnapshots{
}
impl Instanced<Node> for DataSnapshots{
    fn make() -> Self{
        DataSnapshots{}
    }
}
#[methods]
impl DataSnapshots{
    pub fn save_snapshot(&self,data:BarGraphSnapshot){
        let file = gdnative::api::File::new().into_shared();
        let file = unsafe{file.assume_safe()};
        file.open(GodotString::from_str(&data.name),gdnative::api::File::WRITE_READ).map_err(|err| godot_print!("{}",err));
        let str = serde_json::to_string(&data).unwrap();
        file.store_line(GodotString::from_str(str));
        file.close();
    }
    pub fn load(&self,name:String) -> Result<BarGraphSnapshot,&'static str>{
        let file = gdnative::api::File::new().into_shared();
        let file = unsafe{file.assume_safe()};
        file.open(GodotString::from_str(name),gdnative::api::File::WRITE);
        serde_json::from_str::<BarGraphSnapshot>(&file.get_as_text(false).to_string())
            .map_err(|err| "could not map bar graph snapshot")
    } 
}
