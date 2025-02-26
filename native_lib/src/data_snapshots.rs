use gdnative::prelude::*;
use gdnative::api::*;
use std::collections::HashMap;
use serde::{Deserialize,Serialize};

use crate::traits::{Instanced};

#[derive(Deserialize,Serialize)]
pub struct BarGraphSnapshot{
    pub path:String,
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
    pub fn save_snapshot(data:BarGraphSnapshot) -> Result<(),&'static str>{
        let name = &data.name;
        let path = &data.path;
        let file_name = format!("{path:?}/{name:?}").replace("\"","");
        let str = serde_json::to_string(&data).unwrap();
        let directory = Directory::new();
        if !directory.dir_exists(&data.path){
            godot_print!("{}",&data.path);
            let res = directory.make_dir(&data.path).map_err(|err| godot_print!("{}",err));
            if !res.is_ok(){return Err("Could not create directory, see logs for details")}
        }
        let file = gdnative::api::File::new().into_shared();
        let file = unsafe{file.assume_safe()};
        let _ = file.open(GodotString::from_str(&file_name),gdnative::api::File::WRITE_READ)
            .map_err(|err| {godot_print!("{}",err);"Could not open file path, see logs for details"})
            .map(|_| file.store_line(GodotString::from_str(str)));
        file.close();
        Ok(())
    }
    pub fn load(location:String) -> Result<BarGraphSnapshot,&'static str>{
        let file = gdnative::api::File::new().into_shared();
        let file = unsafe{file.assume_safe()};
        let _ = file.open(GodotString::from_str(location.replace("\"","")),gdnative::api::File::READ);
        serde_json::from_str::<BarGraphSnapshot>(&file.get_as_text(false).to_string())
            .map_err(|err| {godot_print!("{}",err);"could not map bar graph snapshot"})
    } 

    pub fn get_available_snapshots(path:String) -> Vec<String>{
        let directory = Directory::new();
        let _ = directory.open(path);
        let _ = directory.list_dir_begin(true,false);
        let mut res = Vec::new();
        while let filename = directory.get_next(){
            if filename == "".into(){break}
            res.push(filename.to_string());
        }
        directory.list_dir_end();
        res
    }
}
