extends StaticBody

onready var gravity_box:GravityBox = GravityBox.new()

var id = "PlanetA"
func _ready():
	ScheduledTransforms.ref.set_mass(self.id,3000000.0)
	self.add_child(gravity_box)
	var size = 512.0 * 64.0
	gravity_box.ref.set_unaffected_radius(1024 * 1.2)
	gravity_box.ref.set_affected_radius(size)
	gravity_box.ref.set_id(self.id)

