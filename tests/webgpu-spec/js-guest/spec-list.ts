import { g as examples } from './cts/src/webgpu/examples.spec';
import { g as compat_api_validation_texture_cubeArray } from './cts/src/webgpu/compat/api/validation/texture/cubeArray.spec';
import { g as compat_api_validation_texture_createTexture } from './cts/src/webgpu/compat/api/validation/texture/createTexture.spec';
import { g as compat_api_validation_render_pipeline_depth_stencil_state } from './cts/src/webgpu/compat/api/validation/render_pipeline/depth_stencil_state.spec';
import { g as compat_api_validation_render_pipeline_unsupported_wgsl } from './cts/src/webgpu/compat/api/validation/render_pipeline/unsupported_wgsl.spec';
import { g as compat_api_validation_render_pipeline_fragment_state } from './cts/src/webgpu/compat/api/validation/render_pipeline/fragment_state.spec';
import { g as compat_api_validation_render_pipeline_vertex_state } from './cts/src/webgpu/compat/api/validation/render_pipeline/vertex_state.spec';
import { g as compat_api_validation_createBindGroup } from './cts/src/webgpu/compat/api/validation/createBindGroup.spec';
import { g as compat_api_validation_encoding_programmable_pipeline_bind_group_compat } from './cts/src/webgpu/compat/api/validation/encoding/programmable/pipeline_bind_group_compat.spec';
import { g as compat_api_validation_encoding_cmds_copyTextureToBuffer } from './cts/src/webgpu/compat/api/validation/encoding/cmds/copyTextureToBuffer.spec';
import { g as compat_api_validation_encoding_cmds_copyTextureToTexture } from './cts/src/webgpu/compat/api/validation/encoding/cmds/copyTextureToTexture.spec';
import { g as compat_api_validation_createBindGroupLayout } from './cts/src/webgpu/compat/api/validation/createBindGroupLayout.spec';
import { g as compat_api_validation_pipeline_creation } from './cts/src/webgpu/compat/api/validation/pipeline_creation.spec';
import { g as web_platform_external_texture_video } from './cts/src/webgpu/web_platform/external_texture/video.spec';
import { g as web_platform_canvas_context_creation } from './cts/src/webgpu/web_platform/canvas/context_creation.spec';
import { g as web_platform_canvas_configure } from './cts/src/webgpu/web_platform/canvas/configure.spec';
import { g as web_platform_canvas_getPreferredCanvasFormat } from './cts/src/webgpu/web_platform/canvas/getPreferredCanvasFormat.spec';
import { g as web_platform_canvas_getCurrentTexture } from './cts/src/webgpu/web_platform/canvas/getCurrentTexture.spec';
import { g as web_platform_canvas_readbackFromWebGPUCanvas } from './cts/src/webgpu/web_platform/canvas/readbackFromWebGPUCanvas.spec';
import { g as web_platform_copyToTexture_ImageBitmap } from './cts/src/webgpu/web_platform/copyToTexture/ImageBitmap.spec';
import { g as web_platform_copyToTexture_video } from './cts/src/webgpu/web_platform/copyToTexture/video.spec';
import { g as web_platform_copyToTexture_image } from './cts/src/webgpu/web_platform/copyToTexture/image.spec';
import { g as web_platform_copyToTexture_ImageData } from './cts/src/webgpu/web_platform/copyToTexture/ImageData.spec';
import { g as web_platform_copyToTexture_canvas } from './cts/src/webgpu/web_platform/copyToTexture/canvas.spec';
import { g as print_environment } from './cts/src/webgpu/print_environment.spec';
import { g as util_texture_texel_data } from './cts/src/webgpu/util/texture/texel_data.spec';
import { g as util_texture_texture_ok } from './cts/src/webgpu/util/texture/texture_ok.spec';
import { g as util_texture_color_space_conversions } from './cts/src/webgpu/util/texture/color_space_conversions.spec';
import { g as api_validation_render_pass_attachment_compatibility } from './cts/src/webgpu/api/validation/render_pass/attachment_compatibility.spec';
import { g as api_validation_render_pass_resolve } from './cts/src/webgpu/api/validation/render_pass/resolve.spec';
import { g as api_validation_render_pass_render_pass_descriptor } from './cts/src/webgpu/api/validation/render_pass/render_pass_descriptor.spec';
import { g as api_validation_texture_bgra8unorm_storage } from './cts/src/webgpu/api/validation/texture/bgra8unorm_storage.spec';
import { g as api_validation_texture_float32_filterable } from './cts/src/webgpu/api/validation/texture/float32_filterable.spec';
import { g as api_validation_texture_destroy } from './cts/src/webgpu/api/validation/texture/destroy.spec';
import { g as api_validation_texture_rg11b10ufloat_renderable } from './cts/src/webgpu/api/validation/texture/rg11b10ufloat_renderable.spec';
import { g as api_validation_createView } from './cts/src/webgpu/api/validation/createView.spec';
import { g as api_validation_queue_destroyed_texture } from './cts/src/webgpu/api/validation/queue/destroyed/texture.spec';
import { g as api_validation_queue_destroyed_buffer } from './cts/src/webgpu/api/validation/queue/destroyed/buffer.spec';
import { g as api_validation_queue_destroyed_query_set } from './cts/src/webgpu/api/validation/queue/destroyed/query_set.spec';
import { g as api_validation_queue_writeTexture } from './cts/src/webgpu/api/validation/queue/writeTexture.spec';
import { g as api_validation_queue_submit } from './cts/src/webgpu/api/validation/queue/submit.spec';
import { g as api_validation_queue_buffer_mapped } from './cts/src/webgpu/api/validation/queue/buffer_mapped.spec';
import { g as api_validation_queue_writeBuffer } from './cts/src/webgpu/api/validation/queue/writeBuffer.spec';
import { g as api_validation_queue_copyToTexture_CopyExternalImageToTexture } from './cts/src/webgpu/api/validation/queue/copyToTexture/CopyExternalImageToTexture.spec';
import { g as api_validation_compute_pipeline } from './cts/src/webgpu/api/validation/compute_pipeline.spec';
import { g as api_validation_resource_usages_texture_in_render_common } from './cts/src/webgpu/api/validation/resource_usages/texture/in_render_common.spec';
import { g as api_validation_resource_usages_texture_in_render_misc } from './cts/src/webgpu/api/validation/resource_usages/texture/in_render_misc.spec';
import { g as api_validation_resource_usages_texture_in_pass_encoder } from './cts/src/webgpu/api/validation/resource_usages/texture/in_pass_encoder.spec';
import { g as api_validation_resource_usages_buffer_in_pass_encoder } from './cts/src/webgpu/api/validation/resource_usages/buffer/in_pass_encoder.spec';
import { g as api_validation_resource_usages_buffer_in_pass_misc } from './cts/src/webgpu/api/validation/resource_usages/buffer/in_pass_misc.spec';
import { g as api_validation_capability_checks_limits_maxTextureDimension3D } from './cts/src/webgpu/api/validation/capability_checks/limits/maxTextureDimension3D.spec';
import { g as api_validation_capability_checks_limits_maxSamplersPerShaderStage } from './cts/src/webgpu/api/validation/capability_checks/limits/maxSamplersPerShaderStage.spec';
import { g as api_validation_capability_checks_limits_maxBindingsPerBindGroup } from './cts/src/webgpu/api/validation/capability_checks/limits/maxBindingsPerBindGroup.spec';
import { g as api_validation_capability_checks_limits_maxComputeWorkgroupStorageSize } from './cts/src/webgpu/api/validation/capability_checks/limits/maxComputeWorkgroupStorageSize.spec';
import { g as api_validation_capability_checks_limits_maxVertexBufferArrayStride } from './cts/src/webgpu/api/validation/capability_checks/limits/maxVertexBufferArrayStride.spec';
import { g as api_validation_capability_checks_limits_minStorageBufferOffsetAlignment } from './cts/src/webgpu/api/validation/capability_checks/limits/minStorageBufferOffsetAlignment.spec';
import { g as api_validation_capability_checks_limits_maxInterStageShaderVariables } from './cts/src/webgpu/api/validation/capability_checks/limits/maxInterStageShaderVariables.spec';
import { g as api_validation_capability_checks_limits_maxDynamicUniformBuffersPerPipelineLayout } from './cts/src/webgpu/api/validation/capability_checks/limits/maxDynamicUniformBuffersPerPipelineLayout.spec';
import { g as api_validation_capability_checks_limits_maxComputeInvocationsPerWorkgroup } from './cts/src/webgpu/api/validation/capability_checks/limits/maxComputeInvocationsPerWorkgroup.spec';
import { g as api_validation_capability_checks_limits_maxUniformBufferBindingSize } from './cts/src/webgpu/api/validation/capability_checks/limits/maxUniformBufferBindingSize.spec';
import { g as api_validation_capability_checks_limits_maxVertexBuffers } from './cts/src/webgpu/api/validation/capability_checks/limits/maxVertexBuffers.spec';
import { g as api_validation_capability_checks_limits_maxBindGroupsPlusVertexBuffers } from './cts/src/webgpu/api/validation/capability_checks/limits/maxBindGroupsPlusVertexBuffers.spec';
import { g as api_validation_capability_checks_limits_maxDynamicStorageBuffersPerPipelineLayout } from './cts/src/webgpu/api/validation/capability_checks/limits/maxDynamicStorageBuffersPerPipelineLayout.spec';
import { g as api_validation_capability_checks_limits_maxStorageTexturesPerShaderStage } from './cts/src/webgpu/api/validation/capability_checks/limits/maxStorageTexturesPerShaderStage.spec';
import { g as api_validation_capability_checks_limits_minUniformBufferOffsetAlignment } from './cts/src/webgpu/api/validation/capability_checks/limits/minUniformBufferOffsetAlignment.spec';
import { g as api_validation_capability_checks_limits_maxStorageBufferBindingSize } from './cts/src/webgpu/api/validation/capability_checks/limits/maxStorageBufferBindingSize.spec';
import { g as api_validation_capability_checks_limits_maxStorageBuffersPerShaderStage } from './cts/src/webgpu/api/validation/capability_checks/limits/maxStorageBuffersPerShaderStage.spec';
import { g as api_validation_capability_checks_limits_maxComputeWorkgroupSizeX } from './cts/src/webgpu/api/validation/capability_checks/limits/maxComputeWorkgroupSizeX.spec';
import { g as api_validation_capability_checks_limits_maxUniformBuffersPerShaderStage } from './cts/src/webgpu/api/validation/capability_checks/limits/maxUniformBuffersPerShaderStage.spec';
import { g as api_validation_capability_checks_limits_maxVertexAttributes } from './cts/src/webgpu/api/validation/capability_checks/limits/maxVertexAttributes.spec';
import { g as api_validation_capability_checks_limits_maxComputeWorkgroupSizeZ } from './cts/src/webgpu/api/validation/capability_checks/limits/maxComputeWorkgroupSizeZ.spec';
import { g as api_validation_capability_checks_limits_maxComputeWorkgroupSizeY } from './cts/src/webgpu/api/validation/capability_checks/limits/maxComputeWorkgroupSizeY.spec';
import { g as api_validation_capability_checks_limits_maxColorAttachments } from './cts/src/webgpu/api/validation/capability_checks/limits/maxColorAttachments.spec';
import { g as api_validation_capability_checks_limits_maxTextureArrayLayers } from './cts/src/webgpu/api/validation/capability_checks/limits/maxTextureArrayLayers.spec';
import { g as api_validation_capability_checks_limits_maxTextureDimension2D } from './cts/src/webgpu/api/validation/capability_checks/limits/maxTextureDimension2D.spec';
import { g as api_validation_capability_checks_limits_maxSampledTexturesPerShaderStage } from './cts/src/webgpu/api/validation/capability_checks/limits/maxSampledTexturesPerShaderStage.spec';
import { g as api_validation_capability_checks_limits_maxBufferSize } from './cts/src/webgpu/api/validation/capability_checks/limits/maxBufferSize.spec';
import { g as api_validation_capability_checks_limits_maxComputeWorkgroupsPerDimension } from './cts/src/webgpu/api/validation/capability_checks/limits/maxComputeWorkgroupsPerDimension.spec';
import { g as api_validation_capability_checks_limits_maxTextureDimension1D } from './cts/src/webgpu/api/validation/capability_checks/limits/maxTextureDimension1D.spec';
import { g as api_validation_capability_checks_limits_maxBindGroups } from './cts/src/webgpu/api/validation/capability_checks/limits/maxBindGroups.spec';
import { g as api_validation_capability_checks_limits_maxColorAttachmentBytesPerSample } from './cts/src/webgpu/api/validation/capability_checks/limits/maxColorAttachmentBytesPerSample.spec';
import { g as api_validation_capability_checks_features_query_types } from './cts/src/webgpu/api/validation/capability_checks/features/query_types.spec';
import { g as api_validation_capability_checks_features_clip_distances } from './cts/src/webgpu/api/validation/capability_checks/features/clip_distances.spec';
import { g as api_validation_capability_checks_features_texture_formats } from './cts/src/webgpu/api/validation/capability_checks/features/texture_formats.spec';
import { g as api_validation_createPipelineLayout } from './cts/src/webgpu/api/validation/createPipelineLayout.spec';
import { g as api_validation_state_device_lost_destroy } from './cts/src/webgpu/api/validation/state/device_lost/destroy.spec';
import { g as api_validation_render_pipeline_multisample_state } from './cts/src/webgpu/api/validation/render_pipeline/multisample_state.spec';
import { g as api_validation_render_pipeline_inter_stage } from './cts/src/webgpu/api/validation/render_pipeline/inter_stage.spec';
import { g as api_validation_render_pipeline_resource_compatibility } from './cts/src/webgpu/api/validation/render_pipeline/resource_compatibility.spec';
import { g as api_validation_render_pipeline_depth_stencil_state } from './cts/src/webgpu/api/validation/render_pipeline/depth_stencil_state.spec';
import { g as api_validation_render_pipeline_shader_module } from './cts/src/webgpu/api/validation/render_pipeline/shader_module.spec';
import { g as api_validation_render_pipeline_misc } from './cts/src/webgpu/api/validation/render_pipeline/misc.spec';
import { g as api_validation_render_pipeline_fragment_state } from './cts/src/webgpu/api/validation/render_pipeline/fragment_state.spec';
import { g as api_validation_render_pipeline_overrides } from './cts/src/webgpu/api/validation/render_pipeline/overrides.spec';
import { g as api_validation_render_pipeline_vertex_state } from './cts/src/webgpu/api/validation/render_pipeline/vertex_state.spec';
import { g as api_validation_render_pipeline_primitive_state } from './cts/src/webgpu/api/validation/render_pipeline/primitive_state.spec';
import { g as api_validation_render_pipeline_float32_blendable } from './cts/src/webgpu/api/validation/render_pipeline/float32_blendable.spec';
import { g as api_validation_createBindGroup } from './cts/src/webgpu/api/validation/createBindGroup.spec';
import { g as api_validation_getBindGroupLayout } from './cts/src/webgpu/api/validation/getBindGroupLayout.spec';
import { g as api_validation_encoding_beginRenderPass } from './cts/src/webgpu/api/validation/encoding/beginRenderPass.spec';
import { g as api_validation_encoding_createRenderBundleEncoder } from './cts/src/webgpu/api/validation/encoding/createRenderBundleEncoder.spec';
import { g as api_validation_encoding_queries_begin_end } from './cts/src/webgpu/api/validation/encoding/queries/begin_end.spec';
import { g as api_validation_encoding_queries_resolveQuerySet } from './cts/src/webgpu/api/validation/encoding/queries/resolveQuerySet.spec';
import { g as api_validation_encoding_queries_general } from './cts/src/webgpu/api/validation/encoding/queries/general.spec';
import { g as api_validation_encoding_encoder_open_state } from './cts/src/webgpu/api/validation/encoding/encoder_open_state.spec';
import { g as api_validation_encoding_beginComputePass } from './cts/src/webgpu/api/validation/encoding/beginComputePass.spec';
import { g as api_validation_encoding_programmable_pipeline_bind_group_compat } from './cts/src/webgpu/api/validation/encoding/programmable/pipeline_bind_group_compat.spec';
import { g as api_validation_encoding_cmds_render_indirect_draw } from './cts/src/webgpu/api/validation/encoding/cmds/render/indirect_draw.spec';
import { g as api_validation_encoding_cmds_render_indirect_multi_draw } from './cts/src/webgpu/api/validation/encoding/cmds/render/indirect_multi_draw.spec';
import { g as api_validation_encoding_cmds_render_draw } from './cts/src/webgpu/api/validation/encoding/cmds/render/draw.spec';
import { g as api_validation_encoding_cmds_render_state_tracking } from './cts/src/webgpu/api/validation/encoding/cmds/render/state_tracking.spec';
import { g as api_validation_encoding_cmds_render_setVertexBuffer } from './cts/src/webgpu/api/validation/encoding/cmds/render/setVertexBuffer.spec';
import { g as api_validation_encoding_cmds_render_setPipeline } from './cts/src/webgpu/api/validation/encoding/cmds/render/setPipeline.spec';
import { g as api_validation_encoding_cmds_render_setIndexBuffer } from './cts/src/webgpu/api/validation/encoding/cmds/render/setIndexBuffer.spec';
import { g as api_validation_encoding_cmds_render_dynamic_state } from './cts/src/webgpu/api/validation/encoding/cmds/render/dynamic_state.spec';
import { g as api_validation_encoding_cmds_index_access } from './cts/src/webgpu/api/validation/encoding/cmds/index_access.spec';
import { g as api_validation_encoding_cmds_debug } from './cts/src/webgpu/api/validation/encoding/cmds/debug.spec';
import { g as api_validation_encoding_cmds_setBindGroup } from './cts/src/webgpu/api/validation/encoding/cmds/setBindGroup.spec';
import { g as api_validation_encoding_cmds_clearBuffer } from './cts/src/webgpu/api/validation/encoding/cmds/clearBuffer.spec';
import { g as api_validation_encoding_cmds_compute_pass } from './cts/src/webgpu/api/validation/encoding/cmds/compute_pass.spec';
import { g as api_validation_encoding_cmds_render_pass } from './cts/src/webgpu/api/validation/encoding/cmds/render_pass.spec';
import { g as api_validation_encoding_cmds_copyBufferToBuffer } from './cts/src/webgpu/api/validation/encoding/cmds/copyBufferToBuffer.spec';
import { g as api_validation_encoding_cmds_copyTextureToTexture } from './cts/src/webgpu/api/validation/encoding/cmds/copyTextureToTexture.spec';
import { g as api_validation_encoding_encoder_state } from './cts/src/webgpu/api/validation/encoding/encoder_state.spec';
import { g as api_validation_encoding_render_bundle } from './cts/src/webgpu/api/validation/encoding/render_bundle.spec';
import { g as api_validation_debugMarker } from './cts/src/webgpu/api/validation/debugMarker.spec';
import { g as api_validation_buffer_mapping } from './cts/src/webgpu/api/validation/buffer/mapping.spec';
import { g as api_validation_buffer_create } from './cts/src/webgpu/api/validation/buffer/create.spec';
import { g as api_validation_buffer_threading } from './cts/src/webgpu/api/validation/buffer/threading.spec';
import { g as api_validation_buffer_destroy } from './cts/src/webgpu/api/validation/buffer/destroy.spec';
import { g as api_validation_createBindGroupLayout } from './cts/src/webgpu/api/validation/createBindGroupLayout.spec';
import { g as api_validation_image_copy_texture_related } from './cts/src/webgpu/api/validation/image_copy/texture_related.spec';
import { g as api_validation_image_copy_buffer_related } from './cts/src/webgpu/api/validation/image_copy/buffer_related.spec';
import { g as api_validation_image_copy_layout_related } from './cts/src/webgpu/api/validation/image_copy/layout_related.spec';
import { g as api_validation_image_copy_buffer_texture_copies } from './cts/src/webgpu/api/validation/image_copy/buffer_texture_copies.spec';
import { g as api_validation_error_scope } from './cts/src/webgpu/api/validation/error_scope.spec';
import { g as api_validation_createSampler } from './cts/src/webgpu/api/validation/createSampler.spec';
import { g as api_validation_createTexture } from './cts/src/webgpu/api/validation/createTexture.spec';
import { g as api_validation_shader_module_entry_point } from './cts/src/webgpu/api/validation/shader_module/entry_point.spec';
import { g as api_validation_shader_module_overrides } from './cts/src/webgpu/api/validation/shader_module/overrides.spec';
import { g as api_validation_query_set_create } from './cts/src/webgpu/api/validation/query_set/create.spec';
import { g as api_validation_query_set_destroy } from './cts/src/webgpu/api/validation/query_set/destroy.spec';
import { g as api_validation_non_filterable_texture } from './cts/src/webgpu/api/validation/non_filterable_texture.spec';
import { g as api_validation_layout_shader_compat } from './cts/src/webgpu/api/validation/layout_shader_compat.spec';
import { g as api_validation_gpu_external_texture_expiration } from './cts/src/webgpu/api/validation/gpu_external_texture_expiration.spec';
import { g as api_operation_render_pass_storeop2 } from './cts/src/webgpu/api/operation/render_pass/storeop2.spec';
import { g as api_operation_render_pass_resolve } from './cts/src/webgpu/api/operation/render_pass/resolve.spec';
import { g as api_operation_render_pass_clear_value } from './cts/src/webgpu/api/operation/render_pass/clear_value.spec';
import { g as api_operation_render_pass_storeOp } from './cts/src/webgpu/api/operation/render_pass/storeOp.spec';
import { g as api_operation_memory_sync_texture_readonly_depth_stencil } from './cts/src/webgpu/api/operation/memory_sync/texture/readonly_depth_stencil.spec';
import { g as api_operation_memory_sync_texture_same_subresource } from './cts/src/webgpu/api/operation/memory_sync/texture/same_subresource.spec';
import { g as api_operation_memory_sync_buffer_multiple_buffers } from './cts/src/webgpu/api/operation/memory_sync/buffer/multiple_buffers.spec';
import { g as api_operation_memory_sync_buffer_single_buffer } from './cts/src/webgpu/api/operation/memory_sync/buffer/single_buffer.spec';
import { g as api_operation_resource_init_texture_zero } from './cts/src/webgpu/api/operation/resource_init/texture_zero.spec';
import { g as api_operation_resource_init_buffer } from './cts/src/webgpu/api/operation/resource_init/buffer.spec';
import { g as api_operation_device_all_limits_and_features } from './cts/src/webgpu/api/operation/device/all_limits_and_features.spec';
import { g as api_operation_device_lost } from './cts/src/webgpu/api/operation/device/lost.spec';
import { g as api_operation_queue_writeBuffer } from './cts/src/webgpu/api/operation/queue/writeBuffer.spec';
import { g as api_operation_buffers_map_oom } from './cts/src/webgpu/api/operation/buffers/map_oom.spec';
import { g as api_operation_buffers_threading } from './cts/src/webgpu/api/operation/buffers/threading.spec';
import { g as api_operation_buffers_map } from './cts/src/webgpu/api/operation/buffers/map.spec';
import { g as api_operation_buffers_map_detach } from './cts/src/webgpu/api/operation/buffers/map_detach.spec';
import { g as api_operation_buffers_map_ArrayBuffer } from './cts/src/webgpu/api/operation/buffers/map_ArrayBuffer.spec';
import { g as api_operation_pipeline_default_layout } from './cts/src/webgpu/api/operation/pipeline/default_layout.spec';
import { g as api_operation_labels } from './cts/src/webgpu/api/operation/labels.spec';
import { g as api_operation_command_buffer_render_state_tracking } from './cts/src/webgpu/api/operation/command_buffer/render/state_tracking.spec';
import { g as api_operation_command_buffer_render_dynamic_state } from './cts/src/webgpu/api/operation/command_buffer/render/dynamic_state.spec';
import { g as api_operation_command_buffer_basic } from './cts/src/webgpu/api/operation/command_buffer/basic.spec';
import { g as api_operation_command_buffer_queries_occlusionQuery } from './cts/src/webgpu/api/operation/command_buffer/queries/occlusionQuery.spec';
import { g as api_operation_command_buffer_programmable_state_tracking } from './cts/src/webgpu/api/operation/command_buffer/programmable/state_tracking.spec';
import { g as api_operation_command_buffer_clearBuffer } from './cts/src/webgpu/api/operation/command_buffer/clearBuffer.spec';
import { g as api_operation_command_buffer_image_copy } from './cts/src/webgpu/api/operation/command_buffer/image_copy.spec';
import { g as api_operation_command_buffer_copyBufferToBuffer } from './cts/src/webgpu/api/operation/command_buffer/copyBufferToBuffer.spec';
import { g as api_operation_command_buffer_copyTextureToTexture } from './cts/src/webgpu/api/operation/command_buffer/copyTextureToTexture.spec';
import { g as api_operation_adapter_info } from './cts/src/webgpu/api/operation/adapter/info.spec';
import { g as api_operation_adapter_requestAdapter } from './cts/src/webgpu/api/operation/adapter/requestAdapter.spec';
import { g as api_operation_adapter_requestDevice } from './cts/src/webgpu/api/operation/adapter/requestDevice.spec';
import { g as api_operation_render_pipeline_culling_tests } from './cts/src/webgpu/api/operation/render_pipeline/culling_tests.spec';
import { g as api_operation_render_pipeline_sample_mask } from './cts/src/webgpu/api/operation/render_pipeline/sample_mask.spec';
import { g as api_operation_render_pipeline_vertex_only_render_pipeline } from './cts/src/webgpu/api/operation/render_pipeline/vertex_only_render_pipeline.spec';
import { g as api_operation_render_pipeline_pipeline_output_targets } from './cts/src/webgpu/api/operation/render_pipeline/pipeline_output_targets.spec';
import { g as api_operation_render_pipeline_overrides } from './cts/src/webgpu/api/operation/render_pipeline/overrides.spec';
import { g as api_operation_render_pipeline_primitive_topology } from './cts/src/webgpu/api/operation/render_pipeline/primitive_topology.spec';
import { g as api_operation_sampling_anisotropy } from './cts/src/webgpu/api/operation/sampling/anisotropy.spec';
import { g as api_operation_sampling_lod_clamp } from './cts/src/webgpu/api/operation/sampling/lod_clamp.spec';
import { g as api_operation_sampling_sampler_texture } from './cts/src/webgpu/api/operation/sampling/sampler_texture.spec';
import { g as api_operation_sampling_filter_mode } from './cts/src/webgpu/api/operation/sampling/filter_mode.spec';
import { g as api_operation_reflection } from './cts/src/webgpu/api/operation/reflection.spec';
import { g as api_operation_texture_view_read } from './cts/src/webgpu/api/operation/texture_view/read.spec';
import { g as api_operation_texture_view_write } from './cts/src/webgpu/api/operation/texture_view/write.spec';
import { g as api_operation_texture_view_format_reinterpretation } from './cts/src/webgpu/api/operation/texture_view/format_reinterpretation.spec';
import { g as api_operation_uncapturederror } from './cts/src/webgpu/api/operation/uncapturederror.spec';
import { g as api_operation_compute_pipeline_entry_point_name } from './cts/src/webgpu/api/operation/compute_pipeline/entry_point_name.spec';
import { g as api_operation_compute_pipeline_overrides } from './cts/src/webgpu/api/operation/compute_pipeline/overrides.spec';
import { g as api_operation_vertex_state_index_format } from './cts/src/webgpu/api/operation/vertex_state/index_format.spec';
import { g as api_operation_vertex_state_correctness } from './cts/src/webgpu/api/operation/vertex_state/correctness.spec';
import { g as api_operation_shader_module_compilation_info } from './cts/src/webgpu/api/operation/shader_module/compilation_info.spec';
import { g as api_operation_storage_texture_read_only } from './cts/src/webgpu/api/operation/storage_texture/read_only.spec';
import { g as api_operation_storage_texture_read_write } from './cts/src/webgpu/api/operation/storage_texture/read_write.spec';
import { g as api_operation_rendering_color_target_state } from './cts/src/webgpu/api/operation/rendering/color_target_state.spec';
import { g as api_operation_rendering_basic } from './cts/src/webgpu/api/operation/rendering/basic.spec';
import { g as api_operation_rendering_indirect_draw } from './cts/src/webgpu/api/operation/rendering/indirect_draw.spec';
import { g as api_operation_rendering_draw } from './cts/src/webgpu/api/operation/rendering/draw.spec';
import { g as api_operation_rendering_robust_access_index } from './cts/src/webgpu/api/operation/rendering/robust_access_index.spec';
import { g as api_operation_rendering_depth_bias } from './cts/src/webgpu/api/operation/rendering/depth_bias.spec';
import { g as api_operation_rendering_stencil } from './cts/src/webgpu/api/operation/rendering/stencil.spec';
import { g as api_operation_rendering_3d_texture_slices } from './cts/src/webgpu/api/operation/rendering/3d_texture_slices.spec';
import { g as api_operation_rendering_depth } from './cts/src/webgpu/api/operation/rendering/depth.spec';
import { g as api_operation_rendering_depth_clip_clamp } from './cts/src/webgpu/api/operation/rendering/depth_clip_clamp.spec';
import { g as api_operation_onSubmittedWorkDone } from './cts/src/webgpu/api/operation/onSubmittedWorkDone.spec';
import { g as api_operation_compute_basic } from './cts/src/webgpu/api/operation/compute/basic.spec';
import { g as shader_execution_memory_layout } from './cts/src/webgpu/shader/execution/memory_layout.spec';
import { g as shader_execution_float_parse } from './cts/src/webgpu/shader/execution/float_parse.spec';
import { g as shader_execution_shader_io_fragment_builtins } from './cts/src/webgpu/shader/execution/shader_io/fragment_builtins.spec';
import { g as shader_execution_shader_io_compute_builtins } from './cts/src/webgpu/shader/execution/shader_io/compute_builtins.spec';
import { g as shader_execution_shader_io_user_io } from './cts/src/webgpu/shader/execution/shader_io/user_io.spec';
import { g as shader_execution_shader_io_shared_structs } from './cts/src/webgpu/shader/execution/shader_io/shared_structs.spec';
import { g as shader_execution_shader_io_workgroup_size } from './cts/src/webgpu/shader/execution/shader_io/workgroup_size.spec';
import { g as shader_execution_shader_io_vertex_builtins } from './cts/src/webgpu/shader/execution/shader_io/vertex_builtins.spec';
import { g as shader_execution_stage } from './cts/src/webgpu/shader/execution/stage.spec';
import { g as shader_execution_memory_model_texture_intra_invocation_coherence } from './cts/src/webgpu/shader/execution/memory_model/texture_intra_invocation_coherence.spec';
import { g as shader_execution_memory_model_weak } from './cts/src/webgpu/shader/execution/memory_model/weak.spec';
import { g as shader_execution_memory_model_coherence } from './cts/src/webgpu/shader/execution/memory_model/coherence.spec';
import { g as shader_execution_memory_model_atomicity } from './cts/src/webgpu/shader/execution/memory_model/atomicity.spec';
import { g as shader_execution_memory_model_barrier } from './cts/src/webgpu/shader/execution/memory_model/barrier.spec';
import { g as shader_execution_memory_model_adjacent } from './cts/src/webgpu/shader/execution/memory_model/adjacent.spec';
import { g as shader_execution_flow_control_complex } from './cts/src/webgpu/shader/execution/flow_control/complex.spec';
import { g as shader_execution_flow_control_switch } from './cts/src/webgpu/shader/execution/flow_control/switch.spec';
import { g as shader_execution_flow_control_loop } from './cts/src/webgpu/shader/execution/flow_control/loop.spec';
import { g as shader_execution_flow_control_phony } from './cts/src/webgpu/shader/execution/flow_control/phony.spec';
import { g as shader_execution_flow_control_eval_order } from './cts/src/webgpu/shader/execution/flow_control/eval_order.spec';
import { g as shader_execution_flow_control_for } from './cts/src/webgpu/shader/execution/flow_control/for.spec';
import { g as shader_execution_flow_control_while } from './cts/src/webgpu/shader/execution/flow_control/while.spec';
import { g as shader_execution_flow_control_if } from './cts/src/webgpu/shader/execution/flow_control/if.spec';
import { g as shader_execution_flow_control_call } from './cts/src/webgpu/shader/execution/flow_control/call.spec';
import { g as shader_execution_flow_control_return } from './cts/src/webgpu/shader/execution/flow_control/return.spec';
import { g as shader_execution_robust_access_vertex } from './cts/src/webgpu/shader/execution/robust_access_vertex.spec';
import { g as shader_execution_limits } from './cts/src/webgpu/shader/execution/limits.spec';
import { g as shader_execution_robust_access } from './cts/src/webgpu/shader/execution/robust_access.spec';
import { g as shader_execution_padding } from './cts/src/webgpu/shader/execution/padding.spec';
import { g as shader_execution_statement_increment_decrement } from './cts/src/webgpu/shader/execution/statement/increment_decrement.spec';
import { g as shader_execution_statement_discard } from './cts/src/webgpu/shader/execution/statement/discard.spec';
import { g as shader_execution_statement_phony } from './cts/src/webgpu/shader/execution/statement/phony.spec';
import { g as shader_execution_statement_compound } from './cts/src/webgpu/shader/execution/statement/compound.spec';
import { g as shader_execution_shadow } from './cts/src/webgpu/shader/execution/shadow.spec';
import { g as shader_execution_zero_init } from './cts/src/webgpu/shader/execution/zero_init.spec';
import { g as shader_execution_value_init } from './cts/src/webgpu/shader/execution/value_init.spec';
import { g as shader_execution_expression_call_user_ptr_params } from './cts/src/webgpu/shader/execution/expression/call/user/ptr_params.spec';
import { g as shader_execution_expression_call_builtin_subgroupElect } from './cts/src/webgpu/shader/execution/expression/call/builtin/subgroupElect.spec';
import { g as shader_execution_expression_call_builtin_firstTrailingBit } from './cts/src/webgpu/shader/execution/expression/call/builtin/firstTrailingBit.spec';
import { g as shader_execution_expression_call_builtin_textureSampleCompareLevel } from './cts/src/webgpu/shader/execution/expression/call/builtin/textureSampleCompareLevel.spec';
import { g as shader_execution_expression_call_builtin_cross } from './cts/src/webgpu/shader/execution/expression/call/builtin/cross.spec';
import { g as shader_execution_expression_call_builtin_step } from './cts/src/webgpu/shader/execution/expression/call/builtin/step.spec';
import { g as shader_execution_expression_call_builtin_abs } from './cts/src/webgpu/shader/execution/expression/call/builtin/abs.spec';
import { g as shader_execution_expression_call_builtin_textureDimensions } from './cts/src/webgpu/shader/execution/expression/call/builtin/textureDimensions.spec';
import { g as shader_execution_expression_call_builtin_determinant } from './cts/src/webgpu/shader/execution/expression/call/builtin/determinant.spec';
import { g as shader_execution_expression_call_builtin_unpack2x16snorm } from './cts/src/webgpu/shader/execution/expression/call/builtin/unpack2x16snorm.spec';
import { g as shader_execution_expression_call_builtin_subgroupMul } from './cts/src/webgpu/shader/execution/expression/call/builtin/subgroupMul.spec';
import { g as shader_execution_expression_call_builtin_transpose } from './cts/src/webgpu/shader/execution/expression/call/builtin/transpose.spec';
import { g as shader_execution_expression_call_builtin_subgroupAdd } from './cts/src/webgpu/shader/execution/expression/call/builtin/subgroupAdd.spec';
import { g as shader_execution_expression_call_builtin_faceForward } from './cts/src/webgpu/shader/execution/expression/call/builtin/faceForward.spec';
import { g as shader_execution_expression_call_builtin_textureNumLevels } from './cts/src/webgpu/shader/execution/expression/call/builtin/textureNumLevels.spec';
import { g as shader_execution_expression_call_builtin_ldexp } from './cts/src/webgpu/shader/execution/expression/call/builtin/ldexp.spec';
import { g as shader_execution_expression_call_builtin_textureGatherCompare } from './cts/src/webgpu/shader/execution/expression/call/builtin/textureGatherCompare.spec';
import { g as shader_execution_expression_call_builtin_unpack4xU8 } from './cts/src/webgpu/shader/execution/expression/call/builtin/unpack4xU8.spec';
import { g as shader_execution_expression_call_builtin_textureSampleGrad } from './cts/src/webgpu/shader/execution/expression/call/builtin/textureSampleGrad.spec';
import { g as shader_execution_expression_call_builtin_workgroupBarrier } from './cts/src/webgpu/shader/execution/expression/call/builtin/workgroupBarrier.spec';
import { g as shader_execution_expression_call_builtin_ceil } from './cts/src/webgpu/shader/execution/expression/call/builtin/ceil.spec';
import { g as shader_execution_expression_call_builtin_quantizeToF16 } from './cts/src/webgpu/shader/execution/expression/call/builtin/quantizeToF16.spec';
import { g as shader_execution_expression_call_builtin_pack2x16unorm } from './cts/src/webgpu/shader/execution/expression/call/builtin/pack2x16unorm.spec';
import { g as shader_execution_expression_call_builtin_subgroupMinMax } from './cts/src/webgpu/shader/execution/expression/call/builtin/subgroupMinMax.spec';
import { g as shader_execution_expression_call_builtin_frexp } from './cts/src/webgpu/shader/execution/expression/call/builtin/frexp.spec';
import { g as shader_execution_expression_call_builtin_fract } from './cts/src/webgpu/shader/execution/expression/call/builtin/fract.spec';
import { g as shader_execution_expression_call_builtin_radians } from './cts/src/webgpu/shader/execution/expression/call/builtin/radians.spec';
import { g as shader_execution_expression_call_builtin_degrees } from './cts/src/webgpu/shader/execution/expression/call/builtin/degrees.spec';
import { g as shader_execution_expression_call_builtin_inversesqrt } from './cts/src/webgpu/shader/execution/expression/call/builtin/inversesqrt.spec';
import { g as shader_execution_expression_call_builtin_clamp } from './cts/src/webgpu/shader/execution/expression/call/builtin/clamp.spec';
import { g as shader_execution_expression_call_builtin_subgroupAll } from './cts/src/webgpu/shader/execution/expression/call/builtin/subgroupAll.spec';
import { g as shader_execution_expression_call_builtin_countOneBits } from './cts/src/webgpu/shader/execution/expression/call/builtin/countOneBits.spec';
import { g as shader_execution_expression_call_builtin_select } from './cts/src/webgpu/shader/execution/expression/call/builtin/select.spec';
import { g as shader_execution_expression_call_builtin_any } from './cts/src/webgpu/shader/execution/expression/call/builtin/any.spec';
import { g as shader_execution_expression_call_builtin_countLeadingZeros } from './cts/src/webgpu/shader/execution/expression/call/builtin/countLeadingZeros.spec';
import { g as shader_execution_expression_call_builtin_sin } from './cts/src/webgpu/shader/execution/expression/call/builtin/sin.spec';
import { g as shader_execution_expression_call_builtin_fwidthFine } from './cts/src/webgpu/shader/execution/expression/call/builtin/fwidthFine.spec';
import { g as shader_execution_expression_call_builtin_sinh } from './cts/src/webgpu/shader/execution/expression/call/builtin/sinh.spec';
import { g as shader_execution_expression_call_builtin_min } from './cts/src/webgpu/shader/execution/expression/call/builtin/min.spec';
import { g as shader_execution_expression_call_builtin_dot } from './cts/src/webgpu/shader/execution/expression/call/builtin/dot.spec';
import { g as shader_execution_expression_call_builtin_reflect } from './cts/src/webgpu/shader/execution/expression/call/builtin/reflect.spec';
import { g as shader_execution_expression_call_builtin_extractBits } from './cts/src/webgpu/shader/execution/expression/call/builtin/extractBits.spec';
import { g as shader_execution_expression_call_builtin_refract } from './cts/src/webgpu/shader/execution/expression/call/builtin/refract.spec';
import { g as shader_execution_expression_call_builtin_log2 } from './cts/src/webgpu/shader/execution/expression/call/builtin/log2.spec';
import { g as shader_execution_expression_call_builtin_sign } from './cts/src/webgpu/shader/execution/expression/call/builtin/sign.spec';
import { g as shader_execution_expression_call_builtin_atan } from './cts/src/webgpu/shader/execution/expression/call/builtin/atan.spec';
import { g as shader_execution_expression_call_builtin_bitcast } from './cts/src/webgpu/shader/execution/expression/call/builtin/bitcast.spec';
import { g as shader_execution_expression_call_builtin_acosh } from './cts/src/webgpu/shader/execution/expression/call/builtin/acosh.spec';
import { g as shader_execution_expression_call_builtin_pack2x16snorm } from './cts/src/webgpu/shader/execution/expression/call/builtin/pack2x16snorm.spec';
import { g as shader_execution_expression_call_builtin_textureNumSamples } from './cts/src/webgpu/shader/execution/expression/call/builtin/textureNumSamples.spec';
import { g as shader_execution_expression_call_builtin_cos } from './cts/src/webgpu/shader/execution/expression/call/builtin/cos.spec';
import { g as shader_execution_expression_call_builtin_subgroupBallot } from './cts/src/webgpu/shader/execution/expression/call/builtin/subgroupBallot.spec';
import { g as shader_execution_expression_call_builtin_workgroupUniformLoad } from './cts/src/webgpu/shader/execution/expression/call/builtin/workgroupUniformLoad.spec';
import { g as shader_execution_expression_call_builtin_dpdx } from './cts/src/webgpu/shader/execution/expression/call/builtin/dpdx.spec';
import { g as shader_execution_expression_call_builtin_pack4x8snorm } from './cts/src/webgpu/shader/execution/expression/call/builtin/pack4x8snorm.spec';
import { g as shader_execution_expression_call_builtin_pack2x16float } from './cts/src/webgpu/shader/execution/expression/call/builtin/pack2x16float.spec';
import { g as shader_execution_expression_call_builtin_fwidthCoarse } from './cts/src/webgpu/shader/execution/expression/call/builtin/fwidthCoarse.spec';
import { g as shader_execution_expression_call_builtin_textureGather } from './cts/src/webgpu/shader/execution/expression/call/builtin/textureGather.spec';
import { g as shader_execution_expression_call_builtin_all } from './cts/src/webgpu/shader/execution/expression/call/builtin/all.spec';
import { g as shader_execution_expression_call_builtin_atan2 } from './cts/src/webgpu/shader/execution/expression/call/builtin/atan2.spec';
import { g as shader_execution_expression_call_builtin_pack4xI8 } from './cts/src/webgpu/shader/execution/expression/call/builtin/pack4xI8.spec';
import { g as shader_execution_expression_call_builtin_subgroupShuffle } from './cts/src/webgpu/shader/execution/expression/call/builtin/subgroupShuffle.spec';
import { g as shader_execution_expression_call_builtin_firstLeadingBit } from './cts/src/webgpu/shader/execution/expression/call/builtin/firstLeadingBit.spec';
import { g as shader_execution_expression_call_builtin_exp2 } from './cts/src/webgpu/shader/execution/expression/call/builtin/exp2.spec';
import { g as shader_execution_expression_call_builtin_dpdyFine } from './cts/src/webgpu/shader/execution/expression/call/builtin/dpdyFine.spec';
import { g as shader_execution_expression_call_builtin_dot4I8Packed } from './cts/src/webgpu/shader/execution/expression/call/builtin/dot4I8Packed.spec';
import { g as shader_execution_expression_call_builtin_round } from './cts/src/webgpu/shader/execution/expression/call/builtin/round.spec';
import { g as shader_execution_expression_call_builtin_pack4xU8Clamp } from './cts/src/webgpu/shader/execution/expression/call/builtin/pack4xU8Clamp.spec';
import { g as shader_execution_expression_call_builtin_floor } from './cts/src/webgpu/shader/execution/expression/call/builtin/floor.spec';
import { g as shader_execution_expression_call_builtin_unpack4x8snorm } from './cts/src/webgpu/shader/execution/expression/call/builtin/unpack4x8snorm.spec';
import { g as shader_execution_expression_call_builtin_unpack4x8unorm } from './cts/src/webgpu/shader/execution/expression/call/builtin/unpack4x8unorm.spec';
import { g as shader_execution_expression_call_builtin_textureSampleBias } from './cts/src/webgpu/shader/execution/expression/call/builtin/textureSampleBias.spec';
import { g as shader_execution_expression_call_builtin_dpdyCoarse } from './cts/src/webgpu/shader/execution/expression/call/builtin/dpdyCoarse.spec';
import { g as shader_execution_expression_call_builtin_unpack4xI8 } from './cts/src/webgpu/shader/execution/expression/call/builtin/unpack4xI8.spec';
import { g as shader_execution_expression_call_builtin_dot4U8Packed } from './cts/src/webgpu/shader/execution/expression/call/builtin/dot4U8Packed.spec';
import { g as shader_execution_expression_call_builtin_modf } from './cts/src/webgpu/shader/execution/expression/call/builtin/modf.spec';
import { g as shader_execution_expression_call_builtin_textureSampleBaseClampToEdge } from './cts/src/webgpu/shader/execution/expression/call/builtin/textureSampleBaseClampToEdge.spec';
import { g as shader_execution_expression_call_builtin_normalize } from './cts/src/webgpu/shader/execution/expression/call/builtin/normalize.spec';
import { g as shader_execution_expression_call_builtin_dpdxCoarse } from './cts/src/webgpu/shader/execution/expression/call/builtin/dpdxCoarse.spec';
import { g as shader_execution_expression_call_builtin_length } from './cts/src/webgpu/shader/execution/expression/call/builtin/length.spec';
import { g as shader_execution_expression_call_builtin_countTrailingZeros } from './cts/src/webgpu/shader/execution/expression/call/builtin/countTrailingZeros.spec';
import { g as shader_execution_expression_call_builtin_arrayLength } from './cts/src/webgpu/shader/execution/expression/call/builtin/arrayLength.spec';
import { g as shader_execution_expression_call_builtin_pow } from './cts/src/webgpu/shader/execution/expression/call/builtin/pow.spec';
import { g as shader_execution_expression_call_builtin_subgroupBroadcast } from './cts/src/webgpu/shader/execution/expression/call/builtin/subgroupBroadcast.spec';
import { g as shader_execution_expression_call_builtin_textureLoad } from './cts/src/webgpu/shader/execution/expression/call/builtin/textureLoad.spec';
import { g as shader_execution_expression_call_builtin_dpdy } from './cts/src/webgpu/shader/execution/expression/call/builtin/dpdy.spec';
import { g as shader_execution_expression_call_builtin_atanh } from './cts/src/webgpu/shader/execution/expression/call/builtin/atanh.spec';
import { g as shader_execution_expression_call_builtin_tanh } from './cts/src/webgpu/shader/execution/expression/call/builtin/tanh.spec';
import { g as shader_execution_expression_call_builtin_texture_utils } from './cts/src/webgpu/shader/execution/expression/call/builtin/texture_utils.spec';
import { g as shader_execution_expression_call_builtin_log } from './cts/src/webgpu/shader/execution/expression/call/builtin/log.spec';
import { g as shader_execution_expression_call_builtin_subgroupBitwise } from './cts/src/webgpu/shader/execution/expression/call/builtin/subgroupBitwise.spec';
import { g as shader_execution_expression_call_builtin_dpdxFine } from './cts/src/webgpu/shader/execution/expression/call/builtin/dpdxFine.spec';
import { g as shader_execution_expression_call_builtin_quadBroadcast } from './cts/src/webgpu/shader/execution/expression/call/builtin/quadBroadcast.spec';
import { g as shader_execution_expression_call_builtin_pack4xI8Clamp } from './cts/src/webgpu/shader/execution/expression/call/builtin/pack4xI8Clamp.spec';
import { g as shader_execution_expression_call_builtin_exp } from './cts/src/webgpu/shader/execution/expression/call/builtin/exp.spec';
import { g as shader_execution_expression_call_builtin_subgroupAny } from './cts/src/webgpu/shader/execution/expression/call/builtin/subgroupAny.spec';
import { g as shader_execution_expression_call_builtin_pack4xU8 } from './cts/src/webgpu/shader/execution/expression/call/builtin/pack4xU8.spec';
import { g as shader_execution_expression_call_builtin_asin } from './cts/src/webgpu/shader/execution/expression/call/builtin/asin.spec';
import { g as shader_execution_expression_call_builtin_acos } from './cts/src/webgpu/shader/execution/expression/call/builtin/acos.spec';
import { g as shader_execution_expression_call_builtin_cosh } from './cts/src/webgpu/shader/execution/expression/call/builtin/cosh.spec';
import { g as shader_execution_expression_call_builtin_unpack2x16float } from './cts/src/webgpu/shader/execution/expression/call/builtin/unpack2x16float.spec';
import { g as shader_execution_expression_call_builtin_insertBits } from './cts/src/webgpu/shader/execution/expression/call/builtin/insertBits.spec';
import { g as shader_execution_expression_call_builtin_textureNumLayers } from './cts/src/webgpu/shader/execution/expression/call/builtin/textureNumLayers.spec';
import { g as shader_execution_expression_call_builtin_quadSwap } from './cts/src/webgpu/shader/execution/expression/call/builtin/quadSwap.spec';
import { g as shader_execution_expression_call_builtin_unpack2x16unorm } from './cts/src/webgpu/shader/execution/expression/call/builtin/unpack2x16unorm.spec';
import { g as shader_execution_expression_call_builtin_saturate } from './cts/src/webgpu/shader/execution/expression/call/builtin/saturate.spec';
import { g as shader_execution_expression_call_builtin_textureSampleCompare } from './cts/src/webgpu/shader/execution/expression/call/builtin/textureSampleCompare.spec';
import { g as shader_execution_expression_call_builtin_smoothstep } from './cts/src/webgpu/shader/execution/expression/call/builtin/smoothstep.spec';
import { g as shader_execution_expression_call_builtin_fwidth } from './cts/src/webgpu/shader/execution/expression/call/builtin/fwidth.spec';
import { g as shader_execution_expression_call_builtin_atomics_atomicMax } from './cts/src/webgpu/shader/execution/expression/call/builtin/atomics/atomicMax.spec';
import { g as shader_execution_expression_call_builtin_atomics_atomicOr } from './cts/src/webgpu/shader/execution/expression/call/builtin/atomics/atomicOr.spec';
import { g as shader_execution_expression_call_builtin_atomics_atomicExchange } from './cts/src/webgpu/shader/execution/expression/call/builtin/atomics/atomicExchange.spec';
import { g as shader_execution_expression_call_builtin_atomics_atomicStore } from './cts/src/webgpu/shader/execution/expression/call/builtin/atomics/atomicStore.spec';
import { g as shader_execution_expression_call_builtin_atomics_atomicXor } from './cts/src/webgpu/shader/execution/expression/call/builtin/atomics/atomicXor.spec';
import { g as shader_execution_expression_call_builtin_atomics_atomicLoad } from './cts/src/webgpu/shader/execution/expression/call/builtin/atomics/atomicLoad.spec';
import { g as shader_execution_expression_call_builtin_atomics_atomicAdd } from './cts/src/webgpu/shader/execution/expression/call/builtin/atomics/atomicAdd.spec';
import { g as shader_execution_expression_call_builtin_atomics_atomicMin } from './cts/src/webgpu/shader/execution/expression/call/builtin/atomics/atomicMin.spec';
import { g as shader_execution_expression_call_builtin_atomics_atomicSub } from './cts/src/webgpu/shader/execution/expression/call/builtin/atomics/atomicSub.spec';
import { g as shader_execution_expression_call_builtin_atomics_atomicAnd } from './cts/src/webgpu/shader/execution/expression/call/builtin/atomics/atomicAnd.spec';
import { g as shader_execution_expression_call_builtin_atomics_atomicCompareExchangeWeak } from './cts/src/webgpu/shader/execution/expression/call/builtin/atomics/atomicCompareExchangeWeak.spec';
import { g as shader_execution_expression_call_builtin_textureSample } from './cts/src/webgpu/shader/execution/expression/call/builtin/textureSample.spec';
import { g as shader_execution_expression_call_builtin_trunc } from './cts/src/webgpu/shader/execution/expression/call/builtin/trunc.spec';
import { g as shader_execution_expression_call_builtin_fma } from './cts/src/webgpu/shader/execution/expression/call/builtin/fma.spec';
import { g as shader_execution_expression_call_builtin_mix } from './cts/src/webgpu/shader/execution/expression/call/builtin/mix.spec';
import { g as shader_execution_expression_call_builtin_sqrt } from './cts/src/webgpu/shader/execution/expression/call/builtin/sqrt.spec';
import { g as shader_execution_expression_call_builtin_asinh } from './cts/src/webgpu/shader/execution/expression/call/builtin/asinh.spec';
import { g as shader_execution_expression_call_builtin_textureSampleLevel } from './cts/src/webgpu/shader/execution/expression/call/builtin/textureSampleLevel.spec';
import { g as shader_execution_expression_call_builtin_distance } from './cts/src/webgpu/shader/execution/expression/call/builtin/distance.spec';
import { g as shader_execution_expression_call_builtin_max } from './cts/src/webgpu/shader/execution/expression/call/builtin/max.spec';
import { g as shader_execution_expression_call_builtin_storageBarrier } from './cts/src/webgpu/shader/execution/expression/call/builtin/storageBarrier.spec';
import { g as shader_execution_expression_call_builtin_reverseBits } from './cts/src/webgpu/shader/execution/expression/call/builtin/reverseBits.spec';
import { g as shader_execution_expression_call_builtin_tan } from './cts/src/webgpu/shader/execution/expression/call/builtin/tan.spec';
import { g as shader_execution_expression_call_builtin_pack4x8unorm } from './cts/src/webgpu/shader/execution/expression/call/builtin/pack4x8unorm.spec';
import { g as shader_execution_expression_call_builtin_textureStore } from './cts/src/webgpu/shader/execution/expression/call/builtin/textureStore.spec';
import { g as shader_execution_expression_constructor_non_zero } from './cts/src/webgpu/shader/execution/expression/constructor/non_zero.spec';
import { g as shader_execution_expression_constructor_zero_value } from './cts/src/webgpu/shader/execution/expression/constructor/zero_value.spec';
import { g as shader_execution_expression_unary_address_of_and_indirection } from './cts/src/webgpu/shader/execution/expression/unary/address_of_and_indirection.spec';
import { g as shader_execution_expression_unary_bool_logical } from './cts/src/webgpu/shader/execution/expression/unary/bool_logical.spec';
import { g as shader_execution_expression_unary_f32_conversion } from './cts/src/webgpu/shader/execution/expression/unary/f32_conversion.spec';
import { g as shader_execution_expression_unary_ai_complement } from './cts/src/webgpu/shader/execution/expression/unary/ai_complement.spec';
import { g as shader_execution_expression_unary_af_arithmetic } from './cts/src/webgpu/shader/execution/expression/unary/af_arithmetic.spec';
import { g as shader_execution_expression_unary_i32_complement } from './cts/src/webgpu/shader/execution/expression/unary/i32_complement.spec';
import { g as shader_execution_expression_unary_af_assignment } from './cts/src/webgpu/shader/execution/expression/unary/af_assignment.spec';
import { g as shader_execution_expression_unary_u32_complement } from './cts/src/webgpu/shader/execution/expression/unary/u32_complement.spec';
import { g as shader_execution_expression_unary_ai_arithmetic } from './cts/src/webgpu/shader/execution/expression/unary/ai_arithmetic.spec';
import { g as shader_execution_expression_unary_ai_assignment } from './cts/src/webgpu/shader/execution/expression/unary/ai_assignment.spec';
import { g as shader_execution_expression_unary_i32_conversion } from './cts/src/webgpu/shader/execution/expression/unary/i32_conversion.spec';
import { g as shader_execution_expression_unary_f16_arithmetic } from './cts/src/webgpu/shader/execution/expression/unary/f16_arithmetic.spec';
import { g as shader_execution_expression_unary_f32_arithmetic } from './cts/src/webgpu/shader/execution/expression/unary/f32_arithmetic.spec';
import { g as shader_execution_expression_unary_bool_conversion } from './cts/src/webgpu/shader/execution/expression/unary/bool_conversion.spec';
import { g as shader_execution_expression_unary_u32_conversion } from './cts/src/webgpu/shader/execution/expression/unary/u32_conversion.spec';
import { g as shader_execution_expression_unary_f16_conversion } from './cts/src/webgpu/shader/execution/expression/unary/f16_conversion.spec';
import { g as shader_execution_expression_unary_i32_arithmetic } from './cts/src/webgpu/shader/execution/expression/unary/i32_arithmetic.spec';
import { g as shader_execution_expression_binary_f32_remainder } from './cts/src/webgpu/shader/execution/expression/binary/f32_remainder.spec';
import { g as shader_execution_expression_binary_f16_matrix_addition } from './cts/src/webgpu/shader/execution/expression/binary/f16_matrix_addition.spec';
import { g as shader_execution_expression_binary_bool_logical } from './cts/src/webgpu/shader/execution/expression/binary/bool_logical.spec';
import { g as shader_execution_expression_binary_f16_matrix_subtraction } from './cts/src/webgpu/shader/execution/expression/binary/f16_matrix_subtraction.spec';
import { g as shader_execution_expression_binary_u32_comparison } from './cts/src/webgpu/shader/execution/expression/binary/u32_comparison.spec';
import { g as shader_execution_expression_binary_f32_matrix_scalar_multiplication } from './cts/src/webgpu/shader/execution/expression/binary/f32_matrix_scalar_multiplication.spec';
import { g as shader_execution_expression_binary_f16_remainder } from './cts/src/webgpu/shader/execution/expression/binary/f16_remainder.spec';
import { g as shader_execution_expression_binary_af_subtraction } from './cts/src/webgpu/shader/execution/expression/binary/af_subtraction.spec';
import { g as shader_execution_expression_binary_f32_multiplication } from './cts/src/webgpu/shader/execution/expression/binary/f32_multiplication.spec';
import { g as shader_execution_expression_binary_i32_comparison } from './cts/src/webgpu/shader/execution/expression/binary/i32_comparison.spec';
import { g as shader_execution_expression_binary_f32_addition } from './cts/src/webgpu/shader/execution/expression/binary/f32_addition.spec';
import { g as shader_execution_expression_binary_f16_subtraction } from './cts/src/webgpu/shader/execution/expression/binary/f16_subtraction.spec';
import { g as shader_execution_expression_binary_f16_multiplication } from './cts/src/webgpu/shader/execution/expression/binary/f16_multiplication.spec';
import { g as shader_execution_expression_binary_f16_addition } from './cts/src/webgpu/shader/execution/expression/binary/f16_addition.spec';
import { g as shader_execution_expression_binary_af_multiplication } from './cts/src/webgpu/shader/execution/expression/binary/af_multiplication.spec';
import { g as shader_execution_expression_binary_f16_division } from './cts/src/webgpu/shader/execution/expression/binary/f16_division.spec';
import { g as shader_execution_expression_binary_af_comparison } from './cts/src/webgpu/shader/execution/expression/binary/af_comparison.spec';
import { g as shader_execution_expression_binary_f32_comparison } from './cts/src/webgpu/shader/execution/expression/binary/f32_comparison.spec';
import { g as shader_execution_expression_binary_af_matrix_scalar_multiplication } from './cts/src/webgpu/shader/execution/expression/binary/af_matrix_scalar_multiplication.spec';
import { g as shader_execution_expression_binary_ai_arithmetic } from './cts/src/webgpu/shader/execution/expression/binary/ai_arithmetic.spec';
import { g as shader_execution_expression_binary_f32_matrix_matrix_multiplication } from './cts/src/webgpu/shader/execution/expression/binary/f32_matrix_matrix_multiplication.spec';
import { g as shader_execution_expression_binary_ai_comparison } from './cts/src/webgpu/shader/execution/expression/binary/ai_comparison.spec';
import { g as shader_execution_expression_binary_f16_matrix_scalar_multiplication } from './cts/src/webgpu/shader/execution/expression/binary/f16_matrix_scalar_multiplication.spec';
import { g as shader_execution_expression_binary_f16_matrix_vector_multiplication } from './cts/src/webgpu/shader/execution/expression/binary/f16_matrix_vector_multiplication.spec';
import { g as shader_execution_expression_binary_af_matrix_matrix_multiplication } from './cts/src/webgpu/shader/execution/expression/binary/af_matrix_matrix_multiplication.spec';
import { g as shader_execution_expression_binary_f32_matrix_subtraction } from './cts/src/webgpu/shader/execution/expression/binary/f32_matrix_subtraction.spec';
import { g as shader_execution_expression_binary_af_matrix_vector_multiplication } from './cts/src/webgpu/shader/execution/expression/binary/af_matrix_vector_multiplication.spec';
import { g as shader_execution_expression_binary_f32_matrix_vector_multiplication } from './cts/src/webgpu/shader/execution/expression/binary/f32_matrix_vector_multiplication.spec';
import { g as shader_execution_expression_binary_af_addition } from './cts/src/webgpu/shader/execution/expression/binary/af_addition.spec';
import { g as shader_execution_expression_binary_af_matrix_subtraction } from './cts/src/webgpu/shader/execution/expression/binary/af_matrix_subtraction.spec';
import { g as shader_execution_expression_binary_af_matrix_addition } from './cts/src/webgpu/shader/execution/expression/binary/af_matrix_addition.spec';
import { g as shader_execution_expression_binary_bitwise } from './cts/src/webgpu/shader/execution/expression/binary/bitwise.spec';
import { g as shader_execution_expression_binary_f16_comparison } from './cts/src/webgpu/shader/execution/expression/binary/f16_comparison.spec';
import { g as shader_execution_expression_binary_af_remainder } from './cts/src/webgpu/shader/execution/expression/binary/af_remainder.spec';
import { g as shader_execution_expression_binary_i32_arithmetic } from './cts/src/webgpu/shader/execution/expression/binary/i32_arithmetic.spec';
import { g as shader_execution_expression_binary_u32_arithmetic } from './cts/src/webgpu/shader/execution/expression/binary/u32_arithmetic.spec';
import { g as shader_execution_expression_binary_f32_matrix_addition } from './cts/src/webgpu/shader/execution/expression/binary/f32_matrix_addition.spec';
import { g as shader_execution_expression_binary_f32_division } from './cts/src/webgpu/shader/execution/expression/binary/f32_division.spec';
import { g as shader_execution_expression_binary_f32_subtraction } from './cts/src/webgpu/shader/execution/expression/binary/f32_subtraction.spec';
import { g as shader_execution_expression_binary_f16_matrix_matrix_multiplication } from './cts/src/webgpu/shader/execution/expression/binary/f16_matrix_matrix_multiplication.spec';
import { g as shader_execution_expression_binary_af_division } from './cts/src/webgpu/shader/execution/expression/binary/af_division.spec';
import { g as shader_execution_expression_binary_bitwise_shift } from './cts/src/webgpu/shader/execution/expression/binary/bitwise_shift.spec';
import { g as shader_execution_expression_precedence } from './cts/src/webgpu/shader/execution/expression/precedence.spec';
import { g as shader_execution_expression_access_matrix_index } from './cts/src/webgpu/shader/execution/expression/access/matrix/index.spec';
import { g as shader_execution_expression_access_structure_index } from './cts/src/webgpu/shader/execution/expression/access/structure/index.spec';
import { g as shader_execution_expression_access_vector_index } from './cts/src/webgpu/shader/execution/expression/access/vector/index.spec';
import { g as shader_execution_expression_access_vector_components } from './cts/src/webgpu/shader/execution/expression/access/vector/components.spec';
import { g as shader_execution_expression_access_array_index } from './cts/src/webgpu/shader/execution/expression/access/array/index.spec';
import { g as shader_validation_const_assert_const_assert } from './cts/src/webgpu/shader/validation/const_assert/const_assert.spec';
import { g as shader_validation_decl_let } from './cts/src/webgpu/shader/validation/decl/let.spec';
import { g as shader_validation_decl_override } from './cts/src/webgpu/shader/validation/decl/override.spec';
import { g as shader_validation_decl_const } from './cts/src/webgpu/shader/validation/decl/const.spec';
import { g as shader_validation_decl_compound_statement } from './cts/src/webgpu/shader/validation/decl/compound_statement.spec';
import { g as shader_validation_decl_context_dependent_resolution } from './cts/src/webgpu/shader/validation/decl/context_dependent_resolution.spec';
import { g as shader_validation_decl_var } from './cts/src/webgpu/shader/validation/decl/var.spec';
import { g as shader_validation_types_array } from './cts/src/webgpu/shader/validation/types/array.spec';
import { g as shader_validation_types_struct } from './cts/src/webgpu/shader/validation/types/struct.spec';
import { g as shader_validation_types_alias } from './cts/src/webgpu/shader/validation/types/alias.spec';
import { g as shader_validation_types_atomics } from './cts/src/webgpu/shader/validation/types/atomics.spec';
import { g as shader_validation_types_ref } from './cts/src/webgpu/shader/validation/types/ref.spec';
import { g as shader_validation_types_pointer } from './cts/src/webgpu/shader/validation/types/pointer.spec';
import { g as shader_validation_types_vector } from './cts/src/webgpu/shader/validation/types/vector.spec';
import { g as shader_validation_types_textures } from './cts/src/webgpu/shader/validation/types/textures.spec';
import { g as shader_validation_types_enumerant } from './cts/src/webgpu/shader/validation/types/enumerant.spec';
import { g as shader_validation_types_matrix } from './cts/src/webgpu/shader/validation/types/matrix.spec';
import { g as shader_validation_shader_io_builtins } from './cts/src/webgpu/shader/validation/shader_io/builtins.spec';
import { g as shader_validation_shader_io_layout_constraints } from './cts/src/webgpu/shader/validation/shader_io/layout_constraints.spec';
import { g as shader_validation_shader_io_entry_point } from './cts/src/webgpu/shader/validation/shader_io/entry_point.spec';
import { g as shader_validation_shader_io_interpolate } from './cts/src/webgpu/shader/validation/shader_io/interpolate.spec';
import { g as shader_validation_shader_io_group_and_binding } from './cts/src/webgpu/shader/validation/shader_io/group_and_binding.spec';
import { g as shader_validation_shader_io_group } from './cts/src/webgpu/shader/validation/shader_io/group.spec';
import { g as shader_validation_shader_io_id } from './cts/src/webgpu/shader/validation/shader_io/id.spec';
import { g as shader_validation_shader_io_workgroup_size } from './cts/src/webgpu/shader/validation/shader_io/workgroup_size.spec';
import { g as shader_validation_shader_io_binding } from './cts/src/webgpu/shader/validation/shader_io/binding.spec';
import { g as shader_validation_shader_io_size } from './cts/src/webgpu/shader/validation/shader_io/size.spec';
import { g as shader_validation_shader_io_pipeline_stage } from './cts/src/webgpu/shader/validation/shader_io/pipeline_stage.spec';
import { g as shader_validation_shader_io_locations } from './cts/src/webgpu/shader/validation/shader_io/locations.spec';
import { g as shader_validation_shader_io_invariant } from './cts/src/webgpu/shader/validation/shader_io/invariant.spec';
import { g as shader_validation_shader_io_align } from './cts/src/webgpu/shader/validation/shader_io/align.spec';
import { g as shader_validation_parse_attribute } from './cts/src/webgpu/shader/validation/parse/attribute.spec';
import { g as shader_validation_parse_must_use } from './cts/src/webgpu/shader/validation/parse/must_use.spec';
import { g as shader_validation_parse_source } from './cts/src/webgpu/shader/validation/parse/source.spec';
import { g as shader_validation_parse_literal } from './cts/src/webgpu/shader/validation/parse/literal.spec';
import { g as shader_validation_parse_blankspace } from './cts/src/webgpu/shader/validation/parse/blankspace.spec';
import { g as shader_validation_parse_semicolon } from './cts/src/webgpu/shader/validation/parse/semicolon.spec';
import { g as shader_validation_parse_comments } from './cts/src/webgpu/shader/validation/parse/comments.spec';
import { g as shader_validation_parse_requires } from './cts/src/webgpu/shader/validation/parse/requires.spec';
import { g as shader_validation_parse_enable } from './cts/src/webgpu/shader/validation/parse/enable.spec';
import { g as shader_validation_parse_diagnostic } from './cts/src/webgpu/shader/validation/parse/diagnostic.spec';
import { g as shader_validation_parse_shadow_builtins } from './cts/src/webgpu/shader/validation/parse/shadow_builtins.spec';
import { g as shader_validation_parse_identifiers } from './cts/src/webgpu/shader/validation/parse/identifiers.spec';
import { g as shader_validation_functions_alias_analysis } from './cts/src/webgpu/shader/validation/functions/alias_analysis.spec';
import { g as shader_validation_functions_restrictions } from './cts/src/webgpu/shader/validation/functions/restrictions.spec';
import { g as shader_validation_statement_increment_decrement } from './cts/src/webgpu/shader/validation/statement/increment_decrement.spec';
import { g as shader_validation_statement_statement_behavior } from './cts/src/webgpu/shader/validation/statement/statement_behavior.spec';
import { g as shader_validation_statement_continue } from './cts/src/webgpu/shader/validation/statement/continue.spec';
import { g as shader_validation_statement_discard } from './cts/src/webgpu/shader/validation/statement/discard.spec';
import { g as shader_validation_statement_switch } from './cts/src/webgpu/shader/validation/statement/switch.spec';
import { g as shader_validation_statement_const_assert } from './cts/src/webgpu/shader/validation/statement/const_assert.spec';
import { g as shader_validation_statement_loop } from './cts/src/webgpu/shader/validation/statement/loop.spec';
import { g as shader_validation_statement_phony } from './cts/src/webgpu/shader/validation/statement/phony.spec';
import { g as shader_validation_statement_continuing } from './cts/src/webgpu/shader/validation/statement/continuing.spec';
import { g as shader_validation_statement_for } from './cts/src/webgpu/shader/validation/statement/for.spec';
import { g as shader_validation_statement_while } from './cts/src/webgpu/shader/validation/statement/while.spec';
import { g as shader_validation_statement_break_if } from './cts/src/webgpu/shader/validation/statement/break_if.spec';
import { g as shader_validation_statement_break } from './cts/src/webgpu/shader/validation/statement/break.spec';
import { g as shader_validation_statement_if } from './cts/src/webgpu/shader/validation/statement/if.spec';
import { g as shader_validation_statement_compound } from './cts/src/webgpu/shader/validation/statement/compound.spec';
import { g as shader_validation_statement_return } from './cts/src/webgpu/shader/validation/statement/return.spec';
import { g as shader_validation_uniformity_uniformity } from './cts/src/webgpu/shader/validation/uniformity/uniformity.spec';
import { g as shader_validation_expression_matrix_add_sub } from './cts/src/webgpu/shader/validation/expression/matrix/add_sub.spec';
import { g as shader_validation_expression_matrix_comparison } from './cts/src/webgpu/shader/validation/expression/matrix/comparison.spec';
import { g as shader_validation_expression_matrix_mul } from './cts/src/webgpu/shader/validation/expression/matrix/mul.spec';
import { g as shader_validation_expression_matrix_and_or_xor } from './cts/src/webgpu/shader/validation/expression/matrix/and_or_xor.spec';
import { g as shader_validation_expression_matrix_div_rem } from './cts/src/webgpu/shader/validation/expression/matrix/div_rem.spec';
import { g as shader_validation_expression_matrix_bitwise_shift } from './cts/src/webgpu/shader/validation/expression/matrix/bitwise_shift.spec';
import { g as shader_validation_expression_call_builtin_subgroupElect } from './cts/src/webgpu/shader/validation/expression/call/builtin/subgroupElect.spec';
import { g as shader_validation_expression_call_builtin_firstTrailingBit } from './cts/src/webgpu/shader/validation/expression/call/builtin/firstTrailingBit.spec';
import { g as shader_validation_expression_call_builtin_value_constructor } from './cts/src/webgpu/shader/validation/expression/call/builtin/value_constructor.spec';
import { g as shader_validation_expression_call_builtin_textureSampleCompareLevel } from './cts/src/webgpu/shader/validation/expression/call/builtin/textureSampleCompareLevel.spec';
import { g as shader_validation_expression_call_builtin_cross } from './cts/src/webgpu/shader/validation/expression/call/builtin/cross.spec';
import { g as shader_validation_expression_call_builtin_step } from './cts/src/webgpu/shader/validation/expression/call/builtin/step.spec';
import { g as shader_validation_expression_call_builtin_abs } from './cts/src/webgpu/shader/validation/expression/call/builtin/abs.spec';
import { g as shader_validation_expression_call_builtin_textureDimensions } from './cts/src/webgpu/shader/validation/expression/call/builtin/textureDimensions.spec';
import { g as shader_validation_expression_call_builtin_determinant } from './cts/src/webgpu/shader/validation/expression/call/builtin/determinant.spec';
import { g as shader_validation_expression_call_builtin_unpack2x16snorm } from './cts/src/webgpu/shader/validation/expression/call/builtin/unpack2x16snorm.spec';
import { g as shader_validation_expression_call_builtin_subgroupMul } from './cts/src/webgpu/shader/validation/expression/call/builtin/subgroupMul.spec';
import { g as shader_validation_expression_call_builtin_transpose } from './cts/src/webgpu/shader/validation/expression/call/builtin/transpose.spec';
import { g as shader_validation_expression_call_builtin_subgroupAdd } from './cts/src/webgpu/shader/validation/expression/call/builtin/subgroupAdd.spec';
import { g as shader_validation_expression_call_builtin_faceForward } from './cts/src/webgpu/shader/validation/expression/call/builtin/faceForward.spec';
import { g as shader_validation_expression_call_builtin_textureNumLevels } from './cts/src/webgpu/shader/validation/expression/call/builtin/textureNumLevels.spec';
import { g as shader_validation_expression_call_builtin_ldexp } from './cts/src/webgpu/shader/validation/expression/call/builtin/ldexp.spec';
import { g as shader_validation_expression_call_builtin_textureGatherCompare } from './cts/src/webgpu/shader/validation/expression/call/builtin/textureGatherCompare.spec';
import { g as shader_validation_expression_call_builtin_unpack4xU8 } from './cts/src/webgpu/shader/validation/expression/call/builtin/unpack4xU8.spec';
import { g as shader_validation_expression_call_builtin_textureSampleGrad } from './cts/src/webgpu/shader/validation/expression/call/builtin/textureSampleGrad.spec';
import { g as shader_validation_expression_call_builtin_ceil } from './cts/src/webgpu/shader/validation/expression/call/builtin/ceil.spec';
import { g as shader_validation_expression_call_builtin_quantizeToF16 } from './cts/src/webgpu/shader/validation/expression/call/builtin/quantizeToF16.spec';
import { g as shader_validation_expression_call_builtin_pack2x16unorm } from './cts/src/webgpu/shader/validation/expression/call/builtin/pack2x16unorm.spec';
import { g as shader_validation_expression_call_builtin_subgroupMinMax } from './cts/src/webgpu/shader/validation/expression/call/builtin/subgroupMinMax.spec';
import { g as shader_validation_expression_call_builtin_frexp } from './cts/src/webgpu/shader/validation/expression/call/builtin/frexp.spec';
import { g as shader_validation_expression_call_builtin_fract } from './cts/src/webgpu/shader/validation/expression/call/builtin/fract.spec';
import { g as shader_validation_expression_call_builtin_radians } from './cts/src/webgpu/shader/validation/expression/call/builtin/radians.spec';
import { g as shader_validation_expression_call_builtin_degrees } from './cts/src/webgpu/shader/validation/expression/call/builtin/degrees.spec';
import { g as shader_validation_expression_call_builtin_clamp } from './cts/src/webgpu/shader/validation/expression/call/builtin/clamp.spec';
import { g as shader_validation_expression_call_builtin_barriers } from './cts/src/webgpu/shader/validation/expression/call/builtin/barriers.spec';
import { g as shader_validation_expression_call_builtin_countOneBits } from './cts/src/webgpu/shader/validation/expression/call/builtin/countOneBits.spec';
import { g as shader_validation_expression_call_builtin_select } from './cts/src/webgpu/shader/validation/expression/call/builtin/select.spec';
import { g as shader_validation_expression_call_builtin_any } from './cts/src/webgpu/shader/validation/expression/call/builtin/any.spec';
import { g as shader_validation_expression_call_builtin_countLeadingZeros } from './cts/src/webgpu/shader/validation/expression/call/builtin/countLeadingZeros.spec';
import { g as shader_validation_expression_call_builtin_sin } from './cts/src/webgpu/shader/validation/expression/call/builtin/sin.spec';
import { g as shader_validation_expression_call_builtin_sinh } from './cts/src/webgpu/shader/validation/expression/call/builtin/sinh.spec';
import { g as shader_validation_expression_call_builtin_min } from './cts/src/webgpu/shader/validation/expression/call/builtin/min.spec';
import { g as shader_validation_expression_call_builtin_dot } from './cts/src/webgpu/shader/validation/expression/call/builtin/dot.spec';
import { g as shader_validation_expression_call_builtin_reflect } from './cts/src/webgpu/shader/validation/expression/call/builtin/reflect.spec';
import { g as shader_validation_expression_call_builtin_subgroupBroadcastFirst } from './cts/src/webgpu/shader/validation/expression/call/builtin/subgroupBroadcastFirst.spec';
import { g as shader_validation_expression_call_builtin_extractBits } from './cts/src/webgpu/shader/validation/expression/call/builtin/extractBits.spec';
import { g as shader_validation_expression_call_builtin_refract } from './cts/src/webgpu/shader/validation/expression/call/builtin/refract.spec';
import { g as shader_validation_expression_call_builtin_log2 } from './cts/src/webgpu/shader/validation/expression/call/builtin/log2.spec';
import { g as shader_validation_expression_call_builtin_sign } from './cts/src/webgpu/shader/validation/expression/call/builtin/sign.spec';
import { g as shader_validation_expression_call_builtin_atan } from './cts/src/webgpu/shader/validation/expression/call/builtin/atan.spec';
import { g as shader_validation_expression_call_builtin_bitcast } from './cts/src/webgpu/shader/validation/expression/call/builtin/bitcast.spec';
import { g as shader_validation_expression_call_builtin_acosh } from './cts/src/webgpu/shader/validation/expression/call/builtin/acosh.spec';
import { g as shader_validation_expression_call_builtin_pack2x16snorm } from './cts/src/webgpu/shader/validation/expression/call/builtin/pack2x16snorm.spec';
import { g as shader_validation_expression_call_builtin_textureNumSamples } from './cts/src/webgpu/shader/validation/expression/call/builtin/textureNumSamples.spec';
import { g as shader_validation_expression_call_builtin_cos } from './cts/src/webgpu/shader/validation/expression/call/builtin/cos.spec';
import { g as shader_validation_expression_call_builtin_subgroupBallot } from './cts/src/webgpu/shader/validation/expression/call/builtin/subgroupBallot.spec';
import { g as shader_validation_expression_call_builtin_workgroupUniformLoad } from './cts/src/webgpu/shader/validation/expression/call/builtin/workgroupUniformLoad.spec';
import { g as shader_validation_expression_call_builtin_atomics } from './cts/src/webgpu/shader/validation/expression/call/builtin/atomics.spec';
import { g as shader_validation_expression_call_builtin_pack4x8snorm } from './cts/src/webgpu/shader/validation/expression/call/builtin/pack4x8snorm.spec';
import { g as shader_validation_expression_call_builtin_pack2x16float } from './cts/src/webgpu/shader/validation/expression/call/builtin/pack2x16float.spec';
import { g as shader_validation_expression_call_builtin_textureGather } from './cts/src/webgpu/shader/validation/expression/call/builtin/textureGather.spec';
import { g as shader_validation_expression_call_builtin_all } from './cts/src/webgpu/shader/validation/expression/call/builtin/all.spec';
import { g as shader_validation_expression_call_builtin_atan2 } from './cts/src/webgpu/shader/validation/expression/call/builtin/atan2.spec';
import { g as shader_validation_expression_call_builtin_pack4xI8 } from './cts/src/webgpu/shader/validation/expression/call/builtin/pack4xI8.spec';
import { g as shader_validation_expression_call_builtin_subgroupShuffle } from './cts/src/webgpu/shader/validation/expression/call/builtin/subgroupShuffle.spec';
import { g as shader_validation_expression_call_builtin_firstLeadingBit } from './cts/src/webgpu/shader/validation/expression/call/builtin/firstLeadingBit.spec';
import { g as shader_validation_expression_call_builtin_exp2 } from './cts/src/webgpu/shader/validation/expression/call/builtin/exp2.spec';
import { g as shader_validation_expression_call_builtin_dot4I8Packed } from './cts/src/webgpu/shader/validation/expression/call/builtin/dot4I8Packed.spec';
import { g as shader_validation_expression_call_builtin_round } from './cts/src/webgpu/shader/validation/expression/call/builtin/round.spec';
import { g as shader_validation_expression_call_builtin_pack4xU8Clamp } from './cts/src/webgpu/shader/validation/expression/call/builtin/pack4xU8Clamp.spec';
import { g as shader_validation_expression_call_builtin_floor } from './cts/src/webgpu/shader/validation/expression/call/builtin/floor.spec';
import { g as shader_validation_expression_call_builtin_unpack4x8snorm } from './cts/src/webgpu/shader/validation/expression/call/builtin/unpack4x8snorm.spec';
import { g as shader_validation_expression_call_builtin_unpack4x8unorm } from './cts/src/webgpu/shader/validation/expression/call/builtin/unpack4x8unorm.spec';
import { g as shader_validation_expression_call_builtin_textureSampleBias } from './cts/src/webgpu/shader/validation/expression/call/builtin/textureSampleBias.spec';
import { g as shader_validation_expression_call_builtin_unpack4xI8 } from './cts/src/webgpu/shader/validation/expression/call/builtin/unpack4xI8.spec';
import { g as shader_validation_expression_call_builtin_derivatives } from './cts/src/webgpu/shader/validation/expression/call/builtin/derivatives.spec';
import { g as shader_validation_expression_call_builtin_dot4U8Packed } from './cts/src/webgpu/shader/validation/expression/call/builtin/dot4U8Packed.spec';
import { g as shader_validation_expression_call_builtin_modf } from './cts/src/webgpu/shader/validation/expression/call/builtin/modf.spec';
import { g as shader_validation_expression_call_builtin_textureSampleBaseClampToEdge } from './cts/src/webgpu/shader/validation/expression/call/builtin/textureSampleBaseClampToEdge.spec';
import { g as shader_validation_expression_call_builtin_normalize } from './cts/src/webgpu/shader/validation/expression/call/builtin/normalize.spec';
import { g as shader_validation_expression_call_builtin_subgroupAnyAll } from './cts/src/webgpu/shader/validation/expression/call/builtin/subgroupAnyAll.spec';
import { g as shader_validation_expression_call_builtin_length } from './cts/src/webgpu/shader/validation/expression/call/builtin/length.spec';
import { g as shader_validation_expression_call_builtin_countTrailingZeros } from './cts/src/webgpu/shader/validation/expression/call/builtin/countTrailingZeros.spec';
import { g as shader_validation_expression_call_builtin_arrayLength } from './cts/src/webgpu/shader/validation/expression/call/builtin/arrayLength.spec';
import { g as shader_validation_expression_call_builtin_pow } from './cts/src/webgpu/shader/validation/expression/call/builtin/pow.spec';
import { g as shader_validation_expression_call_builtin_subgroupBroadcast } from './cts/src/webgpu/shader/validation/expression/call/builtin/subgroupBroadcast.spec';
import { g as shader_validation_expression_call_builtin_textureLoad } from './cts/src/webgpu/shader/validation/expression/call/builtin/textureLoad.spec';
import { g as shader_validation_expression_call_builtin_atanh } from './cts/src/webgpu/shader/validation/expression/call/builtin/atanh.spec';
import { g as shader_validation_expression_call_builtin_tanh } from './cts/src/webgpu/shader/validation/expression/call/builtin/tanh.spec';
import { g as shader_validation_expression_call_builtin_log } from './cts/src/webgpu/shader/validation/expression/call/builtin/log.spec';
import { g as shader_validation_expression_call_builtin_subgroupBitwise } from './cts/src/webgpu/shader/validation/expression/call/builtin/subgroupBitwise.spec';
import { g as shader_validation_expression_call_builtin_quadBroadcast } from './cts/src/webgpu/shader/validation/expression/call/builtin/quadBroadcast.spec';
import { g as shader_validation_expression_call_builtin_pack4xI8Clamp } from './cts/src/webgpu/shader/validation/expression/call/builtin/pack4xI8Clamp.spec';
import { g as shader_validation_expression_call_builtin_exp } from './cts/src/webgpu/shader/validation/expression/call/builtin/exp.spec';
import { g as shader_validation_expression_call_builtin_pack4xU8 } from './cts/src/webgpu/shader/validation/expression/call/builtin/pack4xU8.spec';
import { g as shader_validation_expression_call_builtin_asin } from './cts/src/webgpu/shader/validation/expression/call/builtin/asin.spec';
import { g as shader_validation_expression_call_builtin_acos } from './cts/src/webgpu/shader/validation/expression/call/builtin/acos.spec';
import { g as shader_validation_expression_call_builtin_cosh } from './cts/src/webgpu/shader/validation/expression/call/builtin/cosh.spec';
import { g as shader_validation_expression_call_builtin_unpack2x16float } from './cts/src/webgpu/shader/validation/expression/call/builtin/unpack2x16float.spec';
import { g as shader_validation_expression_call_builtin_insertBits } from './cts/src/webgpu/shader/validation/expression/call/builtin/insertBits.spec';
import { g as shader_validation_expression_call_builtin_textureNumLayers } from './cts/src/webgpu/shader/validation/expression/call/builtin/textureNumLayers.spec';
import { g as shader_validation_expression_call_builtin_quadSwap } from './cts/src/webgpu/shader/validation/expression/call/builtin/quadSwap.spec';
import { g as shader_validation_expression_call_builtin_unpack2x16unorm } from './cts/src/webgpu/shader/validation/expression/call/builtin/unpack2x16unorm.spec';
import { g as shader_validation_expression_call_builtin_saturate } from './cts/src/webgpu/shader/validation/expression/call/builtin/saturate.spec';
import { g as shader_validation_expression_call_builtin_textureSampleCompare } from './cts/src/webgpu/shader/validation/expression/call/builtin/textureSampleCompare.spec';
import { g as shader_validation_expression_call_builtin_smoothstep } from './cts/src/webgpu/shader/validation/expression/call/builtin/smoothstep.spec';
import { g as shader_validation_expression_call_builtin_textureSample } from './cts/src/webgpu/shader/validation/expression/call/builtin/textureSample.spec';
import { g as shader_validation_expression_call_builtin_trunc } from './cts/src/webgpu/shader/validation/expression/call/builtin/trunc.spec';
import { g as shader_validation_expression_call_builtin_fma } from './cts/src/webgpu/shader/validation/expression/call/builtin/fma.spec';
import { g as shader_validation_expression_call_builtin_mix } from './cts/src/webgpu/shader/validation/expression/call/builtin/mix.spec';
import { g as shader_validation_expression_call_builtin_sqrt } from './cts/src/webgpu/shader/validation/expression/call/builtin/sqrt.spec';
import { g as shader_validation_expression_call_builtin_asinh } from './cts/src/webgpu/shader/validation/expression/call/builtin/asinh.spec';
import { g as shader_validation_expression_call_builtin_textureSampleLevel } from './cts/src/webgpu/shader/validation/expression/call/builtin/textureSampleLevel.spec';
import { g as shader_validation_expression_call_builtin_distance } from './cts/src/webgpu/shader/validation/expression/call/builtin/distance.spec';
import { g as shader_validation_expression_call_builtin_max } from './cts/src/webgpu/shader/validation/expression/call/builtin/max.spec';
import { g as shader_validation_expression_call_builtin_reverseBits } from './cts/src/webgpu/shader/validation/expression/call/builtin/reverseBits.spec';
import { g as shader_validation_expression_call_builtin_tan } from './cts/src/webgpu/shader/validation/expression/call/builtin/tan.spec';
import { g as shader_validation_expression_call_builtin_pack4x8unorm } from './cts/src/webgpu/shader/validation/expression/call/builtin/pack4x8unorm.spec';
import { g as shader_validation_expression_call_builtin_inverseSqrt } from './cts/src/webgpu/shader/validation/expression/call/builtin/inverseSqrt.spec';
import { g as shader_validation_expression_call_builtin_textureStore } from './cts/src/webgpu/shader/validation/expression/call/builtin/textureStore.spec';
import { g as shader_validation_expression_unary_address_of_and_indirection } from './cts/src/webgpu/shader/validation/expression/unary/address_of_and_indirection.spec';
import { g as shader_validation_expression_unary_bitwise_complement } from './cts/src/webgpu/shader/validation/expression/unary/bitwise_complement.spec';
import { g as shader_validation_expression_unary_logical_negation } from './cts/src/webgpu/shader/validation/expression/unary/logical_negation.spec';
import { g as shader_validation_expression_unary_arithmetic_negation } from './cts/src/webgpu/shader/validation/expression/unary/arithmetic_negation.spec';
import { g as shader_validation_expression_overload_resolution } from './cts/src/webgpu/shader/validation/expression/overload_resolution.spec';
import { g as shader_validation_expression_binary_comparison } from './cts/src/webgpu/shader/validation/expression/binary/comparison.spec';
import { g as shader_validation_expression_binary_short_circuiting_and_or } from './cts/src/webgpu/shader/validation/expression/binary/short_circuiting_and_or.spec';
import { g as shader_validation_expression_binary_add_sub_mul } from './cts/src/webgpu/shader/validation/expression/binary/add_sub_mul.spec';
import { g as shader_validation_expression_binary_and_or_xor } from './cts/src/webgpu/shader/validation/expression/binary/and_or_xor.spec';
import { g as shader_validation_expression_binary_parse } from './cts/src/webgpu/shader/validation/expression/binary/parse.spec';
import { g as shader_validation_expression_binary_div_rem } from './cts/src/webgpu/shader/validation/expression/binary/div_rem.spec';
import { g as shader_validation_expression_binary_bitwise_shift } from './cts/src/webgpu/shader/validation/expression/binary/bitwise_shift.spec';
import { g as shader_validation_expression_early_evaluation } from './cts/src/webgpu/shader/validation/expression/early_evaluation.spec';
import { g as shader_validation_expression_precedence } from './cts/src/webgpu/shader/validation/expression/precedence.spec';
import { g as shader_validation_expression_access_array } from './cts/src/webgpu/shader/validation/expression/access/array.spec';
import { g as shader_validation_expression_access_vector } from './cts/src/webgpu/shader/validation/expression/access/vector.spec';
import { g as shader_validation_expression_access_matrix } from './cts/src/webgpu/shader/validation/expression/access/matrix.spec';
import { g as shader_validation_expression_access_structure } from './cts/src/webgpu/shader/validation/expression/access/structure.spec';
import { g as shader_validation_extension_dual_source_blending } from './cts/src/webgpu/shader/validation/extension/dual_source_blending.spec';
import { g as shader_validation_extension_clip_distances } from './cts/src/webgpu/shader/validation/extension/clip_distances.spec';
import { g as shader_validation_extension_readonly_and_readwrite_storage_textures } from './cts/src/webgpu/shader/validation/extension/readonly_and_readwrite_storage_textures.spec';
import { g as shader_validation_extension_pointer_composite_access } from './cts/src/webgpu/shader/validation/extension/pointer_composite_access.spec';
import { g as idl_constructable } from './cts/src/webgpu/idl/constructable.spec';
import { g as idl_constants_flags } from './cts/src/webgpu/idl/constants/flags.spec';


export const specs = {
    examples,
    compat_api_validation_texture_cubeArray,
    compat_api_validation_texture_createTexture,
    compat_api_validation_render_pipeline_depth_stencil_state,
    compat_api_validation_render_pipeline_unsupported_wgsl,
    compat_api_validation_render_pipeline_fragment_state,
    compat_api_validation_render_pipeline_vertex_state,
    compat_api_validation_createBindGroup,
    compat_api_validation_encoding_programmable_pipeline_bind_group_compat,
    compat_api_validation_encoding_cmds_copyTextureToBuffer,
    compat_api_validation_encoding_cmds_copyTextureToTexture,
    compat_api_validation_createBindGroupLayout,
    compat_api_validation_pipeline_creation,
    web_platform_external_texture_video,
    web_platform_canvas_context_creation,
    web_platform_canvas_configure,
    web_platform_canvas_getPreferredCanvasFormat,
    web_platform_canvas_getCurrentTexture,
    web_platform_canvas_readbackFromWebGPUCanvas,
    web_platform_copyToTexture_ImageBitmap,
    web_platform_copyToTexture_video,
    web_platform_copyToTexture_image,
    web_platform_copyToTexture_ImageData,
    web_platform_copyToTexture_canvas,
    print_environment,
    util_texture_texel_data,
    util_texture_texture_ok,
    util_texture_color_space_conversions,
    api_validation_render_pass_attachment_compatibility,
    api_validation_render_pass_resolve,
    api_validation_render_pass_render_pass_descriptor,
    api_validation_texture_bgra8unorm_storage,
    api_validation_texture_float32_filterable,
    api_validation_texture_destroy,
    api_validation_texture_rg11b10ufloat_renderable,
    api_validation_createView,
    api_validation_queue_destroyed_texture,
    api_validation_queue_destroyed_buffer,
    api_validation_queue_destroyed_query_set,
    api_validation_queue_writeTexture,
    api_validation_queue_submit,
    api_validation_queue_buffer_mapped,
    api_validation_queue_writeBuffer,
    api_validation_queue_copyToTexture_CopyExternalImageToTexture,
    api_validation_compute_pipeline,
    api_validation_resource_usages_texture_in_render_common,
    api_validation_resource_usages_texture_in_render_misc,
    api_validation_resource_usages_texture_in_pass_encoder,
    api_validation_resource_usages_buffer_in_pass_encoder,
    api_validation_resource_usages_buffer_in_pass_misc,
    api_validation_capability_checks_limits_maxTextureDimension3D,
    api_validation_capability_checks_limits_maxSamplersPerShaderStage,
    api_validation_capability_checks_limits_maxBindingsPerBindGroup,
    api_validation_capability_checks_limits_maxComputeWorkgroupStorageSize,
    api_validation_capability_checks_limits_maxVertexBufferArrayStride,
    api_validation_capability_checks_limits_minStorageBufferOffsetAlignment,
    api_validation_capability_checks_limits_maxInterStageShaderVariables,
    api_validation_capability_checks_limits_maxDynamicUniformBuffersPerPipelineLayout,
    api_validation_capability_checks_limits_maxComputeInvocationsPerWorkgroup,
    api_validation_capability_checks_limits_maxUniformBufferBindingSize,
    api_validation_capability_checks_limits_maxVertexBuffers,
    api_validation_capability_checks_limits_maxBindGroupsPlusVertexBuffers,
    api_validation_capability_checks_limits_maxDynamicStorageBuffersPerPipelineLayout,
    api_validation_capability_checks_limits_maxStorageTexturesPerShaderStage,
    api_validation_capability_checks_limits_minUniformBufferOffsetAlignment,
    api_validation_capability_checks_limits_maxStorageBufferBindingSize,
    api_validation_capability_checks_limits_maxStorageBuffersPerShaderStage,
    api_validation_capability_checks_limits_maxComputeWorkgroupSizeX,
    api_validation_capability_checks_limits_maxUniformBuffersPerShaderStage,
    api_validation_capability_checks_limits_maxVertexAttributes,
    api_validation_capability_checks_limits_maxComputeWorkgroupSizeZ,
    api_validation_capability_checks_limits_maxComputeWorkgroupSizeY,
    api_validation_capability_checks_limits_maxColorAttachments,
    api_validation_capability_checks_limits_maxTextureArrayLayers,
    api_validation_capability_checks_limits_maxTextureDimension2D,
    api_validation_capability_checks_limits_maxSampledTexturesPerShaderStage,
    api_validation_capability_checks_limits_maxBufferSize,
    api_validation_capability_checks_limits_maxComputeWorkgroupsPerDimension,
    api_validation_capability_checks_limits_maxTextureDimension1D,
    api_validation_capability_checks_limits_maxBindGroups,
    api_validation_capability_checks_limits_maxColorAttachmentBytesPerSample,
    api_validation_capability_checks_features_query_types,
    api_validation_capability_checks_features_clip_distances,
    api_validation_capability_checks_features_texture_formats,
    api_validation_createPipelineLayout,
    api_validation_state_device_lost_destroy,
    api_validation_render_pipeline_multisample_state,
    api_validation_render_pipeline_inter_stage,
    api_validation_render_pipeline_resource_compatibility,
    api_validation_render_pipeline_depth_stencil_state,
    api_validation_render_pipeline_shader_module,
    api_validation_render_pipeline_misc,
    api_validation_render_pipeline_fragment_state,
    api_validation_render_pipeline_overrides,
    api_validation_render_pipeline_vertex_state,
    api_validation_render_pipeline_primitive_state,
    api_validation_render_pipeline_float32_blendable,
    api_validation_createBindGroup,
    api_validation_getBindGroupLayout,
    api_validation_encoding_beginRenderPass,
    api_validation_encoding_createRenderBundleEncoder,
    api_validation_encoding_queries_begin_end,
    api_validation_encoding_queries_resolveQuerySet,
    api_validation_encoding_queries_general,
    api_validation_encoding_encoder_open_state,
    api_validation_encoding_beginComputePass,
    api_validation_encoding_programmable_pipeline_bind_group_compat,
    api_validation_encoding_cmds_render_indirect_draw,
    api_validation_encoding_cmds_render_indirect_multi_draw,
    api_validation_encoding_cmds_render_draw,
    api_validation_encoding_cmds_render_state_tracking,
    api_validation_encoding_cmds_render_setVertexBuffer,
    api_validation_encoding_cmds_render_setPipeline,
    api_validation_encoding_cmds_render_setIndexBuffer,
    api_validation_encoding_cmds_render_dynamic_state,
    api_validation_encoding_cmds_index_access,
    api_validation_encoding_cmds_debug,
    api_validation_encoding_cmds_setBindGroup,
    api_validation_encoding_cmds_clearBuffer,
    api_validation_encoding_cmds_compute_pass,
    api_validation_encoding_cmds_render_pass,
    api_validation_encoding_cmds_copyBufferToBuffer,
    api_validation_encoding_cmds_copyTextureToTexture,
    api_validation_encoding_encoder_state,
    api_validation_encoding_render_bundle,
    api_validation_debugMarker,
    api_validation_buffer_mapping,
    api_validation_buffer_create,
    api_validation_buffer_threading,
    api_validation_buffer_destroy,
    api_validation_createBindGroupLayout,
    api_validation_image_copy_texture_related,
    api_validation_image_copy_buffer_related,
    api_validation_image_copy_layout_related,
    api_validation_image_copy_buffer_texture_copies,
    api_validation_error_scope,
    api_validation_createSampler,
    api_validation_createTexture,
    api_validation_shader_module_entry_point,
    api_validation_shader_module_overrides,
    api_validation_query_set_create,
    api_validation_query_set_destroy,
    api_validation_non_filterable_texture,
    api_validation_layout_shader_compat,
    api_validation_gpu_external_texture_expiration,
    api_operation_render_pass_storeop2,
    api_operation_render_pass_resolve,
    api_operation_render_pass_clear_value,
    api_operation_render_pass_storeOp,
    api_operation_memory_sync_texture_readonly_depth_stencil,
    api_operation_memory_sync_texture_same_subresource,
    api_operation_memory_sync_buffer_multiple_buffers,
    api_operation_memory_sync_buffer_single_buffer,
    api_operation_resource_init_texture_zero,
    api_operation_resource_init_buffer,
    api_operation_device_all_limits_and_features,
    api_operation_device_lost,
    api_operation_queue_writeBuffer,
    api_operation_buffers_map_oom,
    api_operation_buffers_threading,
    api_operation_buffers_map,
    api_operation_buffers_map_detach,
    api_operation_buffers_map_ArrayBuffer,
    api_operation_pipeline_default_layout,
    api_operation_labels,
    api_operation_command_buffer_render_state_tracking,
    api_operation_command_buffer_render_dynamic_state,
    api_operation_command_buffer_basic,
    api_operation_command_buffer_queries_occlusionQuery,
    api_operation_command_buffer_programmable_state_tracking,
    api_operation_command_buffer_clearBuffer,
    api_operation_command_buffer_image_copy,
    api_operation_command_buffer_copyBufferToBuffer,
    api_operation_command_buffer_copyTextureToTexture,
    api_operation_adapter_info,
    api_operation_adapter_requestAdapter,
    api_operation_adapter_requestDevice,
    api_operation_render_pipeline_culling_tests,
    api_operation_render_pipeline_sample_mask,
    api_operation_render_pipeline_vertex_only_render_pipeline,
    api_operation_render_pipeline_pipeline_output_targets,
    api_operation_render_pipeline_overrides,
    api_operation_render_pipeline_primitive_topology,
    api_operation_sampling_anisotropy,
    api_operation_sampling_lod_clamp,
    api_operation_sampling_sampler_texture,
    api_operation_sampling_filter_mode,
    api_operation_reflection,
    api_operation_texture_view_read,
    api_operation_texture_view_write,
    api_operation_texture_view_format_reinterpretation,
    api_operation_uncapturederror,
    api_operation_compute_pipeline_entry_point_name,
    api_operation_compute_pipeline_overrides,
    api_operation_vertex_state_index_format,
    api_operation_vertex_state_correctness,
    api_operation_shader_module_compilation_info,
    api_operation_storage_texture_read_only,
    api_operation_storage_texture_read_write,
    api_operation_rendering_color_target_state,
    api_operation_rendering_basic,
    api_operation_rendering_indirect_draw,
    api_operation_rendering_draw,
    api_operation_rendering_robust_access_index,
    api_operation_rendering_depth_bias,
    api_operation_rendering_stencil,
    api_operation_rendering_3d_texture_slices,
    api_operation_rendering_depth,
    api_operation_rendering_depth_clip_clamp,
    api_operation_onSubmittedWorkDone,
    api_operation_compute_basic,
    shader_execution_memory_layout,
    shader_execution_float_parse,
    shader_execution_shader_io_fragment_builtins,
    shader_execution_shader_io_compute_builtins,
    shader_execution_shader_io_user_io,
    shader_execution_shader_io_shared_structs,
    shader_execution_shader_io_workgroup_size,
    shader_execution_shader_io_vertex_builtins,
    shader_execution_stage,
    shader_execution_memory_model_texture_intra_invocation_coherence,
    shader_execution_memory_model_weak,
    shader_execution_memory_model_coherence,
    shader_execution_memory_model_atomicity,
    shader_execution_memory_model_barrier,
    shader_execution_memory_model_adjacent,
    shader_execution_flow_control_complex,
    shader_execution_flow_control_switch,
    shader_execution_flow_control_loop,
    shader_execution_flow_control_phony,
    shader_execution_flow_control_eval_order,
    shader_execution_flow_control_for,
    shader_execution_flow_control_while,
    shader_execution_flow_control_if,
    shader_execution_flow_control_call,
    shader_execution_flow_control_return,
    shader_execution_robust_access_vertex,
    shader_execution_limits,
    shader_execution_robust_access,
    shader_execution_padding,
    shader_execution_statement_increment_decrement,
    shader_execution_statement_discard,
    shader_execution_statement_phony,
    shader_execution_statement_compound,
    shader_execution_shadow,
    shader_execution_zero_init,
    shader_execution_value_init,
    shader_execution_expression_call_user_ptr_params,
    shader_execution_expression_call_builtin_subgroupElect,
    shader_execution_expression_call_builtin_firstTrailingBit,
    shader_execution_expression_call_builtin_textureSampleCompareLevel,
    shader_execution_expression_call_builtin_cross,
    shader_execution_expression_call_builtin_step,
    shader_execution_expression_call_builtin_abs,
    shader_execution_expression_call_builtin_textureDimensions,
    shader_execution_expression_call_builtin_determinant,
    shader_execution_expression_call_builtin_unpack2x16snorm,
    shader_execution_expression_call_builtin_subgroupMul,
    shader_execution_expression_call_builtin_transpose,
    shader_execution_expression_call_builtin_subgroupAdd,
    shader_execution_expression_call_builtin_faceForward,
    shader_execution_expression_call_builtin_textureNumLevels,
    shader_execution_expression_call_builtin_ldexp,
    shader_execution_expression_call_builtin_textureGatherCompare,
    shader_execution_expression_call_builtin_unpack4xU8,
    shader_execution_expression_call_builtin_textureSampleGrad,
    shader_execution_expression_call_builtin_workgroupBarrier,
    shader_execution_expression_call_builtin_ceil,
    shader_execution_expression_call_builtin_quantizeToF16,
    shader_execution_expression_call_builtin_pack2x16unorm,
    shader_execution_expression_call_builtin_subgroupMinMax,
    shader_execution_expression_call_builtin_frexp,
    shader_execution_expression_call_builtin_fract,
    shader_execution_expression_call_builtin_radians,
    shader_execution_expression_call_builtin_degrees,
    shader_execution_expression_call_builtin_inversesqrt,
    shader_execution_expression_call_builtin_clamp,
    shader_execution_expression_call_builtin_subgroupAll,
    shader_execution_expression_call_builtin_countOneBits,
    shader_execution_expression_call_builtin_select,
    shader_execution_expression_call_builtin_any,
    shader_execution_expression_call_builtin_countLeadingZeros,
    shader_execution_expression_call_builtin_sin,
    shader_execution_expression_call_builtin_fwidthFine,
    shader_execution_expression_call_builtin_sinh,
    shader_execution_expression_call_builtin_min,
    shader_execution_expression_call_builtin_dot,
    shader_execution_expression_call_builtin_reflect,
    shader_execution_expression_call_builtin_extractBits,
    shader_execution_expression_call_builtin_refract,
    shader_execution_expression_call_builtin_log2,
    shader_execution_expression_call_builtin_sign,
    shader_execution_expression_call_builtin_atan,
    shader_execution_expression_call_builtin_bitcast,
    shader_execution_expression_call_builtin_acosh,
    shader_execution_expression_call_builtin_pack2x16snorm,
    shader_execution_expression_call_builtin_textureNumSamples,
    shader_execution_expression_call_builtin_cos,
    shader_execution_expression_call_builtin_subgroupBallot,
    shader_execution_expression_call_builtin_workgroupUniformLoad,
    shader_execution_expression_call_builtin_dpdx,
    shader_execution_expression_call_builtin_pack4x8snorm,
    shader_execution_expression_call_builtin_pack2x16float,
    shader_execution_expression_call_builtin_fwidthCoarse,
    shader_execution_expression_call_builtin_textureGather,
    shader_execution_expression_call_builtin_all,
    shader_execution_expression_call_builtin_atan2,
    shader_execution_expression_call_builtin_pack4xI8,
    shader_execution_expression_call_builtin_subgroupShuffle,
    shader_execution_expression_call_builtin_firstLeadingBit,
    shader_execution_expression_call_builtin_exp2,
    shader_execution_expression_call_builtin_dpdyFine,
    shader_execution_expression_call_builtin_dot4I8Packed,
    shader_execution_expression_call_builtin_round,
    shader_execution_expression_call_builtin_pack4xU8Clamp,
    shader_execution_expression_call_builtin_floor,
    shader_execution_expression_call_builtin_unpack4x8snorm,
    shader_execution_expression_call_builtin_unpack4x8unorm,
    shader_execution_expression_call_builtin_textureSampleBias,
    shader_execution_expression_call_builtin_dpdyCoarse,
    shader_execution_expression_call_builtin_unpack4xI8,
    shader_execution_expression_call_builtin_dot4U8Packed,
    shader_execution_expression_call_builtin_modf,
    shader_execution_expression_call_builtin_textureSampleBaseClampToEdge,
    shader_execution_expression_call_builtin_normalize,
    shader_execution_expression_call_builtin_dpdxCoarse,
    shader_execution_expression_call_builtin_length,
    shader_execution_expression_call_builtin_countTrailingZeros,
    shader_execution_expression_call_builtin_arrayLength,
    shader_execution_expression_call_builtin_pow,
    shader_execution_expression_call_builtin_subgroupBroadcast,
    shader_execution_expression_call_builtin_textureLoad,
    shader_execution_expression_call_builtin_dpdy,
    shader_execution_expression_call_builtin_atanh,
    shader_execution_expression_call_builtin_tanh,
    shader_execution_expression_call_builtin_texture_utils,
    shader_execution_expression_call_builtin_log,
    shader_execution_expression_call_builtin_subgroupBitwise,
    shader_execution_expression_call_builtin_dpdxFine,
    shader_execution_expression_call_builtin_quadBroadcast,
    shader_execution_expression_call_builtin_pack4xI8Clamp,
    shader_execution_expression_call_builtin_exp,
    shader_execution_expression_call_builtin_subgroupAny,
    shader_execution_expression_call_builtin_pack4xU8,
    shader_execution_expression_call_builtin_asin,
    shader_execution_expression_call_builtin_acos,
    shader_execution_expression_call_builtin_cosh,
    shader_execution_expression_call_builtin_unpack2x16float,
    shader_execution_expression_call_builtin_insertBits,
    shader_execution_expression_call_builtin_textureNumLayers,
    shader_execution_expression_call_builtin_quadSwap,
    shader_execution_expression_call_builtin_unpack2x16unorm,
    shader_execution_expression_call_builtin_saturate,
    shader_execution_expression_call_builtin_textureSampleCompare,
    shader_execution_expression_call_builtin_smoothstep,
    shader_execution_expression_call_builtin_fwidth,
    shader_execution_expression_call_builtin_atomics_atomicMax,
    shader_execution_expression_call_builtin_atomics_atomicOr,
    shader_execution_expression_call_builtin_atomics_atomicExchange,
    shader_execution_expression_call_builtin_atomics_atomicStore,
    shader_execution_expression_call_builtin_atomics_atomicXor,
    shader_execution_expression_call_builtin_atomics_atomicLoad,
    shader_execution_expression_call_builtin_atomics_atomicAdd,
    shader_execution_expression_call_builtin_atomics_atomicMin,
    shader_execution_expression_call_builtin_atomics_atomicSub,
    shader_execution_expression_call_builtin_atomics_atomicAnd,
    shader_execution_expression_call_builtin_atomics_atomicCompareExchangeWeak,
    shader_execution_expression_call_builtin_textureSample,
    shader_execution_expression_call_builtin_trunc,
    shader_execution_expression_call_builtin_fma,
    shader_execution_expression_call_builtin_mix,
    shader_execution_expression_call_builtin_sqrt,
    shader_execution_expression_call_builtin_asinh,
    shader_execution_expression_call_builtin_textureSampleLevel,
    shader_execution_expression_call_builtin_distance,
    shader_execution_expression_call_builtin_max,
    shader_execution_expression_call_builtin_storageBarrier,
    shader_execution_expression_call_builtin_reverseBits,
    shader_execution_expression_call_builtin_tan,
    shader_execution_expression_call_builtin_pack4x8unorm,
    shader_execution_expression_call_builtin_textureStore,
    shader_execution_expression_constructor_non_zero,
    shader_execution_expression_constructor_zero_value,
    shader_execution_expression_unary_address_of_and_indirection,
    shader_execution_expression_unary_bool_logical,
    shader_execution_expression_unary_f32_conversion,
    shader_execution_expression_unary_ai_complement,
    shader_execution_expression_unary_af_arithmetic,
    shader_execution_expression_unary_i32_complement,
    shader_execution_expression_unary_af_assignment,
    shader_execution_expression_unary_u32_complement,
    shader_execution_expression_unary_ai_arithmetic,
    shader_execution_expression_unary_ai_assignment,
    shader_execution_expression_unary_i32_conversion,
    shader_execution_expression_unary_f16_arithmetic,
    shader_execution_expression_unary_f32_arithmetic,
    shader_execution_expression_unary_bool_conversion,
    shader_execution_expression_unary_u32_conversion,
    shader_execution_expression_unary_f16_conversion,
    shader_execution_expression_unary_i32_arithmetic,
    shader_execution_expression_binary_f32_remainder,
    shader_execution_expression_binary_f16_matrix_addition,
    shader_execution_expression_binary_bool_logical,
    shader_execution_expression_binary_f16_matrix_subtraction,
    shader_execution_expression_binary_u32_comparison,
    shader_execution_expression_binary_f32_matrix_scalar_multiplication,
    shader_execution_expression_binary_f16_remainder,
    shader_execution_expression_binary_af_subtraction,
    shader_execution_expression_binary_f32_multiplication,
    shader_execution_expression_binary_i32_comparison,
    shader_execution_expression_binary_f32_addition,
    shader_execution_expression_binary_f16_subtraction,
    shader_execution_expression_binary_f16_multiplication,
    shader_execution_expression_binary_f16_addition,
    shader_execution_expression_binary_af_multiplication,
    shader_execution_expression_binary_f16_division,
    shader_execution_expression_binary_af_comparison,
    shader_execution_expression_binary_f32_comparison,
    shader_execution_expression_binary_af_matrix_scalar_multiplication,
    shader_execution_expression_binary_ai_arithmetic,
    shader_execution_expression_binary_f32_matrix_matrix_multiplication,
    shader_execution_expression_binary_ai_comparison,
    shader_execution_expression_binary_f16_matrix_scalar_multiplication,
    shader_execution_expression_binary_f16_matrix_vector_multiplication,
    shader_execution_expression_binary_af_matrix_matrix_multiplication,
    shader_execution_expression_binary_f32_matrix_subtraction,
    shader_execution_expression_binary_af_matrix_vector_multiplication,
    shader_execution_expression_binary_f32_matrix_vector_multiplication,
    shader_execution_expression_binary_af_addition,
    shader_execution_expression_binary_af_matrix_subtraction,
    shader_execution_expression_binary_af_matrix_addition,
    shader_execution_expression_binary_bitwise,
    shader_execution_expression_binary_f16_comparison,
    shader_execution_expression_binary_af_remainder,
    shader_execution_expression_binary_i32_arithmetic,
    shader_execution_expression_binary_u32_arithmetic,
    shader_execution_expression_binary_f32_matrix_addition,
    shader_execution_expression_binary_f32_division,
    shader_execution_expression_binary_f32_subtraction,
    shader_execution_expression_binary_f16_matrix_matrix_multiplication,
    shader_execution_expression_binary_af_division,
    shader_execution_expression_binary_bitwise_shift,
    shader_execution_expression_precedence,
    shader_execution_expression_access_matrix_index,
    shader_execution_expression_access_structure_index,
    shader_execution_expression_access_vector_index,
    shader_execution_expression_access_vector_components,
    shader_execution_expression_access_array_index,
    shader_validation_const_assert_const_assert,
    shader_validation_decl_let,
    shader_validation_decl_override,
    shader_validation_decl_const,
    shader_validation_decl_compound_statement,
    shader_validation_decl_context_dependent_resolution,
    shader_validation_decl_var,
    shader_validation_types_array,
    shader_validation_types_struct,
    shader_validation_types_alias,
    shader_validation_types_atomics,
    shader_validation_types_ref,
    shader_validation_types_pointer,
    shader_validation_types_vector,
    shader_validation_types_textures,
    shader_validation_types_enumerant,
    shader_validation_types_matrix,
    shader_validation_shader_io_builtins,
    shader_validation_shader_io_layout_constraints,
    shader_validation_shader_io_entry_point,
    shader_validation_shader_io_interpolate,
    shader_validation_shader_io_group_and_binding,
    shader_validation_shader_io_group,
    shader_validation_shader_io_id,
    shader_validation_shader_io_workgroup_size,
    shader_validation_shader_io_binding,
    shader_validation_shader_io_size,
    shader_validation_shader_io_pipeline_stage,
    shader_validation_shader_io_locations,
    shader_validation_shader_io_invariant,
    shader_validation_shader_io_align,
    shader_validation_parse_attribute,
    shader_validation_parse_must_use,
    shader_validation_parse_source,
    shader_validation_parse_literal,
    shader_validation_parse_blankspace,
    shader_validation_parse_semicolon,
    shader_validation_parse_comments,
    shader_validation_parse_requires,
    shader_validation_parse_enable,
    shader_validation_parse_diagnostic,
    shader_validation_parse_shadow_builtins,
    shader_validation_parse_identifiers,
    shader_validation_functions_alias_analysis,
    shader_validation_functions_restrictions,
    shader_validation_statement_increment_decrement,
    shader_validation_statement_statement_behavior,
    shader_validation_statement_continue,
    shader_validation_statement_discard,
    shader_validation_statement_switch,
    shader_validation_statement_const_assert,
    shader_validation_statement_loop,
    shader_validation_statement_phony,
    shader_validation_statement_continuing,
    shader_validation_statement_for,
    shader_validation_statement_while,
    shader_validation_statement_break_if,
    shader_validation_statement_break,
    shader_validation_statement_if,
    shader_validation_statement_compound,
    shader_validation_statement_return,
    shader_validation_uniformity_uniformity,
    shader_validation_expression_matrix_add_sub,
    shader_validation_expression_matrix_comparison,
    shader_validation_expression_matrix_mul,
    shader_validation_expression_matrix_and_or_xor,
    shader_validation_expression_matrix_div_rem,
    shader_validation_expression_matrix_bitwise_shift,
    shader_validation_expression_call_builtin_subgroupElect,
    shader_validation_expression_call_builtin_firstTrailingBit,
    shader_validation_expression_call_builtin_value_constructor,
    shader_validation_expression_call_builtin_textureSampleCompareLevel,
    shader_validation_expression_call_builtin_cross,
    shader_validation_expression_call_builtin_step,
    shader_validation_expression_call_builtin_abs,
    shader_validation_expression_call_builtin_textureDimensions,
    shader_validation_expression_call_builtin_determinant,
    shader_validation_expression_call_builtin_unpack2x16snorm,
    shader_validation_expression_call_builtin_subgroupMul,
    shader_validation_expression_call_builtin_transpose,
    shader_validation_expression_call_builtin_subgroupAdd,
    shader_validation_expression_call_builtin_faceForward,
    shader_validation_expression_call_builtin_textureNumLevels,
    shader_validation_expression_call_builtin_ldexp,
    shader_validation_expression_call_builtin_textureGatherCompare,
    shader_validation_expression_call_builtin_unpack4xU8,
    shader_validation_expression_call_builtin_textureSampleGrad,
    shader_validation_expression_call_builtin_ceil,
    shader_validation_expression_call_builtin_quantizeToF16,
    shader_validation_expression_call_builtin_pack2x16unorm,
    shader_validation_expression_call_builtin_subgroupMinMax,
    shader_validation_expression_call_builtin_frexp,
    shader_validation_expression_call_builtin_fract,
    shader_validation_expression_call_builtin_radians,
    shader_validation_expression_call_builtin_degrees,
    shader_validation_expression_call_builtin_clamp,
    shader_validation_expression_call_builtin_barriers,
    shader_validation_expression_call_builtin_countOneBits,
    shader_validation_expression_call_builtin_select,
    shader_validation_expression_call_builtin_any,
    shader_validation_expression_call_builtin_countLeadingZeros,
    shader_validation_expression_call_builtin_sin,
    shader_validation_expression_call_builtin_sinh,
    shader_validation_expression_call_builtin_min,
    shader_validation_expression_call_builtin_dot,
    shader_validation_expression_call_builtin_reflect,
    shader_validation_expression_call_builtin_subgroupBroadcastFirst,
    shader_validation_expression_call_builtin_extractBits,
    shader_validation_expression_call_builtin_refract,
    shader_validation_expression_call_builtin_log2,
    shader_validation_expression_call_builtin_sign,
    shader_validation_expression_call_builtin_atan,
    shader_validation_expression_call_builtin_bitcast,
    shader_validation_expression_call_builtin_acosh,
    shader_validation_expression_call_builtin_pack2x16snorm,
    shader_validation_expression_call_builtin_textureNumSamples,
    shader_validation_expression_call_builtin_cos,
    shader_validation_expression_call_builtin_subgroupBallot,
    shader_validation_expression_call_builtin_workgroupUniformLoad,
    shader_validation_expression_call_builtin_atomics,
    shader_validation_expression_call_builtin_pack4x8snorm,
    shader_validation_expression_call_builtin_pack2x16float,
    shader_validation_expression_call_builtin_textureGather,
    shader_validation_expression_call_builtin_all,
    shader_validation_expression_call_builtin_atan2,
    shader_validation_expression_call_builtin_pack4xI8,
    shader_validation_expression_call_builtin_subgroupShuffle,
    shader_validation_expression_call_builtin_firstLeadingBit,
    shader_validation_expression_call_builtin_exp2,
    shader_validation_expression_call_builtin_dot4I8Packed,
    shader_validation_expression_call_builtin_round,
    shader_validation_expression_call_builtin_pack4xU8Clamp,
    shader_validation_expression_call_builtin_floor,
    shader_validation_expression_call_builtin_unpack4x8snorm,
    shader_validation_expression_call_builtin_unpack4x8unorm,
    shader_validation_expression_call_builtin_textureSampleBias,
    shader_validation_expression_call_builtin_unpack4xI8,
    shader_validation_expression_call_builtin_derivatives,
    shader_validation_expression_call_builtin_dot4U8Packed,
    shader_validation_expression_call_builtin_modf,
    shader_validation_expression_call_builtin_textureSampleBaseClampToEdge,
    shader_validation_expression_call_builtin_normalize,
    shader_validation_expression_call_builtin_subgroupAnyAll,
    shader_validation_expression_call_builtin_length,
    shader_validation_expression_call_builtin_countTrailingZeros,
    shader_validation_expression_call_builtin_arrayLength,
    shader_validation_expression_call_builtin_pow,
    shader_validation_expression_call_builtin_subgroupBroadcast,
    shader_validation_expression_call_builtin_textureLoad,
    shader_validation_expression_call_builtin_atanh,
    shader_validation_expression_call_builtin_tanh,
    shader_validation_expression_call_builtin_log,
    shader_validation_expression_call_builtin_subgroupBitwise,
    shader_validation_expression_call_builtin_quadBroadcast,
    shader_validation_expression_call_builtin_pack4xI8Clamp,
    shader_validation_expression_call_builtin_exp,
    shader_validation_expression_call_builtin_pack4xU8,
    shader_validation_expression_call_builtin_asin,
    shader_validation_expression_call_builtin_acos,
    shader_validation_expression_call_builtin_cosh,
    shader_validation_expression_call_builtin_unpack2x16float,
    shader_validation_expression_call_builtin_insertBits,
    shader_validation_expression_call_builtin_textureNumLayers,
    shader_validation_expression_call_builtin_quadSwap,
    shader_validation_expression_call_builtin_unpack2x16unorm,
    shader_validation_expression_call_builtin_saturate,
    shader_validation_expression_call_builtin_textureSampleCompare,
    shader_validation_expression_call_builtin_smoothstep,
    shader_validation_expression_call_builtin_textureSample,
    shader_validation_expression_call_builtin_trunc,
    shader_validation_expression_call_builtin_fma,
    shader_validation_expression_call_builtin_mix,
    shader_validation_expression_call_builtin_sqrt,
    shader_validation_expression_call_builtin_asinh,
    shader_validation_expression_call_builtin_textureSampleLevel,
    shader_validation_expression_call_builtin_distance,
    shader_validation_expression_call_builtin_max,
    shader_validation_expression_call_builtin_reverseBits,
    shader_validation_expression_call_builtin_tan,
    shader_validation_expression_call_builtin_pack4x8unorm,
    shader_validation_expression_call_builtin_inverseSqrt,
    shader_validation_expression_call_builtin_textureStore,
    shader_validation_expression_unary_address_of_and_indirection,
    shader_validation_expression_unary_bitwise_complement,
    shader_validation_expression_unary_logical_negation,
    shader_validation_expression_unary_arithmetic_negation,
    shader_validation_expression_overload_resolution,
    shader_validation_expression_binary_comparison,
    shader_validation_expression_binary_short_circuiting_and_or,
    shader_validation_expression_binary_add_sub_mul,
    shader_validation_expression_binary_and_or_xor,
    shader_validation_expression_binary_parse,
    shader_validation_expression_binary_div_rem,
    shader_validation_expression_binary_bitwise_shift,
    shader_validation_expression_early_evaluation,
    shader_validation_expression_precedence,
    shader_validation_expression_access_array,
    shader_validation_expression_access_vector,
    shader_validation_expression_access_matrix,
    shader_validation_expression_access_structure,
    shader_validation_extension_dual_source_blending,
    shader_validation_extension_clip_distances,
    shader_validation_extension_readonly_and_readwrite_storage_textures,
    shader_validation_extension_pointer_composite_access,
    idl_constructable,
    idl_constants_flags
};
