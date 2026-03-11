shader_type canvas_item;

uniform float threshold : hint_range(0.0, 1.0) = 0.0;
uniform vec4  color     : source_color = vec4(0.0, 0.0, 0.0, 1.0);
uniform int   pixel_size : hint_range(1, 16) = 8;
uniform bool  invert    = false;

// Bayer 8×8 matrix  (values 0–63, normalised to 0–1)
float bayer8(ivec2 p) {
    const int[64] B = int[64](
         0, 32,  8, 40,  2, 34, 10, 42,
        48, 16, 56, 24, 50, 18, 58, 26,
        12, 44,  4, 36, 14, 46,  6, 38,
        60, 28, 52, 20, 62, 30, 54, 22,
         3, 35, 11, 43,  1, 33,  9, 41,
        51, 19, 59, 27, 49, 17, 57, 25,
        15, 47,  7, 39, 13, 45,  5, 37,
        63, 31, 55, 23, 61, 29, 53, 21
    );
    int idx = (p.y % 8) * 8 + (p.x % 8);
    return float(B[idx]) / 63.0;
}

void fragment() {
    ivec2 px    = ivec2(FRAGCOORD.xy) / pixel_size;
    float limit = bayer8(px);
    float t     = clamp((threshold - 0.05) * 1.1 + 0.05, 0.0, 1.0);
    bool show   = invert ? (limit > t) : (limit < t);
    if (!show) {
        discard;
    }

    COLOR = color;
}
