#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec2 y_uv;
layout(location = 1) in vec2 u_uv;
layout(location = 2) in vec2 v_uv;
layout(location = 0) out vec4 target0;

layout(set = 0, binding = 0) uniform texture2D u_texture;
layout(set = 0, binding = 1) uniform sampler u_sampler;

void main() {
    mediump vec3 yuv;
    mediump vec3 rgb;
    yuv.x = texture(sampler2D(u_texture, u_sampler), y_uv).x;
    yuv.y = texture(sampler2D(u_texture, u_sampler), u_uv).x - 0.5;
    yuv.z = texture(sampler2D(u_texture, u_sampler), v_uv).x - 0.5;

    rgb.r = 2*(yuv.x/2 + 1.402/2 * yuv.z);
    rgb.g = 2*(yuv.x/2 - 0.344136 * yuv.y/2 - 0.714136 * yuv.z/2);
    rgb.b = 2*(yuv.x/2 + 1.773/2 * yuv.y);

    target0 = vec4(rgb.rgb, 1);
}
