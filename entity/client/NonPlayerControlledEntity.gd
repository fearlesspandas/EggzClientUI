extends ClientPlayerEntity
class_name NonPlayerControlledEntity
var zone_radiusradius = 1000
func _ready():
	self.is_npc = true
	self.mod = 8
	GlobalSignalsClient.connect("player_location",self,"default_update_player_location")


