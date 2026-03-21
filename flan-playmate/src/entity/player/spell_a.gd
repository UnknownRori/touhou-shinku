extends NodeState

@export var sprite: AnimatedSprite2D
@export var entity: CharacterBody2D
@export var bullet: BulletSpawnerComponent
@export var velocity: VelocityComponent
@export var attack_cooldown: Timer

var speed_modifier = 0.8

func _ready() -> void:
    attack_cooldown.timeout.connect(_timeout)

func _on_process(_dt: float) -> void:
    var dir = GameInputs.update_movement()
    dir *= speed_modifier
    velocity.set_dir(dir)
    velocity.set_focus(GameInputs.is_focus())
    
    if dir.x == 1.0:
        sprite.play("right")
    if dir.x == -1.0:
        sprite.play("left")    

func _on_enter() -> void:
    attack_cooldown.start()

func _on_leave() -> void:
    attack_cooldown.stop()

func _timeout():
    bullet.spawn(entity.global_position, Vector2(0, -100), 0.)
    AudioManager.play_sfx("player_shot")

func _on_next_transition() -> void:
    if attack_cooldown.is_stopped():
        if !GameInputs.is_move():
            transition.emit("idle")
        if GameInputs.is_move():
            transition.emit("move")
