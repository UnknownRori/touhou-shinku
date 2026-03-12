extends HBoxContainer

func _ready() -> void:
    GameState.player_hp.hp_change.connect(_on_update_hp)
    $Label.text = str(GameState.player_hp.current_hp)

func _on_update_hp(new_value: float):
    $Label.text = str(new_value)
