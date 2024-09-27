extends Node


var viewport:Viewport

func set_viewport(view:Viewport):
	viewport = view
	
var command_menu:CommandMenu

func set_command_menu(cm:CommandMenu):
	command_menu = cm

var destination_manager:DestinationManager
func set_destination_manager(dm:DestinationManager):
	destination_manager = dm
