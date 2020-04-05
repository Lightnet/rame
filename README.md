# rame
```
 Rust
 Amethyst
 Module
 Engine
```
# Created By: Lightnet

# License: MIT

# Required:
 * install rust lang
 * Amethyst 0.15
 
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
```powershell
cargo run --example main_customgamedata // working
cargo run --example main_examplestate // working
cargo run --example main_simple // working
```

```powershell

cargo build --example main_network

& "target\debug\examples\main_network" --server "128.0.0.1:8080"

& "target\debug\examples\main_network" --client "128.0.0.1:8080"
```
# Notes:
 * network code might be tricky to setup with command line

# Links:
 * https://github.com/amethyst/amethyst
 * https://github.com/amethyst/amethyst/tree/master/examples/custom_game_data customgamedata
 * https://github.com/amethyst/amethyst/blob/master/examples/states_ui/menu.rs menu ui
 * https://github.com/amethyst/shred Dispatcher and System types
 * https://book.amethyst.rs/stable/controlling_system_execution/custom_game_data.html customgamedata
 * https://github.com/Gekkio/imgui-rs imgui
 * https://github.com/clap-rs/clap/tree/master/examples comand line args
 * 