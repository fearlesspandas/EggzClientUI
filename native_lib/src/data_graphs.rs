use gdnative::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc,Mutex,atomic::{AtomicU32,Ordering}};
use crate::traits::{Instanced};
use crate::data_snapshots::{DataSnapshots,BarGraphSnapshot};
use tokio::{
    runtime::Runtime,
    sync::mpsc
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

fn position_within_parent(owner:TRef<Control>){
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
#[derive(NativeClass)]
#[inherit(Control)]
pub struct AggregateStats{
    label: Ref<Label>,
    avg:f32,
    min:f32,
    max:f32,
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
    fn _process(&self,#[base] owner:TRef<Control>,_delta:f64){
        let label = unsafe{self.label.assume_safe()};
        let avg = self.avg;
        let min = self.min;
        let max = self.max;
        label.set_size(owner.size(),false);
        label.set_text(format!("Avg:{avg:?}\nMin:{min:?}\nMax:{max:?}"));
    }
    fn calculate_avg(values:Vec<&f32>) -> f32{
        let mut avg_accum = 0.0;
        let num = values.len() as f32;
        for value in values{
            avg_accum += value;
        }
        avg_accum/num
    }
    fn calculate_min(values:Vec<f32>) -> f32{
       values.iter().fold(f32::MAX,|acc,curr| f32::min(acc,*curr))
    }
    fn calculate_max(values:Vec<f32>) -> f32{
       values.iter().fold(f32::MIN,|acc,curr| f32::max(acc,*curr))
    }
    fn set_avg(&mut self,value:f32){
        self.avg = value;
    }
    fn set_min(&mut self,value:f32){
        self.min = value;
    }
    fn set_max(&mut self,value:f32){
        self.max = value;
    }
    fn set_calc_avg(&mut self,values:Vec<&f32>){
        self.avg = Self::calculate_avg(values);
    }
    #[method]
    fn set_calc_min(&mut self,values:Vec<f32>){
        self.min = Self::calculate_min(values);
    }
    #[method]
    fn set_calc_max(&mut self,values:Vec<f32>){
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
impl Instanced<Control> for HoverStats{
    fn make() -> Self{
        HoverStats{
            value: 0.0,
            tag:"".to_string(), 
            stats_label: Label::new().into_shared(),
        }
    }

}
#[methods]
impl HoverStats{
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
    fn _process(&self,#[base] owner:TRef<Control>,_delta:f64 ){
        let label = unsafe{self.stats_label.assume_safe()};
        let value = self.value;
        let tag = &self.tag;
        label.set_text(format!("{tag:?} \n value: {value:?}"));
        position_within_parent(owner);
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
impl Instanced<Control> for BarGraphColumn{
    fn make() -> Self{
        BarGraphColumn{
            tag: "".to_string(),
            value : 0.0,
            bar: ColorRect::new().into_shared(),
            hovering: false,
        }
    }
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
        let _ = owner.connect("mouse_entered",owner,"hover",VariantArray::new_shared(),0);
        let _ = owner.connect("mouse_exited",owner,"unhover",VariantArray::new_shared(),0);
        let _ = bar.connect("mouse_entered",owner,"hover",VariantArray::new_shared(),0);
        let _ = bar.connect("mouse_exited",owner,"unhover",VariantArray::new_shared(),0);
    }

    #[method]
    fn _process(&self,#[base] owner: TRef<Control>,_delta:f64){
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
    current_avg: Arc<AtomicU32>,
    current_max: Arc<AtomicU32>,
    current_min: Arc<AtomicU32>,
    actions_tx: Sender<DataActions>,
    graph_actions_tx: Sender<GraphActions>,
    graph_actions_rx: Receiver<GraphActions>,
    resize_timer : Ref<Timer>,
    snapshot_api: Instance<DataSnapshots>,
} 
impl Instanced<Control> for BarGraph{
    fn make() -> Self{
        let (tx,mut rx) = mpsc::unbounded_channel::<DataActions>();
        let (gtx,grx) = mpsc::unbounded_channel::<GraphActions>();
        let c_gtx = gtx.clone();
        let tag_to_data = Arc::new(Mutex::new(HashMap::new()));
        let c_data = tag_to_data.clone();
        let max_at = Arc::new(AtomicU32::new(0));
        let min_at = Arc::new(AtomicU32::new(0));
        let avg_at = Arc::new(AtomicU32::new(0));
        let c_max = max_at.clone();
        let c_min = min_at.clone();
        let c_avg = avg_at.clone();
        let columns = HashMap::new();
        let rt = Runtime::new().unwrap();
        rt.spawn(async move{
            let tag_to_data = tag_to_data.clone();
            while let Some(action) = rx.recv().await{
                match action{
                    DataActions::UpdateColumn(tag,value) => {
                       let mut tag_to_data = tag_to_data.lock().unwrap();
                       if !tag_to_data.contains_key(&tag){
                           let _ = gtx.send(GraphActions::CreateColumn(tag.clone()));
                       }
                       if tag_to_data.contains_key(&tag){
                           let current_avg = f32::from_bits(avg_at.load(Ordering::Relaxed));
                           let current_val = tag_to_data.get(&tag).unwrap();
                           let num = tag_to_data.len() as f32;
                           let calc_avg = ((current_avg * num) - current_val + value)/num;
                           avg_at.store(calc_avg.to_bits(),Ordering::Relaxed);
                       }
                       else{
                           let current_avg = f32::from_bits(avg_at.load(Ordering::Relaxed));
                           let num = tag_to_data.len() as f32;
                           let calc_avg = ((current_avg * num) + value)/(num + 1.0);
                           avg_at.store(calc_avg.to_bits(),Ordering::Relaxed);
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
        BarGraph{
            tag_to_data: c_data,
            columns: columns,
            labels_y: [Label::new().into_shared(),Label::new().into_shared(),Label::new().into_shared()],
            aggregate_stats:AggregateStats::make_instance().into_shared(),
            hover_stats: HoverStats::make_instance().into_shared(),
            current_avg: c_avg,
            current_max: c_max,
            current_min: c_min,
            actions_tx: tx,
            graph_actions_tx: c_gtx,
            graph_actions_rx: grx,
            resize_timer : Timer::new().into_shared(),
            snapshot_api : DataSnapshots::make_instance().into_shared(),
        }
    }
}
#[methods]
impl BarGraph{
    #[method]
    fn _ready(&mut self,#[base] owner:TRef<Control>){
        let resize_timer = unsafe{self.resize_timer.assume_safe()};
        let (max_label,center_label,min_label) = unsafe{ 
            (self.labels_y[0].assume_safe(),self.labels_y[1].assume_safe(),self.labels_y[2].assume_safe())
        };
        let hover_stats = unsafe{self.hover_stats.assume_safe()};
        let aggregate_stats = unsafe{self.aggregate_stats.assume_safe()};
        resize_timer.set_wait_time(3.0);
        let _ = resize_timer.connect("timeout",owner,"resize_graph",VariantArray::new_shared(),0);
        owner.add_child(resize_timer,true);
        resize_timer.start(-1.0);
        owner.add_child(max_label,true);
        owner.add_child(center_label,true);
        owner.add_child(min_label,true);
        let _ = hover_stats.map(|_,canvas| {
            owner.add_child(canvas,true);
        });
        let _ = aggregate_stats.map(|_,control| owner.add_child(control,true));
        //aggregate_stats.map(|_,control| control.set_visible(false));
    }
    #[method]
    fn resize_graph(&self, #[base] _owner:TRef<Control>){
       let tag_to_data = self.tag_to_data.lock().unwrap();
       let values = tag_to_data.values().into_iter().map(|x|*x).collect::<Vec<f32>>();
       let calc_max = values.iter().fold(0.0,|acc,curr| f32::max(acc,*curr));
       let calc_min = values.iter().fold(0.0,|acc,curr| f32::min(acc,*curr));
       self.current_max.store(calc_max.to_bits(),Ordering::Relaxed);
       self.current_min.store(calc_min.to_bits(),Ordering::Relaxed);
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Control>,_delta:f64){
        if !owner.is_visible(){
            return;
        }
       match self.graph_actions_rx.try_recv(){
           Ok(GraphActions::CreateColumn(tag)) => {
               let columns = &mut self.columns;
               let hover_stats = unsafe{self.hover_stats.assume_safe()};
               let bar = BarGraphColumn::make_instance().into_shared();
               let tag_label : Ref<Label> = Label::new().into_shared();
               columns.insert(tag.clone(),(bar.clone(),tag_label));
               let tag_label = unsafe{tag_label.assume_safe()};
               tag_label.set_text(&tag);
               let bar = unsafe{bar.assume_safe()};
               let _ = bar.map(|_,control| owner.add_child(control,true));
               let _ = bar.map_mut(|obj,_| obj.set_tag(tag));
               owner.add_child(tag_label,true);
               let _ = hover_stats.map(|_,canvas|{
                   let _ = bar.map(|_,control|{
                       let _ = control.connect("hovered",canvas,"display_stats",VariantArray::new_shared(),0);
                       let _ = control.connect("unhovered",canvas,"hide_stats",VariantArray::new_shared(),0);
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
       let label_size = Vector2{x:column_width, y: 10.0};
       let mut sorted_keys = tag_to_data.keys().into_iter().collect::<Vec<&String>>();
       sorted_keys.sort();
       for tag in sorted_keys{
           let value = tag_to_data.get(tag).unwrap();
           let col = columns.get(tag);
           let _ = col.map(|column|{
               let (column,label) = column;
               let column = unsafe{column.assume_safe()};
               let label = unsafe{label.assume_safe()};
               column_size.y = (value/(max_diff + ((max_diff == 0.0) as i32 as f32 * value))) * owner_size.y;
               let _ = column.map(|_,control| control.set_size(column_size,false));
               let _ = column.map_mut(|bar,_|bar.set_value(*value as f64));
               loc.x = column_width * idx;
               loc.y = owner_size.y - column_size.y;
               let _ = column.map(|_,control| control.set_position(loc,false));
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
       self.update_aggregate_stats();
       let agg_stats = unsafe{self.aggregate_stats.assume_safe()};
       let _ = agg_stats.map(|_,control| control.set_size(owner_size/4.0,false));
       let _ = agg_stats.map(|_,control| control.set_global_position(owner.global_position() - Vector2{x:0.0,y:control.size().y},false));
    }
    fn update_aggregate_stats(&self){
        let aggregate_stats = unsafe{self.aggregate_stats.assume_safe()};
        if aggregate_stats.map(|_,control|!control.is_visible()).unwrap_or(false){
            return
        }
        let current_avg = f32::from_bits(self.current_avg.load(Ordering::Relaxed));
        let current_min = f32::from_bits(self.current_min.load(Ordering::Relaxed));
        let current_max = f32::from_bits(self.current_max.load(Ordering::Relaxed));
        let _ = aggregate_stats.map_mut(|obj,_| obj.set_avg(current_avg));
        let _ = aggregate_stats.map_mut(|obj,_| obj.set_min(current_min));
        let _ = aggregate_stats.map_mut(|obj,_| obj.set_max(current_max));
    }
    pub fn snapshot(&self, name:String) -> Result<(),&'static str>{
        let tag_to_data = self.tag_to_data.lock().unwrap();
        let data_obj = BarGraphSnapshot{
            path:"user://snapshots".to_string().replace("\"",""),
            name:name.replace("\"",""),
            data:(*tag_to_data).clone()
        };
        DataSnapshots::save_snapshot(data_obj)
        //snapshots.map(|obj,_|obj.save_snapshot(data_obj)).unwrap()
    }
    pub fn load(&self,name:String) -> Result<BarGraphSnapshot,&'static str>{
        DataSnapshots::load(format!("user://snapshots/{name:?}"))
    }
    pub fn toggle_agg_stats(&self){
        let aggregate_stats = unsafe{self.aggregate_stats.assume_safe()};
        let _ = aggregate_stats.map(|_,control| control.set_visible(!control.is_visible()));
    }
    pub fn add_data(&mut self,tag:String,data:DataValue){
        let _ = self.actions_tx.send(DataActions::UpdateColumn(tag,data));
    }
    pub fn calc_update_aggregate_stats(&self){
        let aggregate_stats = unsafe{self.aggregate_stats.assume_safe()};
        let tag_to_data = self.tag_to_data.lock().unwrap();
        let data = tag_to_data.values().into_iter().collect::<Vec<&f32>>();
        let _ = aggregate_stats.map_mut(|obj,_| obj.set_calc_avg(data));
    }
    pub fn show_aggregate_stats(&self){
        let aggregate_stats = unsafe{self.aggregate_stats.assume_safe()};
        let tag_to_data = self.tag_to_data.lock().unwrap();
        let data = tag_to_data.values().into_iter().collect::<Vec<&f32>>();
        let _ = aggregate_stats.map_mut(|obj,_| obj.set_calc_avg(data));
        let _ = aggregate_stats.map(|_,control| control.set_visible(true));
    }
    pub fn queue_clear(&mut self){
        let _ = self.graph_actions_tx.send(GraphActions::ClearGraph);
    }
    fn clear_graph(&mut self , owner:TRef<Control>){
        self.tag_to_data.lock().unwrap().clear();
        let hover_stats = unsafe{self.hover_stats.assume_safe()};
        let _ = hover_stats.map(|_,control| {
            for (_,data_bar) in &self.columns {
                let (column,label) = data_bar;
                let (column,label) = (unsafe{column.assume_safe()},unsafe{label.assume_safe()});
                let _ = column.map(|_,col|{
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
        self.current_avg.store(f32::to_bits(0.0),Ordering::Relaxed);
        self.current_min.store(f32::to_bits(0.0),Ordering::Relaxed);
        self.current_max.store(f32::to_bits(0.0),Ordering::Relaxed);
    }
}


