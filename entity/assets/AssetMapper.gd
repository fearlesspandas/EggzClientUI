extends Node

#assets that are the same between client and server
#i.e. basic collision boxes
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
	13 : "res://entity/server/NPCServerEntity.tscn",
	14 : "res://entity/server/PlayerServerEntity.tscn",
	15 : "res://entity/client/tutorial/AxisSpiderClient.tscn",
	#16 skip (client/server specific)
	17 : "res://entity/server/ProwlerServerEntity.tscn",
	18 : "res://entity/client/ProwlerEntity.tscn",
	19 : "res://entity/client/tutorial/AxisSpider.tscn",
	20 : "res://entity/client/NPCPlayerEntity.tscn",
}

#client specific assets
var client_assets = {
	16:"res://world/client/ProwlerAnchorMesh.tscn"
}

#server specific assets
var server_assets = {
	16:"res://world/ProwlerAnchorServer.tscn",
}
var asset_resources = {}
var client_asset_resources = {}
var server_asset_resources = {}

var mesh = {
	6:7,
	9:8,
	11:12
}

enum {
	player_model = 0,
	server_player_model = 14,
	server_spawn = 10,
	client_spawn = 3,
	npc_model = 4,
	username_font = 5,
	npc_server_model = 13,
	spider = 15,
	prowler_server_entity = 17,
	prowler_client_entity = 18,
	local_spider_entity = 19,
	npc_player_entity = 20,
}

func _ready():
	for k in assets.keys():
		asset_resources[k] = load(assets[k])
	for k in client_assets.keys():
		client_asset_resources[k] = load(client_assets[k])
	for k in server_assets.keys():
		server_asset_resources[k] = load(server_assets[k])

#any specific client/server assets should be in both 
func verify_client_server_assets():
	pass
		
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

func matchClientAsset(id:int) -> Resource:
	if client_assets.has(id):
		return client_asset_resources[id]
	else:
		return matchAsset(id)
	
func matchServerAsset(id:int) -> Resource:
	if server_assets.has(id):
		return server_asset_resources[id]
	else:
		return matchAsset(id)
