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



