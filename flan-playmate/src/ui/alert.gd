class_name Alert
extends Control

@onready var animation: AnimationPlayer = $AnimationPlayer
@onready var label: Label = $Control/Label

signal done

func _ready() -> void:
    GameState.show_alert.connect(show_alert)

func show_alert(text: String, duration: float):
    label.text = text
    animation.play("fade_in")
    await animation.animation_finished
    
    await get_tree().create_timer(duration).timeout
    
    animation.play("fade_out")
    await animation.animation_finished

    done.emit()
    GameState.done_alert.emit()
