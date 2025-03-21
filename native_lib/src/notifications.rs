
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Defaulted,Instanced,InstancedDefault};
use crate::field_abilities::{AbilityType};
use crate::ui_traits::{AnimationWindow,Windowed,LabelButton,Action,Centering,TileButton};
use crate::button_tiles::{TileType,Tile};
use tokio::sync::mpsc;
use std::collections::{HashMap,HashSet};

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

pub enum NotificationAction{
    unhandled,
    toggle_expand(i64),
    remove_displayed(i64),
}
impl From<Action> for NotificationAction{
    fn from(item:Action) -> NotificationAction{ NotificationAction::unhandled }
}
#[derive(NativeClass)]
#[inherit(Control)]
pub struct TextPanel{
    tx:Sender<NotificationAction>,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    hovering:bool,
}
impl InstancedDefault<Control,Sender<NotificationAction>> for TextPanel{
    fn make(args:&Sender<NotificationAction>) -> Self{
        TextPanel{
            tx:args.clone(),
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            hovering:false,
        }
    }
}
impl Windowed<NotificationAction> for TextPanel{
    const BG_HIGHLIGHT_COLOR:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const BG_COLOR:Color = Color{r:0.0,g:0.0,b:0.0,a:0.5};
    const MAIN_COLOR:Color = Color{r:0.0,g:0.0,b:0.0,a:0.6};
    const MARGIN_SIZE:f32 = 5.0;
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn centering(&self) -> Centering {Centering::center}
    fn tx(&self) -> &Sender<NotificationAction> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
}
impl LabelButton<NotificationAction> for TextPanel{
    fn label(&self) -> &Ref<Label>{ &self.label }
}
#[methods]
impl TextPanel{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as LabelButton<NotificationAction>>::ready(self,owner);
        let _ = owner.connect("mouse_entered",owner,"hover",VariantArray::new_shared(),0);
        let _ = owner.connect("mouse_exited",owner,"unhover",VariantArray::new_shared(),0);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        <Self as LabelButton<NotificationAction>>::process(self,owner,delta);
    }
    #[method]
    fn hover(&mut self){
        <Self as Windowed<NotificationAction>>::hover(self);
    }
    #[method]
    fn unhover(&mut self){
        <Self as Windowed<NotificationAction>>::unhover(self);
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
pub struct Details{
    tx:Sender<NotificationAction>,
    entities:Vec<Instance<TextPanel>>,
}
impl InstancedDefault<Control,Sender<NotificationAction>> for Details{
    fn make(args:&Sender<NotificationAction>) -> Self{
        Details{
            tx:args.clone(),
            entities:Vec::new(),
        }
    }
}
#[methods]
impl Details{
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
        let list_element = TextPanel::make_instance(&self.tx).into_shared();
        self.entities.push(list_element.clone());
        let list_element = unsafe{list_element.assume_safe()};
        let _ = list_element.map(|obj,_| obj.set_text(id));
        //let _ = list_element.map(|_,control| control.set_mouse_filter(Control::MOUSE_FILTER_PASS));
        owner.add_child(list_element,true);
    }
}
pub struct NotificationConfig{
    id:Option<i64>,
    entity_ids:Vec<String>,
    tx:Option<Sender<NotificationAction>>
}
impl Defaulted for NotificationConfig{
    fn default() -> Self{
        NotificationConfig{
            id:None,
            entity_ids:Vec::new(),
            tx:None,
        }
    }
}
impl NotificationConfig{
    fn new(id:i64,entity_ids:&Vec<String>,tx:Option<Sender<NotificationAction>>) -> Self{
        NotificationConfig{
            id:Some(id),
            entity_ids:entity_ids.clone(),
            tx:tx,
        }
    }
}

#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_window_signals)]
pub struct Notification{
    id:i64,
    entity_ids:Vec<String>,
    entity_list:Instance<Details>,
    tx:Sender<NotificationAction>,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    hovering:bool,
    font:Ref<DynamicFont>,
    timer:Ref<Timer>,
}
impl InstancedDefault<Control,NotificationConfig> for Notification{
    fn make(args:&NotificationConfig) -> Self{
        Notification{
            id:args.id.unwrap(),
            entity_ids:args.entity_ids.clone(),
            entity_list:Details::make_instance(&args.tx.clone().unwrap()).into_shared(),
            tx:args.tx.clone().unwrap(),
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            hovering:false,
            font: Self::base_font().unwrap(),
            timer:Timer::new().into_shared(),
        }
    }
}
impl Windowed<NotificationAction> for Notification{
    const BG_HIGHLIGHT_COLOR:Color = Color{r:255.0,g:25.0,b:0.0,a:1.0};
    const BG_COLOR:Color = Color{r:0.0,g:50.0,b:50.0,a:0.7};
    const MAIN_COLOR:Color = Color{r:0.0,g:0.0,b:0.0,a:1.0};
    const MARGIN_SIZE:f32 = 5.0;
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn centering(&self) -> Centering {Centering::center}
    fn tx(&self) -> &Sender<NotificationAction> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
    fn from_command(&self,cmd:Action) -> NotificationAction{
        match cmd{
            Action::clicked => NotificationAction::toggle_expand(self.id),
            _ => NotificationAction::from(cmd),
        }
    }
}
impl LabelButton<NotificationAction> for Notification{
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
impl Notification{
    const OUTLINE_SIZE:i64 = 1;
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as LabelButton<NotificationAction>>::ready(self,owner);
        self.set_text(self.id.to_string());

        let entity_list = unsafe{self.entity_list.assume_safe()};
        for id in &self.entity_ids{
            let _ = entity_list.map_mut(|obj,control| obj.add(control,id.clone()));
        }
        let _ = entity_list.map_mut(|_,control| control.set_visible(false));
        //let _ = entity_list.map(|_,control| control.set_mouse_filter(Control::MOUSE_FILTER_PASS));

        owner.add_child(entity_list,true);

        let timer = unsafe{self.timer.assume_safe()};
        let _ = timer.connect("timeout",owner,"display_timeout",VariantArray::new_shared(),0);
        owner.add_child(timer,true);
        timer.start(3.0);

        let _ = owner.connect("mouse_entered",owner,"hover",VariantArray::new_shared(),0);
        let _ = owner.connect("mouse_exited",owner,"unhover",VariantArray::new_shared(),0);
        let _ = owner.connect("clicked",owner,"clicked",VariantArray::new_shared(),0);
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        <Self as LabelButton<NotificationAction>>::process(self,owner,delta);
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
        <Self as Windowed<NotificationAction>>::input(self,owner,event.clone());
    }
    #[method]
    fn hover(&mut self){
        <Self as Windowed<NotificationAction>>::hover(self);
    }
    #[method]
    fn unhover(&mut self){
        <Self as Windowed<NotificationAction>>::unhover(self);
    }
    #[method]
    fn clicked(&self,#[base] owner:TRef<Control>){
        let entity_list = unsafe{self.entity_list.assume_safe()};
        let _ = entity_list.map_mut(|_,control| control.set_visible(!control.is_visible()));
        let timer = unsafe{self.timer.assume_safe()};
        self.toggle_timer();
    }
    #[method]
    fn toggle_timer(&self){
        let timer = unsafe{self.timer.assume_safe()};
        if timer.is_stopped(){
            timer.start(3.0);
        }else{
            timer.stop();
        }
    }
    #[method]
    fn display_timeout(&self){
        let _ = self.tx.send(NotificationAction::remove_displayed(self.id));
    }
    fn set_status(&self,value:bool){
        if value{
            self.set_font_outline(Color::from_rgba(0.0,255.0,0.0,1.0),Self::OUTLINE_SIZE);
        }else{
            self.set_font_outline(Color::from_rgba(255.0,0.0,0.0,1.0),Self::OUTLINE_SIZE);
        }
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_window_signals)]
pub struct Notifications{
    tx:Sender<NotificationAction>,
    rx:Receiver<NotificationAction>,
    bg_rect:Ref<ColorRect>,
    main_rect:Ref<ColorRect>,
    label:Ref<Label>,
    hovering:bool,
    notifications:Vec<Instance<Notification>>,
    expanded:Option<i64>,
    displayed:HashSet<i64>,
}
impl Instanced<Control> for Notifications{
    fn make() -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<NotificationAction>();
        Notifications{
            tx:tx,
            rx:rx,
            bg_rect:ColorRect::new().into_shared(),
            main_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            hovering:false,
            notifications:Vec::new(),
            expanded:None,
            displayed:HashSet::new(),
        }
    }
}
impl Windowed<NotificationAction> for Notifications{
    const BG_HIGHLIGHT_COLOR:Color = Color{r:255.0,g:255.0,b:255.0,a:1.0};
    const BG_COLOR:Color = Color{r:0.0,g:0.0,b:0.0,a:1.0};
    const MAIN_COLOR:Color = Color{r:255.0,g:0.0,b:0.0,a:1.0};
    const MARGIN_SIZE:f32 = 5.0;
    fn hovering(&self) -> bool {self.hovering}
    fn set_hovering(&mut self,value:bool){self.hovering = value}
    fn centering(&self) -> Centering {Centering::center}
    fn tx(&self) -> &Sender<NotificationAction> {&self.tx}
    fn bg_rect(&self) -> &Ref<ColorRect>{&self.bg_rect}
    fn main_rect(&self) -> &Ref<ColorRect>{&self.main_rect}
}
impl LabelButton<NotificationAction> for Notifications{
    fn label(&self) -> &Ref<Label>{&self.label}
}
#[methods]
impl Notifications{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        <Self as Windowed<NotificationAction>>::ready(self,owner);
        let _ = owner.connect("mouse_entered",owner,"hover",VariantArray::new_shared(),0);
        let _ = owner.connect("mouse_exited",owner,"unhover",VariantArray::new_shared(),0);
        owner.set_visible(true);
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Control>,delta:f64){
        <Self as Windowed<NotificationAction>>::process(self,owner,delta);
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
        let num_connections = std::cmp::max(self.displayed.len(),1) as f32;
        let panel_size = Vector2::new(owner_size.x,owner_size.y/num_connections);
        let mut idx = 0.0;
        for id in &self.displayed{
            let notification = &self.notifications[*id as usize];
            let notification = unsafe{notification.assume_safe()};
            let _ = notification.map(|_,control| control.set_size(panel_size,false));
            let _ = notification.map(|_,control| control.set_position(Vector2::new(0.0,panel_size.y * idx),false));
            idx += 1.0;
        }

    }
    fn handle_sizing_if_expanded(&self,owner:TRef<Control>,expanded:i64){
        let notification = &self.notifications[expanded as usize];
        let notification = unsafe{notification.assume_safe()};
        let _ = notification.map(|_,control| control.set_size(owner.size(),false));
        let _ = notification.map(|_,control| control.set_position(Vector2::new(0.0,0.0),false));
    }
    fn handle_panel_rx(&mut self,owner:TRef<Control>,delta:f64){
        match self.rx.try_recv(){
            Ok(NotificationAction::toggle_expand(id)) => {
                match self.expanded{
                    Some(curr_id) if id == curr_id => {
                        for notification in &self.notifications{
                            let notification = unsafe{notification.assume_safe()};
                            let _ = notification.map(|obj,control|{control.set_visible(true)});
                        }
                        self.expanded = None;
                    }
                    Some(_) => assert!(false),
                    None => {
                        for notification in &self.notifications{
                            let notification = unsafe{notification.assume_safe()};
                            let _ = notification.map(|obj,control|if obj.id != id{control.set_visible(false) });
                        }
                        self.expanded = Some(id);
                    }
                }
            }
            Ok(NotificationAction::remove_displayed(id)) => {
                let notification = &self.notifications[id as usize];
                let notification = unsafe{notification.assume_safe()};
                let _ = notification.map(|_,control| control.set_visible(false));
                self.displayed.remove(&id);
            }
            Ok(NotificationAction::unhandled) => {}
            Err(_) => {}
        }
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control>,event:Ref<InputEvent>){
        <Self as Windowed<NotificationAction>>::input(self,owner,event.clone());
        if let Ok(event) = event.try_cast::<InputEventKey>(){
            let event = unsafe{event.assume_safe()};
            if event.is_action_released("server_stats_toggle",true){
                owner.set_visible(!owner.is_visible());
            }
        }
    }
    #[method]
    fn hover(&mut self){
        <Self as Windowed<NotificationAction>>::hover(self);
    }
    #[method]
    fn unhover(&mut self){
        <Self as Windowed<NotificationAction>>::unhover(self);
    }
    #[method]
    fn add_notification(&mut self,#[base] owner:TRef<Control>,entities:Vec<String>) -> Result<(),Error>{
        let id = self.notifications.len() as i64;
        let config = NotificationConfig::new(id,&entities,Some(self.tx.clone()));
        let notification = Notification::make_instance(&config).into_shared(); 
        self.notifications.push(notification.clone());
        self.displayed.insert(id);
        owner.add_child(notification,true);
        Ok(())
    }

    #[method]
    fn set_status(&self,id:i64,value:bool){
        let notification = &self.notifications[id as usize];
        let notification = unsafe{notification.assume_safe()};
        let _ = notification.map(|obj,_| obj.set_status(value));
    }

}
#[derive(Clone)]
pub enum Error{
    AddNotificationError,
}
impl Into<u8> for Error{
    fn into(self) -> u8{
        match self{
            Error::AddNotificationError => 1 
        }
    }
}
impl ToVariant for Error{
    fn to_variant(&self) -> Variant{
        match self{
            Error::AddNotificationError => Variant::new(Into::<u8>::into(self.clone()))
        }
    }
}
