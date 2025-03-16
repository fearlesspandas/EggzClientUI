extends Node

#onready var physics_native_shared_socket = load("res://native_lib/SharedRuntime.gdns").new()
onready var physics_native_shared_socket = load("res://native_lib/SharedRuntimeBytes.gdns").new()

func _ready():
	ClientTerminalGlobalSignals.connect("request_data",self,"send_requested_data")
	physics_native_shared_socket.connect("connected",self,"socket_connected")
	physics_native_shared_socket.connect("disconnected",self,"socket_disconnected")

func initialize_sockets():
	var url = NetworkConfig.physics_host
	physics_native_shared_socket.set_url(url)
	self.add_child(physics_native_shared_socket)

func send_requested_data(data_type):
	match data_type:
		ClientTerminalGlobalSignals.StreamDataType.bytes_received_all:
			ClientTerminalGlobalSignals.add_graph_data("bytes_received_total_mb_" + str(OS.get_ticks_msec()) ,float(physics_native_shared_socket.num_all_bytes_received())/1000000.0)
		_:
			pass

func socket_connected(socket_id,entities):
	print_debug("Socket Connected ", socket_id, " for ", str(entities))

func socket_disconnected(socket_id,entities):
	assert(false)

