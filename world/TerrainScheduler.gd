extends Node

#var terrain_neighbors = {}
#var terrain = {}

#func add_terain(chunk:Chunk):
#	var neighbors = []
#	for t in terrain.values():
#		var dist:Vector3 = t.center - chunk.center
#		if dist.length() < t.radius + chunk.radius:
#			neighbors.append(t)
#			terrain_neighbors[t.uuid] = terrain_neighbors[t.uuid].append(t)
#	terrain_neighbors[chunk.uuid] = neighbors
#	terrain[chunk.uuid] = chunk
#	print_debug("successfully added terrain ", chunk.uuid)
	
	
#func start_chunk_and_neighbors(uuid:String):
#	print_debug("starting chunk and neighbors for ", uuid)
#	for t in terrain_neighbors[uuid]:
#		t.start_timer()
#	terrain[uuid].start_timer()
