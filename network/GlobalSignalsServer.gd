extends Node

signal client_id_verified(client_id)
func client_id_verified(client_id):
	emit_signal("client_id_verified",client_id)

signal prowler_created(id,prowler)
func prowler_created(id,prowler):
	emit_signal("prowler_created",id,prowler)


signal axis_spider_created(id,axis_spider)
func axis_spider_created(id,axis_spider):
	emit_signal("axis_spider_created",id,axis_spider)

signal player_created(id,player)
func player_created(id,player):
	emit_signal("player_created",id,player)

#emits field signal
signal field(client_id,contents)
func field(client_id:String,contents:Dictionary):
	emit_signal("field",client_id,contents)
