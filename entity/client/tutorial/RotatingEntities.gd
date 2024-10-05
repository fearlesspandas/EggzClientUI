extends Spatial
class_name RotatingEntities

onready var center:Spatial = Spatial.new()
onready var tower:ProgressTower = ProgressTower.new()
onready var blobs = {} 

var center_point:Vector3
var radius:float

var height = 128
var blob_count:int = 100
var rotation_speed = 0.01

func _ready():
	assert(center_point != null and center_point != Vector3())
	assert(radius != null)
	#initialize center
	self.global_transform.origin = center_point
	self.add_child(center)
	#initialize tower
	tower.global_transform.origin = center_point
	tower.connect("completed",self,"completed")
	self.add_child(tower)
	#initialize blob state
	initialize_blobs()
	#set_random_rotation()

	
func completed():
	for blob in blobs.values():
		blob.stop()
		blob.set_color(Color.green)
	var spider:AxisSpider = AssetMapper.matchAsset(AssetMapper.local_spider_entity).instance()
	spider.global_transform.origin = Vector3(0,0,800)
	center.add_child(spider)

func set_random_rotation():
	var x = rand_range(0,360)
	var y = rand_range(0,360)
	var z = rand_range(0,360)
	center.global_rotation = Vector3(x,y,z)
	
func initialize_blobs():
	for i in range(0,blob_count):
		var blob = GoalBlob.new()
		blob.id = i
		blob.connect("collided",self,"blob_collision")
		blobs[blob.id] = blob

	for blob in blobs.values():
		#set_random_rotation()
		center.add_child(blob)
		var y = rand_range(0,height)
		var x = rand_range(-radius,radius)
		var z = rand_range(-radius,radius)
		blob.global_transform.origin += Vector3(x,y,z)
		#random_reposition(blob)
		
	
func random_reposition(blob:GoalBlob):
	blob.global_transform.origin = Vector3(
		rand_range(-radius,radius),
		clamp(rand_range(-radius,radius),0,height),
		rand_range(-radius,radius)
	)

func blob_collision(id):
	tower.increment()
	random_reposition(blobs[id])
	
func tick_rotate(delta):
	center.global_rotation += Vector3(0,delta * self.rotation_speed,0)

func _physics_process(delta):
	tick_rotate(delta)


