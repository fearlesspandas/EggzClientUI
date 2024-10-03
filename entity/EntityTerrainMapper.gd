extends Node

var unregistered = []

var map = {}

func queue_mapping(terrain):
	unregistered.push_back(terrain)

var npc_count:int = 0

var client_id_server

func generate_name(typ) -> String:
	match typ :
		NPCType.PROWLER:
			#obviously not the actual count, this is just to make naming convenient
			npc_count += 1
			return "Prowler_" + str(npc_count)
	
		_:
			print_debug("no handler found for type ",typ)
			return ""

func increment():
	npc_count += 1

func decrement():
	npc_count -= 1

enum NPCType{
	PROWLER
}


