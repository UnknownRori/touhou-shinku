#[macro_export]
macro_rules! vec2f {
    ($x:expr) => {
        godot::builtin::Vector2::new($x, $x)
    };
    ($x:expr, $y:expr) => {
        godot::builtin::Vector2::new($x, $y)
    };
}
