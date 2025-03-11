extends Node
class_name ServerConsoleEnv

onready var server_console = load("res://native_lib/ServerConsole.gdns").new()
onready var par = get_parent()

var players = {}
var queued_updates = []
func _ready():
	assert(par!=null,"parent is null")
	par.add_child(server_console)
	GlobalSignalsServer.connect("player_created",self,"add_player_chart")
	server_console.connect("request_location",self,"request_location")

func add_player_chart(id,player):
	server_console.add_player(id)
	players[id] = player

func request_location(id:String):
	if players.has(id) and players[id] != null:
		var loc = players[id].body.global_transform.origin
		queued_updates.push_back([id,loc])
		

func _process(delta):
	if not queued_updates.empty():
		match queued_updates.pop_front():
			[var id,var loc]:
				server_console.update_location(id,loc)
			_:
				assert(false)
