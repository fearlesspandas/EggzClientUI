use gdnative::prelude::*;
use gdnative::api::*;
use std::collections::HashMap;
use std::fmt;
use std::cmp::max;
use std::sync::{Arc,Mutex,atomic::{AtomicU32,AtomicU64,Ordering}};
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

enum DataActions{
    UpdateColumn(String,DataValue)
}
enum GraphActions{
    CreateColumn(String),
}
type DataValue = f32;
#[derive(NativeClass)]
#[inherit(Control)]
pub struct BarGraph{
    tag_to_data: Arc<Mutex<HashMap<String,DataValue>>>,
    columns: HashMap<String,(Ref<ColorRect>,Ref<Label>)>,
    current_max: Arc<AtomicU32>,
    current_min: Arc<AtomicU32>,
    actions_tx: Sender<DataActions>,
    graph_actions_tx: Sender<GraphActions>,
    graph_actions_rx: Receiver<GraphActions>,
    runtime:Runtime,
} 
#[methods]
impl BarGraph{
    pub fn new(base: &Control) -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<DataActions>();
        let (gtx,grx) = mpsc::unbounded_channel::<GraphActions>();
        BarGraph{
            tag_to_data: todo!(),
            columns: todo!(),
            current_max: todo!(),
            current_min: todo!(),
            actions_tx: tx,
            graph_actions_tx: gtx,
            graph_actions_rx: grx,
            runtime:todo!(),
        }
    }

    pub fn make() -> Instance<BarGraph,Unique>{
        let (tx,mut rx) = mpsc::unbounded_channel::<DataActions>();
        let (gtx,grx) = mpsc::unbounded_channel::<GraphActions>();
        let c_gtx = gtx.clone();
        let tag_to_data = Arc::new(Mutex::new(HashMap::new()));
        let c_data = tag_to_data.clone();
        let max_at = Arc::new(AtomicU32::new(0));
        let min_at = Arc::new(AtomicU32::new(0));
        let c_max = max_at.clone();
        let c_min = min_at.clone();
        let columns = HashMap::new();
        let rt = Runtime::new().unwrap();
        rt.spawn(async move{
            let tag_to_data = tag_to_data.clone();
            while let Some(action) = rx.recv().await{
                match action{
                    DataActions::UpdateColumn(tag,value) => {
                       let mut tag_to_data = tag_to_data.lock().unwrap();
                       if !tag_to_data.contains_key(&tag){
                           gtx.send(GraphActions::CreateColumn(tag.clone()));
                       }
                       tag_to_data.insert(tag.clone(),value);
                       let current_max = f32::from_bits(max_at.load(Ordering::Relaxed));
                       let current_min = f32::from_bits(min_at.load(Ordering::Relaxed));
                       if current_max < value{
                           max_at.store(value.to_bits(),Ordering::Relaxed);
                       }
                       if current_min > value{
                           min_at.store(value.to_bits(),Ordering::Relaxed);
                       }
                    }
                }
            }
        });
        Instance::emplace(
            BarGraph{
                tag_to_data: c_data,
                columns: columns,
                current_max: c_max,
                current_min: c_min,
                actions_tx: tx,
                graph_actions_tx: c_gtx,
                graph_actions_rx: grx,
                runtime:rt,
            })
    }

    #[method]
    fn _ready(&mut self,#[base] owner:&Control){
    }
    
    #[method]
    fn _process(&mut self,#[base] owner:&Control,delta:f64){
       let mut columns = &mut self.columns;
       let tag_to_data = self.tag_to_data.lock().unwrap();
       match self.graph_actions_rx.try_recv(){
           Ok(GraphActions::CreateColumn(tag)) => {
               let color_rect: Ref<ColorRect> = ColorRect::new().into_shared();
               let tag_label : Ref<Label> = Label::new().into_shared();
               columns.insert(tag.clone(),(color_rect,tag_label));
               let tag_label = unsafe{tag_label.assume_safe()};
               tag_label.set_text(&tag);
               let color_rect = unsafe{color_rect.assume_safe()};
               color_rect.set_frame_color(Color{r:250.0,g:0.0,b:0.0,a:1.0});
               owner.add_child(color_rect,true);
               owner.add_child(tag_label,true);
               
           }
           Err(_) => {} 
       } 
       let num_columns = columns.len() as i32;
       let owner_size = owner.size();
       let column_width = owner_size.x/(num_columns + ((num_columns == 0) as i32 )) as f32;
       let current_max = f32::from_bits(self.current_max.load(Ordering::Relaxed));
       let current_min = f32::from_bits(self.current_min.load(Ordering::Relaxed));
       let max_diff = current_max - current_min;
       let mut idx: f32 = 0.0;
       let mut loc:Vector2 = Vector2{x:0.0,y:0.0};
       let mut column_size = Vector2{x:column_width,y:0.0};
       let mut label_size = Vector2{x:column_width, y: 10.0};
       for (tag,value) in &*tag_to_data{
           let col = columns.get(tag);
           col.map(|column|{
               let (column,label) = column;
               let column = unsafe{column.assume_safe()};
               let label = unsafe{label.assume_safe()};
               column_size.y = (value/(max_diff + ((max_diff == 0.0) as i32 as f32 * value))) * owner_size.y;
               column.set_size(column_size,false);
               loc.x = column_width * idx;
               loc.y = owner_size.y - column_size.y;
               column.set_position(loc,false);
               loc.y = owner_size.y - column_size.y - label_size.y;
               label.set_size(label_size,false);
               label.set_position(loc,false)
               //godot_print!("{}",format!("tag {tag:?} column size {column_size:?} loc {loc:?} max diff {max_diff:?}"));
           });
           idx+=1.0;
       }
    }

    pub fn add_data(&mut self,tag:String,data:DataValue){
        self.actions_tx.send(DataActions::UpdateColumn(tag,data));
    }
}


