[[block]]
struct VoxelBuffer {
    data: [[stride(4)]] array<u32>;
};

[[binding(0), group(0)]]
var<storage> voxel_buffer: [[access(read_write)]] VoxelBuffer;

[[stage(compute), workgroup_size(16, 16)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    let x: u32 = global_id.x;
    let y: u32 = global_id.y;

    // Add your cellular automata logic here

    // Write the result back to the voxel buffer
    voxel_buffer.data[y * 16 + x] = result;
}