use std::borrow::Cow;

use bevy::{
    core::FloatOrd,
    core_pipeline::{Opaque3d, Transparent3d},
    pbr::{
        DrawMesh, MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup,
        SetMeshViewBindGroup,
    },
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_asset::RenderAssets,
        render_phase::{AddRenderCommand, DrawFunctions, RenderPhase, SetItemPipeline},
        render_resource::{
            RenderPipelineCache, RenderPipelineDescriptor, SpecializedPipeline,
            SpecializedPipelines, TextureFormat, VertexAttribute, VertexBufferLayout, VertexFormat,
            VertexState, VertexStepMode,
        },
        view::ExtractedView,
        RenderApp, RenderStage,
    },
};

#[derive(Component, Default)]
pub struct StylizedWireframe;

pub struct StylizedWireframePipeline {
    mesh_pipeline: MeshPipeline,
    shader: Handle<Shader>,
}

impl FromWorld for StylizedWireframePipeline {
    fn from_world(render_world: &mut World) -> Self {
        Self {
            mesh_pipeline: render_world.get_resource::<MeshPipeline>().unwrap().clone(),
            shader: STYLIZED_WIREFRAME_SHADER_HANDLE.typed(),
        }
    }
}

impl SpecializedPipeline for StylizedWireframePipeline {
    type Key = MeshPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut descriptor = self.mesh_pipeline.specialize(key);
        descriptor.label = Some(Cow::Borrowed("stylized_wireframe_pipeline"));
        descriptor.vertex.shader = self.shader.clone_weak();
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone_weak();
        descriptor.primitive.cull_mode = None;

        // Barycentric_Position Vec3
        // Vertex_Normal Vec3
        // Vertex_Position Vec3
        // Vertex_Uv Vec2

        let mut attributes = Vec::new();
        let mut cursor = 0;

        // Barycentric_Position
        attributes.push(VertexAttribute {
            format: VertexFormat::Float32x3,
            offset: cursor,
            shader_location: 3,
        });
        cursor += VertexFormat::Float32x3.size();

        // Vertex_Normal
        attributes.push(VertexAttribute {
            format: VertexFormat::Float32x3,
            offset: cursor,
            shader_location: 1,
        });
        cursor += VertexFormat::Float32x3.size();

        // Vertex_Position
        attributes.push(VertexAttribute {
            format: VertexFormat::Float32x3,
            offset: cursor,
            shader_location: 0,
        });
        cursor += VertexFormat::Float32x3.size();

        // Vertex_Uv
        attributes.push(VertexAttribute {
            format: VertexFormat::Float32x2,
            offset: cursor,
            shader_location: 2,
        });
        cursor += VertexFormat::Float32x2.size();

        descriptor.vertex.buffers = vec![VertexBufferLayout {
            array_stride: cursor,
            step_mode: VertexStepMode::Vertex,
            attributes: attributes,
        }];

        //dbg!(&descriptor);
        descriptor
    }
}

// This specifies how to render a colored 2d mesh
type DrawStylizedWireframes = (
    // Set the pipeline
    SetItemPipeline,
    // Set the view uniform as bind group 0
    SetMeshViewBindGroup<0>,
    // Set the mesh uniform as bind group 1
    SetMeshBindGroup<1>,
    // Draw the mesh
    DrawMesh,
);

/// Handle to the custom shader with a unique random ID
pub const STYLIZED_WIREFRAME_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 4260050316210531118);

#[derive(Debug, Default)]
pub struct StylizedWireframePlugin;

impl Plugin for StylizedWireframePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        shaders.set_untracked(
            STYLIZED_WIREFRAME_SHADER_HANDLE,
            Shader::from_wgsl(include_str!("shaders/wireframe.wgsl")),
        );

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .add_render_command::<Transparent3d, DrawStylizedWireframes>()
                .init_resource::<StylizedWireframePipeline>()
                .init_resource::<SpecializedPipelines<StylizedWireframePipeline>>()
                .add_system_to_stage(RenderStage::Extract, extract_wireframes)
                .add_system_to_stage(RenderStage::Queue, queue_wireframes);
        }
    }
}

fn extract_wireframes(mut commands: Commands, query: Query<Entity, With<StylizedWireframe>>) {
    for entity in query.iter() {
        commands.get_or_spawn(entity).insert(StylizedWireframe);
    }
}

fn queue_wireframes(
    opaque_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
    render_meshes: Res<RenderAssets<Mesh>>,
    wireframe_pipeline: Res<StylizedWireframePipeline>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    mut specialized_pipelines: ResMut<SpecializedPipelines<StylizedWireframePipeline>>,
    msaa: Res<Msaa>,
    material_meshes: Query<(Entity, &Handle<Mesh>, &MeshUniform), With<StylizedWireframe>>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent3d>)>,
) {
    let draw_custom = opaque_3d_draw_functions
        .read()
        .get_id::<DrawStylizedWireframes>()
        .unwrap();

    let key = MeshPipelineKey::from_msaa_samples(msaa.samples);
    for (view, mut transparent_phase) in views.iter_mut() {
        let view_matrix = view.transform.compute_matrix();
        let view_row_2 = view_matrix.row(2);

        let add_render_phase =
            |(entity, mesh_handle, mesh_uniform): (Entity, &Handle<Mesh>, &MeshUniform)| {
                if let Some(mesh) = render_meshes.get(mesh_handle) {
                    let key =
                        key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
                    transparent_phase.add(Transparent3d {
                        entity,
                        pipeline: specialized_pipelines.specialize(
                            &mut pipeline_cache,
                            &wireframe_pipeline,
                            key,
                        ),
                        draw_function: draw_custom,
                        distance: view_row_2.dot(mesh_uniform.transform.col(3)),
                    });
                }
            };

        material_meshes.iter().for_each(add_render_phase);
    }
}
