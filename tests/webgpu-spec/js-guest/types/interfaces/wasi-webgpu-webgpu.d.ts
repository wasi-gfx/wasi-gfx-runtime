declare module 'wasi:webgpu/webgpu@0.3.0-rc.2' {
  export function getGpu(): Gpu;
  export type CreateQuerySetErrorKind = CreateQuerySetErrorKindTypeError;
  export interface CreateQuerySetErrorKindTypeError {
    tag: 'type-error',
  }
  export interface CreateQuerySetError {
    kind: CreateQuerySetErrorKind,
    message: string,
  }
  export type GetMappedRangeErrorKind = GetMappedRangeErrorKindOperationError | GetMappedRangeErrorKindRangeError | GetMappedRangeErrorKindTypeError;
  export interface GetMappedRangeErrorKindOperationError {
    tag: 'operation-error',
  }
  export interface GetMappedRangeErrorKindRangeError {
    tag: 'range-error',
  }
  export interface GetMappedRangeErrorKindTypeError {
    tag: 'type-error',
  }
  export interface GetMappedRangeError {
    kind: GetMappedRangeErrorKind,
    message: string,
  }
  /**
   * # Variants
   * 
   * ## `"clamp-to-edge"`
   * 
   * ## `"repeat"`
   * 
   * ## `"mirror-repeat"`
   */
  export type GpuAddressMode = "clamp-to-edge" | "repeat" | "mirror-repeat";
  /**
   * # Variants
   * 
   * ## `"zero"`
   * 
   * ## `"one"`
   * 
   * ## `"src"`
   * 
   * ## `"one-minus-src"`
   * 
   * ## `"src-alpha"`
   * 
   * ## `"one-minus-src-alpha"`
   * 
   * ## `"dst"`
   * 
   * ## `"one-minus-dst"`
   * 
   * ## `"dst-alpha"`
   * 
   * ## `"one-minus-dst-alpha"`
   * 
   * ## `"src-alpha-saturated"`
   * 
   * ## `"constant"`
   * 
   * ## `"one-minus-constant"`
   * 
   * ## `"src1"`
   * 
   * ## `"one-minus-src1"`
   * 
   * ## `"src1-alpha"`
   * 
   * ## `"one-minus-src1-alpha"`
   */
  export type GpuBlendFactor = "zero" | "one" | "src" | "one-minus-src" | "src-alpha" | "one-minus-src-alpha" | "dst" | "one-minus-dst" | "dst-alpha" | "one-minus-dst-alpha" | "src-alpha-saturated" | "constant" | "one-minus-constant" | "src1" | "one-minus-src1" | "src1-alpha" | "one-minus-src1-alpha";
  /**
   * # Variants
   * 
   * ## `"add"`
   * 
   * ## `"subtract"`
   * 
   * ## `"reverse-subtract"`
   * 
   * ## `"min"`
   * 
   * ## `"max"`
   */
  export type GpuBlendOperation = "add" | "subtract" | "reverse-subtract" | "min" | "max";
  export interface GpuBlendComponent {
    operation?: GpuBlendOperation,
    srcFactor?: GpuBlendFactor,
    dstFactor?: GpuBlendFactor,
  }
  export interface GpuBlendState {
    color: GpuBlendComponent,
    alpha: GpuBlendComponent,
  }
  /**
   * # Variants
   * 
   * ## `"uniform"`
   * 
   * ## `"storage"`
   * 
   * ## `"read-only-storage"`
   */
  export type GpuBufferBindingType = "uniform" | "storage" | "read-only-storage";
  export type GpuBufferDynamicOffset = number;
  /**
   * # Variants
   * 
   * ## `"unmapped"`
   * 
   * ## `"pending"`
   * 
   * ## `"mapped"`
   */
  export type GpuBufferMapState = "unmapped" | "pending" | "mapped";
  export interface GpuBufferUsage {
    mapRead?: boolean,
    mapWrite?: boolean,
    copySrc?: boolean,
    copyDst?: boolean,
    index?: boolean,
    vertex?: boolean,
    uniform?: boolean,
    storage?: boolean,
    indirect?: boolean,
    queryResolve?: boolean,
  }
  /**
   * # Variants
   * 
   * ## `"opaque"`
   * 
   * ## `"premultiplied"`
   */
  export type GpuCanvasAlphaMode = "opaque" | "premultiplied";
  /**
   * # Variants
   * 
   * ## `"standard"`
   * 
   * ## `"extended"`
   */
  export type GpuCanvasToneMappingMode = "standard" | "extended";
  export interface GpuCanvasToneMapping {
    mode?: GpuCanvasToneMappingMode,
  }
  export interface GpuColor {
    r: number,
    g: number,
    b: number,
    a: number,
  }
  export interface GpuColorWrite {
    red?: boolean,
    green?: boolean,
    blue?: boolean,
    alpha?: boolean,
    all?: boolean,
  }
  export interface GpuCommandBufferDescriptor {
    label?: string,
  }
  export interface GpuCommandEncoderDescriptor {
    label?: string,
  }
  /**
   * # Variants
   * 
   * ## `"never"`
   * 
   * ## `"less"`
   * 
   * ## `"equal"`
   * 
   * ## `"less-equal"`
   * 
   * ## `"greater"`
   * 
   * ## `"not-equal"`
   * 
   * ## `"greater-equal"`
   * 
   * ## `"always"`
   */
  export type GpuCompareFunction = "never" | "less" | "equal" | "less-equal" | "greater" | "not-equal" | "greater-equal" | "always";
  /**
   * # Variants
   * 
   * ## `"error"`
   * 
   * ## `"warning"`
   * 
   * ## `"info"`
   */
  export type GpuCompilationMessageType = "error" | "warning" | "info";
  /**
   * # Variants
   * 
   * ## `"none"`
   * 
   * ## `"front"`
   * 
   * ## `"back"`
   */
  export type GpuCullMode = "none" | "front" | "back";
  export type GpuDepthBias = number;
  /**
   * # Variants
   * 
   * ## `"unknown"`
   * 
   * ## `"destroyed"`
   */
  export type GpuDeviceLostReason = "unknown" | "destroyed";
  /**
   * # Variants
   * 
   * ## `"validation"`
   * 
   * ## `"out-of-memory"`
   * 
   * ## `"internal"`
   */
  export type GpuErrorFilter = "validation" | "out-of-memory" | "internal";
  export type GpuErrorKind = GpuErrorKindValidationError | GpuErrorKindOutOfMemoryError | GpuErrorKindInternalError;
  export interface GpuErrorKindValidationError {
    tag: 'validation-error',
  }
  export interface GpuErrorKindOutOfMemoryError {
    tag: 'out-of-memory-error',
  }
  export interface GpuErrorKindInternalError {
    tag: 'internal-error',
  }
  /**
   * # Variants
   * 
   * ## `"core-features-and-limits"`
   * 
   * ## `"depth-clip-control"`
   * 
   * ## `"depth32float-stencil8"`
   * 
   * ## `"texture-compression-bc"`
   * 
   * ## `"texture-compression-bc-sliced3d"`
   * 
   * ## `"texture-compression-etc2"`
   * 
   * ## `"texture-compression-astc"`
   * 
   * ## `"texture-compression-astc-sliced3d"`
   * 
   * ## `"timestamp-query"`
   * 
   * ## `"indirect-first-instance"`
   * 
   * ## `"shader-f16"`
   * 
   * ## `"rg11b10ufloat-renderable"`
   * 
   * ## `"bgra8unorm-storage"`
   * 
   * ## `"float32-filterable"`
   * 
   * ## `"float32-blendable"`
   * 
   * ## `"clip-distances"`
   * 
   * ## `"dual-source-blending"`
   * 
   * ## `"subgroups"`
   * 
   * ## `"texture-formats-tier1"`
   * 
   * ## `"texture-formats-tier2"`
   * 
   * ## `"primitive-index"`
   * 
   * ## `"texture-component-swizzle"`
   */
  export type GpuFeatureName = "core-features-and-limits" | "depth-clip-control" | "depth32float-stencil8" | "texture-compression-bc" | "texture-compression-bc-sliced3d" | "texture-compression-etc2" | "texture-compression-astc" | "texture-compression-astc-sliced3d" | "timestamp-query" | "indirect-first-instance" | "shader-f16" | "rg11b10ufloat-renderable" | "bgra8unorm-storage" | "float32-filterable" | "float32-blendable" | "clip-distances" | "dual-source-blending" | "subgroups" | "texture-formats-tier1" | "texture-formats-tier2" | "primitive-index" | "texture-component-swizzle";
  /**
   * # Variants
   * 
   * ## `"nearest"`
   * 
   * ## `"linear"`
   */
  export type GpuFilterMode = "nearest" | "linear";
  /**
   * # Variants
   * 
   * ## `"ccw"`
   * 
   * ## `"cw"`
   */
  export type GpuFrontFace = "ccw" | "cw";
  /**
   * # Variants
   * 
   * ## `"uint16"`
   * 
   * ## `"uint32"`
   */
  export type GpuIndexFormat = "uint16" | "uint32";
  export type GpuIndex32 = number;
  export type GpuIntegerCoordinate = number;
  export interface GpuExtent3D {
    width: GpuIntegerCoordinate,
    height?: GpuIntegerCoordinate,
    depthOrArrayLayers?: GpuIntegerCoordinate,
  }
  export type GpuIntegerCoordinateOut = number;
  /**
   * # Variants
   * 
   * ## `"load"`
   * 
   * ## `"clear"`
   */
  export type GpuLoadOp = "load" | "clear";
  export interface GpuMapMode {
    read?: boolean,
    write?: boolean,
  }
  /**
   * # Variants
   * 
   * ## `"nearest"`
   * 
   * ## `"linear"`
   */
  export type GpuMipmapFilterMode = "nearest" | "linear";
  export interface GpuOrigin3D {
    x?: GpuIntegerCoordinate,
    y?: GpuIntegerCoordinate,
    z?: GpuIntegerCoordinate,
  }
  export type GpuPipelineConstantValue = number;
  /**
   * # Variants
   * 
   * ## `"validation"`
   * 
   * ## `"internal"`
   */
  export type GpuPipelineErrorReason = "validation" | "internal";
  export type CreatePipelineErrorKind = CreatePipelineErrorKindGpuPipelineError;
  export interface CreatePipelineErrorKindGpuPipelineError {
    tag: 'gpu-pipeline-error',
    val: GpuPipelineErrorReason,
  }
  export interface CreatePipelineError {
    kind: CreatePipelineErrorKind,
    message: string,
  }
  export type GpuLayoutMode = GpuLayoutModeSpecific | GpuLayoutModeAuto;
  export interface GpuLayoutModeSpecific {
    tag: 'specific',
    val: GpuPipelineLayout,
  }
  export interface GpuLayoutModeAuto {
    tag: 'auto',
  }
  /**
   * # Variants
   * 
   * ## `"low-power"`
   * 
   * ## `"high-performance"`
   */
  export type GpuPowerPreference = "low-power" | "high-performance";
  /**
   * # Variants
   * 
   * ## `"point-list"`
   * 
   * ## `"line-list"`
   * 
   * ## `"line-strip"`
   * 
   * ## `"triangle-list"`
   * 
   * ## `"triangle-strip"`
   */
  export type GpuPrimitiveTopology = "point-list" | "line-list" | "line-strip" | "triangle-list" | "triangle-strip";
  export interface GpuPrimitiveState {
    topology?: GpuPrimitiveTopology,
    stripIndexFormat?: GpuIndexFormat,
    frontFace?: GpuFrontFace,
    cullMode?: GpuCullMode,
    unclippedDepth?: boolean,
  }
  /**
   * # Variants
   * 
   * ## `"occlusion"`
   * 
   * ## `"timestamp"`
   */
  export type GpuQueryType = "occlusion" | "timestamp";
  export interface GpuQueueDescriptor {
    label?: string,
  }
  export interface GpuRenderBundleDescriptor {
    label?: string,
  }
  export interface GpuRequestAdapterOptions {
    featureLevel?: string,
    powerPreference?: GpuPowerPreference,
    forceFallbackAdapter?: boolean,
    xrCompatible?: boolean,
  }
  export type GpuSampleMask = number;
  /**
   * # Variants
   * 
   * ## `"filtering"`
   * 
   * ## `"non-filtering"`
   * 
   * ## `"comparison"`
   */
  export type GpuSamplerBindingType = "filtering" | "non-filtering" | "comparison";
  export interface GpuSamplerBindingLayout {
    type?: GpuSamplerBindingType,
  }
  export interface GpuSamplerDescriptor {
    addressModeU?: GpuAddressMode,
    addressModeV?: GpuAddressMode,
    addressModeW?: GpuAddressMode,
    magFilter?: GpuFilterMode,
    minFilter?: GpuFilterMode,
    mipmapFilter?: GpuMipmapFilterMode,
    lodMinClamp?: number,
    lodMaxClamp?: number,
    compare?: GpuCompareFunction,
    maxAnisotropy?: number,
    label?: string,
  }
  export interface GpuShaderModuleCompilationHint {
    entryPoint: string,
    layout?: GpuLayoutMode,
  }
  export interface GpuShaderModuleDescriptor {
    code: string,
    compilationHints?: Array<GpuShaderModuleCompilationHint>,
    label?: string,
  }
  export interface GpuShaderStage {
    vertex?: boolean,
    fragment?: boolean,
    compute?: boolean,
  }
  export type GpuSignedOffset32 = number;
  export type GpuSize32 = number;
  export interface GpuComputePassTimestampWrites {
    querySet: GpuQuerySet,
    beginningOfPassWriteIndex?: GpuSize32,
    endOfPassWriteIndex?: GpuSize32,
  }
  export interface GpuComputePassDescriptor {
    timestampWrites?: GpuComputePassTimestampWrites,
    label?: string,
  }
  export interface GpuMultisampleState {
    count?: GpuSize32,
    mask?: GpuSampleMask,
    alphaToCoverageEnabled?: boolean,
  }
  export interface GpuPipelineLayoutDescriptor {
    bindGroupLayouts: Array<GpuBindGroupLayout | undefined>,
    immediateSize?: GpuSize32,
    label?: string,
  }
  export interface GpuQuerySetDescriptor {
    type: GpuQueryType,
    count: GpuSize32,
    label?: string,
  }
  export interface GpuRenderPassTimestampWrites {
    querySet: GpuQuerySet,
    beginningOfPassWriteIndex?: GpuSize32,
    endOfPassWriteIndex?: GpuSize32,
  }
  export type GpuSize32Out = number;
  export type GpuSize64 = bigint;
  export interface GpuBufferBinding {
    buffer: GpuBuffer,
    offset?: GpuSize64,
    size?: GpuSize64,
  }
  export interface GpuBufferBindingLayout {
    type?: GpuBufferBindingType,
    hasDynamicOffset?: boolean,
    minBindingSize?: GpuSize64,
  }
  export interface GpuBufferDescriptor {
    size: GpuSize64,
    usage: GpuBufferUsage,
    mappedAtCreation?: boolean,
    label?: string,
  }
  export type GpuSize64Out = bigint;
  /**
   * # Variants
   * 
   * ## `"keep"`
   * 
   * ## `"zero"`
   * 
   * ## `"replace"`
   * 
   * ## `"invert"`
   * 
   * ## `"increment-clamp"`
   * 
   * ## `"decrement-clamp"`
   * 
   * ## `"increment-wrap"`
   * 
   * ## `"decrement-wrap"`
   */
  export type GpuStencilOperation = "keep" | "zero" | "replace" | "invert" | "increment-clamp" | "decrement-clamp" | "increment-wrap" | "decrement-wrap";
  export interface GpuStencilFaceState {
    compare?: GpuCompareFunction,
    failOp?: GpuStencilOperation,
    depthFailOp?: GpuStencilOperation,
    passOp?: GpuStencilOperation,
  }
  export type GpuStencilValue = number;
  /**
   * # Variants
   * 
   * ## `"write-only"`
   * 
   * ## `"read-only"`
   * 
   * ## `"read-write"`
   */
  export type GpuStorageTextureAccess = "write-only" | "read-only" | "read-write";
  /**
   * # Variants
   * 
   * ## `"store"`
   * 
   * ## `"discard"`
   */
  export type GpuStoreOp = "store" | "discard";
  export interface GpuTexelCopyBufferInfo {
    buffer: GpuBuffer,
    offset?: GpuSize64,
    bytesPerRow?: GpuSize32,
    rowsPerImage?: GpuSize32,
  }
  export interface GpuTexelCopyBufferLayout {
    offset?: GpuSize64,
    bytesPerRow?: GpuSize32,
    rowsPerImage?: GpuSize32,
  }
  /**
   * # Variants
   * 
   * ## `"all"`
   * 
   * ## `"stencil-only"`
   * 
   * ## `"depth-only"`
   */
  export type GpuTextureAspect = "all" | "stencil-only" | "depth-only";
  export interface GpuTexelCopyTextureInfo {
    texture: GpuTexture,
    mipLevel?: GpuIntegerCoordinate,
    origin?: GpuOrigin3D,
    aspect?: GpuTextureAspect,
  }
  /**
   * # Variants
   * 
   * ## `"d1"`
   * 
   * ## `"d2"`
   * 
   * ## `"d3"`
   */
  export type GpuTextureDimension = "d1" | "d2" | "d3";
  /**
   * # Variants
   * 
   * ## `"r8unorm"`
   * 
   * ## `"r8snorm"`
   * 
   * ## `"r8uint"`
   * 
   * ## `"r8sint"`
   * 
   * ## `"r16unorm"`
   * 
   * ## `"r16snorm"`
   * 
   * ## `"r16uint"`
   * 
   * ## `"r16sint"`
   * 
   * ## `"r16float"`
   * 
   * ## `"rg8unorm"`
   * 
   * ## `"rg8snorm"`
   * 
   * ## `"rg8uint"`
   * 
   * ## `"rg8sint"`
   * 
   * ## `"r32uint"`
   * 
   * ## `"r32sint"`
   * 
   * ## `"r32float"`
   * 
   * ## `"rg16unorm"`
   * 
   * ## `"rg16snorm"`
   * 
   * ## `"rg16uint"`
   * 
   * ## `"rg16sint"`
   * 
   * ## `"rg16float"`
   * 
   * ## `"rgba8unorm"`
   * 
   * ## `"rgba8unorm-srgb"`
   * 
   * ## `"rgba8snorm"`
   * 
   * ## `"rgba8uint"`
   * 
   * ## `"rgba8sint"`
   * 
   * ## `"bgra8unorm"`
   * 
   * ## `"bgra8unorm-srgb"`
   * 
   * ## `"rgb9e5ufloat"`
   * 
   * ## `"rgb10a2uint"`
   * 
   * ## `"rgb10a2unorm"`
   * 
   * ## `"rg11b10ufloat"`
   * 
   * ## `"rg32uint"`
   * 
   * ## `"rg32sint"`
   * 
   * ## `"rg32float"`
   * 
   * ## `"rgba16unorm"`
   * 
   * ## `"rgba16snorm"`
   * 
   * ## `"rgba16uint"`
   * 
   * ## `"rgba16sint"`
   * 
   * ## `"rgba16float"`
   * 
   * ## `"rgba32uint"`
   * 
   * ## `"rgba32sint"`
   * 
   * ## `"rgba32float"`
   * 
   * ## `"stencil8"`
   * 
   * ## `"depth16unorm"`
   * 
   * ## `"depth24plus"`
   * 
   * ## `"depth24plus-stencil8"`
   * 
   * ## `"depth32float"`
   * 
   * ## `"depth32float-stencil8"`
   * 
   * ## `"bc1-rgba-unorm"`
   * 
   * ## `"bc1-rgba-unorm-srgb"`
   * 
   * ## `"bc2-rgba-unorm"`
   * 
   * ## `"bc2-rgba-unorm-srgb"`
   * 
   * ## `"bc3-rgba-unorm"`
   * 
   * ## `"bc3-rgba-unorm-srgb"`
   * 
   * ## `"bc4-r-unorm"`
   * 
   * ## `"bc4-r-snorm"`
   * 
   * ## `"bc5-rg-unorm"`
   * 
   * ## `"bc5-rg-snorm"`
   * 
   * ## `"bc6h-rgb-ufloat"`
   * 
   * ## `"bc6h-rgb-float"`
   * 
   * ## `"bc7-rgba-unorm"`
   * 
   * ## `"bc7-rgba-unorm-srgb"`
   * 
   * ## `"etc2-rgb8unorm"`
   * 
   * ## `"etc2-rgb8unorm-srgb"`
   * 
   * ## `"etc2-rgb8a1unorm"`
   * 
   * ## `"etc2-rgb8a1unorm-srgb"`
   * 
   * ## `"etc2-rgba8unorm"`
   * 
   * ## `"etc2-rgba8unorm-srgb"`
   * 
   * ## `"eac-r11unorm"`
   * 
   * ## `"eac-r11snorm"`
   * 
   * ## `"eac-rg11unorm"`
   * 
   * ## `"eac-rg11snorm"`
   * 
   * ## `"astc4x4-unorm"`
   * 
   * ## `"astc4x4-unorm-srgb"`
   * 
   * ## `"astc5x4-unorm"`
   * 
   * ## `"astc5x4-unorm-srgb"`
   * 
   * ## `"astc5x5-unorm"`
   * 
   * ## `"astc5x5-unorm-srgb"`
   * 
   * ## `"astc6x5-unorm"`
   * 
   * ## `"astc6x5-unorm-srgb"`
   * 
   * ## `"astc6x6-unorm"`
   * 
   * ## `"astc6x6-unorm-srgb"`
   * 
   * ## `"astc8x5-unorm"`
   * 
   * ## `"astc8x5-unorm-srgb"`
   * 
   * ## `"astc8x6-unorm"`
   * 
   * ## `"astc8x6-unorm-srgb"`
   * 
   * ## `"astc8x8-unorm"`
   * 
   * ## `"astc8x8-unorm-srgb"`
   * 
   * ## `"astc10x5-unorm"`
   * 
   * ## `"astc10x5-unorm-srgb"`
   * 
   * ## `"astc10x6-unorm"`
   * 
   * ## `"astc10x6-unorm-srgb"`
   * 
   * ## `"astc10x8-unorm"`
   * 
   * ## `"astc10x8-unorm-srgb"`
   * 
   * ## `"astc10x10-unorm"`
   * 
   * ## `"astc10x10-unorm-srgb"`
   * 
   * ## `"astc12x10-unorm"`
   * 
   * ## `"astc12x10-unorm-srgb"`
   * 
   * ## `"astc12x12-unorm"`
   * 
   * ## `"astc12x12-unorm-srgb"`
   */
  export type GpuTextureFormat = "r8unorm" | "r8snorm" | "r8uint" | "r8sint" | "r16unorm" | "r16snorm" | "r16uint" | "r16sint" | "r16float" | "rg8unorm" | "rg8snorm" | "rg8uint" | "rg8sint" | "r32uint" | "r32sint" | "r32float" | "rg16unorm" | "rg16snorm" | "rg16uint" | "rg16sint" | "rg16float" | "rgba8unorm" | "rgba8unorm-srgb" | "rgba8snorm" | "rgba8uint" | "rgba8sint" | "bgra8unorm" | "bgra8unorm-srgb" | "rgb9e5ufloat" | "rgb10a2uint" | "rgb10a2unorm" | "rg11b10ufloat" | "rg32uint" | "rg32sint" | "rg32float" | "rgba16unorm" | "rgba16snorm" | "rgba16uint" | "rgba16sint" | "rgba16float" | "rgba32uint" | "rgba32sint" | "rgba32float" | "stencil8" | "depth16unorm" | "depth24plus" | "depth24plus-stencil8" | "depth32float" | "depth32float-stencil8" | "bc1-rgba-unorm" | "bc1-rgba-unorm-srgb" | "bc2-rgba-unorm" | "bc2-rgba-unorm-srgb" | "bc3-rgba-unorm" | "bc3-rgba-unorm-srgb" | "bc4-r-unorm" | "bc4-r-snorm" | "bc5-rg-unorm" | "bc5-rg-snorm" | "bc6h-rgb-ufloat" | "bc6h-rgb-float" | "bc7-rgba-unorm" | "bc7-rgba-unorm-srgb" | "etc2-rgb8unorm" | "etc2-rgb8unorm-srgb" | "etc2-rgb8a1unorm" | "etc2-rgb8a1unorm-srgb" | "etc2-rgba8unorm" | "etc2-rgba8unorm-srgb" | "eac-r11unorm" | "eac-r11snorm" | "eac-rg11unorm" | "eac-rg11snorm" | "astc4x4-unorm" | "astc4x4-unorm-srgb" | "astc5x4-unorm" | "astc5x4-unorm-srgb" | "astc5x5-unorm" | "astc5x5-unorm-srgb" | "astc6x5-unorm" | "astc6x5-unorm-srgb" | "astc6x6-unorm" | "astc6x6-unorm-srgb" | "astc8x5-unorm" | "astc8x5-unorm-srgb" | "astc8x6-unorm" | "astc8x6-unorm-srgb" | "astc8x8-unorm" | "astc8x8-unorm-srgb" | "astc10x5-unorm" | "astc10x5-unorm-srgb" | "astc10x6-unorm" | "astc10x6-unorm-srgb" | "astc10x8-unorm" | "astc10x8-unorm-srgb" | "astc10x10-unorm" | "astc10x10-unorm-srgb" | "astc12x10-unorm" | "astc12x10-unorm-srgb" | "astc12x12-unorm" | "astc12x12-unorm-srgb";
  export interface GpuColorTargetState {
    format: GpuTextureFormat,
    blend?: GpuBlendState,
    writeMask?: GpuColorWrite,
  }
  export interface GpuDepthStencilState {
    format: GpuTextureFormat,
    depthWriteEnabled?: boolean,
    depthCompare?: GpuCompareFunction,
    stencilFront?: GpuStencilFaceState,
    stencilBack?: GpuStencilFaceState,
    stencilReadMask?: GpuStencilValue,
    stencilWriteMask?: GpuStencilValue,
    depthBias?: GpuDepthBias,
    depthBiasSlopeScale?: number,
    depthBiasClamp?: number,
  }
  export interface GpuRenderBundleEncoderDescriptor {
    depthReadOnly?: boolean,
    stencilReadOnly?: boolean,
    colorFormats: Array<GpuTextureFormat | undefined>,
    depthStencilFormat?: GpuTextureFormat,
    sampleCount?: GpuSize32,
    label?: string,
  }
  /**
   * # Variants
   * 
   * ## `"float"`
   * 
   * ## `"unfilterable-float"`
   * 
   * ## `"depth"`
   * 
   * ## `"sint"`
   * 
   * ## `"uint"`
   */
  export type GpuTextureSampleType = "float" | "unfilterable-float" | "depth" | "sint" | "uint";
  export interface GpuTextureUsage {
    copySrc?: boolean,
    copyDst?: boolean,
    textureBinding?: boolean,
    storageBinding?: boolean,
    renderAttachment?: boolean,
    transientAttachment?: boolean,
  }
  export type GpuBindingResource = GpuBindingResourceGpuBuffer | GpuBindingResourceGpuBufferBinding | GpuBindingResourceGpuSampler | GpuBindingResourceGpuTexture | GpuBindingResourceGpuTextureView;
  export interface GpuBindingResourceGpuBuffer {
    tag: 'gpu-buffer',
    val: GpuBuffer,
  }
  export interface GpuBindingResourceGpuBufferBinding {
    tag: 'gpu-buffer-binding',
    val: GpuBufferBinding,
  }
  export interface GpuBindingResourceGpuSampler {
    tag: 'gpu-sampler',
    val: GpuSampler,
  }
  export interface GpuBindingResourceGpuTexture {
    tag: 'gpu-texture',
    val: GpuTexture,
  }
  export interface GpuBindingResourceGpuTextureView {
    tag: 'gpu-texture-view',
    val: GpuTextureView,
  }
  export interface GpuBindGroupEntry {
    binding: GpuIndex32,
    resource: GpuBindingResource,
  }
  export interface GpuBindGroupDescriptor {
    layout: GpuBindGroupLayout,
    entries: Array<GpuBindGroupEntry>,
    label?: string,
  }
  export interface GpuRenderPassColorAttachment {
    view: GpuTextureView,
    depthSlice?: GpuIntegerCoordinate,
    resolveTarget?: GpuTextureView,
    clearValue?: GpuColor,
    loadOp: GpuLoadOp,
    storeOp: GpuStoreOp,
  }
  export interface GpuRenderPassDepthStencilAttachment {
    view: GpuTextureView,
    depthClearValue?: number,
    depthLoadOp?: GpuLoadOp,
    depthStoreOp?: GpuStoreOp,
    depthReadOnly?: boolean,
    stencilClearValue?: GpuStencilValue,
    stencilLoadOp?: GpuLoadOp,
    stencilStoreOp?: GpuStoreOp,
    stencilReadOnly?: boolean,
  }
  export interface GpuRenderPassDescriptor {
    colorAttachments: Array<GpuRenderPassColorAttachment | undefined>,
    depthStencilAttachment?: GpuRenderPassDepthStencilAttachment,
    occlusionQuerySet?: GpuQuerySet,
    timestampWrites?: GpuRenderPassTimestampWrites,
    maxDrawCount?: GpuSize64,
    label?: string,
  }
  /**
   * # Variants
   * 
   * ## `"d1"`
   * 
   * ## `"d2"`
   * 
   * ## `"d2-array"`
   * 
   * ## `"cube"`
   * 
   * ## `"cube-array"`
   * 
   * ## `"d3"`
   */
  export type GpuTextureViewDimension = "d1" | "d2" | "d2-array" | "cube" | "cube-array" | "d3";
  export interface GpuStorageTextureBindingLayout {
    access?: GpuStorageTextureAccess,
    format: GpuTextureFormat,
    viewDimension?: GpuTextureViewDimension,
  }
  export interface GpuTextureBindingLayout {
    sampleType?: GpuTextureSampleType,
    viewDimension?: GpuTextureViewDimension,
    multisampled?: boolean,
  }
  export interface GpuBindGroupLayoutEntry {
    binding: GpuIndex32,
    visibility: GpuShaderStage,
    buffer?: GpuBufferBindingLayout,
    sampler?: GpuSamplerBindingLayout,
    texture?: GpuTextureBindingLayout,
    storageTexture?: GpuStorageTextureBindingLayout,
  }
  export interface GpuBindGroupLayoutDescriptor {
    entries: Array<GpuBindGroupLayoutEntry>,
    label?: string,
  }
  export interface GpuTextureDescriptor {
    size: GpuExtent3D,
    mipLevelCount?: GpuIntegerCoordinate,
    sampleCount?: GpuSize32,
    dimension?: GpuTextureDimension,
    format: GpuTextureFormat,
    usage: GpuTextureUsage,
    viewFormats?: Array<GpuTextureFormat>,
    textureBindingViewDimension?: GpuTextureViewDimension,
    label?: string,
  }
  export interface GpuTextureViewDescriptor {
    format?: GpuTextureFormat,
    dimension?: GpuTextureViewDimension,
    usage?: GpuTextureUsage,
    aspect?: GpuTextureAspect,
    baseMipLevel?: GpuIntegerCoordinate,
    mipLevelCount?: GpuIntegerCoordinate,
    baseArrayLayer?: GpuIntegerCoordinate,
    arrayLayerCount?: GpuIntegerCoordinate,
    swizzle?: string,
    label?: string,
  }
  /**
   * # Variants
   * 
   * ## `"uint8"`
   * 
   * ## `"uint8x2"`
   * 
   * ## `"uint8x4"`
   * 
   * ## `"sint8"`
   * 
   * ## `"sint8x2"`
   * 
   * ## `"sint8x4"`
   * 
   * ## `"unorm8"`
   * 
   * ## `"unorm8x2"`
   * 
   * ## `"unorm8x4"`
   * 
   * ## `"snorm8"`
   * 
   * ## `"snorm8x2"`
   * 
   * ## `"snorm8x4"`
   * 
   * ## `"uint16"`
   * 
   * ## `"uint16x2"`
   * 
   * ## `"uint16x4"`
   * 
   * ## `"sint16"`
   * 
   * ## `"sint16x2"`
   * 
   * ## `"sint16x4"`
   * 
   * ## `"unorm16"`
   * 
   * ## `"unorm16x2"`
   * 
   * ## `"unorm16x4"`
   * 
   * ## `"snorm16"`
   * 
   * ## `"snorm16x2"`
   * 
   * ## `"snorm16x4"`
   * 
   * ## `"float16"`
   * 
   * ## `"float16x2"`
   * 
   * ## `"float16x4"`
   * 
   * ## `"float32"`
   * 
   * ## `"float32x2"`
   * 
   * ## `"float32x3"`
   * 
   * ## `"float32x4"`
   * 
   * ## `"uint32"`
   * 
   * ## `"uint32x2"`
   * 
   * ## `"uint32x3"`
   * 
   * ## `"uint32x4"`
   * 
   * ## `"sint32"`
   * 
   * ## `"sint32x2"`
   * 
   * ## `"sint32x3"`
   * 
   * ## `"sint32x4"`
   * 
   * ## `"unorm1010102"`
   * 
   * ## `"unorm8x4-bgra"`
   */
  export type GpuVertexFormat = "uint8" | "uint8x2" | "uint8x4" | "sint8" | "sint8x2" | "sint8x4" | "unorm8" | "unorm8x2" | "unorm8x4" | "snorm8" | "snorm8x2" | "snorm8x4" | "uint16" | "uint16x2" | "uint16x4" | "sint16" | "sint16x2" | "sint16x4" | "unorm16" | "unorm16x2" | "unorm16x4" | "snorm16" | "snorm16x2" | "snorm16x4" | "float16" | "float16x2" | "float16x4" | "float32" | "float32x2" | "float32x3" | "float32x4" | "uint32" | "uint32x2" | "uint32x3" | "uint32x4" | "sint32" | "sint32x2" | "sint32x3" | "sint32x4" | "unorm1010102" | "unorm8x4-bgra";
  export interface GpuVertexAttribute {
    format: GpuVertexFormat,
    offset: GpuSize64,
    shaderLocation: GpuIndex32,
  }
  /**
   * # Variants
   * 
   * ## `"vertex"`
   * 
   * ## `"instance"`
   */
  export type GpuVertexStepMode = "vertex" | "instance";
  export interface GpuVertexBufferLayout {
    arrayStride: GpuSize64,
    stepMode?: GpuVertexStepMode,
    attributes: Array<GpuVertexAttribute>,
  }
  export type MapAsyncErrorKind = MapAsyncErrorKindOperationError | MapAsyncErrorKindRangeError | MapAsyncErrorKindAbortError;
  export interface MapAsyncErrorKindOperationError {
    tag: 'operation-error',
  }
  export interface MapAsyncErrorKindRangeError {
    tag: 'range-error',
  }
  export interface MapAsyncErrorKindAbortError {
    tag: 'abort-error',
  }
  export interface MapAsyncError {
    kind: MapAsyncErrorKind,
    message: string,
  }
  export type PopErrorScopeErrorKind = PopErrorScopeErrorKindOperationError;
  export interface PopErrorScopeErrorKindOperationError {
    tag: 'operation-error',
  }
  export interface PopErrorScopeError {
    kind: PopErrorScopeErrorKind,
    message: string,
  }
  /**
   * # Variants
   * 
   * ## `"srgb"`
   * 
   * ## `"display-p3"`
   */
  export type PredefinedColorSpace = "srgb" | "display-p3";
  export interface GpuCanvasConfiguration {
    device: GpuDevice,
    format: GpuTextureFormat,
    usage?: GpuTextureUsage,
    viewFormats?: Array<GpuTextureFormat>,
    colorSpace?: PredefinedColorSpace,
    toneMapping?: GpuCanvasToneMapping,
    alphaMode?: GpuCanvasAlphaMode,
  }
  export interface GpuCanvasConfigurationOwned {
    device: GpuDevice,
    format: GpuTextureFormat,
    usage?: GpuTextureUsage,
    viewFormats?: Array<GpuTextureFormat>,
    colorSpace?: PredefinedColorSpace,
    toneMapping?: GpuCanvasToneMapping,
    alphaMode?: GpuCanvasAlphaMode,
  }
  export interface GpuFragmentState {
    targets: Array<GpuColorTargetState | undefined>,
    module: GpuShaderModule,
    entryPoint?: string,
    constants?: RecordGpuPipelineConstantValue,
  }
  export interface GpuProgrammableStage {
    module: GpuShaderModule,
    entryPoint?: string,
    constants?: RecordGpuPipelineConstantValue,
  }
  export interface GpuComputePipelineDescriptor {
    compute: GpuProgrammableStage,
    layout: GpuLayoutMode,
    label?: string,
  }
  export interface GpuVertexState {
    buffers?: Array<GpuVertexBufferLayout | undefined>,
    module: GpuShaderModule,
    entryPoint?: string,
    constants?: RecordGpuPipelineConstantValue,
  }
  export interface GpuRenderPipelineDescriptor {
    vertex: GpuVertexState,
    primitive?: GpuPrimitiveState,
    depthStencil?: GpuDepthStencilState,
    multisample?: GpuMultisampleState,
    fragment?: GpuFragmentState,
    layout: GpuLayoutMode,
    label?: string,
  }
  export interface GpuDeviceDescriptor {
    requiredFeatures?: Array<GpuFeatureName>,
    requiredLimits?: RecordOptionGpuSize64,
    defaultQueue?: GpuQueueDescriptor,
    label?: string,
  }
  export type RequestDeviceErrorKind = RequestDeviceErrorKindTypeError | RequestDeviceErrorKindOperationError;
  export interface RequestDeviceErrorKindTypeError {
    tag: 'type-error',
  }
  export interface RequestDeviceErrorKindOperationError {
    tag: 'operation-error',
  }
  export interface RequestDeviceError {
    kind: RequestDeviceErrorKind,
    message: string,
  }
  export type SetBindGroupErrorKind = SetBindGroupErrorKindRangeError;
  export interface SetBindGroupErrorKindRangeError {
    tag: 'range-error',
  }
  export interface SetBindGroupError {
    kind: SetBindGroupErrorKind,
    message: string,
  }
  export type UnmapErrorKind = UnmapErrorKindAbortError;
  export interface UnmapErrorKindAbortError {
    tag: 'abort-error',
  }
  export interface UnmapError {
    kind: UnmapErrorKind,
    message: string,
  }
  export type WriteBufferErrorKind = WriteBufferErrorKindOperationError;
  export interface WriteBufferErrorKindOperationError {
    tag: 'operation-error',
  }
  export interface WriteBufferError {
    kind: WriteBufferErrorKind,
    message: string,
  }
  export type Option<T> = { tag: 'none' } | { tag: 'some', val: T };
  
  export class Gpu implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    requestAdapter(options: GpuRequestAdapterOptions | undefined): Promise<GpuAdapter | undefined>;
    getPreferredCanvasFormat(): GpuTextureFormat;
    wgslLanguageFeatures(): WgslLanguageFeatures;
    [Symbol.dispose](): void;
  }
  
  export class GpuAdapter implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    features(): GpuSupportedFeatures;
    limits(): GpuSupportedLimits;
    info(): GpuAdapterInfo;
    /**
    * @throws {Error & { payload: RequestDeviceError }} The error of the result, as the `payload` of the thrown error.
    */
    requestDevice(descriptor: GpuDeviceDescriptor | undefined): Promise<GpuDevice>;
    [Symbol.dispose](): void;
  }
  
  export class GpuAdapterInfo implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    vendor(): string;
    architecture(): string;
    device(): string;
    description(): string;
    subgroupMinSize(): number;
    subgroupMaxSize(): number;
    isFallbackAdapter(): boolean;
    [Symbol.dispose](): void;
  }
  
  export class GpuBindGroup implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    label(): string;
    setLabel(label: string): void;
    [Symbol.dispose](): void;
  }
  
  export class GpuBindGroupLayout implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    label(): string;
    setLabel(label: string): void;
    [Symbol.dispose](): void;
  }
  
  export class GpuBuffer implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    size(): GpuSize64Out;
    usage(): GpuBufferUsage;
    mapState(): GpuBufferMapState;
    /**
    * @throws {Error & { payload: MapAsyncError }} The error of the result, as the `payload` of the thrown error.
    */
    mapAsync(mode: GpuMapMode, offset: GpuSize64 | undefined, size: GpuSize64 | undefined): Promise<void>;
    /**
    * @throws {Error & { payload: GetMappedRangeError }} The error of the result, as the `payload` of the thrown error.
    */
    getMappedRangeGetWithCopy(offset: GpuSize64 | undefined, size: GpuSize64 | undefined): Uint8Array;
    /**
    * @throws {Error & { payload: UnmapError }} The error of the result, as the `payload` of the thrown error.
    */
    unmap(): void;
    destroy(): void;
    label(): string;
    setLabel(label: string): void;
    /**
    * @throws {Error & { payload: GetMappedRangeError }} The error of the result, as the `payload` of the thrown error.
    */
    getMappedRangeSetWithCopy(data: Uint8Array, offset: GpuSize64 | undefined, size: GpuSize64 | undefined): void;
    [Symbol.dispose](): void;
  }
  
  export class GpuCanvasContext implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    configure(configuration: GpuCanvasConfiguration): void;
    unconfigure(): void;
    getConfiguration(): GpuCanvasConfigurationOwned | undefined;
    getCurrentTexture(): GpuTexture;
    [Symbol.dispose](): void;
  }
  
  export class GpuCommandBuffer implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    label(): string;
    setLabel(label: string): void;
    [Symbol.dispose](): void;
  }
  
  export class GpuCommandEncoder implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    beginRenderPass(descriptor: GpuRenderPassDescriptor): GpuRenderPassEncoder;
    beginComputePass(descriptor: GpuComputePassDescriptor | undefined): GpuComputePassEncoder;
    copyBufferToBuffer(source: GpuBuffer, sourceOffset: GpuSize64 | undefined, destination: GpuBuffer, destinationOffset: GpuSize64 | undefined, size: GpuSize64 | undefined): void;
    copyBufferToTexture(source: GpuTexelCopyBufferInfo, destination: GpuTexelCopyTextureInfo, copySize: GpuExtent3D): void;
    copyTextureToBuffer(source: GpuTexelCopyTextureInfo, destination: GpuTexelCopyBufferInfo, copySize: GpuExtent3D): void;
    copyTextureToTexture(source: GpuTexelCopyTextureInfo, destination: GpuTexelCopyTextureInfo, copySize: GpuExtent3D): void;
    clearBuffer(buffer: GpuBuffer, offset: GpuSize64 | undefined, size: GpuSize64 | undefined): void;
    resolveQuerySet(querySet: GpuQuerySet, firstQuery: GpuSize32, queryCount: GpuSize32, destination: GpuBuffer, destinationOffset: GpuSize64): void;
    finish(descriptor: GpuCommandBufferDescriptor | undefined): GpuCommandBuffer;
    label(): string;
    setLabel(label: string): void;
    pushDebugGroup(groupLabel: string): void;
    popDebugGroup(): void;
    insertDebugMarker(markerLabel: string): void;
    [Symbol.dispose](): void;
  }
  
  export class GpuCompilationInfo implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    messages(): Array<GpuCompilationMessage>;
    [Symbol.dispose](): void;
  }
  
  export class GpuCompilationMessage implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    message(): string;
    type(): GpuCompilationMessageType;
    lineNum(): bigint;
    linePos(): bigint;
    offset(): bigint;
    length(): bigint;
    [Symbol.dispose](): void;
  }
  
  export class GpuComputePassEncoder implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    setPipeline(pipeline: GpuComputePipeline): void;
    dispatchWorkgroups(workgroupCountX: GpuSize32, workgroupCountY: GpuSize32 | undefined, workgroupCountZ: GpuSize32 | undefined): void;
    dispatchWorkgroupsIndirect(indirectBuffer: GpuBuffer, indirectOffset: GpuSize64): void;
    end(): void;
    label(): string;
    setLabel(label: string): void;
    pushDebugGroup(groupLabel: string): void;
    popDebugGroup(): void;
    insertDebugMarker(markerLabel: string): void;
    /**
    * @throws {Error & { payload: SetBindGroupError }} The error of the result, as the `payload` of the thrown error.
    */
    setBindGroup(index: GpuIndex32, bindGroup: GpuBindGroup | undefined, dynamicOffsetsData: Uint32Array | undefined, dynamicOffsetsDataStart: GpuSize64 | undefined, dynamicOffsetsDataLength: GpuSize32 | undefined): void;
    setImmediates(rangeOffset: GpuSize32, data: Uint8Array, dataOffset: GpuSize64 | undefined, dataSize: GpuSize64 | undefined): void;
    [Symbol.dispose](): void;
  }
  
  export class GpuComputePipeline implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    label(): string;
    setLabel(label: string): void;
    getBindGroupLayout(index: number): GpuBindGroupLayout;
    [Symbol.dispose](): void;
  }
  
  export class GpuDevice implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    features(): GpuSupportedFeatures;
    limits(): GpuSupportedLimits;
    adapterInfo(): GpuAdapterInfo;
    queue(): GpuQueue;
    destroy(): void;
    createBuffer(descriptor: GpuBufferDescriptor): GpuBuffer;
    createTexture(descriptor: GpuTextureDescriptor): GpuTexture;
    createSampler(descriptor: GpuSamplerDescriptor | undefined): GpuSampler;
    createBindGroupLayout(descriptor: GpuBindGroupLayoutDescriptor): GpuBindGroupLayout;
    createPipelineLayout(descriptor: GpuPipelineLayoutDescriptor): GpuPipelineLayout;
    createBindGroup(descriptor: GpuBindGroupDescriptor): GpuBindGroup;
    createShaderModule(descriptor: GpuShaderModuleDescriptor): GpuShaderModule;
    createComputePipeline(descriptor: GpuComputePipelineDescriptor): GpuComputePipeline;
    createRenderPipeline(descriptor: GpuRenderPipelineDescriptor): GpuRenderPipeline;
    /**
    * @throws {Error & { payload: CreatePipelineError }} The error of the result, as the `payload` of the thrown error.
    */
    createComputePipelineAsync(descriptor: GpuComputePipelineDescriptor): Promise<GpuComputePipeline>;
    /**
    * @throws {Error & { payload: CreatePipelineError }} The error of the result, as the `payload` of the thrown error.
    */
    createRenderPipelineAsync(descriptor: GpuRenderPipelineDescriptor): Promise<GpuRenderPipeline>;
    createCommandEncoder(descriptor: GpuCommandEncoderDescriptor | undefined): GpuCommandEncoder;
    createRenderBundleEncoder(descriptor: GpuRenderBundleEncoderDescriptor): GpuRenderBundleEncoder;
    /**
    * @throws {Error & { payload: CreateQuerySetError }} The error of the result, as the `payload` of the thrown error.
    */
    createQuerySet(descriptor: GpuQuerySetDescriptor): GpuQuerySet;
    label(): string;
    setLabel(label: string): void;
    lost(): PromiseLike<GpuDeviceLostInfo> & { [Symbol.dispose]?(): void };
    pushErrorScope(filter: GpuErrorFilter): void;
    /**
    * @throws {Error & { payload: PopErrorScopeError }} The error of the result, as the `payload` of the thrown error.
    */
    popErrorScope(): Promise<GpuError | undefined>;
    onUncapturedError(): AsyncIterable<GpuError>;
    [Symbol.dispose](): void;
  }
  
  export class GpuDeviceLostInfo implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    reason(): GpuDeviceLostReason;
    message(): string;
    [Symbol.dispose](): void;
  }
  
  export class GpuError implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    message(): string;
    kind(): GpuErrorKind;
    [Symbol.dispose](): void;
  }
  
  export class GpuPipelineLayout implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    label(): string;
    setLabel(label: string): void;
    [Symbol.dispose](): void;
  }
  
  export class GpuQuerySet implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    destroy(): void;
    type(): GpuQueryType;
    count(): GpuSize32Out;
    label(): string;
    setLabel(label: string): void;
    [Symbol.dispose](): void;
  }
  
  export class GpuQueue implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    submit(commandBuffers: Array<GpuCommandBuffer>): void;
    onSubmittedWorkDone(): Promise<void>;
    /**
    * @throws {Error & { payload: WriteBufferError }} The error of the result, as the `payload` of the thrown error.
    */
    writeBufferWithCopy(buffer: GpuBuffer, bufferOffset: GpuSize64, data: Uint8Array, dataOffset: GpuSize64 | undefined, size: GpuSize64 | undefined): void;
    writeTextureWithCopy(destination: GpuTexelCopyTextureInfo, data: Uint8Array, dataLayout: GpuTexelCopyBufferLayout, size: GpuExtent3D): void;
    label(): string;
    setLabel(label: string): void;
    [Symbol.dispose](): void;
  }
  
  export class GpuRenderBundle implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    label(): string;
    setLabel(label: string): void;
    [Symbol.dispose](): void;
  }
  
  export class GpuRenderBundleEncoder implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    finish(descriptor: GpuRenderBundleDescriptor | undefined): GpuRenderBundle;
    label(): string;
    setLabel(label: string): void;
    pushDebugGroup(groupLabel: string): void;
    popDebugGroup(): void;
    insertDebugMarker(markerLabel: string): void;
    /**
    * @throws {Error & { payload: SetBindGroupError }} The error of the result, as the `payload` of the thrown error.
    */
    setBindGroup(index: GpuIndex32, bindGroup: GpuBindGroup | undefined, dynamicOffsetsData: Uint32Array | undefined, dynamicOffsetsDataStart: GpuSize64 | undefined, dynamicOffsetsDataLength: GpuSize32 | undefined): void;
    setImmediates(rangeOffset: GpuSize32, data: Uint8Array, dataOffset: GpuSize64 | undefined, dataSize: GpuSize64 | undefined): void;
    setPipeline(pipeline: GpuRenderPipeline): void;
    setIndexBuffer(buffer: GpuBuffer, indexFormat: GpuIndexFormat, offset: GpuSize64 | undefined, size: GpuSize64 | undefined): void;
    setVertexBuffer(slot: GpuIndex32, buffer: GpuBuffer | undefined, offset: GpuSize64 | undefined, size: GpuSize64 | undefined): void;
    draw(vertexCount: GpuSize32, instanceCount: GpuSize32 | undefined, firstVertex: GpuSize32 | undefined, firstInstance: GpuSize32 | undefined): void;
    drawIndexed(indexCount: GpuSize32, instanceCount: GpuSize32 | undefined, firstIndex: GpuSize32 | undefined, baseVertex: GpuSignedOffset32 | undefined, firstInstance: GpuSize32 | undefined): void;
    drawIndirect(indirectBuffer: GpuBuffer, indirectOffset: GpuSize64): void;
    drawIndexedIndirect(indirectBuffer: GpuBuffer, indirectOffset: GpuSize64): void;
    [Symbol.dispose](): void;
  }
  
  export class GpuRenderPassEncoder implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    setViewport(x: number, y: number, width: number, height: number, minDepth: number, maxDepth: number): void;
    setScissorRect(x: GpuIntegerCoordinate, y: GpuIntegerCoordinate, width: GpuIntegerCoordinate, height: GpuIntegerCoordinate): void;
    setBlendConstant(color: GpuColor): void;
    setStencilReference(reference: GpuStencilValue): void;
    beginOcclusionQuery(queryIndex: GpuSize32): void;
    endOcclusionQuery(): void;
    executeBundles(bundles: Array<GpuRenderBundle>): void;
    end(): void;
    label(): string;
    setLabel(label: string): void;
    pushDebugGroup(groupLabel: string): void;
    popDebugGroup(): void;
    insertDebugMarker(markerLabel: string): void;
    /**
    * @throws {Error & { payload: SetBindGroupError }} The error of the result, as the `payload` of the thrown error.
    */
    setBindGroup(index: GpuIndex32, bindGroup: GpuBindGroup | undefined, dynamicOffsetsData: Uint32Array | undefined, dynamicOffsetsDataStart: GpuSize64 | undefined, dynamicOffsetsDataLength: GpuSize32 | undefined): void;
    setImmediates(rangeOffset: GpuSize32, data: Uint8Array, dataOffset: GpuSize64 | undefined, dataSize: GpuSize64 | undefined): void;
    setPipeline(pipeline: GpuRenderPipeline): void;
    setIndexBuffer(buffer: GpuBuffer, indexFormat: GpuIndexFormat, offset: GpuSize64 | undefined, size: GpuSize64 | undefined): void;
    setVertexBuffer(slot: GpuIndex32, buffer: GpuBuffer | undefined, offset: GpuSize64 | undefined, size: GpuSize64 | undefined): void;
    draw(vertexCount: GpuSize32, instanceCount: GpuSize32 | undefined, firstVertex: GpuSize32 | undefined, firstInstance: GpuSize32 | undefined): void;
    drawIndexed(indexCount: GpuSize32, instanceCount: GpuSize32 | undefined, firstIndex: GpuSize32 | undefined, baseVertex: GpuSignedOffset32 | undefined, firstInstance: GpuSize32 | undefined): void;
    drawIndirect(indirectBuffer: GpuBuffer, indirectOffset: GpuSize64): void;
    drawIndexedIndirect(indirectBuffer: GpuBuffer, indirectOffset: GpuSize64): void;
    [Symbol.dispose](): void;
  }
  
  export class GpuRenderPipeline implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    label(): string;
    setLabel(label: string): void;
    getBindGroupLayout(index: number): GpuBindGroupLayout;
    [Symbol.dispose](): void;
  }
  
  export class GpuSampler implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    label(): string;
    setLabel(label: string): void;
    [Symbol.dispose](): void;
  }
  
  export class GpuShaderModule implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    getCompilationInfo(): Promise<GpuCompilationInfo>;
    label(): string;
    setLabel(label: string): void;
    [Symbol.dispose](): void;
  }
  
  export class GpuSupportedFeatures implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    has(value: string): boolean;
    [Symbol.dispose](): void;
  }
  
  export class GpuSupportedLimits implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    maxTextureDimension1D(): number;
    maxTextureDimension2D(): number;
    maxTextureDimension3D(): number;
    maxTextureArrayLayers(): number;
    maxBindGroups(): number;
    maxBindGroupsPlusVertexBuffers(): number;
    maxImmediateSize(): number;
    maxBindingsPerBindGroup(): number;
    maxDynamicUniformBuffersPerPipelineLayout(): number;
    maxDynamicStorageBuffersPerPipelineLayout(): number;
    maxSampledTexturesPerShaderStage(): number;
    maxSamplersPerShaderStage(): number;
    maxStorageBuffersPerShaderStage(): number;
    maxStorageBuffersInVertexStage(): number;
    maxStorageBuffersInFragmentStage(): number;
    maxStorageTexturesPerShaderStage(): number;
    maxStorageTexturesInVertexStage(): number;
    maxStorageTexturesInFragmentStage(): number;
    maxUniformBuffersPerShaderStage(): number;
    maxUniformBufferBindingSize(): bigint;
    maxStorageBufferBindingSize(): bigint;
    minUniformBufferOffsetAlignment(): number;
    minStorageBufferOffsetAlignment(): number;
    maxVertexBuffers(): number;
    maxBufferSize(): bigint;
    maxVertexAttributes(): number;
    maxVertexBufferArrayStride(): number;
    maxInterStageShaderVariables(): number;
    maxColorAttachments(): number;
    maxColorAttachmentBytesPerSample(): number;
    maxComputeWorkgroupStorageSize(): number;
    maxComputeInvocationsPerWorkgroup(): number;
    maxComputeWorkgroupSizeX(): number;
    maxComputeWorkgroupSizeY(): number;
    maxComputeWorkgroupSizeZ(): number;
    maxComputeWorkgroupsPerDimension(): number;
    [Symbol.dispose](): void;
  }
  
  export class GpuTexture implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    createView(descriptor: GpuTextureViewDescriptor | undefined): GpuTextureView;
    destroy(): void;
    width(): GpuIntegerCoordinateOut;
    height(): GpuIntegerCoordinateOut;
    depthOrArrayLayers(): GpuIntegerCoordinateOut;
    mipLevelCount(): GpuIntegerCoordinateOut;
    sampleCount(): GpuSize32Out;
    dimension(): GpuTextureDimension;
    format(): GpuTextureFormat;
    usage(): GpuTextureUsage;
    textureBindingViewDimension(): GpuTextureViewDimension | undefined;
    label(): string;
    setLabel(label: string): void;
    [Symbol.dispose](): void;
  }
  
  export class GpuTextureView implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    label(): string;
    setLabel(label: string): void;
    [Symbol.dispose](): void;
  }
  
  export class GpuUncapturedErrorEvent implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    error(): GpuError;
    [Symbol.dispose](): void;
  }
  
  export class RecordGpuPipelineConstantValue implements Disposable {
    constructor()
    add(key: string, value: GpuPipelineConstantValue): void;
    get(key: string): GpuPipelineConstantValue | undefined;
    has(key: string): boolean;
    remove(key: string): void;
    keys(): Array<string>;
    values(): Float64Array;
    entries(): Array<[string, GpuPipelineConstantValue]>;
    [Symbol.dispose](): void;
  }
  
  export class RecordOptionGpuSize64 implements Disposable {
    constructor()
    add(key: string, value: GpuSize64 | undefined): void;
    get(key: string): Option<GpuSize64 | undefined>;
    has(key: string): boolean;
    remove(key: string): void;
    keys(): Array<string>;
    values(): Array<GpuSize64 | undefined>;
    entries(): Array<[string, GpuSize64 | undefined]>;
    [Symbol.dispose](): void;
  }
  
  export class WgslLanguageFeatures implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    has(value: string): boolean;
    [Symbol.dispose](): void;
  }
}
