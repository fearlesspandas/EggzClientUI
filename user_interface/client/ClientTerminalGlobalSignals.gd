extends Node

enum SocketMode{
	Native,
	NativeProcess,
	GodotClient,
	
}
signal set_entity_socket_mode(id,mode)
func register_terminal(terminal):
	terminal.connect("set_entity_socket_mode",self,"set_entity_socket_mode")

func set_entity_socket_mode(id,mode):
	emit_signal("set_entity_socket_mode",id,SocketMode.get(mode))

