extends Node

func _ready():
	ClientTerminalGlobalSignals.connect("request_data",self,"send_data_to_terminal")


func send_data_to_terminal(data_type):
	match data_type:
		ClientTerminalGlobalSignals.StreamDataType.fps:
			var ticks = str(OS.get_ticks_msec())
			ClientTerminalGlobalSignals.add_graph_data("fps_" + ticks,Engine.get_frames_per_second())
			

