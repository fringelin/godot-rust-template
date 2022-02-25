extends MeshInstance

export var rotate_speed = 1

# Called when the node enters the scene tree for the first time.
func _ready():
	var ecs = get_node("/root/ECSController")
	ecs.add_node_to_ecs(self, rotate_speed)
