extends Control

class_name ItemSlot

onready var bg_rect:ColorRect = ColorRect.new()
onready var contents:RichTextLabel = AssetMapper.matchAsset(AssetMapper.chicago_font_control).instance()
onready var font:DynamicFont = DynamicFont.new()
onready var font_data:DynamicFontData = DynamicFontData.new()
onready var activated_timer:Timer = Timer.new()

var slot_id:int
var bg_offset = 5
var bg_offset_vec = Vector2(bg_offset,bg_offset)

var is_empty = true
var is_active = false
var colors = [Color.red,Color.purple,Color.blue,Color.black,Color.pink,Color.white]
var base_color = colors[int(rand_range(0,colors.size()))]

func _ready():
	assert(slot_id != null)

	bg_rect.color = base_color 
	self.add_child(bg_rect)

	#font_data.font_path = AssetMapper.matchPath(AssetMapper.username_font)
	#font.font_data = font_data
	#font.size = 40
	#font.outline_color = Color.red
	#font.outline_size = 0
	#contents.push_font(font)
	#contents.scroll_active = false
	#contents.selection_enabled = false
	self.add_child(contents)

	GlobalSignalsClient.connect("activate_ability",self,"activate")

	activated_timer.wait_time = 1
	activated_timer.connect("timeout",self,"activated")
	self.add_child(activated_timer)

func size_and_position():
	bg_rect.rect_size = self.rect_size
	contents.rect_size = bg_rect.rect_size

func empty():
	is_empty = true
	bg_rect.visible = !is_empty

func fill(item:int):
	is_empty = false
	bg_rect.visible = !is_empty
	contents.text = map_items(item)

func activate(client_id,slot_id):
	if slot_id == self.slot_id:
		is_active = true	
		activated_timer.start()
		bg_rect.color = Color.yellow
		bg_rect.color.a = 0.3

func map_items(item_id:int) -> String:
	match item_id:
		0:
			return "smash"
		1:
			return "glob tele"
		_:
			return "unknown item"

func activated():
	is_active = false
	activated_timer.stop()
	bg_rect.color = base_color
	bg_rect.color.a = 1
	
func _process(delta):
	size_and_position()

