extends Spatial


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	var ecs = get_node("/root/ECSController")
	ecs.add_game_to_ecs(self)


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
