extends Node

var assets = {
	0 : "res://entity/Player.tscn",
	1 : "res://entity/server/ServerEntity.tscn",
	2 : "res://world/SpawnBlockFrame.tscn",
	3 : "res://world/client/SpawnBlockNoPlayer.tscn",
	4 : "res://entity/client/NonPlayerControlledEntity.tscn",
	5 : "res://user_interface/client/overheads/Chicago.ttf",
	6 : "res://world/BlockTerrain.tscn",
	7 : "res://world/client/BlockTerrainMesh.tscn",
	8 : "res://world/client/SpawnBlockMesh.tscn",
	9 : "res://world/SpawnFrame.tscn",
	10: "res://world/EmptySpawnSpace.tscn",
	11 : "res://world/HealthStars.tscn",
	12 : "res://world/client/HealthStarMesh.tscn",
}
var asset_resources = {}

var mesh = {
	6:7,
	9:8,
	11:12
}

enum {
	player_model = 0,
	server_player_model = 1,
	server_spawn = 10,
	client_spawn = 3,
	npc_model = 4,
	username_font = 5
	}

func _ready():
	for k in assets.keys():
		asset_resources[k] = load(assets[k])
		
func matchAsset(id:int) -> Resource:
	if assets.has(id):
		#var path:String = assets[id]
		var res:Resource = asset_resources[id]
		return res #load(path)
	else:
		return null
		
func matchPath(id:int) -> String:
	if assets.has(id):
		return assets[id]
	else:
		return ""

func matchMesh(id:int) -> Resource:
	if mesh.has(id):
		var mesh_id = mesh[id]
		return matchAsset(mesh_id)
	else:
		return null
