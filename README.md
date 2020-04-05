# rame
 Rust
 Amethyst
 Module
 Engine

# License: MIT

# required:
 * install rust lang
 
# Information:
	Work in progress. Just a prototype test. To create a simple game mode with creatitve, rpg and other game type to keep thing simple.

# Design:
	In hope to design an API and easy to understand build. To used .ron files for menu, entity object, and other things. It depend on the design of the code layout and structure.

# Project:
```
cargo build // build applicaton
cargo run // run applicaton
cargo clean // delete folder target

cargo build --example main_network //wip
cargo run --example main_network //wip
```

# Examples:
```
cargo run --example main_customgamedata // working
cargo run --example main_examplestate // working
cargo run --example main_simple // working
```

```powershell

cargo build --example main_network

& "target\debug\examples\main_network" --server "128.0.0.1:8080"
```
# Notes:
 * network code might be tricky to setup with command line

# Links:
 * https://github.com/amethyst/amethyst
 * https://github.com/amethyst/amethyst/tree/master/examples/custom_game_data
 * https://book.amethyst.rs/stable/controlling_system_execution/custom_game_data.html
 * https://github.com/Gekkio/imgui-rs
 * https://github.com/amethyst/amethyst/blob/master/examples/states_ui/menu.rs
 * https://github.com/clap-rs/clap/tree/master/examples
 * 