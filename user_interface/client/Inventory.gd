extends Node

class_name Inventory

onready var inventory_menu = load("res://native_lib/InventoryMenu.gdns").new()
onready var pocket = load("res://native_lib/Pocket.gdns").new()
onready var parent = get_parent()

onready var init_timer:Timer = Timer.new()

var client_id:String = ""

func _ready():
	assert(client_id != null and client_id.length() > 0)
	parent.add_child(inventory_menu)
	parent.add_child(pocket)
	
	GlobalSignalsClient.connect("item_added",self,"add_item")
	GlobalSignalsClient.connect("item_removed",inventory_menu,"remove_client_item")
	GlobalSignalsClient.connect("pocketed_item",self,"pocketed_item")
	GlobalSignalsClient.connect("unpocketed_item",self,"unpocketed_item")

	GlobalSignalsClient.connect("inventory",self,"refresh_contents")
	GlobalSignalsClient.connect("pocket",self,"refresh_pocket_contents")

	inventory_menu.connect("pocketed",self,"pocket_item")
	inventory_menu.connect("unpocketed",self,"unpocket_item")
	inventory_menu.connect("lift",self,"remove_field_ability")

	pocket.connect("unpocketed",self,"unpocket_item")
	pocket.connect("lift",self,"remove_field_ability")

	self.init_timer.wait_time = 1.5
	self.init_timer.connect("timeout",self,"init_requests")
	self.add_child(self.init_timer)
	self.init_timer.start()

func init_requests():
	self.init_timer.one_shot = true
	var socket = ServerNetwork.get(client_id)
	socket.get_pocket(client_id)

func remove_field_ability(item:int):
	ServerNetwork.get(client_id).remove_ability(client_id,item)

func refresh_contents(id,contents):
	inventory_menu.clear();
	for item in contents:
		inventory_menu.fill_slot(int(item),int(contents[item]))

func refresh_pocket_contents(id,contents):
	pocket.clear();
	for item in contents:
		pocket.fill_slot(int(item),int(contents[item]))

func add_item(id,ability_id):
	if self.client_id == id:
		inventory_menu.fill_slot(ability_id,1)
	else:
		assert(false)

func pocketed_item(client_id,item,amount):
	if client_id == self.client_id:
		pocket.fill_slot(item,amount)

func unpocketed_item(client_id,item,amount):
	if client_id == self.client_id:
		pocket.remove_item(item,amount)

func pocket_item(item,amount):
	ServerNetwork.get(client_id).pocket_ability(client_id,item,amount)

func unpocket_item(item,amount):
	ServerNetwork.get(client_id).unpocket_ability(client_id,item,amount)
