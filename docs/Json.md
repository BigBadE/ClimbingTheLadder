# JSON

JSON is used for named type loading and setting saving/loading.

Setting saving is manually implemented and not in the scope of this document,
because the game doesn't support saving.

# Load method

The load method provides the implementor complete control over the
deserialization process.

# JsonResource

Deriving JsonResource allows for the generation of automatic deserialization
of certain types. This is only done for primitives due to difficulty with
the reading of more advanced type's AST (generics in specific).

This just adds a method called `__load_{type}({type}, &JsonValue)`.
Calling this will deserialize all fields not annotated with `#[ignore_field]`.

Any fields not included in the JSON will have their values unchanged.

`#[require_field]` will make the JSON error if that field is missing.
If `#[require_field]` isn't used, the Result can be unwrapped safely.