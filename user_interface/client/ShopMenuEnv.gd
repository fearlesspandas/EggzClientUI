extends Node


#onready var shop_menu = load("res://native_lib/ShopMenu.gdns").new()

func buy_ability(client_id,ability_id):
	var socket:ClientWebSocket = ServerNetwork.get(client_id)
	socket.buy_item(client_id,ability_id)
 
func sell_ability(client_id,ability_id):
	var socket:ClientWebSocket = ServerNetwork.get(client_id)
	socket.sell_item(client_id,ability_id)
 

#signal clear_inventory(client_id)
#func clear_inventory(client_id):
#	emit_signal("clear_inventory",client_id)

#signal add_to_inventory(client_id,item)
#func add_to_inventory(client_id,item):
#	emit_signal("add_to_inventory",client_id,item)

signal clear_inventory()
func clear_inventory():
	emit_signal("clear_inventory")

signal add_to_inventory(item)
func add_to_inventory(item):
	emit_signal("add_to_inventory",item)

func _ready():
	#self.add_child(shop_menu)
	pass

