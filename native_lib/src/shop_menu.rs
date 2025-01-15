use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{CreateSignal,Instanced,InstancedDefault,Defaulted};
use tokio::sync::mpsc;

type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

impl <T> Defaulted for Sender<T>{
    fn default() -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<T>();
        tx
    }
}

trait ToName{
    fn to_name(&self) -> String;
}
trait ToDescription{
    fn to_description(&self) -> String;
}
#[derive(Clone)]
enum ItemType{
    Empty,
    Smack,
    GlobTeleport,
}
impl ToName for ItemType{
    fn to_name(&self) -> String{
        match self{
            ItemType::Empty => "Empty".to_string(),
            ItemType::Smack => "Smack".to_string(),
            ItemType::GlobTeleport => "GlobTeleport".to_string(),
        }
    }
}
impl ToDescription for ItemType{
    fn to_description(&self) -> String{
        match self{
            ItemType::Empty => "Placeholder item slot".to_string(),
            ItemType::Smack => "small explosion that does 10 damage".to_string(),
            ItemType::GlobTeleport => "creates polygon that teleports entities to an anchor point".to_string(),
        }
    }
}
impl Into<u8> for ItemType{
    fn into(self) -> u8 {
        match self{
            ItemType::Smack => 0,
            ItemType::GlobTeleport => 1,
            ItemType::Empty => 255,
        }
    }
}
impl From<u8> for ItemType{
    fn from(value:u8) -> Self{
        match value{
            0 => ItemType::Smack,
            1 => ItemType::GlobTeleport,
            255 => ItemType::Empty,
            _ => todo!(),
        }
    }
}
enum Command{
    AddItem(ItemType),
    BuyItem(ItemType),
    SellItem(ItemType),
    ClearShop,
}
#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct MenuButton{
    bg_rect:Ref<ColorRect>,
    display_rect:Ref<ColorRect>,
    label:Ref<Label>,
    bg_color:Color,
    display_color:Color,
    hovering:bool,
}
impl Instanced<Control> for MenuButton{
    fn make() -> Self{
        MenuButton{
            bg_rect:ColorRect::new().into_shared(),
            display_rect:ColorRect::new().into_shared(),
            label:Label::new().into_shared(),
            bg_color:Color{r:75.0,g:200.0,b:200.0,a:1.0},
            display_color:Color{r:0.0,g:0.0,b:0.0,a:1.0},
            hovering:false,
        }
    }
}
#[methods]
impl MenuButton{
    fn register_signals(builder:&ClassBuilder<Self>){
        builder.signal("clicked").done();
    }
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>) {
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        let display_rect = unsafe{self.display_rect.assume_safe()};
        let label = unsafe{self.label.assume_safe()};
        bg_rect.set_frame_color(self.bg_color);
        display_rect.set_frame_color(self.display_color);
        owner.add_child(bg_rect,true);
        owner.add_child(display_rect,true);
        owner.add_child(label,true);
        owner.connect("mouse_entered",owner,"hover",VariantArray::new_shared(),0);
        owner.connect("mouse_exited",owner,"unhover",VariantArray::new_shared(),0);
        bg_rect.set_mouse_filter(control::MouseFilter::IGNORE.into());
        display_rect.set_mouse_filter(control::MouseFilter::IGNORE.into());
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        let display_rect = unsafe{self.display_rect.assume_safe()};
        let label = unsafe{self.label.assume_safe()};
        //sizing and positioning
        let bg_offset = Vector2{x:10.0,y:10.0};
        //bg_rect
        bg_rect.set_size(owner.size(),false);
        bg_rect.set_position(Vector2{x:bg_offset.x/-2.0,y:0.0},false);
        //display_rect
        display_rect.set_size(bg_rect.size() - bg_offset,false);
        display_rect.set_position(bg_rect.position() + bg_offset/2.0,false);
        //label
        label.set_size(bg_rect.size()/2.0,false);
        let label_position = bg_rect.size()/2.0 - (label.size()/2.0) ;
        label.set_position(label_position,false);
    }
    #[method]
    fn _input(&self,#[base] owner:TRef<Control>,event:Ref<InputEvent>){
        if let Ok(event) = event.try_cast::<InputEventMouseButton>(){
            let event = unsafe{event.assume_safe()};
            if event.is_action_released("left_click",true) && self.hovering{
                owner.emit_signal("clicked",&[]);
            }
        }
    }
    #[method]
    fn hover(&mut self,#[base] owner:TRef<Control>){
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        bg_rect.set_frame_color(Color{r:255.0,g:255.0,b:255.0,a:1.0});
        self.hovering = true;
    }
    #[method]
    fn unhover(&mut self,#[base] owner:TRef<Control>){
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        bg_rect.set_frame_color(self.bg_color);
        self.hovering = false;
    }
    #[method]
    fn set_label_text(&self,text:String){
        let label = unsafe{self.label.assume_safe()};
        label.set_text(text);
    }
    #[method]
    fn set_bg_color(&mut self,color:Color){
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        bg_rect.set_frame_color(color);
        self.bg_color = color;
    }
    #[method]
    fn set_display_color(&mut self,color:Color){
        let display_rect = unsafe{self.display_rect.assume_safe()};
        display_rect.set_frame_color(color);
        self.display_color = color;
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
pub struct ShopItem{
    item_type:ItemType,
    bg_rect:Ref<ColorRect>,
    display_rect:Ref<ColorRect>,
    name:Ref<Label>,
    description:Ref<Label>,
    buy_button:Instance<MenuButton>,
    sell_button:Instance<MenuButton>,
    menu_tx:Sender<Command>,
    color:Color,
    hovering:bool,
}
impl InstancedDefault<Control,Sender<Command>> for ShopItem{
    fn make(args:&Sender<Command>) -> Self{
        ShopItem{
            item_type:ItemType::Empty,
            bg_rect: ColorRect::new().into_shared(),
            display_rect: ColorRect::new().into_shared(),
            name:Label::new().into_shared(),
            description:Label::new().into_shared(),
            buy_button:MenuButton::make_instance().into_shared(),
            sell_button:MenuButton::make_instance().into_shared(),
            menu_tx:args.clone(),
            color:Color{r:0.0,g:255.0,b:255.0,a:1.0},
            hovering:false,
        }
    }
}
#[methods]
impl ShopItem{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        let display_rect = unsafe{self.display_rect.assume_safe()};
        let name = unsafe{self.name.assume_safe()};
        let description = unsafe{self.description.assume_safe()};
        let buy_button = unsafe{self.buy_button.assume_safe()};
        let sell_button = unsafe{self.sell_button.assume_safe()};
        //initialize
        bg_rect.set_frame_color(Color{r:0.0,g:255.0,b:255.0,a:1.0});
        display_rect.set_frame_color(Color{r:0.0,g:0.0,b:0.0,a:1.0});
        name.set_text(self.item_type.to_name());
        description.set_text(self.item_type.to_description());
        buy_button.map_mut(|obj,control| {
            obj.set_bg_color(Color{r:75.0,g:0.0,b:100.0,a:1.0});
            obj.set_label_text("Buy".to_string());
        });
        buy_button.map(|_,control| control.connect("clicked",owner,"buy_item",VariantArray::new_shared(),0));
        sell_button.map_mut(|obj,control| {
            obj.set_bg_color(Color{r:75.0,g:0.0,b:100.0,a:1.0});
            obj.set_label_text("Sell".to_string());
        });
        sell_button.map(|_,control| control.connect("clicked",owner,"sell_item",VariantArray::new_shared(),0));
        //add children
        owner.add_child(bg_rect,true);
        owner.add_child(display_rect,true);
        owner.add_child(name,true);
        owner.add_child(description,true);
        owner.add_child(buy_button,true);
        owner.add_child(sell_button,true);
        //connect signals
        owner.connect("mouse_entered",owner,"hover",VariantArray::new_shared(),0);
        owner.connect("mouse_exited",owner,"unhover",VariantArray::new_shared(),0);
        bg_rect.set_mouse_filter(control::MouseFilter::IGNORE.into());
        display_rect.set_mouse_filter(control::MouseFilter::IGNORE.into());
    }
    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        let display_rect = unsafe{self.display_rect.assume_safe()};
        let name = unsafe{self.name.assume_safe()};
        let description = unsafe{self.description.assume_safe()};
        let buy_button = unsafe{self.buy_button.assume_safe()};
        let sell_button = unsafe{self.sell_button.assume_safe()};
        //bg_rect
        bg_rect.set_size(owner.size(),false);
        let bg_offset = Vector2{x:10.0,y:10.0};
        //display_rect
        display_rect.set_size(bg_rect.size() - bg_offset,false);
        display_rect.set_position(bg_offset/2.0,false);
        //name label
        name.set_size(display_rect.size()/2.0,false);
        name.set_position(display_rect.position(),false);
        //description label
        description.set_size(display_rect.size()/2.0,false);
        description.set_position(display_rect.position() + Vector2{x:0.0,y:name.size().y},false);
        //buy button
        let button_size = owner.size()/2.0;
        buy_button.map(|_,control| {
            let position = Vector2{x:button_size.x,y:bg_offset.y/2.0};
            control.set_size(button_size,false);
            control.set_position(position,false);
        });
        //sell button
        sell_button.map(|_,control| {
            let position = owner.size() - button_size - Vector2{x:0.0,y:bg_offset.y/2.0} ;
            control.set_size(button_size,false);
            control.set_position(position,false);
        });
    }
    #[method]
    fn buy_item(&self){
        let item_type = self.item_type.clone();
        self.menu_tx.send(Command::BuyItem(item_type));
    }
    #[method]
    fn sell_item(&self){
        let item_type = self.item_type.clone();
        self.menu_tx.send(Command::SellItem(item_type));
    }

    #[method]
    fn hover(&mut self,#[base] owner:TRef<Control>){
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        bg_rect.set_frame_color(Color{r:255.0,g:255.0,b:255.0,a:1.0});
        self.hovering = true;
    }
    #[method]
    fn unhover(&mut self,#[base] owner:TRef<Control>){
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        bg_rect.set_frame_color(Color{r:0.0,g:255.0,b:255.0,a:1.0});
        self.hovering = false;
    }
}
#[derive(NativeClass)]
#[inherit(Control)]
#[register_with(Self::register_signals)]
pub struct ShopMenu{
    client_id:Option<String>,
    bg_rect:Ref<ColorRect>,
    items:Vec<Instance<ShopItem>>,
    tx:Sender<Command>,
    rx:Receiver<Command>,
}
impl Instanced<Control> for ShopMenu{
    fn make() -> Self{
        let (tx,rx) = mpsc::unbounded_channel::<Command>();
        ShopMenu{
            client_id:None,
            bg_rect: ColorRect::new().into_shared(),
            items: Vec::new(),
            tx:tx,
            rx:rx,
        }
    }
}
enum Signals{
    buy,
    sell
}
impl ToString for Signals{
    fn to_string(&self) -> String{
        match self{
            Signals::buy => "buy".to_string(),
            Signals::sell => "sell".to_string(),
        }
    }
}
impl CreateSignal<ShopMenu> for Signals{
    fn register(builder:&ClassBuilder<ShopMenu>) {
        builder.signal(&Signals::buy.to_string())
            .with_param("client_id",VariantType::GodotString)
            .with_param("item_type",VariantType::I64)
            .done();
        builder.signal(&Signals::sell.to_string())
            .with_param("client_id",VariantType::GodotString)
            .with_param("item_type",VariantType::I64)
            .done();
    }
}
#[methods]
impl ShopMenu{
    fn register_signals(builder:&ClassBuilder<Self>){
        Signals::register(builder);
    }
    #[method]
    fn _ready(&mut self,#[base] owner:TRef<Control>){
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        bg_rect.set_frame_color(Color{r:255.0,g:255.0,b:255.0,a:1.0});
        owner.add_child(bg_rect,true);
        owner.set_visible(false);
        //self.tx.send(Command::AddItem(ItemType::Smack));
        //self.tx.send(Command::AddItem(ItemType::Empty));
        //self.add_item(0);
        //self.add_item(1);
        //self.clear();

        //self.add_item(255);
    }
    #[method]
    fn clear_from_id(&self,id:String){
        self.client_id.clone().map(|c_id| {
            if c_id == id{
                self.tx.send(Command::ClearShop);
            }
        });
    }
    #[method]
    fn clear(&self){
        godot_print!("Clearing shop");
        self.tx.send(Command::ClearShop);
    }
    #[method]
    fn add_item_from_id(&mut self,id:String,item_type:u8){
        self.client_id.clone().map(|c_id| {
            if c_id == id {
                self.tx.send(Command::AddItem(ItemType::from(item_type)));
            }
        });
    }
    #[method]
    fn add_item(&mut self,item_type:u8){
        godot_print!("item added");
        self.tx.send(Command::AddItem(ItemType::from(item_type)));
    }
    fn add_item_type(&mut self,item_type:ItemType){
        self.tx.send(Command::AddItem(item_type));
    }
    #[method]
    fn _process(&mut self,#[base] owner:TRef<Control>,delta:f64){
        match self.rx.try_recv(){
            Ok(Command::AddItem(typ)) => {
                let item = ShopItem::make_instance(&self.tx).into_shared();
                let item_obj = unsafe{item.assume_safe()};
                item_obj.map_mut(|obj,_|obj.item_type = typ);
                owner.add_child(item.clone(),true);
                self.items.push(item);
            }
            Ok(Command::BuyItem(typ)) => {
                let typ_label:u8 = typ.into();
                self.client_id.as_ref().map(|id|
                    owner.emit_signal(Signals::buy.to_string(),&[Variant::new(id),Variant::new(typ_label.clone())])
                );
                godot_print!("{}",format!("Buying Item {typ_label:?}"));
            }
            Ok(Command::SellItem(typ)) => {
                let typ_label:u8 = typ.into();
                self.client_id.as_ref().map(|id|
                    owner.emit_signal(Signals::sell.to_string(),&[Variant::new(id),Variant::new(typ_label.clone())])
                );
                godot_print!("{}",format!("Selling Item {typ_label:?}"));
            }
            Ok(Command::ClearShop) => {
                for item in &self.items{
                    owner.remove_child(item);
                    let item = unsafe{item.assume_safe()};
                    item.map(|_, control| control.queue_free());
                    
                }
                self.items.clear();
            }
            Err(_) => {}
        } 
        //sizing and positioning
        let vp = owner.get_viewport().unwrap();
        let vp  = unsafe{vp.assume_safe()};
        let size = vp.get_visible_rect().size;
        owner.set_size(size/2.0,true);
        //background
        let bg_rect = unsafe{self.bg_rect.assume_safe()};
        bg_rect.set_size(owner.size(),false);
        //items
        let mut idx = 0.0;
        let owner_size = owner.size();
        let num_items = self.items.len() as f32;
        let item_size = Vector2{x:owner_size.x,y:owner_size.y/num_items};
        for item in &self.items{
            let item = unsafe{item.assume_safe()};
            item.map(|obj,control| {
                control.set_size(item_size,false);
                let position = Vector2{x:0.0,y:item_size.y * idx};
                control.set_position(position,false);
                idx += 1.0;
            });
        }
    }

    #[method]
    fn _input(&self,#[base] owner:TRef<Control>, event:Ref<InputEvent>){
        if let Ok(event) = event.try_cast::<InputEventKey>(){
            let event = unsafe{event.assume_safe()};
            if event.is_action_released("shop_menu_toggle",false){
                owner.set_visible(!owner.is_visible());
            }
        }
    }

    #[method]
    fn set_client_id(&mut self,new_id:String){
        self.client_id = Some(new_id)
    }
}
