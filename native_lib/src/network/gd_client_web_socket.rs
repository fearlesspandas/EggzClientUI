use gdnative::prelude::*;
use gdnative::api::*;
use crate::traits::{Instanced,InstancedArgs};

type Id = String;
type Secret = String;

#[derive(Clone)]
pub struct WebSocketConfig{
    client_id:Id,
    secret:Secret,
}
#[derive(NativeClass)]
#[inherit(WebSocketClient)]
pub struct ClientWebSocket{
    client_id:Id,
    secret:Secret,
    url:Option<String>,
    connected:bool,
}

impl InstancedArgs<WebSocketClient,WebSocketConfig> for ClientWebSocket{
    fn make(args:Option<WebSocketConfig>) -> Self{
        let client_id = args.clone().map(|x|x.client_id).expect("ClientWebSocket: No client_id provided on initialization"); 
        let secret = args.clone().map(|x|x.secret).expect("ClientWebSocket: No secret provided on initialization");
        ClientWebSocket{
            client_id:client_id.clone(),
            secret:secret.clone(),
            url:None,
            connected:false,
        }
    }
}
#[methods]
impl ClientWebSocket{
    #[method]
    fn connect(&self,#[base] owner:TRef<WebSocketClient>) -> Result<(),String>{
        let mut protocols = PoolArray::new();
        let protocol:GodotString = "json".into();
        protocols.push(protocol);
        owner.connect_to_url(
            self.url.clone().expect("no url found during connect"),
            protocols,
            false,
            PoolArray::new()
        ).map_err(|e| "ClientWebSocket:Could not connect ".to_string())
    }
    #[method]
    fn connected(&mut self,#[base] owner:TRef<WebSocketClient>,protocol:String){
        let id = &self.client_id;
        godot_print!("{}",format!("ClientWebSocket: connected with protocol {protocol:?}, for id {id:?}"));
        self.connected = true;
        let peer = owner.get_peer(1).expect("ClientWebSocket: could not get peer");
        let peer = unsafe{peer.assume_safe()};
        peer.set_write_mode(WebSocketPeer::WRITE_MODE_TEXT);
        let _ = peer.put_packet(PoolArray::new());
    }
}
