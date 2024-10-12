extends Node

func destination(type:String, location:Vector3,radius:float):
	return {'dest_type':{type:{}},'location':[location.x,location.y,location.z], 'radius':radius}

func create_glob(id:String,location:Vector3):
	return {'CREATE_GLOB':{'globId':id,'location':[location.x,location.y,location.z]}}
#spawns prowler with id at location;resets prowler if one already exists with the same id
func create_prowler(id:String,location:Vector3):
	return {'CREATE_PROWLER':{'globId':id,'location':[location.x,location.y,location.z]}}
#spawns axis spider with id at location; resets axis_spider if one already exists with the same id
func create_axis_spider(id:String,location:Vector3):
	return {'CREATE_SPIDER':{'globId':id,'location':[location.x,location.y,location.z]}}

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

func add_health(id:String,value:float):
	return {'ADD_HEALTH' : {'id':id,'value':value}}

func remove_health(id:String,value:float):
	return {'REMOVE_HEALTH' : {'id':id,'value':value}}

func getGlobLocation(id:String):
	return {'GET_GLOB_LOCATION':{'id':str(id)}}	
	
func setGlobLocation(id:String,location:Vector3):
	return {'SET_GLOB_LOCATION':{'id':str(id),'location':[location.x,location.y,location.z]}}	
				
func start_egg(eggId,globId):
	return {'START_EGG':{'eggId':str(eggId),'globId':str(globId)}}	

func getAllEntityIds():
	return {'GET_ALL_ENTITY_IDS':{}}	

func add_destination(globId:String,location:Vector3,type:String,radius:float = 5):
	return {'ADD_DESTINATION':{'id':globId,'dest':destination(type,location,radius)}}
	
func get_next_destination(globId:String):
	return {'GET_NEXT_DESTINATION':{'id':globId}}

func get_next_destination_index(globId:String):
	return {'GET_NEXT_INDEX':{'id':globId}}
	
func get_all_destinations(globId:String):
	return {'GET_ALL_DESTINATIONS':{'id':globId}}

func set_destination_mode(globId:String,mode):
	var obj = {mode:{}}
	return {'SET_MODE_DESTINATIONS':{'id':globId,'mode':obj }}

func set_destinations_active(id:String,value:bool):
	return {'SET_ACTIVE':{'id':id,'value':value}}

func set_gravitate(id:String,value:bool):
	return {'SET_GRAVITATE':{'id':id,'value':value}}

func delete_destination(id:String,uuid:String):
	return {'DELETE_DESTINATION':{'id':id,'uuid':uuid}}

func set_active_destination(id:String,uuid:String):
	return {'SET_ACTIVE_DESTINATION':{'id':id,'destination_id':uuid}}

func follow_entity(id:String,target:String):
	return {'FOLLOW_ENTITY':{'id':id,'target':target}}

func unfollow_entity(id:String,target:String):
	return {'UNFOLLOW_ENTITY':{'id':id,'target':target}}

func location_subscribe(id:String):
	return {'SUBSCRIBE':{"query":{'GET_GLOB_LOCATION':{'id':id}}}}
	
func input_subscribe(id:String):
	return {'SUBSCRIBE':{'query':{'GET_INPUT_VECTOR':{'id':id}}}}

func toggle_destinations(id:String):
	return {'TOGGLE_DESTINATIONS':{'id':id}}

func toggle_gravity(id:String):
	return {'TOGGLE_GRAVITATE':{'id':id}}

func apply_vector(id:String,vec:Vector3):
	return {'APPLY_VECTOR':{'id':id,'vec':[vec.x,vec.y,vec.z]}}
	
func getLocationPhysics(id:String):
	return {'type':'GET_GLOB_LOCATION','body':{'id':id}}

func setLocationPhysics(id:String,location:Vector3):
	return {'type':'SET_GLOB_LOCATION','body':{'id':id,'location':[location.x,location.y,location.z]}}

func set_input_physics(id:String,vec:Vector3):
	return {'type':'APPLY_VECTOR','body':{'id':id,'vec':[vec.x,vec.y,vec.z]}}

func get_input_physics(id:String):
	return {'type':'GET_INPUT','body':{'id':id}}

func lock_input_physics(id:String):
	return {'type':'LOCK_INPUT','body':{'id':id}}

func unlock_input_physics(id:String):
	return {'type':'UNLOCK_INPUT','body':{'id':id}}

func get_dir_physics(id:String):
	return {'type':'GET_DIR','body':{'id':id}}

func set_dir_physics(id:String,dir:Vector3):
	return {'type':'SET_DIR','body':{'id':id,'vec':[dir.x,dir.y,dir.z]}}

func get_rot_physics(id:String):
	return {'type':'GET_ROT','body':{'id':id}}

func set_rot_physics(id:String,dir:Vector3):
	return {'type':'SET_ROT','body':{'id':id,'vec':[dir.x,dir.y,dir.z]}}

func clear_destinations(id:String):
	return {'CLEAR_DESTINATIONS':{'id':id}}

func set_lv(id:String,lv:Vector3):
	return {'SET_LV':{'id':id,'lv':[lv.x,lv.y,lv.z]}}
	
func lazy_lv(id:String):
	return {'LAZY_LV':{'id':id}}

func adjust_stats(id:String,delta):
	return {'ADJUST_PHYSICAL_STATS':{'id':id,'delta':delta}}

func adjust_max_speed(id:String,delta:float):
	return {'ADJUST_MAX_SPEED':{'id':id,'delta':delta}}

func set_speed(id,value:float):
	return {'SET_SPEED':{'id':id,'value':value}}
	
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

func get_top_level_terrain():
	return {'GET_TOP_LEVEL_TERRAIN':{}}

func get_top_level_terrain_within_distance(distance:float,location:Vector3):
	return {'GET_TOP_LEVEL_TERRAIN_IN_DISTANCE':{'loc':[location.x,location.y,location.z],'distance':distance}}

func fill_empty_chunk(uuid:String,trigger_entity:String):
	return {'FILL_EMPTY_CHUNK':{'id':uuid,'trigger_entity':trigger_entity}}
	
func get_cached_terrain(uuid:String):
	return {'GET_CACHED_TERRAIN':{"id":uuid}}

func ability(from:String,ability_id:int):
	return {'ABILITY':{'from':from,'ability_id':ability_id}}
	
func add_item(id:String,item:int):
	return {'ADD_ITEM':{'id':id,'item':item}}

func get_inventory(id:String):
	return {'GET_INVENTORY':{'id':id}}
	
func get_next_command():
	return {'NEXT_CMD':{}}
