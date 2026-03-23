extends BaseResourceDatabase

func _ready():
    _load_all("res://resources/upgrade/")

func get_item(id: String) -> UpgradeResource:
    return items.get(id, null)
