extends Node

var _terminal
func register_terminal(terminal):
	terminal.connect("set_entity_socket_mode",self,"set_entity_socket_mode")
	terminal.connect("set_all_entity_socket_mode",self,"set_all_entity_socket_mode")
	terminal.connect("set_active",self,"set_active")
	terminal.connect("start_data_stream",self,"start_data_stream")
	terminal.connect("request_data",self,"request_data")
	_terminal = terminal

####SocketModes#######
enum SocketMode{
	Native,
	NativeProcess,
	GodotClient,
	NativeLocOnly,
	NativeDirOnly,	
}
signal set_entity_socket_mode(id,mode)
func set_entity_socket_mode(id,mode):
	emit_signal("set_entity_socket_mode",id,SocketMode.get(mode))

signal set_all_entity_socket_mode(mode)
func set_all_entity_socket_mode(mode):
	emit_signal("set_all_entity_socket_mode",SocketMode.get(mode))

####SetActive########
signal set_active(value)
func set_active(value):
	emit_signal("set_active",value)

#####DataStreams####
enum StreamDataType{
	socket_mode,
	linear_velocity,
	global_position,
	requests_sent,
	responses_received,
	request_response_delta,
	fps,
}
signal start_data_stream(data_type)
func start_data_stream(data_type):
	emit_signal("start_data_stream",StreamDataType.get(data_type))

signal request_data(data_type)
func request_data(data_type):
	emit_signal("request_data",StreamDataType.get(data_type))

func add_input_data(tag,data):
	_terminal.add_incoming_data(tag,data)

func add_graph_data(tag,data):
	_terminal.add_graph_data(tag,data)

