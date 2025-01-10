extends Node


onready var shop_menu = load("res://native_lib/ShopMenu.gdns").new()

func _ready():
	self.add_child(shop_menu)

