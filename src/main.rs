// RULE NUMBER ONE:
// DO NOT PUT ANYTHING IN THIS FILE!
// put anything new into bot_core/src/lib.rs:entrypoint

use bot_core::entrypoint;

// this #[tokio::main] section is a macro that will turn main into a fn that spawns the
// tokio runtime and starts up the code inside this function on the runtime
// it's not as complex as it sounds
#[tokio::main]
async fn main() {
    entrypoint().await.expect("failed to start the bot");
}
