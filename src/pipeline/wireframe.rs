use bevy::pbr::MeshPipeline;
use bevy::pbr::{DrawMesh, MeshPipelineKey, MeshUniform, SetMeshBindGroup, SetMeshViewBindGroup};
use bevy::app::Plugin;
use bevy::asset::{Assets, Handle, HandleUntyped};
use bevy::core_pipeline::Opaque3d;
use bevy::ecs::{prelude::*, reflect::ReflectComponent};
use bevy::reflect::{Reflect, TypeUuid};
use bevy::render::render_resource::PolygonMode;
use bevy::render::{
    mesh::Mesh,
    render_asset::RenderAssets,
    render_phase::{AddRenderCommand, DrawFunctions, RenderPhase, SetItemPipeline},
    render_resource::{RenderPipelineCache, Shader, SpecializedPipeline, SpecializedPipelines},
    view::{ExtractedView, Msaa},
    RenderApp, RenderStage,
};

pub const STYLIZED_WIREFRAME_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 363712210031989780);

#[derive(Debug, Default)]
pub struct StylizedWireframePlugin;

impl Plugin for StylizedWireframePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        shaders.set_untracked(
            STYLIZED_WIREFRAME_SHADER_HANDLE,
            Shader::from_wgsl(include_str!("render/stylized_wireframe.wgsl")),
        );

        app.sub_app_mut(RenderApp)
            .add_render_command::<Opaque3d, DrawStylizedWireframes>()
            .init_resource::<StylizedWireframePipeline>()
            .init_resource::<SpecializedPipelines<StylizedWireframePipeline>>()
            .add_system_to_stage(RenderStage::Extract, extract_wireframes)
            //.add_system_to_stage(RenderStage::Extract, extract_wireframe_config)
            .add_system_to_stage(RenderStage::Queue, queue_wireframes);
    }
}

fn extract_wireframes(mut commands: Commands, query: Query<Entity, With<StylizedWireframe>>) {
    for entity in query.iter() {
        commands.get_or_spawn(entity).insert(StylizedWireframe);
    }
}

/// Controls whether an entity should rendered in wireframe-mode if the [`WireframePlugin`] is enabled
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct StylizedWireframe;

pub struct StylizedWireframePipeline {
    mesh_pipeline: MeshPipeline,
    shader: Handle<Shader>,
}
impl FromWorld for StylizedWireframePipeline {
    fn from_world(render_world: &mut World) -> Self {
        StylizedWireframePipeline {
            mesh_pipeline: render_world.get_resource::<MeshPipeline>().unwrap().clone(),
            shader: STYLIZED_WIREFRAME_SHADER_HANDLE.typed(),
        }
    }
}

impl SpecializedPipeline for StylizedWireframePipeline {
    type Key = MeshPipelineKey;

    fn specialize(&self, key: Self::Key) -> bevy::render::render_resource::RenderPipelineDescriptor {
        let mut descriptor = self.mesh_pipeline.specialize(key);
        descriptor.vertex.shader = self.shader.clone_weak();
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone_weak();
        descriptor.primitive.polygon_mode = PolygonMode::Fill;
        descriptor.depth_stencil.as_mut().unwrap().bias.slope_scale = 1.0;
        descriptor
    }
}

#[allow(clippy::too_many_arguments)]
fn queue_wireframes(
    opaque_3d_draw_functions: Res<DrawFunctions<Opaque3d>>,
    render_meshes: Res<RenderAssets<Mesh>>,
    wireframe_pipeline: Res<StylizedWireframePipeline>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    mut specialized_pipelines: ResMut<SpecializedPipelines<StylizedWireframePipeline>>,
    msaa: Res<Msaa>,
    material_meshes: Query<(Entity, &Handle<Mesh>, &MeshUniform), With<StylizedWireframe>>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Opaque3d>)>,
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
                    transparent_phase.add(Opaque3d {
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

type DrawStylizedWireframes = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    DrawMesh,
);
