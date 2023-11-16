extends Control
class_name PlayerProfileUI
onready var profile:PlayerProfile
onready var file_location:RichTextLabel = find_node("FileLocation")
onready var game:MainGameScene = MainGameScene.new()
# Called when the node enters the scene tree for the first time.

func _ready():
	self.add_child(game)
	pass # Replace with function body.

func _process(delta):
	if profile != null and file_location != null:
		if profile.file_location != null:
			file_location.text = profile.file_location
		if profile.id != null:
			self



func _on_Button_button_up():
	print("pressed button")
	game.spawn_client_world()
