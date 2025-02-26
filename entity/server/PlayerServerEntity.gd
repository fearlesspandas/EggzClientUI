extends ServerEntity
class_name PlayerServerEntity


onready var field = load("res://native_lib/FieldServer.gdns").new()
onready var init_timer:Timer = Timer.new()
func _ready():
	self.is_npc = false
	self.timer.connect("timeout",self,"timer_polling")
	self.timer.wait_time = 0.25
	self.body.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,true)
	self.body.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,true)
	self.body.set_collision_mask_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)
	self.body.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)
	assert(body is KinematicBody)
	body.add_child(field)
	field.add_zone([1,0])
	field.add_zone([-1,0])
	field.add_zone([0,1])
	field.add_zone([0,-1])
	field.add_zone([1,1])
	field.add_zone([1,-1])
	field.add_zone([-1,-1])
	field.add_zone([-1,1])

	GlobalSignalsServer.connect("field",self,"refresh_field")
	field.connect("damage",self,"do_damage")

	init_timer.connect("timeout",self,"init_requests")
	self.add_child(init_timer)
	init_timer.start(2.56)


func init_requests():
	self.socket.get_field(self.id)
	init_timer.stop()

func do_damage(id:String,amount:float):
	assert(false,"Damaged")

func refresh_field(id:String,contents:Dictionary):
	for item in contents:
		var item_location = contents[item]
		match item_location:
			[..]:
				for loc in item_location:
					match loc:
						[var x, var y]:
							field.add_field_ability(int(item),[int(x),int(y)])
						var coord:
							assert(false,"Misformatted Contents for field zone" + str(coord))
			var coords:
				assert(false,"Misformatted Contents for field " + str(coords))

func timer_polling():
	self.socket.set_lv(self.id,get_lv())

func _physics_process(delta):
	default_physics_process(delta)
	var coll:KinematicCollision = body.get_last_slide_collision()
	if coll != null and coll.collider.has_method("handle_collision"):
		coll.collider.handle_collision(client_id,id)

func _handle_message(msg,delta_accum):
	self.default_handle_message(msg,delta_accum)
