#version 450

layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 1) uniform texture2D Voxel_Texture;
layout(set = 0, binding = 2) uniform sampler Voxel_Sampler;

void main() {
    o_Target = texture(sampler2D(Voxel_Texture, Voxel_Sampler), v_Uv);
}
