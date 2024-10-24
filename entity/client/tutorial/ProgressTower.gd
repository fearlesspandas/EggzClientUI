extends Spatial
class_name ProgressTower
signal completed
var segments = []

var num_segments = 3
var total_height = 128
var chunk_radius = 100

var score = 0

func _ready():
	assert(num_segments != null)
	assert(total_height != null)
	var unit_height = total_height/num_segments
	for i in range(0,num_segments):
		var chunk = TowerChunk.new()
		chunk.height = unit_height
		chunk.radius = chunk_radius
		chunk.radius -= (chunk_radius/num_segments) * i
		segments.push_back(chunk)
		self.add_child(chunk)
		self.connect("completed",chunk,"completed")
		chunk.place_on(Vector3(0,i * chunk.height,0))
		
		
func increment():
	segments[score%segments.size()].set_color(Color.aquamarine)
	score += 1
	if score == segments.size():
		emit_signal("completed")
