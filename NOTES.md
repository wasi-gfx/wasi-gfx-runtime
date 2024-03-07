maybe have create texture from buffer, and then just use webgpu's create-view
Only problem might be if every frame get's a different buffer. But maybe, get_current_texture should return a remote-buffer, and that can be easily turned into a texture, but how do we connect them in the first place?
We might need a configure method anyway.

Another option. Use web gpu primitives (texture/buffer), even in cases where there's no webgpu, e.g. https://github.com/rust-windowing/softbuffer.


We probably need something like https://gpuweb.github.io/gpuweb/#canvas-context and https://docs.rs/wgpu/latest/wgpu/struct.Surface.html


Offline mini-canvas?




Okay, little new idea:
get-current-texture return texture that can be turned into buffer.
Configure happaning on gpu/gpu-context, has nothing to do with canvas.
Or maybe configure should be split? Part on canvas and other parts on gpu.


canvas:
 - alphaMode ( think, unless it's used somehow in the gpu process)
 - colorSpace
 - device


gpu:
 - usage Optional


duno:
 - format
 - viewFormats
 - getPreferredCanvasFormat()??







Another Option:
have an abstract concept of a Context
a context can give you a (remote-)buffer for next frame. Similar to get_current_texture and frame_buffer::Surface::buffer_mut
config can then happen on context? Not sure...
consider: the host will have to know how to connect any two canvas and graphics-api. So you won't be able to mix and match host implementations.






Another though:
There should be some way of locking a gpu-device to a canvas. That way the runtime can figure out stuff upfront rather than on each frame. Or maybe just do whatever and let host figure out?

