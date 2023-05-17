use bevy::{
    prelude::*,
    render::render_resource::*,
};
use futures_lite::future::block_on;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(cellular_automata)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut shaders: ResMut<Assets<Shader>>,
) {
    // Load the WGSL shader
    let shader_handle = asset_server.load_shader("shaders/automata.wgsl");

    // Create the compute pipeline
    let compute_pipeline = ComputePipelineDescriptor {
        shader_stages: ShaderStages {
            compute: ShaderStage {
                shader: shader_handle,
                entry_point: "main".to_string(),
            },
        },
    };

    // Add the pipeline as a resource
    commands.insert_resource(compute_pipeline);
}

fn cellular_automata(
    mut state: ResMut<VoxelWorld>,
    pipeline: Res<ComputePipelineDescriptor>,
    render_device: Res<RenderDevice>,
    render_queue: ResMut<RenderQueue>,
) {
    // Create a buffer to store the voxel world data
    let voxel_buffer = render_device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Voxel Buffer"),
        contents: bytemuck::cast_slice(&state.data),
        usage: BufferUsages::STORAGE,
    });

    // Create a binding group for the compute pipeline
    let binding_group = render_device.create_binding_group(&BindingGroupDescriptor {
        label: Some("Voxel Binding Group"),
        layout: &pipeline.layout,
        entries: &[BindingGroupEntry {
            binding: 0,
            resource: BindingResource::Buffer(voxel_buffer),
        }],
    });

    // Dispatch the compute shader
    render_queue.write_compute_command(&ComputeCommand {
        pipeline: &pipeline.compute,
        binding_group: &binding_group,
        workgroup_count: [state.width / 16, state.height / 16, 1],
    });

    // Read the results back into the VoxelWorld
    let buffer_slice = voxel_buffer.slice(..);
    let future = render_queue.read_buffer(buffer_slice);
    let buffer_data = block_on(future);

    // Update the VoxelWorld with the results
    state.data.copy_from_slice(bytemuck::cast_slice::<u8, u32>(&buffer_data));
}

// voxel_world.rs
pub struct VoxelWorld {
    pub data: Vec<u32>,
    pub width: u32,
    pub height: u32,
}

impl VoxelWorld {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            data: vec![0; (width * height) as usize],
            width,
            height,
        }
    }

    // Add methods for world generation, updates, and interactions as needed
}
