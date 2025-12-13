# buckshot roulette w shock collars

quite literally just filters the log for `death request on instance: yourName` which buckshot roulette writes when you are shot in an online game, and makes an openshock API request when it finds it  
completely untested but it's late so i'm going to bed

just fill out `woaw.toml`

should identify the log location on it's own, it works on linux atleast

to run you need `cargo` installed, fill out the config and just run `cargo run`

[openshock wiki](https://wiki.openshock.org/)
