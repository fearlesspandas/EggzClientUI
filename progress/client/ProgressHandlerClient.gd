extends Node

func handle_message(id:String,progress_args):
	match progress_args:
		{'TutorialComplete':{'stage':var stage}}:
			tutorial_stage_completed(stage)
		_:
			assert(false)

signal tutorial_stage_completed(stage)
func tutorial_stage_completed(stage:int):
	emit_signal("tutorial_stage_completed",stage)

enum TutorialStages{
	not_started = -1,
	stage_1 = 0,
	


}
