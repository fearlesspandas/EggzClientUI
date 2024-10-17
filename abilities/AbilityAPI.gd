#AbilityAPI is a convenience layer for managing AbilityStateData
#When needed it exposes certain functions that interface with
#the DataCacheAPI to store and retrieve data related to abilities.
#todo - may eventually split into client/server layers

extends Node

signal globular_teleport_point_added(client_id,point)
signal globular_teleport_base_added(client_id,point)

var globular_teleport_instance = GlobularTeleport.new()
	
func globular_teleport() -> GlobularTeleport:
	return globular_teleport_instance

class GlobularTeleport:
	#adds anchor point for globular teleport
	#(the target teleport point)
	func add_base(client_id:String,location:Vector3):
		DataCache.add_data(client_id,field(Fields.globular_teleport_base),location)

	#adds point to glob prism for globular teleport
	func add_point(client_id:String,location:Vector3):
		var current_points = DataCache.cached(client_id,field(Fields.globular_teleport_points)) 	
		if current_points == null: 
			current_points = PoolVector3Array()
			current_points.push_back(location)
		elif current_points is PoolVector3Array:
			current_points.push_back(location)
		else:
			current_points = null
		if current_points != null:
			DataCache.add_data(client_id,field(Fields.globular_teleport_points),current_points)
		else: # should only hit this case if types are mismanaged and current_points is not a pooled array
			assert(false)

	#clears current points
	func clear(client_id):
		DataCache.add_data(client_id,field(Fields.globular_teleport_points),null)

	#sends data state to server	
	func do(client_id):
		var base = DataCache.cached(client_id,field(Fields.globular_teleport_base))
		var points = DataCache.cached(client_id,field(Fields.globular_teleport_points))
		if base != null and points != null and base is Vector3 and points is PoolVector3Array:
			var mapped_points = []
			for point in points:
				mapped_points.push_back([point.x,point.y,point.z])
			ServerNetwork.get(client_id).ability(client_id,1,{'Shape':{'points':mapped_points,'location':[base.x,base.y,base.z]}})
			clear(client_id)
		else:
			assert(false)
			
	#Field concept is mainly used to ensure some amount of
	#'type checking', and easy reorganization of values
	enum Fields{
		globular_teleport_base = 0,
		globular_teleport_points = 1
	}

	func field(field:int) -> String:
		match field:
			Fields.globular_teleport_points:
				return 'globular_teleport_points'	
			Fields.globular_teleport_base:
				return 'globular_teleport_base'
			_:
				return ""
