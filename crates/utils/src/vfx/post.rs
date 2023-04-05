use bevy::{
  core_pipeline::{
    core_3d, fullscreen_vertex_shader::fullscreen_shader_vertex_state, prepass::ViewPrepassTextures,
  },
  prelude::*,
  render::{
    extract_component::{
      ComponentUniforms, ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin,
    },
    render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext, SlotInfo, SlotType},
    render_resource::{
      BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
      BindGroupLayoutEntry, BindingResource, BindingType, BufferBindingType,
      CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, MultisampleState,
      Operations, PipelineCache, PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor,
      RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
      ShaderType, TextureFormat, TextureSampleType, TextureViewDimension,
    },
    renderer::{RenderContext, RenderDevice},
    texture::BevyDefault,
    view::{ExtractedView, ViewTarget, ViewUniforms},
    RenderApp,
  },
};

pub struct PostProcessPlugin;
impl Plugin for PostProcessPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(ExtractComponentPlugin::<PostProcessSettings>::default())
      .add_plugin(UniformComponentPlugin::<PostProcessSettings>::default());

    let render_app = app
      .get_sub_app_mut(RenderApp)
      .expect("should have a render app");

    render_app.init_resource::<PostProcessPipeline>();

    let node = PostProcessNode::new(&mut render_app.world);
    let mut graph = render_app.world.resource_mut::<RenderGraph>();
    let core_3d_graph = graph.get_sub_graph_mut(core_3d::graph::NAME).unwrap();

    core_3d_graph.add_node(PostProcessNode::NAME, node);

    core_3d_graph.add_slot_edge(
      core_3d_graph.input_node().id,
      core_3d::graph::input::VIEW_ENTITY,
      PostProcessNode::NAME,
      PostProcessNode::IN_VIEW,
    );

    core_3d_graph.add_node_edge(core_3d::graph::node::MAIN_PASS, PostProcessNode::NAME);
    core_3d_graph.add_node_edge(PostProcessNode::NAME, core_3d::graph::node::TONEMAPPING);
  }
}

struct PostProcessNode {
  query: QueryState<(&'static ViewTarget, &'static ViewPrepassTextures), With<ExtractedView>>,
}

impl PostProcessNode {
  pub const IN_VIEW: &str = "view";
  pub const NAME: &str = "post_process";

  fn new(world: &mut World) -> Self {
    Self {
      query: QueryState::new(world),
    }
  }
}

impl Node for PostProcessNode {
  fn input(&self) -> Vec<SlotInfo> {
    vec![SlotInfo::new(PostProcessNode::IN_VIEW, SlotType::Entity)]
  }

  fn update(&mut self, world: &mut World) {
    self.query.update_archetypes(world);
  }

  fn run(
    &self,
    graph_context: &mut RenderGraphContext,
    render_context: &mut RenderContext,
    world: &World,
  ) -> Result<(), NodeRunError> {
    let view_entity = graph_context.get_input_entity(PostProcessNode::IN_VIEW)?;

    let Ok((view_target, prepass_textures)) = self.query.get_manual(world, view_entity) else {
        return Ok(());
    };

    let post_process_pipeline = world.resource::<PostProcessPipeline>();
    let pipeline_cache = world.resource::<PipelineCache>();

    let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id) else {
        return Ok(());
    };

    let settings_uniforms = world.resource::<ComponentUniforms<PostProcessSettings>>();
    let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
        return Ok(());
    };

    let post_process = view_target.post_process_write();

    let view_uniforms = world.resource::<ViewUniforms>();

    let Some(view_binding) = view_uniforms.uniforms.binding() else {
        return Ok(());
    };

    let bind_group_descriptor = BindGroupDescriptor {
      label: Some("post_process_bind_group"),
      layout: &post_process_pipeline.layout,
      entries: &[
        // screen texture
        BindGroupEntry {
          binding: 0,
          resource: BindingResource::TextureView(post_process.source),
        },
        // sampler
        BindGroupEntry {
          binding: 1,
          resource: BindingResource::Sampler(&post_process_pipeline.sampler),
        },
        // depth
        BindGroupEntry {
          binding: 2,
          resource: BindingResource::TextureView(
            &prepass_textures.depth.as_ref().unwrap().default_view,
          ),
        },
        // normal
        BindGroupEntry {
          binding: 3,
          resource: BindingResource::TextureView(
            &prepass_textures.normal.as_ref().unwrap().default_view,
          ),
        },
        // view
        BindGroupEntry {
          binding: 4,
          resource: view_binding.clone(),
        },
        // config
        BindGroupEntry {
          binding: 5,
          resource: settings_binding.clone(),
        },
      ],
    };

    let bind_group = render_context
      .render_device()
      .create_bind_group(&bind_group_descriptor);

    let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
      label: Some("post_process_pass"),
      color_attachments: &[Some(RenderPassColorAttachment {
        view: post_process.destination,
        resolve_target: None,
        ops: Operations::default(),
      })],
      depth_stencil_attachment: None,
    });

    render_pass.set_render_pipeline(pipeline);
    render_pass.set_bind_group(0, &bind_group, &[]);
    render_pass.draw(0..3, 0..1);

    Ok(())
  }
}

// This contains global data used by the render pipeline. This will be created once on startup.
#[derive(Resource)]
struct PostProcessPipeline {
  layout: BindGroupLayout,
  sampler: Sampler,
  pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for PostProcessPipeline {
  fn from_world(world: &mut World) -> Self {
    let render_device = world.resource::<RenderDevice>();

    // We need to define the bind group layout used for our pipeline
    let layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
      label: Some("post_process_bind_group_layout"),
      entries: &[
        BindGroupLayoutEntry {
          binding: 0,
          visibility: ShaderStages::FRAGMENT,
          ty: BindingType::Texture {
            sample_type: TextureSampleType::Float { filterable: true },
            view_dimension: TextureViewDimension::D2,
            multisampled: false,
          },
          count: None,
        },
        BindGroupLayoutEntry {
          binding: 1,
          visibility: ShaderStages::FRAGMENT,
          ty: BindingType::Sampler(SamplerBindingType::Filtering),
          count: None,
        },
        // Depth
        BindGroupLayoutEntry {
          binding: 2,
          visibility: ShaderStages::FRAGMENT,
          ty: BindingType::Texture {
            multisampled: false,
            sample_type: TextureSampleType::Depth,
            view_dimension: TextureViewDimension::D2,
          },
          count: None,
        },
        // Normals
        BindGroupLayoutEntry {
          binding: 3,
          visibility: ShaderStages::FRAGMENT,
          ty: BindingType::Texture {
            multisampled: false,
            sample_type: TextureSampleType::Float { filterable: true },
            view_dimension: TextureViewDimension::D2,
          },
          count: None,
        },
        // View
        BindGroupLayoutEntry {
          binding: 4,
          visibility: ShaderStages::FRAGMENT,
          ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
          },
          count: None,
        },
        // Config
        BindGroupLayoutEntry {
          binding: 5,
          visibility: ShaderStages::FRAGMENT,
          ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
          },
          count: None,
        },
      ],
    });

    // We can create the sampler here since it won't change at runtime and doesn't depend on the view
    let sampler = render_device.create_sampler(&SamplerDescriptor::default());

    // Get the shader handle
    let shader = world
      .resource::<AssetServer>()
      .load("shaders/post_process_pass.wgsl");

    let pipeline_id = world
      .resource_mut::<PipelineCache>()
      // This will add the pipeline to the cache and queue it's creation
      .queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("post_process_pipeline".into()),
        layout: vec![layout.clone()],
        // This will setup a fullscreen triangle for the vertex state
        vertex: fullscreen_shader_vertex_state(),
        fragment: Some(FragmentState {
          shader,
          shader_defs: vec![],
          entry_point: "fragment".into(),
          targets: vec![Some(ColorTargetState {
            format: TextureFormat::Rgba16Float,
            blend: None,
            write_mask: ColorWrites::ALL,
          })],
        }),
        // All of the following property are not important for this effect so just use the default values.
        // This struct doesn't have the Default trai implemented because not all field can have a default value.
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        multisample: MultisampleState::default(),
        push_constant_ranges: vec![],
      });

    Self {
      layout,
      sampler,
      pipeline_id,
    }
  }
}

// This is the component that will get passed to the shader
#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
pub struct PostProcessSettings {
  pub depth_threshold: f32,
  pub normal_threshold: f32,
  pub color_threshold: f32,
  pub edge_color: Color,
  pub debug: u32,
  pub enabled: u32,
}

impl Default for PostProcessSettings {
  fn default() -> Self {
    Self {
      depth_threshold: 0.2,
      normal_threshold: 0.05,
      color_threshold: 1.0,
      edge_color: Color::BLACK,
      debug: 0,
      enabled: 1,
    }
  }
}
