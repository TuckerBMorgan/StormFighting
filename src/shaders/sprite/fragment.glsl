precision mediump float;

in vec2 v_uv;
in vec4 v_color;
out vec4 a_color;
uniform sampler2D texT;

layout(std140) uniform vertex {
    mat4 ortho;
    vec3 color_pallete_in[5];
    vec3 color_pallete_out[5];
};

float border (vec2 uv){
	float radius = min(1.0, 0.08);
	radius = max(min(min(abs(radius * 2.0), abs(1.0)), abs(1.0)), 1e-5);
	vec2 abs_uv = abs(uv * 2.0 - 1.0) - vec2(1.0, 1.0) + radius;
	float dist = length(max(vec2(0.0), abs_uv)) / radius;
	float square = smoothstep(0.96, 1.0, dist);
	return clamp(1.0 - square, 0.0, 1.0);
}

 vec2 warp(vec2 uv){
	vec2 delta = uv - 0.5;
	float delta2 = dot(delta.xy, delta.xy);
	float delta4 = delta2 * delta2;
	float delta_offset = delta4 * 1.0;
	
	return uv + delta * delta_offset;
}
 
void main() {
    float screen_div_factor = 4.0;
    float screen_width = 1440.0 / screen_div_factor;
    float screen_height = 1080.0 / screen_div_factor;
    vec2 screen_uv = warp(vec2(gl_FragCoord.x / screen_div_factor / screen_width, gl_FragCoord.y / screen_div_factor / screen_height ));

    float scanlines_width = 0.25;
    a_color = texture(texT, v_uv) * v_color;
    float r = a_color.r;
    float g = a_color.g;
    float b = a_color.b;

    float g_r = smoothstep(0.85, 0.95, abs(sin(screen_uv.x  * (screen_width * 3.14159265))));
    r = mix(r, r * g_r, 0.3);
    
    float g_g = smoothstep(0.85, 0.95, abs(sin(1.05 + screen_uv.x  * (screen_width * 3.14159265))));
    g = mix(g, g * g_g, 0.3);
    
    float b_b = smoothstep(0.85, 0.95, abs(sin(2.1 + screen_uv.x * (screen_width * 3.14159265))));
    b = mix(b, b * b_b, 0.3);
    
    a_color.r = clamp(r * 1.4, 0.0, 1.0);
    a_color.g = clamp(g * 1.4, 0.0, 1.0);
    a_color.b = clamp(b * 1.4, 0.0, 1.0);
    
    float scanlines  = 0.5;
    scanlines = smoothstep(scanlines_width, scanlines_width + 0.5, abs(sin(screen_uv.x * (screen_height * 3.14159265))));
    a_color.rgb = mix(a_color.rgb, a_color.rgb * vec3(scanlines), 0.4);
    a_color.rgb *= border(screen_uv);

    if (a_color.a <= 0.0) {
        discard;
    }
}