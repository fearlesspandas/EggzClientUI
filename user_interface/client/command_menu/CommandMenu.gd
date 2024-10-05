extends Control

class_name CommandMenu

onready var bg_rect:ColorRect = ColorRect.new()

onready var request_top_terrain:RequestTopTerrainInDistance = RequestTopTerrainInDistance.new()
onready var toggle_chunk_visibility: ToggleChunkVisibility = ToggleChunkVisibility.new()
onready var get_inventory:GetInventory = GetInventory.new()
onready var get_all_entities:GetAllEntities = GetAllEntities.new()
onready var create_prowler:CreateProwler = CreateProwler.new()
onready var commands = []

var client_id:String

func initialize_commands():
	request_top_terrain.client_id = client_id
	get_inventory.client_id = client_id
	get_all_entities.client_id = client_id
	create_prowler.client_id = client_id
	commands.push_back(request_top_terrain)
	commands.push_back(toggle_chunk_visibility)
	commands.push_back(get_inventory)
	commands.push_back(get_all_entities)
	commands.push_back(create_prowler)
	
func size():
	self.rect_size = OS.window_size
	bg_rect.rect_size = self.rect_size
	for i in range(0,commands.size()):
		var command = commands[i]
		if command is Command:
			command.set_position(Vector2(0,command.rect_size.y * i))
			
func _ready():
	assert(client_id != null and client_id.length() > 0)
	self.add_child(bg_rect)
	bg_rect.color = Color.brown
	self.visible = false
	initialize_commands()
	for command in commands:
		self.add_child(command)
	
func _process(delta):
	size()


func _input(event):
	if event is InputEventKey:
		if event.is_action_released("ui_cancel"):
			self.visible = !self.visible
