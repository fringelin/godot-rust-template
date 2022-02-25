extends MeshInstance

var rotate_speed = 0.05

# Called when the node enters the scene tree for the first time.
func _ready():
	var ecs = get_node("/root/ECSController")
	ecs.add_node_to_ecs(self, rotate_speed)
