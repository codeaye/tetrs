use lazy_static::lazy_static;
use macroquad::color::Color;

pub const WINDOW_HEIGHT: i32 = 620;
pub const WINDOW_WIDTH: i32 = 500;

pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 10;
pub const CELL_SIZE: f32 = 30.;

pub const GAMEOVER_ANIM_DURATION: f64 = 1.;
pub const ROW_DISSAPEAR_ANIM_DURATION: f64 = 0.8;

pub const FONT: &[u8] = include_bytes!("../resources/fonts/FontRegular.otf");

macro_rules! use_colors {
    ($(($name: ident, $col: expr)),+) => {
        lazy_static! {$(pub static ref $name : Color = Color::from_hex($col);)*}
    };
}

use_colors!(
    /*USED*/ (EERIE_BLACK, 0x202020),
    /*USED*/ (COCOA_BROWN, 0x2d211e),
    /*USED*/ (CRATER_BROWN, 0x452923),
    /*USED*/ (PICKLED_BEAN, 0x6d3d29),
    /*USED*/ (BROWN, 0xb16b4a),
    /*USED*/ (GOLD_SAND, 0xe8be82),
    /*USED*/ (CLAY_CREEK, 0x8e9257),
    /*USED*/ (RAVEN, 0x707b88),
    /*USED*/ (CASCADE, 0x8aa7ac),
    /*USED*/ (BURNT_SIENNA1, 0xe55d4d),
    /*USED*/ (BURNT_SIENNA2, 0xf1866c),
    /*USED*/ (FLAME_PEA, 0xd26730),
    /*USED*/ (GOLDEN_GRASS, 0xde9a28),
    /*USED*/ (ZOMBIE, 0xe8d8a5)
);

/*
| EERIE_BLACK | ![#202020](https://via.placeholder.com/10/202020?text=+) #202020|
| COCOA_BROWN | ![#2d211e](https://via.placeholder.com/10/2d211e?text=+) #2d211e|
| CRATER_BROWN | ![#452923](https://via.placeholder.com/10/452923?text=+) #452923|
| PICKLED_BEAN | ![#6d3d29](https://via.placeholder.com/10/6d3d29?text=+) #6d3d29|
| BROWN | ![#b16b4a](https://via.placeholder.com/10/b16b4a?text=+) #b16b4a|
| GOLD_SAND | ![#e8be82](https://via.placeholder.com/10/e8be82?text=+) #e8be82|
| CLAY_CREEK | ![#8e9257](https://via.placeholder.com/10/8e9257?text=+) #8e9257|
| RAVEN | ![#707b88](https://via.placeholder.com/10/707b88?text=+) #707b88|
| CASCADE | ![#8aa7ac](https://via.placeholder.com/10/8aa7ac?text=+) #8aa7ac|
| BURNT_SIENNA1 | ![#e55d4d](https://via.placeholder.com/10/e55d4d?text=+) #e55d4d|
| BURNT_SIENNA2 | ![#f1866c](https://via.placeholder.com/10/f1866c?text=+) #f1866c|
| FLAME_PEA | ![#d26730](https://via.placeholder.com/10/d26730?text=+) #d26730|
| GOLDEN_GRASS | ![#de9a28](https://via.placeholder.com/10/de9a28?text=+) #de9a28|
| ZOMBIE | ![#e8d8a5](https://via.placeholder.com/10/e8d8a5?text=+) #e8d8a5|
*/

pub const FRAGMENT_SHADER: &str = r#"#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;

// https://www.shadertoy.com/view/XtlSD7

vec2 CRTCurveUV(vec2 uv)
{
    uv = uv * 2.0 - 1.0;
    vec2 offset = abs( uv.yx ) / vec2( 6.0, 4.0 );
    uv = uv + uv * offset * offset;
    uv = uv * 0.5 + 0.5;
    return uv;
}

void DrawVignette( inout vec3 color, vec2 uv )
{
    float vignette = uv.x * uv.y * ( 1.0 - uv.x ) * ( 1.0 - uv.y );
    vignette = clamp( pow( 16.0 * vignette, 0.3 ), 0.0, 1.0 );
    color *= vignette;
}


void DrawScanline( inout vec3 color, vec2 uv )
{
    float iTime = 0.1;
    float scanline 	= clamp( 0.95 + 0.05 * cos( 3.14 * ( uv.y + 0.008 * iTime ) * 240.0 * 1.0 ), 0.0, 1.0 );
    float grille 	= 0.85 + 0.15 * clamp( 1.5 * cos( 3.14 * uv.x * 640.0 * 1.0 ), 0.0, 1.0 );
    color *= scanline * grille * 1.2;
}

void main() {
    vec2 crtUV = CRTCurveUV(uv);
    vec3 res = texture2D(Texture, uv).rgb * color.rgb;
    if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0)
    {
        res = vec3(0.0, 0.0, 0.0);
    }
    DrawVignette(res, crtUV);
    DrawScanline(res, uv);
    gl_FragColor = vec4(res, 1.0);

}
"#;

pub const VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}
";

pub static POSSIBLE_POSITIONS: [[[(usize, usize); 4]; 4]; 7] = [
    [
        [(1, 0), (1, 1), (1, 2), (1, 3)],
        [(0, 2), (1, 2), (2, 2), (3, 2)],
        [(2, 0), (2, 1), (2, 2), (2, 3)],
        [(0, 1), (1, 1), (2, 1), (3, 1)],
    ],
    [
        [(0, 0), (0, 1), (1, 0), (1, 1)],
        [(0, 0), (0, 1), (1, 0), (1, 1)],
        [(0, 0), (0, 1), (1, 0), (1, 1)],
        [(0, 0), (0, 1), (1, 0), (1, 1)],
    ],
    [
        [(0, 1), (1, 0), (1, 1), (1, 2)],
        [(0, 1), (1, 1), (1, 2), (2, 1)],
        [(1, 0), (1, 1), (1, 2), (2, 1)],
        [(0, 1), (1, 0), (1, 1), (2, 1)],
    ],
    [
        [(0, 2), (1, 0), (1, 1), (1, 2)],
        [(0, 1), (1, 1), (2, 1), (2, 2)],
        [(1, 0), (1, 1), (1, 2), (2, 0)],
        [(0, 0), (0, 1), (1, 1), (2, 1)],
    ],
    [
        [(0, 1), (0, 2), (1, 0), (1, 1)],
        [(0, 1), (1, 1), (1, 2), (2, 2)],
        [(1, 1), (1, 2), (2, 0), (2, 1)],
        [(0, 0), (1, 0), (1, 1), (2, 1)],
    ],
    [
        [(0, 0), (0, 1), (1, 1), (1, 2)],
        [(0, 2), (1, 1), (1, 2), (2, 1)],
        [(1, 0), (1, 1), (2, 1), (2, 2)],
        [(0, 1), (1, 0), (1, 1), (2, 0)],
    ],
    [
        [(0, 0), (1, 0), (1, 1), (1, 2)],
        [(0, 1), (0, 2), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (1, 2), (2, 2)],
        [(0, 1), (1, 1), (2, 0), (2, 1)],
    ],
];
