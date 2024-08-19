# Lua scripts
The **popusk** program uses [lua](https://www.lua.org/) to help for some commands. For example, *decorative commands* such as `look` and `list` takes lua scripts to produce their output.
## Types
### Progress
Progress of document.
**Fields**
1. `passed`. Integer.
2. `ceiling`. Integer.
**Invariants**
1. `passed <= ceiling`.
2. `ceiling >= 1`.

### Context
Contains context information.
**Fields**:
1. `tshape_w`. Width of terminal. Integer.
2. `tshape_h`. Height of terminal. Integer.

### LibEntity
Contains information about library entity in the library.
**Fields**:
1. `path`. String.
2. `id`. String.
3. `name`. String.
4. `tags`. Array of strings.
5. `etype`. Entity type. String that can be one of the values: "document", "section", "regular".
6. `progress`. Progress of document, available only if `etype` is "document". Progress.

## Scripts file
Scripts file has path `$HOME/.config/popusk/scripts.lua` and has the following content (note that definitions of the functions are abstract and differ from valid *lua*-definitions).
1. Function `look_output(libentity: LibEntity, context: Context) -> string`. Forms output for the `look` command.
2. Function `list_output_narrow(libentities: Array<LibEntity>, context: Context) -> string`. Forms output for the `list` command.
3. Function `list_output_wide(libentities: Array<LibEntity>, context: Context) -> string`. Forms output for the `list --wide` command.
