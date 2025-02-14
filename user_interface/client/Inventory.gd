extends Node

class_name Inventory

onready var inventory_menu = load("res://native_lib/InventoryMenu.gdns").new()
onready var pocket = load("res://native_lib/Pocket.gdns").new()
onready var parent = get_parent()

func _ready():
	parent.add_child(inventory_menu)
	parent.add_child(pocket)
	GlobalSignalsClient.connect("item_added",inventory_menu,"fill_client_slot")
	GlobalSignalsClient.connect("item_removed",inventory_menu,"remove_client_item")
	GlobalSignalsClient.connect("inventory",self,"refresh_contents")

func refresh_contents(id,contents):
	inventory_menu.clear();
	for item in contents:
		inventory_menu.fill_slot(int(item),int(contents[item]))

	
func pocket_item(client_id,item,amount):
	pass
