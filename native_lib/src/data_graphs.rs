use gdnative::prelude::*;
use gdnative::api::*;
use std::collections::HashMap;
use std::fmt;
use std::cmp::max;
use std::sync::{Arc,Mutex,atomic::{AtomicU32,AtomicU64,Ordering}};
use serde::{Deserialize,Serialize};
use crate::traits::{GetAll,Instanced};
use tokio::{
    runtime::Runtime,
    io::{AsyncBufRead, AsyncBufReadExt, AsyncWriteExt, BufReader,BufWriter,ReadHalf,WriteHalf},
    sync::mpsc,
    select
};
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

enum DataActions{
    UpdateColumn(String,DataValue),
}
enum GraphActions{
    CreateColumn(String),
    ClearGraph,
}
type DataValue = f32;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct AggregateStats{
    label: Ref<Label>,
    avg:f64,
    min:f64,
    max:f64,
}
impl Instanced<Control> for AggregateStats{
    fn make() -> Self{
        AggregateStats{
            label: Label::new().into_shared(),
            avg:0.0,
            min:0.0,
            max:0.0,
        }
    }
}
#[methods]
impl AggregateStats{
    #[method]
    fn _ready(&self, #[base] owner:TRef<Control>){
        owner.add_child(self.label,true);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        let label = unsafe{self.label.assume_safe()};
        let avg = self.avg;
        let min = self.min;
        let max = self.max;
        label.set_size(owner.size(),false);
        label.set_text(format!("Avg:{avg:?}\nMin:{min:?}\nMax:{max:?}"));
    }
    fn calculate_avg(values:&Vec<f64>) -> f64{
        let mut avg_accum = 0.0;
        for value in values{
            avg_accum += value;
        }
        avg_accum/(values.len() as f64)
    }
    fn calculate_min(values:&Vec<f64>) -> f64{
       values.iter().fold(f64::MAX,|acc,curr| f64::min(acc,*curr))
    }
    fn calculate_max(values:&Vec<f64>) -> f64{
       values.iter().fold(f64::MIN,|acc,curr| f64::max(acc,*curr))
    }
    fn set_avg(&mut self,values:&Vec<f64>){
        self.avg = Self::calculate_avg(values);
    }
    fn set_min(&mut self,values:&Vec<f64>){
        self.min = Self::calculate_min(values);
    }
    fn set_max(&mut self,values:&Vec<f64>){
        self.max = Self::calculate_max(values);
    }
}

#[derive(NativeClass)]
#[inherit(Control)]
pub struct HoverStats{
    value:f64,
    tag:String,
    stats_label: Ref<Label>
}
#[methods]
impl HoverStats{
    fn new(base:&Control) -> Self{
        todo!()
    }
    fn make() -> Instance<HoverStats,Unique>{
        Instance::emplace(
            HoverStats{
                value: 0.0,
                tag:"".to_string(), 
                stats_label: Label::new().into_shared(),
            }
        )
    }

    #[method]
    fn _ready(&self, #[base] owner: TRef<Control>){
        let label = unsafe{self.stats_label.assume_safe()};
        label.set_size(Vector2{x:200.0,y:100.0},false);
        owner.set_size(Vector2{x:200.0,y:100.0},false);
        owner.add_child(label,true);
        //owner.set_layer(2);
        owner.set_visible(false);
    }

    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64 ){
        let label = unsafe{self.stats_label.assume_safe()};
        let value = self.value;
        let tag = &self.tag;
        label.set_text(format!("{tag:?} \n value: {value:?}"));
        owner.get_parent().map(|parent|{
            let parent = unsafe{parent.assume_safe()};
            parent.cast::<Control>().map(|control|{
                let parent_size = control.size();
                let parent_position = control.global_position();
                let current_position = owner.global_position();
                let current_size = owner.size();
                let max_x = parent_position.x + parent_size.x - current_size.x;
                let label_position_x = (max_x * (current_position.x > max_x ) as i32 as f32) + (current_position.x * (current_position.x <= max_x ) as i32 as f32);
                let label_position_y = parent_position.y - current_size.y;
                owner.set_global_position(Vector2{x:label_position_x,y:label_position_y},false);
            });
        });
    }

    #[method]
    fn display_stats(&mut self,#[base] owner:TRef<Control>,tag:String,value:f64,bar_position:Vector2){
        self.tag = tag;
        self.set_value(value);
        owner.set_visible(true);
        owner.set_global_position(Vector2{x:bar_position.x, y:bar_position.y - 100.0},false);
    }
    #[method]
    fn hide_stats(&self, #[base] owner:TRef<Control>){
        owner.set_visible(false);
    }
    fn set_value(&mut self,val:f64){
        self.value = val;
    }

}


#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct BarGraphColumn{
    tag: String,
    value: f64,
    bar:Ref<ColorRect>,
    hovering: bool,
}
#[methods]
impl BarGraphColumn{
    fn register_signals(builder:&ClassBuilder<Self>){
        builder
            .signal("hovered")
            .with_param("tag",VariantType::GodotString)
            .with_param("value",VariantType::F64)
            .with_param("position",VariantType::Vector2)
            .done();
        builder
            .signal("unhovered")
            .done();
    }
    fn new(base: &Control) -> Self{
        BarGraphColumn{
            tag: "".to_string(),
            value : 0.0,
            bar: ColorRect::new().into_shared(),
            hovering: false,
        }
    }
    fn make() -> Instance<BarGraphColumn,Unique>{
        Instance::emplace(
            BarGraphColumn{
                tag: "".to_string(),
                value : 0.0,
                bar: ColorRect::new().into_shared(),
                hovering: false,
            }
        )
    }

    fn set_tag(&mut self, tag:String){
        self.tag = tag;
    }
    fn set_value(&mut self, value:f64){
        self.value = value;
    }

    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        let bar = unsafe{self.bar.assume_safe()};
        //bar.set_frame_color(Color{r:250.0,g:0.0,b:0.0,a:1.0});
        owner.add_child(bar,true);
        owner.connect("mouse_entered",owner,"hover",VariantArray::new_shared(),0);
        owner.connect("mouse_exited",owner,"unhover",VariantArray::new_shared(),0);
        bar.connect("mouse_entered",owner,"hover",VariantArray::new_shared(),0);
        bar.connect("mouse_exited",owner,"unhover",VariantArray::new_shared(),0);
    }

    #[method]
    fn _process(&self,#[base] owner: TRef<Control>,delta:f64){
        let bar = unsafe{self.bar.assume_safe()};
        bar.set_size(owner.size(),false);
        if self.hovering{
            bar.set_frame_color(Color{r:255.0,g:255.0,b:255.0,a:1.0});

        }else{
            bar.set_frame_color(Color{r:250.0,g:0.0,b:0.0,a:1.0});
        }
    }

    #[method]
    fn hover(&mut self,#[base] owner:TRef<Control>){
        let bar = unsafe{self.bar.assume_safe()};
        self.hovering = true;
        let global_pos = owner.global_position();
        bar.set_frame_color(Color{r:255.0,g:255.0,b:255.0,a:1.0});
        owner.emit_signal("hovered",&[Variant::new(&self.tag),Variant::new(&self.value),Variant::new(global_pos)]);
    }

    #[method]
    fn unhover(&mut self,#[base] owner:TRef<Control>){
        let bar = unsafe{self.bar.assume_safe()};
        self.hovering = false;
        bar.set_frame_color(Color{r:250.0,g:0.0,b:0.0,a:1.0});
        owner.emit_signal("unhovered",&[]);
    }


}
#[derive(NativeClass)]
#[inherit(Control)]
pub struct BarGraph{
    tag_to_data: Arc<Mutex<HashMap<String,DataValue>>>,
    columns: HashMap<String,(Instance<BarGraphColumn>,Ref<Label>)>,
    labels_y: [Ref<Label>;3],
    aggregate_stats:Instance<AggregateStats>,
    hover_stats: Instance<HoverStats>,
    current_max: Arc<AtomicU32>,
    current_min: Arc<AtomicU32>,
    actions_tx: Sender<DataActions>,
    graph_actions_tx: Sender<GraphActions>,
    graph_actions_rx: Receiver<GraphActions>,
    runtime:Runtime,
    resize_timer : Ref<Timer>,
} 
#[methods]
impl BarGraph{
    pub fn new(base: &Control) -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<DataActions>();
        let (gtx,grx) = mpsc::unbounded_channel::<GraphActions>();
        BarGraph{
            tag_to_data: todo!(),
            columns: todo!(),
            labels_y : todo!(),
            aggregate_stats:todo!(),
            hover_stats: todo!(),
            current_max: todo!(),
            current_min: todo!(),
            actions_tx: tx,
            graph_actions_tx: gtx,
            graph_actions_rx: grx,
            runtime:todo!(),
            resize_timer : Timer::new().into_shared(),
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
                labels_y: [Label::new().into_shared(),Label::new().into_shared(),Label::new().into_shared()],
                aggregate_stats:AggregateStats::make_instance().into_shared(),
                hover_stats: HoverStats::make().into_shared(),
                current_max: c_max,
                current_min: c_min,
                actions_tx: tx,
                graph_actions_tx: c_gtx,
                graph_actions_rx: grx,
                runtime:rt,
                resize_timer : Timer::new().into_shared(),
            })
    }

    #[method]
    fn _ready(&mut self,#[base] owner:TRef<Control>){
        let resize_timer = unsafe{self.resize_timer.assume_safe()};
        let (max_label,center_label,min_label) = unsafe{ 
            (self.labels_y[0].assume_safe(),self.labels_y[1].assume_safe(),self.labels_y[2].assume_safe())
        };
        let hover_stats = unsafe{self.hover_stats.assume_safe()};
        resize_timer.set_wait_time(3.0);
        resize_timer.connect("timeout",owner,"resize_graph",VariantArray::new_shared(),0);
        owner.add_child(resize_timer,true);
        resize_timer.start(-1.0);

        owner.add_child(max_label,true);
        owner.add_child(center_label,true);
        owner.add_child(min_label,true);

        hover_stats.map(|obj,canvas| {
            owner.add_child(canvas,true);
        });

    }

    #[method]
    fn resize_graph(&self, #[base] owner:TRef<Control>){
       let tag_to_data = self.tag_to_data.lock().unwrap();
       let mut calculated_max:&f32  = &0.0;
       let mut calculated_min:&f32  = &0.0;
       for v in tag_to_data.values(){
          if v > &calculated_max{calculated_max = v;} 
          if v < &calculated_min {calculated_min = v;}
       }
       self.current_max.store(calculated_max.to_bits(),Ordering::Relaxed);
       self.current_min.store(calculated_min.to_bits(),Ordering::Relaxed);
    }
    
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Control>,delta:f64){
        if !owner.is_visible(){
            return;
        }
       match self.graph_actions_rx.try_recv(){
           Ok(GraphActions::CreateColumn(tag)) => {
               let mut columns = &mut self.columns;
               let hover_stats = unsafe{self.hover_stats.assume_safe()};
               let color_rect: Ref<ColorRect> = ColorRect::new().into_shared();
               let bar = BarGraphColumn::make().into_shared();
               let tag_label : Ref<Label> = Label::new().into_shared();
               columns.insert(tag.clone(),(bar.clone(),tag_label));
               let tag_label = unsafe{tag_label.assume_safe()};
               tag_label.set_text(&tag);
               let bar = unsafe{bar.assume_safe()};
               bar.map(|_,control| owner.add_child(control,true));
               bar.map_mut(|obj,_| obj.set_tag(tag));
               owner.add_child(tag_label,true);
               hover_stats.map(|_,canvas|{
                   bar.map(|_,control|{
                       control.connect("hovered",canvas,"display_stats",VariantArray::new_shared(),0);
                       control.connect("unhovered",canvas,"hide_stats",VariantArray::new_shared(),0);
                   });
               });
           }
           Ok(GraphActions::ClearGraph) => {self.clear_graph(owner);}
           Err(_) => {} 
       } 
       let tag_to_data = self.tag_to_data.lock().unwrap();
       let columns = &self.columns;
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
       let mut sorted_keys = tag_to_data.keys().into_iter().collect::<Vec<&String>>();
       sorted_keys.sort();
       for (tag) in sorted_keys{
           let value = tag_to_data.get(tag).unwrap();
           let col = columns.get(tag);
           col.map(|column|{
               let (column,label) = column;
               let column = unsafe{column.assume_safe()};
               let label = unsafe{label.assume_safe()};
               column_size.y = (value/(max_diff + ((max_diff == 0.0) as i32 as f32 * value))) * owner_size.y;
               column.map(|_,control| control.set_size(column_size,false));
               column.map_mut(|bar,_|bar.set_value(*value as f64));
               loc.x = column_width * idx;
               loc.y = owner_size.y - column_size.y;
               column.map(|_,control| control.set_position(loc,false));
               loc.y = owner_size.y - column_size.y - label_size.y;
               label.set_size(label_size,false);
               label.set_position(loc,false);
               //godot_print!("{}",format!("tag {tag:?} column size {column_size:?} loc {loc:?} max diff {max_diff:?}"));
           });
           idx+=1.0;
       }
       let max_label_x = unsafe{self.labels_y[0].assume_safe()};
       let center_label_x = unsafe{self.labels_y[1].assume_safe()};
       let min_label_x = unsafe{self.labels_y[2].assume_safe()};
       let midpoint = (current_max - current_min)/2.0;
       max_label_x.set_text(format!("{current_max:?}"));
       center_label_x.set_text(format!("{midpoint:?}"));
       min_label_x.set_text(format!("{current_min:?}"));
       min_label_x.set_position(Vector2{x:0.0,y:owner_size.y},false);
       center_label_x.set_position(Vector2{x:0.0,y:owner_size.y/2.0},false);
       max_label_x.set_position(Vector2{x:0.0,y:0.0},false);
       let label_x_size = Vector2{x:50.0,y:50.0};
       max_label_x.set_size(label_x_size,false);
       min_label_x.set_size(label_x_size,false);
       center_label_x.set_size(label_x_size,false);
    }

    pub fn add_data(&mut self,tag:String,data:DataValue){
        self.actions_tx.send(DataActions::UpdateColumn(tag,data));
    }
    pub fn queue_clear(&mut self){
        self.graph_actions_tx.send(GraphActions::ClearGraph);
    }

    fn clear_graph(&mut self , owner:TRef<Control>){
        self.tag_to_data.lock().unwrap().clear();
        let hover_stats = unsafe{self.hover_stats.assume_safe()};
        hover_stats.map(|_,control| {
            for (_,data_bar) in &self.columns {
                let (column,label) = data_bar;
                let (column,label) = (unsafe{column.assume_safe()},unsafe{label.assume_safe()});
                column.map(|_,col|{
                    col.disconnect("hovered",control,"display_stats");
                    col.disconnect("unhovered",control,"hide_stats");
                    owner.remove_child(col);
                    col.queue_free();
                });
                owner.remove_child(label);
                label.queue_free();
            }
        });
        self.columns.clear();
    }
}


