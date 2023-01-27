# Shaders

Shaders use the WGPU format.

# Load first

The UI shaders need to be loaded first to show the UI screen, so any shaders in the load_first folder are loaded first.
Mods don't use the load_first folder because they're not loaded yet.
The rest of the shaders are loaded from the shaders folder.