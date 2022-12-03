# rust-game-tutorial
Rust Tutorial from /r/roguelikedev

## Personal Rust Notes (ongoing)

### Helpful Commands
- start project: `cargo run`
- `println!`, anything with ! after is a procedural macro (vs derived macro)
  - Procedural macros run like a function - they define a procedure, they just greatly reduce your typing.
- if use cargo init, program is a cargo _crate_
- useful cmds
 - `cargo`
    - `init`
    - `build`
    - `update`
    - `update --dryrun` to view updates without making them
    - `clean`
    - `verify-project` - check if settings correct
    - `install` install programs via cargo

- `cargo fmt` to format code to rust standards
- `cargo clippy` run linter
- `cargo search packageName` check pckg version

### Using a namespace
```rust
use rltk::{Rltk, GameState};
```
equivalent
```ts
import { Rltk, GameState} from 'rltk'
```

### General
- `struct` is structure, like a `Class`
```rust
impl GameState for State
``` 
GameState is a Trait
equivalent Typescript
```ts
class State implements GameState`
```
- `&mut self` means `fn` can change variables inside parent struct
` `self` means `fn` can view/see content of par struct but can't change.
- "If you omit the &self altogether, the function can't see the structure at all - but can be called as if the structure was a namespace (you see this a lot with functions called new - they make a new copy of the structure for you)"
- `&` means pass a reference | _pointer_ to the original value, if you change it, it will change the original
-  `main` fn tells the Operating System where to start the program.
- `RltkBuilder::simple80x50()` example of calling fn from inside of a struct, where that struct doesn't take a `self` function. 
- "derive macros are a special type of macro that implements additional functionality for a structure on your behalf - saving lots of typing."
- Any function that ends with a statement that lacks a semicolon treats that line as a return statement.
- "A vector is like an array, which lets you put a bunch of data into a list and access each element. Unlike an array, a Vec doesn't have a size limit - and the size can change while the program runs. So you can push (add) new items, and remove them as you go."

### Updating Rust
"you can type `rustup self update`. This updates the Rust update tools (I know that sounds rather recursive). You can then type `rustup update` and install the latest versions of all of the tools."

###  Acronyms
- ECS - Entity Component System
- POD - "Plain Old Data"