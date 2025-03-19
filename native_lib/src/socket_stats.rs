
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Defaulted,Instanced,InstancedDefault};
use crate::field_abilities::{AbilityType};
use crate::ui_traits::{AnimationWindow,Windowed,LabelButton,Action,Centering,TileButton};
use crate::button_tiles::{TileType,Tile};
use tokio::sync::mpsc;
use std::collections::HashMap;

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

pub enum PanelAction{
    unhandled,
    toggle_expand(i64),
}
impl From<Action> for PanelAction{
    fn from(item:Action) -> PanelAction{ PanelAction::unhandled }
}

#[derive(NativeClass)]
#[inherit(Control)]
pub struct EntityId{
    tx:Sender<PanelAction>,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    hovering:bool,
}
impl InstancedDefault<Control,Sender<PanelAction>> for EntityId{
    fn make(args:&Sender<PanelAction>) -> Self{
        EntityId{
            tx:args.clone(),
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            hovering:false,
        }
    }
}
impl Windowed<PanelAction> for EntityId{
    const BG_HIGHLIGHT_COLOR:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const BG_COLOR:Color = Color{r:0.0,g:0.0,b:0.0,a:0.5};
    const MAIN_COLOR:Color = Color{r:0.0,g:0.0,b:0.0,a:0.6};
    const MARGIN_SIZE:f32 = 5.0;
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn centering(&self) -> Centering {Centering::center}
    fn tx(&self) -> &Sender<PanelAction> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
}
impl LabelButton<PanelAction> for EntityId{
    fn label(&self) -> &Ref<Label>{ &self.label }
}
#[methods]
impl EntityId{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as LabelButton<PanelAction>>::ready(self,owner);
        let _ = owner.connect("mouse_entered",owner,"hover",VariantArray::new_shared(),0);
        let _ = owner.connect("mouse_exited",owner,"unhover",VariantArray::new_shared(),0);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        <Self as LabelButton<PanelAction>>::process(self,owner,delta);
    }
    #[method]
    fn hover(&mut self){
        <Self as Windowed<PanelAction>>::hover(self);
    }
    #[method]
    fn unhover(&mut self){
        <Self as Windowed<PanelAction>>::unhover(self);
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
pub struct EntityList{
    tx:Sender<PanelAction>,
    entities:Vec<Instance<EntityId>>,
}
impl InstancedDefault<Control,Sender<PanelAction>> for EntityList{
    fn make(args:&Sender<PanelAction>) -> Self{
        EntityList{
            tx:args.clone(),
            entities:Vec::new(),
        }
    }
}
#[methods]
impl EntityList{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        let owner_size = owner.size();
        let num_elements = std::cmp::max(self.entities.len(),1) as f32;
        let element_size = Vector2::new(owner_size.x,owner_size.y/num_elements);
        let mut idx = 0.0;
        for entity in &self.entities{
            let entity = unsafe{entity.assume_safe()};
            let _ = entity.map(|_,control| control.set_size(element_size,false));
            let _ = entity.map(|_,control| control.set_position(Vector2::new(0.0,element_size.y * idx),false));
            idx += 1.0;
        }
    }
    #[method]
    fn add(&mut self,#[base] owner:TRef<Control>, id:String){
        let list_element = EntityId::make_instance(&self.tx).into_shared();
        self.entities.push(list_element.clone());
        let list_element = unsafe{list_element.assume_safe()};
        let _ = list_element.map(|obj,_| obj.set_text(id));
        //let _ = list_element.map(|_,control| control.set_mouse_filter(Control::MOUSE_FILTER_PASS));
        owner.add_child(list_element,true);
    }
}
pub struct ServerPanelConfig{
    id:Option<i64>,
    entity_ids:Vec<String>,
    tx:Option<Sender<PanelAction>>
}
impl Defaulted for ServerPanelConfig{
    fn default() -> Self{
        ServerPanelConfig{
            id:None,
            entity_ids:Vec::new(),
            tx:None,
        }
    }
}
impl ServerPanelConfig{
    fn new(id:i64,entity_ids:&Vec<String>,tx:Option<Sender<PanelAction>>) -> Self{
        ServerPanelConfig{
            id:Some(id),
            entity_ids:entity_ids.clone(),
            tx:tx,
        }
    }
}

#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_window_signals)]
pub struct ServerPanel{
    id:i64,
    entity_ids:Vec<String>,
    entity_list:Instance<EntityList>,
    tx:Sender<PanelAction>,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    hovering:bool,
    font:Ref<DynamicFont>,
}
impl InstancedDefault<Control,ServerPanelConfig> for ServerPanel{
    fn make(args:&ServerPanelConfig) -> Self{
        ServerPanel{
            id:args.id.unwrap(),
            entity_ids:args.entity_ids.clone(),
            entity_list:EntityList::make_instance(&args.tx.clone().unwrap()).into_shared(),
            tx:args.tx.clone().unwrap(),
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            hovering:false,
            font: Self::base_font().unwrap(),
        }
    }
}
impl Windowed<PanelAction> for ServerPanel{
    const BG_HIGHLIGHT_COLOR:Color = Color{r:255.0,g:25.0,b:0.0,a:1.0};
    const BG_COLOR:Color = Color{r:0.0,g:50.0,b:50.0,a:0.7};
    const MAIN_COLOR:Color = Color{r:0.0,g:0.0,b:0.0,a:1.0};
    const MARGIN_SIZE:f32 = 5.0;
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn centering(&self) -> Centering {Centering::center}
    fn tx(&self) -> &Sender<PanelAction> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
    fn from_command(&self,cmd:Action) -> PanelAction{
        match cmd{
            Action::clicked => PanelAction::toggle_expand(self.id),
            _ => PanelAction::from(cmd),
        }
    }
}
impl LabelButton<PanelAction> for ServerPanel{
    fn base_font() -> Option<Ref<DynamicFont>>{
        let font_ref = DynamicFont::new().into_shared();
        let font = unsafe{font_ref.assume_safe()};
        let font_data = DynamicFontData::new().into_shared();
        let font_data = unsafe{font_data.assume_safe()};
        font_data.set_font_path("res://user_interface/client/overheads/Chicago.ttf");
        font.set_font_data(font_data);
        font.set_size(48);
        font.set_outline_color(Color{r:255.0,g:0.0,b:0.0,a:1.0});
        font.set_outline_size(Self::OUTLINE_SIZE);
        Some(font_ref)
    }
    fn font(&self) -> Option<Ref<DynamicFont>>{
        Some(self.font.clone())
    }
    fn label(&self) -> &Ref<Label>{&self.label}
}

#[methods]
impl ServerPanel{
    const OUTLINE_SIZE:i64 = 1;
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as LabelButton<PanelAction>>::ready(self,owner);
        self.set_text(self.id.to_string());

        let entity_list = unsafe{self.entity_list.assume_safe()};
        for id in &self.entity_ids{
            let _ = entity_list.map_mut(|obj,control| obj.add(control,id.clone()));
        }
        let _ = entity_list.map_mut(|_,control| control.set_visible(false));
        //let _ = entity_list.map(|_,control| control.set_mouse_filter(Control::MOUSE_FILTER_PASS));

        owner.add_child(entity_list,true);

        let _ = owner.connect("mouse_entered",owner,"hover",VariantArray::new_shared(),0);
        let _ = owner.connect("mouse_exited",owner,"unhover",VariantArray::new_shared(),0);
        let _ = owner.connect("clicked",owner,"clicked",VariantArray::new_shared(),0);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        <Self as LabelButton<PanelAction>>::process(self,owner,delta);
        let main_rect = unsafe{self.main_rect.assume_safe()};
        
        let main_size = main_rect.size();
        let owner_size = owner.size();
        let entity_list = unsafe{self.entity_list.assume_safe()};
        let _ = entity_list.map(|_,control|control.set_size(main_size,false));
        let _ = entity_list.map(|_,control|control.set_position(main_rect.position(),false));
        self.set_font_size((main_size.y/2.0).round() as i64);
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control>,event:Ref<InputEvent>){
        <Self as Windowed<PanelAction>>::input(self,owner,event.clone());
    }
    #[method]
    fn hover(&mut self){
        <Self as Windowed<PanelAction>>::hover(self);
    }
    #[method]
    fn unhover(&mut self){
        <Self as Windowed<PanelAction>>::unhover(self);
    }
    #[method]
    fn clicked(&self,#[base] owner:TRef<Control>){
        let entity_list = unsafe{self.entity_list.assume_safe()};
        let _ = entity_list.map_mut(|_,control| control.set_visible(!control.is_visible()));
    }
    fn set_connected(&self,is_connected:bool){
        if is_connected{
            self.set_font_outline(Color::from_rgba(0.0,255.0,0.0,1.0),Self::OUTLINE_SIZE);
        }else{
            self.set_font_outline(Color::from_rgba(255.0,0.0,0.0,1.0),Self::OUTLINE_SIZE);
        }
    }
}

#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_window_signals)]
pub struct ServerStats{
    tx:Sender<PanelAction>,
    rx:Receiver<PanelAction>,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    hovering:bool,
    connections:HashMap<i64,Instance<ServerPanel>>,
    expanded:Option<i64>,
}
impl Instanced<Control> for ServerStats{
    fn make() -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<PanelAction>();
        ServerStats{
            tx:tx,
            rx:rx,
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            hovering:false,
            connections:HashMap::new(),
            expanded:None,
        }
    }
}
impl Windowed<PanelAction> for ServerStats{
    const BG_HIGHLIGHT_COLOR:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const BG_COLOR:Color = Color{r:0.0,g:0.0,b:0.0,a:1.0};
    const MAIN_COLOR:Color = Color{r:255.0,g:0.0,b:0.0,a:1.0};
    const MARGIN_SIZE:f32 = 5.0;
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn centering(&self) -> Centering {Centering::center}
    fn tx(&self) -> &Sender<PanelAction> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
}
impl LabelButton<PanelAction> for ServerStats{
    fn label(&self) -> &Ref<Label>{&self.label}
}
#[methods]
impl ServerStats{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as Windowed<PanelAction>>::ready(self,owner);
        let _ = owner.connect("mouse_entered",owner,"hover",VariantArray::new_shared(),0);
        let _ = owner.connect("mouse_exited",owner,"unhover",VariantArray::new_shared(),0);
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Control>,delta:f64){
        <Self as Windowed<PanelAction>>::process(self,owner,delta);
        let screen_size = OS::godot_singleton().window_size();
        owner.set_size(screen_size/2.0,false);
        self.handle_panel_rx(owner,delta);
        match self.expanded{
            Some(id) => self.handle_sizing_if_expanded(owner,id),
            None => self.handle_sizing(owner),
        }
    }
    fn handle_sizing(&self,owner:TRef<Control>){
        let owner_size = owner.size();
        let num_connections = std::cmp::max(self.connections.len(),1) as f32;
        let panel_size = Vector2::new(owner_size.x,owner_size.y/num_connections);
        let mut idx = 0.0;
        for panel in self.connections.values(){
            let panel = unsafe{panel.assume_safe()};
            let _ = panel.map(|_,control| control.set_size(panel_size,false));
            let _ = panel.map(|_,control| control.set_position(Vector2::new(0.0,panel_size.y * idx),false));
            idx += 1.0;
        }

    }
    fn handle_sizing_if_expanded(&self,owner:TRef<Control>,expanded:i64){
        let panel = self.connections.get(&expanded).unwrap();
        let panel = unsafe{panel.assume_safe()};
        let _ = panel.map(|_,control| control.set_size(owner.size(),false));
        let _ = panel.map(|_,control| control.set_position(Vector2::new(0.0,0.0),false));
    }
    fn handle_panel_rx(&mut self,owner:TRef<Control>,delta:f64){
        match self.rx.try_recv(){
            Ok(PanelAction::toggle_expand(id)) => {
                match self.expanded{
                    Some(curr_id) if id == curr_id => {
                        for panel in self.connections.values(){
                            let panel = unsafe{panel.assume_safe()};
                            let _ = panel.map(|obj,control|{control.set_visible(true)});
                        }
                        self.expanded = None;
                    }
                    Some(_) => assert!(false),
                    None => {
                        for panel in self.connections.values(){
                            let panel = unsafe{panel.assume_safe()};
                            let _ = panel.map(|obj,control|if obj.id != id{control.set_visible(false) });
                        }
                        self.expanded = Some(id);
                    }
                }
            }
            Ok(PanelAction::unhandled) => {}
            Err(_) => {}
        }
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control>,event:Ref<InputEvent>){
        <Self as Windowed<PanelAction>>::input(self,owner,event.clone());
        if let Ok(event) = event.try_cast::<InputEventKey>(){
            let event = unsafe{event.assume_safe()};
            if event.is_action_released("server_stats_toggle",true){
                owner.set_visible(!owner.is_visible());
            }
        }
    }
    #[method]
    fn hover(&mut self){
        <Self as Windowed<PanelAction>>::hover(self);
    }
    #[method]
    fn unhover(&mut self){
        <Self as Windowed<PanelAction>>::unhover(self);
    }
    #[method]
    fn add_connection(&mut self,#[base] owner:TRef<Control>,id:i64,entities:Vec<String>) -> Result<(),Error>{
        let config = ServerPanelConfig::new(id,&entities,Some(self.tx.clone()));
        let panel = ServerPanel::make_instance(&config).into_shared(); 
        self.connections.insert(id,panel.clone());
        owner.add_child(panel,true);
        Ok(())
    }

    #[method]
    fn set_connection_status(&self,id:i64,is_connected:bool){
        let panel = self.connections.get(&id).unwrap();
        let panel = unsafe{panel.assume_safe()};
        let _ = panel.map(|obj,_| obj.set_connected(is_connected));
    }

}
#[derive(Clone)]
pub enum Error{
    AddConnectionError,
}
impl Into<u8> for Error{
    fn into(self) -> u8{
        match self{
            Error::AddConnectionError => 1 
        }
    }
}
impl ToVariant for Error{
    fn to_variant(&self) -> Variant{
        match self{
            Error::AddConnectionError => Variant::new(Into::<u8>::into(self.clone()))
        }
    }
}
