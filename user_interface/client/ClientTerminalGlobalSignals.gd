extends Node

func register_terminal(terminal):
	terminal.connect("set_entity_socket_mode",self,"set_entity_socket_mode")
	terminal.connect("set_all_entity_socket_mode",self,"set_all_entity_socket_mode")
	terminal.connect("set_active",self,"set_active")

enum SocketMode{
	Native,
	NativeProcess,
	GodotClient,
	
}
signal set_entity_socket_mode(id,mode)
func set_entity_socket_mode(id,mode):
	emit_signal("set_entity_socket_mode",id,SocketMode.get(mode))

signal set_all_entity_socket_mode(id,mode)
func set_all_entity_socket_mode(id,mode):
	emit_signal("set_all_entity_socket_mode",SocketMode.get(mode))

signal set_active(value)
func set_active(value):
	emit_signal("set_active",value)
