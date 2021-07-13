pub mod cmd_buttons;
pub mod cmd_dropdowns;
pub mod cmd_example;
pub mod cmd_help;
pub mod cmd_ping;
pub mod groups;

// these glob imports let you use `use bot_commands::*;` if you so desire to import every command
pub use cmd_buttons::*;
pub use cmd_dropdowns::*;
pub use cmd_example::*;
pub use cmd_help::*;
pub use cmd_ping::*;
pub use groups::*;
