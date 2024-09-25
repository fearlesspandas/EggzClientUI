extends Area

class_name SmackServer

onready var collision_shape:CollisionShape = CollisionShape.new()

func _ready():
	var shape:SphereShape = SphereShape.new()
	shape.radius = 10
	
