extends Control

signal is_active(active)
class_name ClientControl

onready var connection_indicator:ConnectionIndicator = ConnectionIndicator.new()
onready var viewport_container:ViewportContainer = ViewportContainer.new()
onready var viewport:Viewport = Viewport.new()
onready var entity_management:ClientEntityManager = ClientEntityManager.new()
onready var auth_request:AuthenticationRequest = AuthenticationRequest.new()
onready var lv_indicator:LinearVelocityIndicator = LinearVelocityIndicator.new()
onready var max_speed_slider:MaxSpeedSlider = MaxSpeedSlider.new()
onready var click_menu:ClickMenu = ClickMenu.new()

var profile:PlayerProfile
var connection_ind_size = 30

func _ready():
	self.add_child(auth_request)
	#starts auth request that retrieves server secret to be sent on socket startup
	auth_request.connect("session_created",self,"load_scene")
	auth_request._initiate_auth_request(profile.id)
	
func load_scene(id,secret):
	profile.secret = secret
	print("entering ClientControl")
	viewport_container.set_size(self.rect_size)
	viewport_container.stretch = true
	
	viewport_container.set_position(Vector2(0,0))
	#viewport.size_override_stretch(true)
	viewport_container.add_child(viewport)
	viewport.size = viewport_container.rect_size
	
	self.add_child(viewport_container)
	
	entity_management.client_id = profile.id
	entity_management.viewport = viewport
	self.add_child(entity_management)
	
	lv_indicator.client_id = profile.id
	lv_indicator.rect_size = self.rect_size / 4
	lv_indicator.set_position(Vector2(0,0))
	self.add_child(lv_indicator)
	
	click_menu.spawn = viewport
	click_menu.client_id = profile.id
	self.add_child(click_menu)
	
	max_speed_slider.client_id = profile.id
	max_speed_slider.rect_size = self.rect_size / 4
	max_speed_slider.set_position(
		Vector2(
			self.rect_size.x - max_speed_slider.rect_size.x,
			self.rect_size.y - (max_speed_slider.rect_size.y * 2)
		)
	)
	self.add_child(max_speed_slider)
	
	ServerNetwork.init(profile.id,profile.secret,entity_management,"_on_data")
	
	connection_indicator.set_size(Vector2(connection_ind_size,connection_ind_size))
	connection_indicator.set_global_position(Vector2(connection_ind_size,connection_ind_size))
	connection_indicator.client_id = entity_management.client_id
	self.add_child(connection_indicator)
	
	entity_management.spawn_client_world(viewport,Vector3(0,-10,0))
	var player = entity_management.create_character_entity_client(profile.id,Vector3(0,0,0),viewport)
	
	self.connect("is_active",player,"set_active")
	self.connect("is_active",entity_management,"set_active")
	player.curserRay.connect("intersection_clicked",click_menu,"handle_clicked")
	#entity_management.entity_scanner.start()
	
func handle_new_entity(entity,parent,server_entity):
	print("new entity in clientControl")
	pass

func set_active(active: bool):
	print("setting is active for control:",active)
	emit_signal("is_active",active)
	
func _process(delta):
	if (self.rect_size - OS.get_window_safe_area().size).length() > 5:
		self.set_size(OS.get_window_safe_area().size,true)
		viewport_container.set_size(self.rect_size,true)
		max_speed_slider.rect_size = self.rect_size / 4
		max_speed_slider.set_position(
			Vector2(
				self.rect_size.x - max_speed_slider.rect_size.x,
				self.rect_size.y - (max_speed_slider.rect_size.y * 2)
			)
		)
