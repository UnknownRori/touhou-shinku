extends Node

var bgm_bus: AudioStreamPlayer
var sfx_bus: AudioStreamPlayer
var bgm_name: String

signal bgm_change(name: String)

func _ready() -> void:
    bgm_bus = AudioStreamPlayer.new()
    bgm_bus.autoplay = false
    bgm_bus.bus = "BGM"
    add_child(bgm_bus)
    
    sfx_bus = AudioStreamPlayer.new()
    sfx_bus.autoplay = false
    sfx_bus.bus = "SFX"
    add_child(sfx_bus)

func play_sfx(id: String):
    var audio = AudioDatabase.get_item(id)
    sfx_bus.volume_linear = audio.volume
    sfx_bus.stream = audio.src
    sfx_bus.play(0.0)
    
func play_bgm(id: String):
    var audio = AudioDatabase.get_item(id)
    bgm_name = audio.name
    bgm_bus.stream = audio.src
    sfx_bus.volume_linear = audio.volume
    bgm_bus.playing = true
    bgm_change.emit(audio.name)
