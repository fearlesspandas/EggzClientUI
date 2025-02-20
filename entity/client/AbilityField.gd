extends Node
class_name AbilityField

onready var ref = load("res://native_lib/Field.gdns").new()
onready var parent = get_parent()
var client_id = ""

func _ready():
	assert(client_id != null and client_id is String and client_id.length() > 0)
	parent.body.add_child(ref)
	ref.add_zone([1,0])
	ref.add_zone([-1,0])
	ref.add_zone([0,1])
	ref.add_zone([0,-1])
	ref.add_zone([1,1])
	ref.add_zone([1,-1])
	ref.add_zone([-1,-1])
	ref.add_zone([-1,1])
	GlobalSignalsClient.connect("pocketed_item",self,"add_abilities_to_menu")
	GlobalSignalsClient.connect("pocket",self,"refresh_abilities")

	ref.connect("add_ability",self,"add_ability_to_field")
	ref.connect("do_ability",self,"do_ability")
	ref.connect("modify_ability",self,"change_ability_state")

func add_abilities_to_menu(client_id,item,amount):
	ref.add_op_to_menus(item)
	
func refresh_abilities(id,contents):
	ref.clear_all_operations()
	for item in contents:
		ref.add_op_to_menus(int(item))
		

func change_ability_state(location,op_id):
	match op_id: 
		0:
			AbilityAPI.globular_teleport().add_base(client_id,parent.body.global_transform.origin)
		1:
			AbilityAPI.globular_teleport().add_point(client_id,parent.body.global_transform.origin)
		_:
			assert(false)
			

func do_ability(location,ability_id):
	match ability_id:
		0:
			ServerNetwork.get(client_id).ability(client_id,0,Vector2(location[0],location[1]))	
		1:
			AbilityAPI.globular_teleport().do(client_id)
		_:
			print_debug("AbwiliITES" + str(ability_id))

func add_ability_to_field(location,ability_id):
	print_debug("signaled loc" ,location)
	ServerNetwork.get(client_id).add_ability(self.client_id,ability_id,Vector2(location[0],location[1]))

func add_field_ability(ability_id:int,location:Vector2): 
	ref.add_field_ability(ability_id,[int(location.x),int(location.y)])
