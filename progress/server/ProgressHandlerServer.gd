extends Node

var client_id = null

func _ready():
	GlobalSignalsServer.connect("client_id_verified",self,"set_client_id")

func set_client_id(client_id):
	self.client_id = client_id

func handle_message(id:String,progress_args):
	match progress_args:
		{'TutorialComplete':{'stage':var stage}}:
			tutorial_stage_completed(id,stage)
		_:
			assert(false)

signal tutorial_stage_completed(id,stage)
func tutorial_stage_completed(id:String,stage:int):
	emit_signal("tutorial_stage_completed",id,stage)

enum TutorialStages{
	not_started = -1,
	stage_1 = 0,


}
