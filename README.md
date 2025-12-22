# buckshot roulette w shock collars

quite literally just filters `STDOUT` for `death request on instance: yourName` which buckshot roulette writes when you are shot in an online game, and makes an openshock API request when it finds it  
tested on linux, probably works on windows

to use, fill out `woaw.toml`

it should identify the game location on windows and linux assuming the default game location, you can always override the path with `game_path` in the config

to run you need rust and `cargo` installed, fill out the config and just run `cargo run` and the game should launch

[openshock wiki](https://wiki.openshock.org/)
