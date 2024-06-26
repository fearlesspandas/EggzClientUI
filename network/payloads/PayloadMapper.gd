extends Node

func destination(type:String, location:Vector3,radius:float):
	return {'dest_type':{type:{}},'location':[location.x,location.y,location.z], 'radius':radius}
func create_glob(id:String,location:Vector3):
	#print("calling create glob")
	return {'CREATE_GLOB':{'globId':id,'location':[location.x,location.y,location.z]}}

func create_repair_egg(eggId:String,globId:String):
	return {'CREATE_REPAIR_EGG':{'eggId':eggId,'globId':globId}}

func get_blob(id:String):
	return {'GET_BLOB':{'id':id}}

func relate_eggs(id1:String,id2:String,globid:String,bidirectional:bool):
	return {'RELATE_EGGS':{'egg1':id1,'egg2':id2,'globId':globid,'bidirectional':bidirectional}}
	
func unrelate_eggs(id1:String,id2:String,globid:String,bidirectional:bool):
	return {'UNRELATE_EGGS':{'egg1':id1,'egg2':id2,'globId':globid,'bidirectional':bidirectional}}

func tick_eggs():	
	return {'TICK_WORLD':{}}

func getAllGlobs():
	return {'GET_ALL_GLOBS':{}}

func getAllEggs():
	return {'GET_ALL_EGGZ':{}}	

func getGlobLocation(id:String):
	return {'GET_GLOB_LOCATION':{'id':str(id)}}	

func setGlobLocation(id:String,location:Vector3):
	return {'SET_GLOB_LOCATION':{'id':str(id),'location':[location.x,location.y,location.z]}}	

func setGlobRotation(id,rotation):
	return {'SET_GLOB_ROTATION':{'id':str(id),'rotation':rotation}}	
				
func start_egg(eggId,globId):
	return {'START_EGG':{'eggId':str(eggId),'globId':str(globId)}}	

func getAllEntityIds():
	return {'GET_ALL_ENTITY_IDS':{}}	

func add_destination(globId:String,location:Vector3,type:String,radius:float = 5):
	return {'ADD_DESTINATION':{'id':globId,'dest':destination(type,location,radius)}}
	
func get_next_destination(globId:String):
	return {'GET_NEXT_DESTINATION':{'id':globId}}	
	
func get_all_destinations(globId:String):
	return {'GET_ALL_DESTINATIONS':{'id':globId}}	
	
func location_subscribe(id:String):
	return {'SUBSCRIBE':{"query":{'GET_GLOB_LOCATION':{'id':id}}}}
	
func input_subscribe(id:String):
	return {'SUBSCRIBE':{'query':{'GET_INPUT_VECTOR':{'id':id}}}}

func apply_vector(id:String,vec:Vector3):
	return {'APPLY_VECTOR':{'id':id,'vec':[vec.x,vec.y,vec.z]}}

func clear_destinations(id:String):
	return {'CLEAR_DESTINATIONS':{'id':id}}

func set_lv(id:String,lv:Vector3):
	return {'SET_LV':{'id':id,'lv':[lv.x,lv.y,lv.z]}}
	
func lazy_lv(id:String):
	return {'LAZY_LV':{'id':id}}

func adjust_stats(id:String,delta):
	return {'ADJUST_PHYSICAL_STATS':{'id':id,'delta':delta}}
	
func get_physical_stats(id:String):
	return {'GET_PHYSICAL_STATS':{'id': id}}

func subscribe_general(query):
	return {'SUBSCRIBE':{'query':query}}
	
func get_all_terrain(id:String,nonrelative:bool):
	return {'GET_ALL_TERRAIN':{'id':id,'non_relative':nonrelative}}

func create_terrain(id:String,location:Vector3):
	return {'ADD_TERRAIN':{'id':id,'location':[location.x,location.y,location.z]}}

func get_terrain_within_player_distance(id:String,radius:float):
	return {'GET_TERRAIN_WITHIN_PLAYER_DISTANCE':{'id':id,'radius':radius}}
