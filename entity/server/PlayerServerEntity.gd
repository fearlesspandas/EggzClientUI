extends ServerEntity
class_name PlayerServerEntity


func _ready():
	self.is_npc = false
	self.timer.connect("timeout",self,"timer_polling")
	self.timer.wait_time = 10 
	self.body.set_collision_layer_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,true)
	self.body.set_collision_layer_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,true)
	self.body.set_collision_mask_bit(EntityConstants.SERVER_PLAYER_COLLISION_LAYER,false)
	self.body.set_collision_mask_bit(EntityConstants.SERVER_TERRAIN_COLLISION_LAYER,false)


func timer_polling():
	self.socket.set_lv(self.id,get_lv())

#instance - don't abstract out
func _physics_process(delta):
	default_physics_process(delta)
	if body is KinematicBody: 
		var coll:KinematicCollision = body.get_last_slide_collision()
		if coll != null and coll.collider.has_method("handle_collision"):
			coll.collider.handle_collision(client_id,id)

func _handle_message(msg,delta_accum):
	self.default_handle_message(msg,delta_accum)
