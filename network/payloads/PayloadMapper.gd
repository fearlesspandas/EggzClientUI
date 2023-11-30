extends Node


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
	
func add_destination(globId:String,location:Vector3):
	return {'ADD_DESTINATION':{'id':globId,'location':[location.x,location.y,location.z]}}	
	
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
