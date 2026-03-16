extends Control

func _ready() -> void:
    AudioManager.bgm_change.connect(_bgm_change)
    update_bgm_name(AudioManager.bgm_name)

func _bgm_change(bgm_name: String):
    update_bgm_name(bgm_name)

func update_bgm_name(bgm_name: String):
    $Label.text = bgm_name
