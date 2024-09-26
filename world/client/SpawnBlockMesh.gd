extends Spatial


func _ready():
	var re = RotatingEntities.new()
	re.center_point = Vector3(0,10,0)
	re.radius = 500
	self.add_child(re)
