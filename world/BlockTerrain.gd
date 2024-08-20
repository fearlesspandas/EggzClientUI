extends StaticBody

onready var health_replenish_timer:Timer = Timer.new()

var MAX_HEALTH = 10
var available_health = MAX_HEALTH

func _ready():
	health_replenish_timer.wait_time = 1
	health_replenish_timer.connect("timeout",self,"replenish_health")
	self.add_child(health_replenish_timer)

func handle_collision(client_id:String,player_id:String):
	if available_health > 0:
		ServerNetwork.get(client_id).add_health(player_id,available_health)
		available_health = 0
	health_replenish_timer.start()

func replenish_health():
	if available_health < MAX_HEALTH:
		available_health += 1
	else:
		health_replenish_timer.stop()
