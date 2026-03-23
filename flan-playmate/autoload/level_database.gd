extends BaseResourceDatabase

func _ready():
    _load_all("res://resources/levels/")

func get_item(id: String) -> LevelResource:
    return items.get(id, null)
