extends Node

#onready var physics_native_shared_socket = load("res://native_lib/SharedRuntime.gdns").new()
onready var physics_native_shared_socket = load("res://native_lib/SharedRuntimeBytes.gdns").new()

func _ready():
	var url = NetworkConfig.physics_host
	physics_native_shared_socket.set_url(url)
	self.add_child(physics_native_shared_socket)
	ClientTerminalGlobalSignals.connect("request_data",self,"send_requested_data")

func send_requested_data(data_type):
	match data_type:
		ClientTerminalGlobalSignals.StreamDataType.bytes_received_all:
			ClientTerminalGlobalSignals.add_graph_data("bytes_received_total_mb_" + str(OS.get_ticks_msec()) ,float(physics_native_shared_socket.num_all_bytes_received())/1000000.0)
		_:
			pass

