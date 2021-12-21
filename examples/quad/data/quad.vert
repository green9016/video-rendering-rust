#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(constant_id = 0) const float scale = 1.2f;

layout(location = 0) in vec2 a_pos;
layout(location = 1) in vec2 a_uv;
layout(location = 0) out vec2 y_uv;
layout(location = 1) out vec2 u_uv;
layout(location = 2) out vec2 v_uv;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    //y_uv = a_uv;
    y_uv = vec2(a_uv.x, a_uv.y * 2.0 / 3.0);
    u_uv = vec2(a_uv.x * 0.5, a_uv.y * 1.0 / 6.0 + 2.0 / 3.0);
    v_uv = vec2(a_uv.x * 0.5, a_uv.y * 1.0 / 6.0 + 1.0 / 6.0 + 2.0 / 3.0);
    gl_Position = vec4(scale * a_pos, 0.0, 1.0);
}
