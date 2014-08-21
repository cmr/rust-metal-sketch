pub trait Resource;
pub trait Buffer : Resource { }
pub trait Texture : Resource { }
pub trait Sampler;
pub trait ShaderProgram;
pub trait CommandQueue;

pub struct BufferHints;
pub struct TextureDescriptor;
pub struct SamplerDescriptor;
pub struct DepthStencilStateDescriptor;
pub struct RenderPipelineDescriptor;

pub trait Device {
    // error types. use proposed ToError/Error trait instead of Show.
    type ShaderProgramCreationError : Show;
    type CommandQueueCreationError : Show;
    type BufferCreationError : Show;
    type SamplerCreationError : Show;
    type RenderPipelineCreationError : Show;

    // input types
    type ShaderProgramInput;

    // "handle" types
    type ShaderProgram : ShaderProgram;
    type CommandQueue : CommandQueue;
    type Buffer : Buffer;
    type Texture : Texture;
    type Sampler : Sampler;
    type DepthStencilState;
    type RenderPipeline;

    /// Create a new shader program.
    ///
    /// The new shader program is a "ready to use" shader program that can be used for rendering.
    ///
    /// See the `ShaderProgram` trait for more details on how a shader program is used.
    fn create_shader_program(&mut self, source: ShaderProgramInput) -> Result<ShaderProgram, ShaderProgramCreationError>;

    /// Create a new command queue.
    ///
    /// The command queue will have a maximum of `count` uncompleted command buffers if it is
    /// `Some`, otherwise it will be unbounded and grow as necessary.
    ///
    /// See the `CommandQueue` trait for more details on how a `CommandQueue` is used.
    fn create_command_queue(&mut self, count: Option<uint>) -> Result<CommandQueue, CommandQueueCreationError>;

    /// Create a new buffer.
    ///
    /// The buffer will have the capacity to store `length` bytes. If `Some`, the `hints` will
    /// influence the behavior of the created buffer. Otherwise, the default hints shall be used.
    /// These hints cannot be changed after creation.
    ///
    /// See the `BufferHints` struct for the exact set of hints which may be used.
    fn create_buffer(&mut self, length: uint, hints: Option<BufferHints>) -> Result<Buffer, BufferCreationError>;

    // todo: no-copy buffer creation, `newBufferWithBytesNoCopy:length:options:deallocator` in
    // metal-speak. in particular, it takes a page-aligned buffer and shares it with the gpu
    // directly, calling the deallocator function when it is done with it.

    /// Create a new texture.
    ///
    /// The `desc` describes the layout that this texture has.
    ///
    /// See the `TextureDescriptor` struct for the exact set of properties that a texture
    /// encompasses.
    fn create_texture(&mut self, desc: TextureDescriptor) -> Result<Texture, TextureCreationError>;

    /// Create a new sampler.
    ///
    /// A sampler is a description of how a shader should sample a texture. Once created, it cannot
    /// be changed. The `desc` describes the properties the sampler will have.
    ///
    /// See the `SamplerDescriptor` struct for the exact set of properties that a sampler
    /// encompasses.
    fn create_sampler(&mut self, desc: SamplerDescriptor) -> Result<Sampler, SamplerCreationError>;

    /// Create a new depth/stencil state.
    ///
    /// This depth/stencil test state is used to configure the depth and stencil stages of the
    /// rendering pipeline. The `desc` is the configuration that this state will use.
    ///
    /// See the `DepthStencilStateDescriptor` struct for the configuration options available.

    // note: this can't fail. There really isn't any work a device would have to do for this, and I
    // suspect most implementations will use the DepthStencilStateDescriptor as the
    // DepthStencilState directly!
    fn create_depth_stencil_state(&mut self, desc: DepthStencilStateDescriptor) -> DepthStencilState;

    /// Create a render pipeline.
    ///
    /// A render pipeline encodes the state required to issue a draw call. It can be relatively
    /// expensive to construct a render pipeline, so it should be done as infrequently as possible.
    /// The `desc` contains the state the render pipeline encodes.
    ///
    /// See the `RenderPipelineDescriptor` struct for the state that must be set.
    fn create_render_pipeline(&mut self, desc: RenderPipelineDescriptor) -> Result<RenderPipeline, RenderPipelineCreationError>;

    // todo: pipeline reflection
    // todo: compute
}
