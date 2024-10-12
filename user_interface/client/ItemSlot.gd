extends Control

class_name ItemSlot

onready var bg_rect:ColorRect = ColorRect.new()

var slot_id:int
var bg_offset = 5
var bg_offset_vec = Vector2(bg_offset,bg_offset)

var colors = [Color.red,Color.purple,Color.blue,Color.black,Color.pink,Color.white]
func _ready():
	assert(slot_id != null)
	bg_rect.color = colors[int(rand_range(0,colors.size()))]
	self.add_child(bg_rect)


func size_and_position():
	bg_rect.rect_size = self.rect_size

func _process(delta):
	size_and_position()
