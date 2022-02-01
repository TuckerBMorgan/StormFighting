precision mediump float;

in vec2 v_uv;
in vec4 v_color;
out vec4 a_color;

uniform sampler2D texT;

 
void main() {

    float count = 1080 * 2.5;// * 2.5;
    vec2 sl = vec2(sin(gl_FragCoord.y * count), cos(gl_FragCoord.y * count));
	vec4 scanlines = vec4(sl.x, sl.x, sl.x, 1.0);
    scanlines += vec4(0.01, 0.01, 0.01, 0.0);
    a_color = texture(texT, v_uv) * v_color;
    a_color *= scanlines * 1.3;

    if (a_color.a <= 0.0) {
        discard;
    }
}