extends MeshInstance

class_name Username

onready var text_mesh:TextMesh = TextMesh.new()
onready var font:DynamicFont = DynamicFont.new()
onready var font_data:DynamicFontData = DynamicFontData.new()
onready var id 

func _ready():
	assert(id != null)
	assert(Subscriptions.get(id) != null)
	font_data.font_path = AssetMapper.matchPath(AssetMapper.username_font)
	font.font_data = font_data
	font.size = 80
	font.outline_color = Color.red
	font.outline_size = 5
	text_mesh.text = Subscriptions.get(id)
	text_mesh.font = font
	self.global_transform.origin += Vector3.UP * 3
	self.mesh = text_mesh
	
func init_id():
	id = Subscriptions.generate_id()
