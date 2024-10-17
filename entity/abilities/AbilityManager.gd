extends Node

var client_spawn:Spatial
var server_spawn:Spatial
#change this to be included in ability command
var client_id_server:String



var ability_data = {}

#spawns an abilities mesh at location (client only)
func ability_client(ability_id:int,location:Vector3,args:Dictionary):
	match ability_id:
		0:
			var sc = SmackClient.new()
			client_spawn.add_child(sc)
			sc.global_transform.origin = location
		1:
			match args:
				{'Shape':{'points':var points,'location':[var lx , var ly , var lz]}}:
					var gt =  GlobularTeleportClient.new()
					for point in points:
						match point:
							[var x, var y , var z]:
								gt.vertices.push_back(Vector3(float(x),float(y),float(z)))
					gt.base = Vector3(lx,ly,lz)
					#always unclear if i can add_child after setting global_transform.origin
					#gt.global_transform.origin = Vector3(lx,ly,lz)
					client_spawn.add_child(gt)
				_:
					assert(false)
		_:
			print_debug("No ability found with id ", ability_id)



#spawns an active ability at location
func ability_server(ability_id:int,location:Vector3):
	match ability_id:
		Abilities.smack:
			var sc = SmackServer.new()
			server_spawn.add_child(sc)
			sc.global_transform.origin = location
		_:
			print_debug("No ability found with id ", ability_id)

#applies an abilities effects to the entity with id = entity_id
func do_ability_server(ability_id:int,entity_id:String):
	match ability_id:
		Abilities.smack:
			ServerNetwork.get(client_id_server).remove_health(entity_id,10)
		_:
			print_debug("No ability behavior defined for id ",ability_id)


enum Abilities{
	smack = 0,
	globular_teleport = 1,
}
