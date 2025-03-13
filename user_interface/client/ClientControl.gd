extends Control

signal is_active(active)
class_name ClientControl

onready var connection_indicator:ConnectionIndicator = ConnectionIndicator.new()
onready var viewport_container:ViewportContainer = ViewportContainer.new()
onready var viewport:Viewport = Viewport.new()
onready var entity_management:ClientEntityManager = ClientEntityManager.new()
onready var auth_request:AuthenticationRequest = AuthenticationRequest.new()
onready var physical_stats:PhysicalStatsDisplay = PhysicalStatsDisplay.new()
onready var max_speed_slider:MaxSpeedSlider = MaxSpeedSlider.new()
onready var click_menu:ClickMenu = ClickMenu.new()
onready var destination_display:DestinationDisplay = DestinationDisplay.new()
onready var destination_display_window:DestinationWindow = DestinationWindow.new()
onready var destination_type_indicator:DestinationTypeIndicator = DestinationTypeIndicator.new()
onready var command_menu : CommandMenu = CommandMenu.new()
onready var client_terminal = load("res://native_lib/ClientTerminal.gdns").new()
onready var inventory_menu_rust = Inventory.new()

var profile_id:String
var connection_ind_size = 30

func _ready():
	Engine.physics_jitter_fix = 0
	self.add_child(auth_request)
	#starts auth request that retrieves server secret to be sent on socket startup
	auth_request.connect("session_created",self,"load_scene")
	auth_request._initiate_auth_request(profile_id)
	
func load_scene(id,secret):
	SharedRuntimeEnv.initialize_sockets()
	var profile = ProfileManager.get_profile(profile_id)
	#profile.set_secret_from_encrypted(secret)
	viewport_container.set_size(self.rect_size)
	viewport_container.stretch = true
	
	viewport_container.set_position(Vector2(0,0))
	#viewport.size_override_stretch(true)
	viewport_container.add_child(viewport)
	viewport.size = viewport_container.rect_size
	
	self.add_child(viewport_container)
	
	click_menu.spawn = viewport
	click_menu.client_id = profile.id
	self.add_child(click_menu)
	
	ServerNetwork.init(profile.id,profile.secret,entity_management,"_on_data")
	ServerNetwork.init_physics(profile.id,profile.secret,entity_management,"_on_physics_data")
	
	entity_management.client_id = profile.id
	entity_management.viewport = viewport
	self.connect("is_active",entity_management,"set_active")
	self.add_child(entity_management)
	self.add_child(destination_display_window)
	entity_management.connect("spawned_player_character",click_menu,"player_character_spawned")
	entity_management.connect("spawned_player_character",self,"player_character_spawned")
	entity_management.destinations.connect("new_destination",destination_display_window.destination_display,"add_destination")
	entity_management.destinations.connect("refresh_destinations",destination_display_window.destination_display,"refresh_destinations")
	entity_management.destinations.connect("clear_destinations",destination_display_window.destination_display,"erase_destinations")
	entity_management.destinations.connect("index_set",destination_display_window.destination_display,"set_index")
	entity_management.destinations.connect("destination_deleted",destination_display_window.destination_display,"destination_deleted")
	destination_display_window.destination_display.connect("delete_destination",entity_management.destinations,"delete_destination")
	destination_display_window.destination_display.connect("set_active_destination",entity_management.destinations,"set_active_destination")
	destination_display_window.connect("load_destinations",entity_management.destinations,"request_destination_refresh")
	
	
	max_speed_slider.client_id = profile.id
	self.connect("is_active",max_speed_slider,"set_active")
	self.add_child(max_speed_slider)
	
	connection_indicator.set_size(Vector2(connection_ind_size,connection_ind_size))
	connection_indicator.set_global_position(Vector2(connection_ind_size,connection_ind_size))
	connection_indicator.client_id = entity_management.client_id
	ClientTerminalGlobalSignals.connect("set_active",connection_indicator,"set_terminal_active")
	self.add_child(connection_indicator)

	
	self.add_child(destination_type_indicator)
	
	
	self.add_child(physical_stats)
	entity_management.connect("spawned_player_character",physical_stats.position_indicator,"player_character_spawned")
	physical_stats.lv_indicator.client_id = profile.id
	physical_stats.physics_data_stats.client_id = profile.id
	self.connect("is_active",physical_stats,"set_active")
	entity_management.spawn_client_world(viewport,Vector3(0,-10,0))
	
	command_menu.client_id = entity_management.client_id
	self.add_child(command_menu)
	
	client_terminal.custom_viewport = viewport
	self.connect("is_active",client_terminal,"set_active")
	self.add_child(client_terminal)
	client_terminal.visible = false
	ClientTerminalGlobalSignals.register_terminal(client_terminal)
	ServerTerminalGlobalSignals.register_terminal(client_terminal)

	self.add_child(ShopMenuEnv.shop_menu)
	ShopMenuEnv.set_client_id(profile.id)

	inventory_menu_rust.client_id = profile.id
	self.add_child(inventory_menu_rust)

	#ServerTerminalGlobalSignals.connect_terminal(client_terminal)
	#OS.window_fullscreen = true
	
func player_character_spawned(player:Player):
	assert(player != null)
	self.connect("is_active",player,"set_active")
	player.connect("set_destination_mode",destination_type_indicator,"set_destination_mode")
	player.connect("set_destinations_active",destination_type_indicator,"set_destinations_active")
	player.connect("set_gravity_active",destination_type_indicator,"set_gravity_active")
	
func set_active(active: bool):
	print_debug("setting is active for control:",active)
	ClientReferences.set_viewport(self.viewport)
	ClientReferences.set_command_menu(self.command_menu)
	#if active:
	#	self.mouse_filter = Control.MOUSE_FILTER_IGNORE
	#	self.get_parent().mouse_filter = Control.MOUSE_FILTER_IGNORE
	#	viewport_container.mouse_filter = Control.MOUSE_FILTER_IGNORE
	#	click_menu.mouse_filter = Control.MOUSE_FILTER_IGNORE
	emit_signal("is_active",active)

func _input(event):
	if event.is_action_pressed("toggle_fullscreen"): 
		OS.window_fullscreen = !OS.window_fullscreen

	
func _process(delta):
	if (self.rect_size - OS.get_window_safe_area().size).length() > 0:
		self.set_size(OS.get_window_safe_area().size,true)
		viewport_container.set_size(self.rect_size,true)
