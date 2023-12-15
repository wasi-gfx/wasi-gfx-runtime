# Webgpu:

The webgpu parts should be based on the official webgpu spec.

We might end up deviating slightly from the spec in cases where it's deemed to be better, but the spec is definitely the starting point for this api.

## Building graphical user applications:

Webgpu by itself can't do everything a normal graphical user application needs. Webgpu can only do computations on the gpu, it can't render to the screen directly, it can't take user input, and it can't open an application window. It doesn't even know how often the screen refreshes to be able to do computations on each frame.

Since these things are necessary for most graphical applications, we should try to get at least some of them solved.

### Screen rendering / canvas:

There's a simple canvas spec in [`/wit/canvas.wit`](/wit/canvas.wit). It can't really do much, but can do basic rendering. See later section on how to connect the canvas with webgpu.

### User input:

There's a fair bit we can take from the web here as well.

##### Pointer events:

For mouse / touch / pencil / etc. input we can do something similar to [`PointerEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent) on the web. We can't really take it wholesale, as events on web assume that html elements exist. But we can build a very similar interface, built on similar principles.

The only drawback I can think of is 3D, I'm not sure if pointer events can be made to work with 3D applications like XR. Would love input from engineers that have experience in this area.

##### Keyboard events:

Most applications expect keyboard events in one way or another. We can do something similar to pointer events where we define something similar to what the web offers with [`KeyboardEvent`](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent).

##### Other use input:

Other user input, like scroll wheel, clipboard, drag-over, device motion, device orientation, gamepad input, etc. Are all useful in lot's of applications, but since they're a bit less common, I didn't want to get bogged down with them. But they should be added eventually.

### Windowing:

Defining a full windowing interface is going to take a lot of time, so it's out of scope for this project.

For now, creating a new canvas should be enough. The runtime can figure out how to deal with windowing.

### Frame callbacks:
On the web, this is achieved by [`requestAnimationFrame`](https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame). 

Here we're using a `wasi-io` `pollable`. You can find the wit for this in /wit/animation-frame.wit


## Connecting canvas to webgpu

I didn't want webgpu and canvas to rely on each other, as I'd like one to be usable without the other.

#### Potential use cases where canvas might make sense without webgpu include:
- Vulken, if it ever becomes truly cross platform.
- OpenGL might make sense for some use cases.
- Future gpu api's that will almost certainly come eventually.
- Using simple arrays as buffers for simple drawings, especially useful in embedded. E.g. https://github.com/rust-windowing/softbuffer.

#### Potential use cases where webgpu might make sense without canvas include:
- Using gpu for compute only. (the web actually allows this, even though webgpu is tied into canvas on web.)
- Potential full wasi-windowing spec that does away with this simple mini-canvas.

On the web, canvas and webgpu are actually tied to each other. e.g. you get a `GPUContext` from `canvas.getContext("webgpu")`. But I'd like them to be separated in wasi if possible.

Maybe y'all will tell me that separating them is a stupid idea, in which case I'll bow down and build a time machine fueled by pure awkwardness, traveling back to the moment before I even suggested this.

### Common dependency:
If we wanna seperate them we'll need a common dependency that both can connect to. I have a few ideas that I thought of.

#### Context:
In webgpu on the web, the point of connection between webgpu and canvas is the `GPUCanvasContext`. Maybe we can have something similar, but as an abstract without connection between canvas and gpu.

I'm thinking of a context that can be connected to any current or future graphics api (webgpu, opengl, etc.) on one side, and can be connected to any current or future canvas on the other side.

It should have a way to get the next frame as a buffer, but not necessarily as a buffer whose contents can be read. Rather, the graphics api in question should be able to turn that buffer into something it can make use of. E.g. webgpu would be able to turn the buffer into a GPUTexture. The runtime can easily, in the background, represent the buffer as a GPUTexture, and the conversion would just be a noop.

This would look something like this:

```wit
interface graphics {
  resource context {
    new: static func() -> context;
    configure: func(desc: configure-context-desc);
    get-current-buffer: func() -> buffer;
  }
  resource buffer { }
}
interface webgpu {
  resource gpu-device {
    connect-context: func(context: borrow<graphics.context>);
    ...
  }
  resource gpu-texture {
    from-graphics-buffer: func(buffer: graphics.buffer) -> gpu-texture;
    ...
  }
  ...
}
interface canvas {
  resource canvas {
    connect-context: func(context: borrow<graphics.context>)
    ...
  }
  ...
}
```

There's another option that I put in issue #1, but I think this one would be better as it's closer to how webgpu does things, but still let's us keep canvas and webgpu separated.

If the runtime sees that the they're both connected to the same common dependency, it can do some magic internally to optimize the connection in the background.

## Simple frame buffer rendering:

Since it's so simple, and can be used as a proof of concept for multiple ways to render to the canvas, we're adding a simple frame buffer api as well. /wit/frame??.wit
