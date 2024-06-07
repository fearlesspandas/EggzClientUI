extends Node

var assets = {
	0 : "res://entity/Player.tscn",
	1 : "res://entity/server/ServerEntity.tscn",
	2 : "res://world/SpawnBlockFrame.tscn",
	3 : "res://world/client/SpawnBlockNoPlayer.tscn",
	4 : "res://entity/client/NonPlayerControlledEntity.tscn",
	5 : "res://user_interface/client/overheads/Chicago.ttf",
	6 : "res://world/BlockTerrain.tscn",
	7 : "res://world/client/BlockTerrainMesh.tscn"
}

var mesh = {
	2:3,
	6:7
}

enum {
	player_model = 0,
	server_player_model = 1,
	server_spawn = 2,
	client_spawn = 3,
	npc_model = 4,
	username_font = 5
	}

func matchAsset(id:int) -> Resource:
	if assets.has(id):
		var path:String = assets[id]
		return load(path)
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
