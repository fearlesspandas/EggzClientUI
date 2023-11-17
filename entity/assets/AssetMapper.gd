extends Node

var assets = {
	0 : preload("res://entity/Player.tscn"),
	1 : preload("res://entity/server/ServerEntity.tscn"),
	2 : preload("res://world/SpawnBlockFrame.tscn"),
	3 : preload("res://world/client/SpawnBlockNoPlayer.tscn"),
	4 : preload("res://entity/client/NonPlayerControlledEntity.tscn")
}

var player_model = 0

var server_player_model = 1

var server_spawn = 2

var client_spawn = 3

var npc_model = 4

func matchAsset(id:int) -> Resource:
	return assets[id]

