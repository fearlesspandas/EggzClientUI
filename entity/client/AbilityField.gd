extends Node
class_name AbilityField

onready var ref = load("res://native_lib/Field.gdns").new()
onready var parent = get_parent()
onready var init_timer:Timer = Timer.new()
var client_id = ""
var socket:ClientWebSocket = null


func _ready():
	assert(client_id != null and client_id is String and client_id.length() > 0)
	socket = ServerNetwork.get(client_id)
	assert(self.socket != null)
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
	GlobalSignalsClient.connect("unpocketed_item",self,"remove_abilities_from_menu")
	GlobalSignalsClient.connect("pocket",self,"refresh_abilities")
	GlobalSignalsClient.connect("field",self,"refresh_field")


	ref.connect("add_ability",self,"add_ability_to_field")
	ref.connect("do_ability",self,"do_ability")
	ref.connect("modify_ability",self,"change_ability_state")

	init_timer.connect("timeout",self,"initialize_field")
	self.add_child(init_timer)
	init_timer.start(3.76)

func initialize_field():
	socket.get_field(client_id)
	init_timer.stop()
	
func add_abilities_to_menu(client_id:String,item:int,amount:int):
	ref.add_op_to_menus(item,amount)
	
func remove_abilities_from_menu(client_id:String,item:int,amount:int):
	ref.remove_op_from_menus(item,amount)
	
func refresh_abilities(id:String,contents:Dictionary):
	ref.clear_all_operations()
	for item in contents:
		if !((contents[item] is float or contents[item] is int) and (item is float or item is int or item is String)):
			assert(false, "Malformed contents for abilities " + str(contents))
		ref.add_op_to_menus(int(item),int(contents[item]))

func refresh_field(id:String,contents:Dictionary):
	for item in contents:
		var item_location = contents[item]
		#assert(false,str(item_location))
		match item_location:
			[..]:
				for loc in item_location:
					match loc:
						[var x, var y]:
							ref.add_field_ability(int(item),[int(x),int(y)])
						var coord:
							assert(false,"Misformatted Contents for field zone" + str(coord))
			var coords:
				assert(false,"Misformatted Contents for field " + str(coords))
				
func add_field_ability(ability_id:int,location:Vector2): 
	ref.add_field_ability(ability_id,[int(location.x),int(location.y)])

func change_ability_state(_location,op_id:int):
	match op_id: 
		0:
			AbilityAPI.globular_teleport().add_base(client_id,parent.body.global_transform.origin)
		1:
			AbilityAPI.globular_teleport().add_point(client_id,parent.body.global_transform.origin)
		_:
			assert(false)
			

func do_ability(location:Array,ability_id:int):
	match ability_id:
		0:
			match location:
				[var x , var y]:
					socket.ability(client_id,0,Vector2(x,y))	
				_:
					assert(false,"Malformed location for do ability " + str(ability_id))
		1:
			AbilityAPI.globular_teleport().do(client_id)
		_:
			print_debug("AbwiliITES" + str(ability_id))

func add_ability_to_field(location:Array,ability_id:int):
	match location:
		[var x, var y]:
			socket.add_ability(self.client_id,ability_id,Vector2(x,y))
		_:
			assert(false,"Malformed location for add_ability_to_field " + str(ability_id))

