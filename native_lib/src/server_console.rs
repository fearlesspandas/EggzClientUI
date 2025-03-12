
use std::collections::{HashMap};
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Instanced,InstancedDefault};
use crate::field_abilities::{AbilityType};
use crate::ui_traits::{Windowed,LabelButton,Action,Centering,TileButton};
use crate::button_tiles::{TileType,Tile};
use tokio::sync::mpsc;

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

#[derive(Clone)]
pub enum ConsoleCommand{
    unhandled,
    clicked(String),
    request_location(String),

}

impl From<Action> for ConsoleCommand{
    fn from(item:Action) -> ConsoleCommand{
        ConsoleCommand::unhandled
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_window_signals)]
pub struct PlayerChart{
    id:Option<String>,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    tx:Sender<ConsoleCommand>,
    hovering:bool,
    location:Instance<PlayerLocation>,
}

impl InstancedDefault<Control,Sender<ConsoleCommand>> for PlayerChart{
    fn make(args:&Sender<ConsoleCommand>) -> Self{
        PlayerChart{
            id:None,
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            tx:args.clone(),
            hovering:false,
            location:PlayerLocation::make_instance(args).into_shared(),
        }
    }
}
impl Windowed<ConsoleCommand> for PlayerChart{
    const BG_HIGHLIGHT_COLOR:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const BG_COLOR:Color = Color{r:0.0,g:255.0,b:0.0,a:1.0};
    const MAIN_COLOR:Color = Color{r:255.0,g:0.0,b:0.0,a:1.0};
    const MARGIN_SIZE:f32 = 5.0;
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn centering(&self) -> Centering {Centering::center}
    fn tx(&self) -> &Sender<ConsoleCommand> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
    fn from_command(&self,cmd:Action) -> ConsoleCommand{
        match cmd{
            Action::clicked => self.id
                .as_ref()
                .map(|id| ConsoleCommand::clicked(id.clone()))
                .unwrap_or(ConsoleCommand::unhandled),
            _ => ConsoleCommand::unhandled,
        }
    }
}
#[methods]
impl PlayerChart{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as Windowed<ConsoleCommand>>::ready(self,owner);
        let location = unsafe{self.location.assume_safe()};
        owner.add_child(location.clone(),true);
        
        let _ = location.map(|_,control| control.set_visible(false));
        let _ = location.map(|_,control| control.set_mouse_filter(Control::MOUSE_FILTER_PASS));

        let label = unsafe{self.label.assume_safe()};
        owner.add_child(label,true);
        
        let _ = owner.connect("mouse_entered",owner,"entered",VariantArray::new_shared(),0);
        let _ = owner.connect("mouse_exited",owner,"exited",VariantArray::new_shared(),0);
        let _ = owner.connect("clicked",owner,"clicked",VariantArray::new_shared(),0);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        <Self as Windowed<ConsoleCommand>>::process(self,owner,delta);
        let location = unsafe{self.location.assume_safe()};
        let label = unsafe{self.label.assume_safe()};
        let owner_size = owner.size();
        let location_size = owner.size() / 3.0;
        let _ = location.map(|_,control| control.set_size(location_size,false));
        let location_position = Vector2::new(0.0,0.0);
        let _ = location.map(|_ , control| control.set_position(location_position,false));


        label.set_size(owner.size(),false);
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control> , event:Ref<InputEvent>){
        <Self as Windowed<ConsoleCommand>>::input(self,owner,event);
    }
    #[method]
    fn entered(&mut self){
        self.hover();
    }
    #[method]
    fn exited(&mut self){
        self.unhover();
    }
    #[method]
    fn clicked(&self) {
        let location = unsafe{self.location.assume_safe()};
        let _ = location.map(|_,control|control.set_visible(!control.is_visible()));
        self.id.clone().map(|id| self.tx().send(ConsoleCommand::request_location(id)));
    }
    #[method]
    fn set_id(&mut self, id:String){
        self.id = Some(id.clone());
        let label = unsafe{self.label.assume_safe()};
        label.set_text(id);
    }
    #[method]
    fn set_location(&mut self,location:Vector3) -> Result<(),ConsoleError>{
        let player_location = unsafe{self.location.assume_safe()};
        player_location
            .map_mut(|obj,_| obj.set_location(location))
            .unwrap_or(Err(ConsoleError::ChartLocationUpdateError))
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_window_signals)]
pub struct PlayerLocation{
    location:Option<Vector3>,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    tx:Sender<ConsoleCommand>,
    hovering:bool,
}

impl InstancedDefault<Control,Sender<ConsoleCommand>> for PlayerLocation{
    fn make(args:&Sender<ConsoleCommand>) -> Self{
        PlayerLocation{
            location:None,
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            tx:args.clone(),
            hovering:false,
        }
    }
}
impl Windowed<ConsoleCommand> for PlayerLocation{
    const BG_HIGHLIGHT_COLOR:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const BG_COLOR:Color = Color{r:0.0,g:255.0,b:0.0,a:1.0};
    const MAIN_COLOR:Color = Color{r:255.0,g:0.0,b:0.0,a:1.0};
    const MARGIN_SIZE:f32 = 5.0;
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn centering(&self) -> Centering {Centering::center}
    fn tx(&self) -> &Sender<ConsoleCommand> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
}
impl LabelButton<ConsoleCommand> for PlayerLocation{
    fn label(&self) -> &Ref<Label>{
        &self.label
    }
}
#[methods]
impl PlayerLocation{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as LabelButton<ConsoleCommand>>::ready(self,owner);
        let _ = owner.connect("mouse_entered",owner,"entered",VariantArray::new_shared(),0);
        let _ = owner.connect("mouse_exited",owner,"exited",VariantArray::new_shared(),0);
        let _ = owner.connect("clicked",owner,"clicked",VariantArray::new_shared(),0);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        <Self as LabelButton<ConsoleCommand>>::process(self,owner,delta);
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control> , event:Ref<InputEvent>){
        <Self as Windowed<ConsoleCommand>>::input(self,owner,event);
    }
    #[method]
    fn entered(&mut self){
        self.hover();
    }
    #[method]
    fn exited(&mut self){
           self.unhover();
    }
    #[method]
    fn clicked(&self) {
    }
    #[method]
    fn set_location(&mut self,location:Vector3) -> Result<(),ConsoleError>{
        self.location = Some(location);
        let (x,y,z) = (location.x,location.y,location.z);
        self.set_text(format!("x:{x:?} y:{y:?} z:{z:?}"));
        Ok(())
    }
}


#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct ServerConsole{
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    players:HashMap<String,Instance<PlayerChart>>,
    tx:Sender<ConsoleCommand>,
    rx:Receiver<ConsoleCommand>,
    atx:Sender<Action>,
    arx:Receiver<Action>,
    hovered:bool,
}
impl Instanced<Control> for ServerConsole{
    fn make() -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<ConsoleCommand>();
        let (atx,arx) = mpsc::unbounded_channel::<Action>();
        ServerConsole{
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            players:HashMap::new(),
            tx:tx,
            rx:rx,
            atx:atx,
            arx:arx,
            hovered:false,
        }
    }
}
impl Windowed<Action> for ServerConsole{

    const BG_HIGHLIGHT_COLOR:Color = Color{r:100.0,g:100.0,b:100.0,a:1.0};
    const BG_COLOR:Color   = Color{r:0.0,g:100.0,b:0.0,a:1.0};
    const MAIN_COLOR:Color = Color{r:100.0,g:0.0,b:0.0,a:1.0};
    const MARGIN_SIZE:f32  = 10.0;
    fn hovering(&self) -> bool{
        return self.hovered
    }
    fn set_hovering(&mut self,value:bool){
        self.hovered = value;
    }
    fn tx(&self) -> &Sender<Action>{
        &self.atx
    }

    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
    fn centering(&self) -> Centering{Centering::center}

}
#[methods]
impl ServerConsole{
    fn register_signals(builder:&ClassBuilder<Self>){
        Windowed::<Action>::register_window_signals(builder);
        builder
            .signal("request_location")
            .with_param("player_id",VariantType::GodotString)
            .done();
    }
    #[method]
    fn _ready(&mut self,#[base] owner:TRef<Control>){
        <Self as Windowed<Action>>::ready(self,owner);
        let _ = owner.connect("mouse_entered",owner,"entered",VariantArray::new_shared(),0);
        let _ = owner.connect("mouse_exited",owner,"exited",VariantArray::new_shared(),0);
        let _ = owner.connect("clicked",owner,"clicked",VariantArray::new_shared(),0);

        //let add_1 = self.add_player(owner,"Bananas1".to_string());
        //let add_2 = self.add_player(owner,"Bananas2".to_string());
        //let add_3 = self.add_player(owner,"Bananas3".to_string());
        //godot_print!("{}",format!("Add 1 console success : {add_1:?}"));
        //godot_print!("{}",format!("Add 2 console success : {add_2:?}"));
        //godot_print!("{}",format!("Add 3 console success : {add_3:?}"));
        godot_print!("ServerConsole Readied");
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control> , event:Ref<InputEvent>){
        <Self as Windowed<Action>>::input(self,owner,event);
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Control>,delta:f64){
        match self.rx.try_recv(){
            Ok(ConsoleCommand::request_location(id)) => {
                owner.emit_signal("request_location",&[Variant::new(id)]);
            }
            Ok(_) => {}
            Err(_) => {}
        }
        <Self as Windowed<Action>>::process(self,owner,delta);
        //size Self/Owner
        let window_size = OS::godot_singleton().window_size();
        owner.set_size(window_size/2.0,false);
        owner.set_position(Vector2::new(window_size.x/2.0,0.0),false);
        //size player charts
        let num_players = self.players.values().len() as f32;
        if num_players < 1.0 {return ;}
        let owner_size = owner.size();
        let chart_size = Vector2::new(owner_size.x/2.0,owner_size.y/num_players);
        let mut idx = 0.0;
        for player_chart in self.players.values() {
            let player_chart = unsafe{player_chart.assume_safe()};
            let _ = player_chart.map(|_,control| control.set_size(chart_size,false));
            let position = Vector2::new(owner_size.x - chart_size.x,idx * chart_size.y);
            let _ = player_chart.map(|_,control| control.set_position(position,false));
            idx += 1.0;
        }
    }
    #[method]
    fn entered(&mut self){
        self.hover();
    }
    #[method]
    fn exited(&mut self){
           self.unhover();
    }
    #[method]
    fn clicked(&self) {
    }
    #[method]
    fn add_player(&mut self,#[base] owner:TRef<Control>,id:String) -> Result<(),ConsoleError>{
        if self.players.contains_key(&id){return Err(ConsoleError::PlayerAlreadyExists)}
        let chart = PlayerChart::make_instance(&self.tx).into_shared();

        self.players.insert(id.clone(),chart.clone());

        let chart = unsafe{chart.assume_safe()};

        owner.add_child(chart.clone(),true);
        chart
            .map_mut(|obj,_| obj.set_id(id))
            .map_err(|_| ConsoleError::FailedIDError("Could Not set id for player chart".to_string()))
    }
    #[method]
    fn update_location(&self,id:String,location:Vector3) -> Result<(),ConsoleError>{
        let player_chart = self.players.get(&id);
        match player_chart{
            Some(chart) => {
                let chart = unsafe{chart.assume_safe()};
                chart.map_mut(|obj,_| 
                    obj.set_location(location)
                ).unwrap_or(Err(ConsoleError::LocationUpdateFailed))
            }
            _ => Err(ConsoleError::NoChartFoundError),
        }
        
    }
}
#[derive(Clone,Debug)]
pub enum ConsoleError{
    FailedIDError(String),
    NoChartFoundError,
    LocationUpdateFailed,
    ChartLocationUpdateError,
    PlayerAlreadyExists,

}
impl Into<u8> for ConsoleError{
    fn into(self) -> u8{
        match self{
            _ => 255
        }
    }
}
impl ToVariant for ConsoleError{
    fn to_variant(&self) -> Variant{
        Variant::new(Into::<u8>::into(self.clone()))
    }
}
