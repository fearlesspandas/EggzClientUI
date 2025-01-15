extends Node


#onready var shop_menu = load("res://native_lib/ShopMenu.gdns").new()

func buy_ability(client_id,ability_id):
	var socket:ClientWebSocket = ServerNetwork.get(client_id)
	socket.buy_item(client_id,ability_id)
 
func sell_ability(client_id,ability_id):
	var socket:ClientWebSocket = ServerNetwork.get(client_id)
	socket.sell_item(client_id,ability_id)
 
func _ready():
	#self.add_child(shop_menu)
	pass

