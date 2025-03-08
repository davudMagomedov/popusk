# Tutorial

Once you have installed the binary, create a folder `~/.config/popusk/`.
Next, create files `scripts.lua` and in the directory. Look at [this](lua_script.md) for next steps.

## Usage

Now you have everything you need to start using the app.
1. Move to the directory you want to use as a library.
2. Execute `popusk init`.
3. Add library entities through commands in `popusk --help` list. Remember, commands starting with `llc_` shouldn't be used.

New library entities are added via the `popusk add_libentity` command.
Library entities are deleted via the `popusk del_libentity` command.
If you need a description for some command, use template `popusk <command> --help`.
There are visual commands (i.e. the main purpose of which is to display aesthetically pleasing text). For example, `look` and `list` are visual commands.

## Library entities

**Popusk** operates with so-called *Library Entities*.
**Library entity** is an addition to a file. It includes name, tags, type, etc (iow addition information). It points to a file.
**Libentity** has following fields:
1. *Path*. Points to a file the *libentity* is connected to.
2. *ID*. Just unique identifier for the *libentity*.
3. *Name*. Name of the *libentity*.
4. *Tags*. Tag list of the *libentity*.
5. *Entity type*. There're 3 variants of *entity type*: *document*, *regular file* and *section*.
    - *Document*. In theory, the most common. It is something that can be opened as a book, document, etc.
    - *Section*. Points to directory.
    - *Regular file*. Everything else.
6. *Progress*. Definetly exists if the *entity type* is *document*.
7. *Description*. Optional.
8. *FreeData*. Optional.

### Progress

A library entity with `etype == "document"` must have a *progress*. Otherwise, the library entity must not have a *progress*.

Progress contains *passed* and *ceiling* values. For example, let's have a book with 571 pages in total and your imaginary bookmark is on 184 page. This information is kept in *progress* with `passed = 184` and `ceiling = 571`.

### Free Data

*FreeData* stores all values that do not fit in the fields of *LibraryEntity*. It is a dictionary in the root.

There are several types that *FreeData* supports:
1. *String*
2. *Integer*
3. *Float*
4. *Boolean*
5. *Dictionary* is `Map<String, Object>`
6. *Array*
7. *Null*

## Low-level commands

**Remember: you should never use low-level commands unless you got a bug related to the app**.

You operate with *llc* via *ID*, not *path*.

Low-level commands starts with `llc_`. Check the `popusk help`.

## *Beautiful* commands

There're *beautiful* commands aimed only at beautiful content output. For example, the `look` command outputs a kind of cover of the file.

You can specify output format for those functions in [scripts](lua_script.md).
