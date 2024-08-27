# TODOLIST
Here are all functions that will be implemented in the future.

## Commands
- `init`
- `llc_add_path <path>`
- `llc_add_progress <id>`
- `llc_add_entitybase <id>`
- `llc_del_path <path>`
- `llc_del_progress <id>`
- `llc_del_entitybase <id>`
- `get_id <path>`
- `get_progress <id>`
- `get_entitybase <id>`
- `add_libentity <path> [--name <name>] [--tags tag1,tag2,...] [--prog_ceil <prog_ceil>]`
- `del_libentity <path>`
- `status [<path>] [--hidden] [--ignore f1,f2,...]`
- `look <path>`
- `open [--just_look(-j)] <path>`
- `list [--wide]`
- `change_progress <id> <progress_update>`
- `add_tags <id> tag1,tag2,...`
- `del_tags <id>`


## Upgrades
Empty.

## Features
- Taking Lua-script to output data produced by `list`, `look` and so on. Those functions are decorative.
- Descriptions for library entities.
