extends Node
class_name AbilityFieldServer

onready var ref = load("res://native_lib/FieldServer.gdns").new()
onready var parent = get_parent()

func _ready():
	GlobalSignalsServer.connect("field",self,"refresh_field")
	parent.add_child(ref)
	ref.add_zone([1,0])
	ref.add_zone([-1,0])
	ref.add_zone([0,1])
	ref.add_zone([0,-1])
	ref.add_zone([1,1])
	ref.add_zone([1,-1])
	ref.add_zone([-1,-1])
	ref.add_zone([-1,1])

func refresh_field(id:String,contents:Dictionary):
	for item in contents:
		var item_location = contents[item]
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

func add_field_ability(ability_id:int,location:Vector2,occupied:Array):
	ref.add_field_ability(ability_id,[int(location.x),int(location.y)])
	for zone in occupied:
		ref.add_field_ability(-1,[int(location.x),int(location.y)])

func remove_field_ability(ability_id:int,freed:Array):
	ref.remove_field_ability(ability_id)
