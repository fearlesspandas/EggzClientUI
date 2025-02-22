

use std::collections::HashMap;
use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{CreateSignal,Instanced,InstancedDefault,Defaulted};
use tokio::sync::mpsc;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct Tile{
    typ:TileType,
    symbol:Ref<Line2D>,
    points:Vec<Vector2>,
    color:Color,
}

impl Instanced<Control> for Tile{
    fn make() -> Self{
        Tile{
            typ:TileType::empty,
            symbol:Line2D::new().into_shared(),
            points:Vec::new(),
            color:Color{r:255.0,g:255.0,b:255.0,a:255.0},
        }
    }
}
#[methods]
impl Tile{
    #[method]
    fn _ready(&self,#[base] owner:TRef<Control>){
        let symbol = unsafe{self.symbol.assume_safe()};
        symbol.set_width(1.0);
        symbol.set_default_color(self.color);
        owner.add_child(symbol,true);
    }

    #[method]
    fn _process(&self,#[base] owner:TRef<Control>,delta:f64){
        let symbol = unsafe{self.symbol.assume_safe()};
        symbol.clear_points();
        for point in &self.points{
            let owner_size = owner.size();
            let point = Vector2{x:point.x * owner_size.x,y:point.y * owner_size.y};
            symbol.add_point(point,0);
        }
    }
    pub fn set_type(&mut self,typ:TileType){
        self.points = typ.to_points();
        self.typ = typ;
    }
    pub fn set_color(&mut self,color:Color){
        self.color = color;
        let symbol = unsafe{self.symbol.assume_safe()};
        symbol.set_default_color(self.color);
    }

}
pub enum TileType{
    empty,
    down_arrow,
}
impl TileType{
    pub fn to_points(&self) -> Vec<Vector2>{
        match self{
            TileType::down_arrow => {
                let mut v = Vec::new();
                v.push(Vector2{x:0.25,y:0.0});
                v.push(Vector2{x:0.25,y:0.5});
                v.push(Vector2{x:0.0,y:0.5});
                v.push(Vector2{x:0.5,y:1.0});
                v.push(Vector2{x:1.0,y:0.5});
                v.push(Vector2{x:0.75,y:0.5});
                v.push(Vector2{x:0.75,y:0.0});
                v
            }
            TileType::empty => todo!()
        }
    }
}
