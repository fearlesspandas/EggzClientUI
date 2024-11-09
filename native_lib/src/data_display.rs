use gdnative::prelude::*;
use gdnative::api::*;
use std::collections::HashMap;
use std::fmt;
use serde::{Deserialize,Serialize};
use crate::traits::{GetAll};
type DataLabel = String;

#[derive(Deserialize,Serialize,Debug)]
pub enum DataType{
    socket_mode,
}
impl GetAll for DataType{
    fn get_all() -> Vec<Self> where Self:Sized{
        let mut v = Vec::new();
        v.push(DataType::socket_mode);
        v
    }
}
impl fmt::Display for DataType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            DataType::socket_mode => {
                write!(f,"socket_mode")
            }
        }
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
pub struct DataDisplay{
    tag_to_data: HashMap<DataLabel,String>,
    text_display: Ref<RichTextLabel>,
} 
#[methods]
impl DataDisplay{
    pub fn new(base: &Control) -> Self{
        DataDisplay{
            tag_to_data: HashMap::new(),
            text_display: RichTextLabel::new().into_shared(),
        }
    }

    pub fn make() -> Instance<DataDisplay,Unique>{
        Instance::emplace(
            DataDisplay{
                tag_to_data:HashMap::new(),
                text_display: RichTextLabel::new().into_shared(),
            })
    }

    #[method]
    fn _ready(&self,#[base] owner:&Control){
        let text_display = unsafe{self.text_display.assume_safe()};
        owner.add_child(text_display,true);
    }
    #[method]
    fn _process(&self,#[base] owner:&Control,delta:f64){
        let text_display = unsafe{self.text_display.assume_safe()};
        text_display.set_size(owner.size(),true);
        let mut text_data = "".to_string();
        for (tag,data) in &self.tag_to_data{
            text_data.push_str(&tag);
            text_data.push_str(" ");
            text_data.push_str(&data);
            text_data.push_str("\n");
        }
        text_display.set_text(text_data);
    }


    pub fn add_data(&mut self,tag:DataLabel,data:String){
        self.tag_to_data.insert(tag,data);
    }


}
