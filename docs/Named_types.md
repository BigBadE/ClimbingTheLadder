# Named Types

Named types provide structure for things in the game, basically telling
the code what implementations to use for certain functions. For example,
what texture a cube should have, what happens when a player clicks
with a specific item, etc...

# Basic implementation

All named types go in the types folder. Any subfolders are ignored when
loading. Arrays and objects are allowed, but nothing else.

All Named Types need two things: a name and a type.

These are the fields "$name" and "$type".

Names are unique, so all mods should prefix names with the mod name.

Types are the name of the implementor meant to be used.
You can figure out implementor names from other types or add your own.

# Creating implementors

