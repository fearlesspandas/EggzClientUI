extends Node

signal url_set(url)
func url_set(url:String):
	emit_signal("url_set",url)

signal physics_url_set(url)
func physics_url_set(url:String):
	emit_signal("physics_url_set",url)


