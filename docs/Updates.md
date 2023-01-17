# Update Loop

The update loop is run asynchronously, with each step running in the following order:

The renderer stores the static data (like textures and model) in a RWLock, which
the update can not change. The rest of the data is double-buffered by the renderer when rendering, and cached.

