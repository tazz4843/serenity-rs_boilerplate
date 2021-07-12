# `serenity-rs` Boilerplate

This is a nice little bot template you are free to use in your bots for whatever reason.
It has been split up into multiple crates to cut down on compilation time after small edits.
A few other nice utilities include a PostgreSQL database (`bot_db`), dynamic prefix by default,
and a logging system built in.

I hope I have left enough comments sprinkled throughout for you to easily get started, but if
you're confused about something, feel free to contact me on Discord: you can find me in the serenity-rs server
(https://discord.gg/serenity-rs) as 0/0#0001.

## Important Notes (PLEASE READ)

### Windows Compatibility
This boilerplate may not work on Windows. I personally am running Linux Mint with no access to any 
Windows machine, so I am unable to test on those systems. I will not be fixing any bugs, or accepting
any PRs related to Windows machines.

### Rust Compiler Version
You should use a recent nightly compiler with this. It uses some nightly features (such as `once_cell`).
It has been tested on this system:
```
$ rustc --version --verbose
rustc 1.55.0-nightly (5a7834050 2021-06-23)
binary: rustc
commit-hash: 5a7834050f3a0ebcd117b4ddf0bc1e8459594309
commit-date: 2021-06-23
host: x86_64-unknown-linux-gnu
release: 1.55.0-nightly
LLVM version: 12.0.1

$ cargo --version --verbose
cargo 1.54.0-nightly (44456677b 2021-06-12)
release: 1.54.0
commit-hash: 44456677b5d1d82fe981c955dc5c67734b31f340
commit-date: 2021-06-12
```

## How To Use

You should not use this directly without edits. It is not designed to be used that way 
(although it will work). Download this repo as a ZIP file (upper right corner), and unzip it into a folder.
Then open up the folder in your favorite IDE (this was created with CLion, so I recommend using that), and
begin editing it to your needs.

## Configuration

### Disabling the database
The database can be disabled by changing `DATABASE_ENABLED` in `bot_db/src/lib.rs` to `false`.
```rust
pub const DATABASE_ENABLED: bool = false;
```
Note that doing so will result in:
* no dynamic prefixes
* errors when trying to use the `PgPool` (it won't be inserted into the client)

This is useful for bots with no state or need to save things, but for most bots you should leave it on.

**Disabling the database will not affect runtime speed!** As the variable is a `const`, it is replaced 
at compile time, which results in the compiler simply removing the code that won't be executed.
Do note, however, that `sqlx` is still used as a dependency.

### Slash Commands
Discord's ~~most broken~~ nicest new feature.
Support for them will be added soon.

### Buttons
See `bot_commands/src/cmd_buttons.rs`.

### Dropdowns (aka Select Menus)
See `bot_commands/src/cmd_dropdowns.rs`.
