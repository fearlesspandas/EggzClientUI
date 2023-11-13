extends Spatial


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
export var blockterrain:Resource
onready var playerspace = find_node("Player")

onready var scenery = find_node("Scenery")
# Called when the node enters the scene tree for the first time.

var rng = RandomNumberGenerator.new()
func generate_random_vecs(n:int,minVec:Vector3,maxvec:Vector3):
	var vecs = []
	for i in n:
		vecs.append(
			Vector3(
				rng.randf_range(minVec.x,maxvec.x),
				rng.randf_range(minVec.y,maxvec.y),
				rng.randf_range(minVec.z,maxvec.z)
			)
		)
	
	return vecs
func _ready():
	var block_resource =load(blockterrain.resource_path) 
	
	var vecs = generate_random_vecs(100,Vector3(-100,0,-100),Vector3(100,30,100))
	for v in vecs:
		var block = block_resource.instance()
		self.add_child(block)
		block.global_transform.origin = v
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
