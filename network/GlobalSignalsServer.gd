extends Node

#class_name GlobalSignalsServer

signal prowler_created(id,prowler)

func send_prowler_created(id,prowler):
	emit_signal("prowler_created",id,prowler)



