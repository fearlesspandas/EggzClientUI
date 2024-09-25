extends StaticBody
class_name BlockTerrain
onready var health_replenish_timer:Timer = Timer.new()

var MAX_HEALTH = 10
var available_health = MAX_HEALTH
var uuid:String

func _ready():
	health_replenish_timer.wait_time = 1
	health_replenish_timer.connect("timeout",self,"replenish_health")
	self.add_child(health_replenish_timer)
	self.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COUNTABLE_COLLISION_LAYER,true)
	self.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COUNTABLE_COLLISION_LAYER,true)

func handle_collision(client_id:String,player_id:String):
	if available_health > 0:
		ServerNetwork.get(client_id).add_health(player_id,available_health)
		var stats = {'max_speed_delta':available_health,'speed_delta':0}
		ServerNetwork.get(client_id).adjust_max_speed(player_id,available_health)
		available_health = 0
	health_replenish_timer.start()

func replenish_health():
	if available_health < MAX_HEALTH:
		available_health += 1
	else:
		health_replenish_timer.stop()

func init_with_id(uuid:String):
	self.uuid = uuid
