extends Node

const FADE_DURATION: float = 0.5
const SHADER_PATH: String = "res://shaders/SceneTransition.gdshader"

var _overlay: ColorRect
var _mat: ShaderMaterial

func _ready() -> void:
    _setup_overlay()

func _setup_overlay() -> void:
    var canvas := CanvasLayer.new()
    canvas.layer = 100
    add_child(canvas)
    
    _overlay = ColorRect.new()
    _overlay.color = Color.BLACK
    _overlay.set_anchors_preset(Control.PRESET_FULL_RECT)
    _overlay.mouse_filter = Control.MOUSE_FILTER_IGNORE
    _overlay.visible = false
    var shader := load(SHADER_PATH) as Shader
    if shader == null:
       push_error("Cannot create shader")
       return
    _mat = ShaderMaterial.new()
    _mat.shader = shader
    _overlay.material = _mat
    canvas.add_child(_overlay)

func go_to(path: String) -> void:
    _overlay.visible = true
    await _fade_out()
    get_tree().change_scene_to_file(path)
    await get_tree().process_frame
    await _fade_in()
    _overlay.visible = false
    
    
func _fade_out() -> void:
    var tween := create_tween()
    tween.set_ease(Tween.EASE_IN)
    tween.set_trans(Tween.TRANS_CUBIC)
    tween.tween_method(_set_threshold, 0.0, 1.0, FADE_DURATION)
    await tween.finished

func _fade_in() -> void:
    var tween := create_tween()
    tween.set_ease(Tween.EASE_OUT)
    tween.set_trans(Tween.TRANS_CUBIC)
    tween.tween_method(_set_threshold, 1.0, 0.0, FADE_DURATION)
    await tween.finished

func _set_threshold(value: float) -> void:
    _mat.set_shader_parameter("threshold", value)
