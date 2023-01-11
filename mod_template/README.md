# Mod template

Welcome to the mod template.
This is designed to allow creating a loadable mod for any version of the game (except web).

# Building / Running

Any mods in the "mod" folder will be automatically detected and loaded by the game.

The game requires an assembly

Running `build.sh` will build the mod and allow the game to use it.

# Modding

This entire game is open source, so you can look at the code / documentation relevant to your goals.

This basic template should be enough to get you started.

# Releasing

Releasing requires building every computer that can run the game, which sounds worse than it is.

It does take a few commands though, which is why `build-all-platforms.sh` will do it for you

Make sure the final release doesn't include the target folder, or any IDE folders.
