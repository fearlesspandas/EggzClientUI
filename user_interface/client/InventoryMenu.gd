extends Control
class_name InventoryMenu

onready var bg_rect:ColorRect = ColorRect.new()

var bg_offset = 5
var bg_offset_vec = Vector2(bg_offset,bg_offset)

var client_id:String
var socket:ClientWebSocket
var num_slots = 5
var items = []


func _ready():
	assert(client_id != null and client_id.length() > 0)
	socket = ServerNetwork.get(client_id)
	assert(socket != null)
	GlobalSignalsServer.connect("item_added",self,"add_item")

	bg_rect.color = Color.gray
	self.add_child(bg_rect)
	init_slots()


func init_slots():
	for i in range(0,num_slots):
		var item_slot = ItemSlot.new()	
		item_slot.slot_id = i
		items.push_back(item_slot)
		self.add_child(item_slot)

func size_and_position():
	self.rect_size = Vector2(OS.window_size.x/4,OS.window_size.y/16) - 2*bg_offset_vec
	bg_rect.rect_size = self.rect_size


	self.set_position(
		Vector2(
			(OS.window_size.x - self.rect_size.x)/2,
			OS.window_size.y - 4*self.rect_size.y
		)
	)
	bg_rect.set_position(Vector2.ZERO)

	for item in items:
		item.rect_size = Vector2(self.rect_size.x/num_slots,self.rect_size.y - 2*bg_offset)
		item.set_position(Vector2(item.rect_size.x * item.slot_id,bg_offset ))
	

func add_item(id,item):
	assert(false)

func _process(delta):
	size_and_position()
