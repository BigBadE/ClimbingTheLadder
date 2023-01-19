The game runs with two main thread pools:
- CPU thread pool, with threads = amount of cores
- IO thread pool, with 10 threads

On WASM, there is only one thread and no I/O.

What runs async:
- Worlds (and most of their data) runs async, the world struct only wraps around that thread.
- Rooms all run on the same world thread
- Renderer runs on the main window thread

Types sent across threads:
- FrameData (each frame)
- Mesh (when updater updates the static data)
- ModMain (on load)