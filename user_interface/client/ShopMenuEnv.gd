extends Node


onready var shop_menu = load("res://native_lib/ShopMenu.gdns").new()

func _ready():
	shop_menu.connect("buy",self,"buy_ability")
	shop_menu.connect("sell",self,"sell_ability")
	self.connect("clear_inventory",shop_menu,"clear")
	self.connect("add_to_inventory",shop_menu,"add_item")
	self.connect("show",shop_menu,"show")
	self.connect("hide",shop_menu,"hide")

func set_client_id(id:String):
	assert(id.length() > 0)
	shop_menu.set_client_id(id)

func buy_ability(client_id,ability_id):
	var socket:ClientWebSocket = ServerNetwork.get(client_id)
	socket.buy_item(client_id,ability_id)
 
func sell_ability(client_id,ability_id):
	var socket:ClientWebSocket = ServerNetwork.get(client_id)
	socket.sell_item(client_id,ability_id)
 
signal clear_inventory()
func clear_inventory():
	emit_signal("clear_inventory")

signal add_to_inventory(item)
func add_to_inventory(item):
	emit_signal("add_to_inventory",item)

signal show
func show():
	emit_signal("show")

signal hide
func hide():
	emit_signal("hide")


