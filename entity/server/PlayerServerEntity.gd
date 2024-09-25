extends ServerEntity
class_name PlayerServerEntity


func _ready():
	self.is_npc = false
	timer.connect("timeout",self,"timer_polling")
	timer.wait_time = 0.25
	self.body.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,true)
	self.body.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,true)
	self.body.set_collision_mask_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,true)
	self.body.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,true)


func timer_polling():
	socket.set_lv(id,get_lv())

func _physics_process(delta):
	if body is KinematicBody: 
		var coll:KinematicCollision = body.get_last_slide_collision()
		if coll != null and coll.collider.has_method("handle_collision"):
			coll.collider.handle_collision(client_id,id)
