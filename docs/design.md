



# Information:
There are many components to able to run a game or application to deal with the entity and game state.

# Components:
 * GameData (SimpleState) and GameDataBuilder ( GameDataBuilder )
 * GameState (SimpleState or ExampleState)( game, menu, input )
 * System component (run game conditions, input, loop)
 * Entity component (build entity component as well others)

# GameData and GameDataBuilder:
    This setup root of Game Data and Game Data Builder. As well build bundle setup.

    There is add on for system component to run loop checks. But note that with_base (with), with_base_bundle (with_bundle), and other functions parameter config by default if wanted more control of the gamedata and gamedatabuilder.

Default config:
```rust
let game_data = GameDataBuilder::default()
.with_bundle(TransformBundle::new())?
```
Custom config: (customgamedata)
```rust
let game_data = CustomGameDataBuilder::default()
.with_base_bundle(TransformBundle::new())
```

# GameState:
    This handle entity, state loop, input handle, as well menu. The game state can change to different state. For example loading, game, pause, config, and etc.

# System component:
    This handle loops such user input, game condtion for win or lose, physics, and other thing that does update.

# entity component:
    This is enity such as stuct, player data.

# Notes:
 * This some what I understand.
 * Not in depth on loading objects.


To access to custom data

 ```rust
impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for ExampleState {

    fn on_start(&mut self, data: StateData<CustomGameData>) {
        let world = data.world;
        let customgamedata = data.data; //CustomGameData
        println!("Networking menu");
    }

    fn handle_event(&mut self, data: StateData<CustomGameData>, event: StateEvent,) -> Trans<CustomGameData<'a, 'b>, StateEvent> {

        Trans::None //return value
    }

    fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a, 'b>, StateEvent> {

        Trans::None
    }

    fn on_stop(&mut self, data: StateData<CustomGameData>) {
        //clean up
    }

}
 ```