extends MeshInstance

class_name HealthDisplay

onready var text_mesh:TextMesh = TextMesh.new()
onready var font:DynamicFont = DynamicFont.new()
onready var font_data:DynamicFontData = DynamicFontData.new()
 
var value:float = 0
func _ready():
	font_data.font_path = AssetMapper.matchPath(AssetMapper.username_font)
	font.font_data = font_data
	font.size = 50
	font.outline_color = Color.red
	font.outline_size = 5
	text_mesh.font = font
	self.global_transform.origin += Vector3.UP * 4
	self.mesh = text_mesh


func set_value(value:float):
	self.value = value
	text_mesh.text = str(value)


