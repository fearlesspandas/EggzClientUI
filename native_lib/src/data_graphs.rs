use gdnative::prelude::*;
use gdnative::api::*;
use std::collections::HashMap;
use std::fmt;
use std::cmp::max;
use serde::{Deserialize,Serialize};
use crate::traits::{GetAll};
use tokio::{
    runtime::Runtime,
    io::{AsyncBufRead, AsyncBufReadExt, AsyncWriteExt, BufReader,BufWriter,ReadHalf,WriteHalf},
    sync::mpsc,
    select
};
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;
//type DataLabel = String;

enum GraphActions{
    UpdateColumn(String,DataValue)
}
type DataValue = f32;
#[derive(NativeClass)]
#[inherit(Control)]
pub struct BarGraph{
    tag_to_data: HashMap<String,DataValue>,
    columns: HashMap<String,Ref<ColorRect>>,
    current_max: DataValue,
    current_min: DataValue,
    actions_tx: Sender<GraphActions>,
    actions_rx: Receiver<GraphActions>,
} 
#[methods]
impl BarGraph{
    pub fn new(base: &Control) -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<GraphActions>();
        BarGraph{
            tag_to_data: HashMap::new(),
            columns: HashMap::new(),
            current_max: 0.0,
            current_min: 0.0,
            actions_tx: tx,
            actions_rx: rx,
        }
    }

    pub fn make() -> Instance<BarGraph,Unique>{
        let (tx,rx) = mpsc::unbounded_channel::<GraphActions>();
        Instance::emplace(
            BarGraph{
                tag_to_data:HashMap::new(),
                columns: HashMap::new(),
                current_max: 0.0,
                current_min: 0.0,
                actions_tx: tx,
                actions_rx: rx,
            })
    }

    #[method]
    fn _ready(&self,#[base] owner:&Control){
       //self.actions_tx.send(GraphActions::UpdateColumn("test".to_string(),100.0));
    }
    #[method]
    fn _process(&mut self,#[base] owner:&Control,delta:f64){
       match self.actions_rx.try_recv(){
           Ok(GraphActions::UpdateColumn(tag,value)) => {
               self.tag_to_data.insert(tag.clone(),value);
               self.current_max += ((value - self.current_max) * ((value > self.current_max) as i32 as f32));
               self.current_min += ((value + self.current_min) * ((value < self.current_min) as i32 as f32));
               if !self.columns.contains_key(&tag){
                   let color_rect: Ref<ColorRect> = ColorRect::new().into_shared();
                   let tag_label : Ref<Label> = Label::new().into_shared();
                   let tag_label = unsafe{tag_label.assume_safe()};
                   tag_label.set_text(&tag);
                   self.columns.insert(tag,color_rect);
                   let color_rect = unsafe{color_rect.assume_safe()};
                   color_rect.set_frame_color(Color{r:250.0,g:0.0,b:0.0,a:1.0});
                  // color_rect.add_child(tag_label,true);
                   
                   owner.add_child(color_rect,true);
               }
               
           }
           Err(_) => {} 
       } 
       let num_columns = self.columns.len() as i32;
       let owner_size = owner.size();
       let column_width = owner_size.x/(num_columns + ((num_columns == 0) as i32 )) as f32;
       let max_diff = self.current_max - self.current_min;
       let mut idx: f32 = 0.0;
       let mut loc:Vector2 = Vector2{x:0.0,y:0.0};
       let mut column_size = Vector2{x:column_width,y:0.0};
       for (tag,value) in &self.tag_to_data{
           let col = self.columns.get(tag);
           col.map(|column|{
               let column = unsafe{column.assume_safe()};
               column_size.y = (value/(max_diff + ((max_diff == 0.0) as i32 as f32 * value))) * owner_size.y;
               column.set_size(column_size,false);
               loc.x = column_width * idx;
               loc.y = owner_size.y - column_size.y;
               column.set_position(loc,false);
               //godot_print!("{}",format!("tag {tag:?} column size {column_size:?} loc {loc:?} max diff {max_diff:?}"));
           });
           idx+=1.0;
       }
    }

    pub fn add_data(&mut self,tag:String,data:DataValue){
        self.actions_tx.send(GraphActions::UpdateColumn(tag,data));
        
    }

}


