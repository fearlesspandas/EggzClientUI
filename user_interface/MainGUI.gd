extends Node
export var spawnWorld :Resource
export var serverWorld: Resource
onready var serverControl = find_node("ServerControl")
#starting server means loading server entities which will automatically
#start updating with message traffic
#similarly starting a client is just instantiating spawn map currenly
func _ready():
	var spawn = load(spawnWorld.resource_path).instance()
	self.add_child(spawn)
	ServerNetwork._ready()
	pass



