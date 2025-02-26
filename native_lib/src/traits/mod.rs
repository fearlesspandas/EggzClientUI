use gdnative::prelude::*;
use tokio::{
    runtime::Runtime,
};

//useful for making sure we can retrieve all values of an enum
pub trait GetAll{
    fn get_all() -> Vec<Self> where Self:Sized;
}
//matches enum type variant to an autocomplete function
pub trait Autocomplete{
    fn auto_complete(&self) -> fn(Vec<&str>) -> Vec<String>;
}
//handles creation of typed args as well as expects an autocomplete function implementation
pub trait FromArgs<T>{
    fn new(args:&T) -> Result<Self,&'static str> where Self:Sized;
    fn autocomplete_args(args:Vec<&str>) -> Vec<String>;
}
pub trait CreateSignal<T>{
    fn register(builder:&ClassBuilder<T>);
}
pub trait EmitSignal{
    fn emit_signal<T>(&self, owner:T);
}
pub trait Instanced<T>{
    fn make() -> Self where Self:Sized;
    fn new(_base:&T) -> Self where Self:Sized{
        Self::make()
    }
    fn make_instance() -> Instance<Self,Unique> 
        where Self:Sized,Self:NativeClass, 
        <Self as gdnative::prelude::NativeClass>::Base: Instanciable
    {
        Instance::emplace(Self::make())
    }
}
pub trait Defaulted{
    fn default() -> Self;
}
pub trait InstancedDefault<T,A:Defaulted>{
    fn make(args:&A) -> Self where Self:Sized;
    fn new(_base:&T) -> Self where Self:Sized{
        Self::make(&Defaulted::default())
    }
    fn make_instance(args:&A) -> Instance<Self,Unique> 
        where Self:Sized,Self:NativeClass, 
        <Self as gdnative::prelude::NativeClass>::Base: Instanciable
    {
        Instance::emplace(Self::make(args))
    }
}
pub trait RuntimeInstanced<T>{
    fn make(runtime:&Runtime) -> Self where Self:Sized;
    fn new(_base:&T) -> Self where Self:Sized{
        Self::make(&Runtime::new().unwrap())
    }
    fn make_instance(runtime:&Runtime) -> Instance<Self,Unique> 
        where Self:Sized,Self:NativeClass, 
        <Self as gdnative::prelude::NativeClass>::Base: Instanciable
    {
        Instance::emplace(Self::make(runtime))
    }
}


#[derive(NativeClass)]
#[inherit(Control)]
pub struct TestClassInstance{}
impl Instanced<Control> for TestClassInstance{
    fn make() -> Self where Self:Sized{
        TestClassInstance{}
    }
}
#[methods]
impl TestClassInstance{
    #[method]
    fn _ready(&self,#[base]owner:TRef<Control>){
        TestClassInstance::new(&owner);
        TestClassInstance::make_instance();
    }
}
