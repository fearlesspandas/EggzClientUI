extends Command
signal toggle_chunks_visible(visible)
class_name ToggleChunkVisibility
var is_visible = true

func _init():
	self.command_name = "toggle_chunks_visible"

func _ready():
	self.connect("button_clicked",self,"toggle_chunk_visibility")

func  toggle_chunk_visibility(argmap):
	self.is_visible = !self.is_visible
	DataCache.add_data("CLIENT","chunks_visible",self.is_visible)
	emit_signal("toggle_chunks_visible",self.is_visible)

