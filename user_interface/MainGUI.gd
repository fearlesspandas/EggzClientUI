extends Node
export var spawnWorld :Resource
export var serverWorld: Resource
export var maincharacter:Resource
export var servercharacter:Resource
onready var clientControl = find_node("ClientControl")
onready var serverControl = find_node("ServerControl")
#starting server means loading server entities which will automatically
#start updating with message traffic
#similarly starting a client is just instantiating spawn map currenly
func _ready():
	var spawn = load(spawnWorld.resource_path).instance()
	var serverSpawn = load(serverWorld.resource_path).instance()
	clientControl.add_child(spawn)
	serverControl.add_child(serverSpawn)
	ServerNetwork._ready()
	var location = Vector3(0,5,0)
	#creates main character in client and adds it to spawn area
	EntityManager.create_entity("1",location,spawn,maincharacter,false)
	EntityManager.create_entity("1",location,serverSpawn,servercharacter,true)
	pass



