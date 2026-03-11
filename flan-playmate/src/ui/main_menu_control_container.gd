extends VBoxContainer

@export var new_run_scene: String = "res://scenes/DemoScene.tscn"

func _on_new_run_pressed() -> void:
    SceneManager.go_to(new_run_scene)
    $"New Run".disabled = true


func _on_exit_pressed() -> void:
    get_tree().quit()
    $Exit.disabled = true
