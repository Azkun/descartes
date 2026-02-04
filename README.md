# Descartes
![Rust](https://ziadoua.github.io/m3-Markdown-Badges/badges/Rust/rust3.svg) ![MIT License](https://ziadoua.github.io/m3-Markdown-Badges/badges/LicenceMIT/licencemit3.svg)

Descartes is an application made for different tests, meant to be used inside a [Numworks calculator](https://www.numworks.com/). For now, its purpose remains undetermined, and I can only hope to one day find a true useful aim for it. It was made using [NumworksAppsRust](https://github.com/yannis300307/NumworksAppsRust), which is a pretty easy to use template to make Rust apps and compile them for the calculator.

I am still thinking about what kind of original application would be useful for a student. From hard to imagine spreadsheets to dumb enough pomodoro apps, I will take my time to find something truly satisfying! 

## Current behavior of the app

Descartes currently displays a very simple interface, which tell the user about the current time since the startup of the calculator in milliseconds, and the used OS between Simulator, Epsilon or Upsilon.

## Setup

If you are on a Debian based Linux Distro (Debian, Ubuntu, Linux Mint, ...), you simply have to run `bash ./setup.sh` to install all the dependencies (You might have to reopen your terminal to reload the PATH).

You can now run `just sim` to see a simulator version of the app.
You can now run `just build-epsilon` to build the app for Epsilon (native OS of the calculator)
The build's path is in `/target/thumbv7em-none-eabihf/release/Descartes`, which you can then send to your calculator using `just send-epsilon` or the [Extra apps' page on the Numworks' website](https://my.numworks.com/apps).

| Command               | Description                                                                                                                              |
|-----------------------|------------------------------------------------------------------------------------------------------------------------------------------|
|`just build-epsilon`   | Build the app for **Epsilon** and create a NWA file that can be imported to the Numworks' [app page](https://my.numworks.com/apps).      |
|`just build-upsilon`   | Build the app for **Upsilon**.                                                                                                           |
|`just send-epsilon`    | Build and send the app to the calculator running **Epsilon**.                                                                            |
|`just send-upsilon`    | Build and send the app to the calculator running **Upsilon**.                                                                            |
|`just release-upsilon` | Build and make the project ready to be added to the [Upsilon-external](https://github.com/UpsilonNumworks/Upsilon-External/) repository. |
|`just check`           | Run `cargo check` for every targets and and every firmwares.                                                                             |
|`just sim [job-count]` | Build the app as a NWB and run the Epsilon simulator. The job count will be added to the cc `-j` argument (default 1).                   |
|`just clean`           | Clean the build cache for the app and the files used by Upsilon-External.                                                                |
|`just clear`           | Remove the build cache, the files for Upsilon-External and the simulator.                                                                |

## Quick documentation

The `src` folder contains a main.rs file and a nadk folder. The nadk folder is a rust module that contains the cross platform api.

The `main.rs` file uses 2 macros `setup_allocator!()` and `init_heap!();` in order to init the heap allocator.

You can find the list of all the available nadk's modules in `src/nadk/mod.rs`. The modules and the functions should be self explantory.

Note that the heap on Upsilon is only 80 Ko compared to the 100 Ko on Epsilon.

If you want to use a crate but only on calculator, for instance `alloc`, import this crate using the `calc_use!(crate)` macro. To import a crate only on the simulator, use `sim_use!(crate)`.

## Credits

Original sample app-rust by **numworks** : [Github Repo](https://github.com/numworks/epsilon-sample-app-rust)
Original NumworksAppsRust by **yannis300307** : [Github Repo](github.com/yannis300307/NumworksAppsRust)
Which uses [storage.c by Yaya.cout](https://framagit.org/Yaya.Cout/numworks-extapp-storage) to access the storage of the calculator, and [yannis300307's NumcraftRust project](https://github.com/yannis300307/NumcraftRust) for things like the simulator support or the allocator.

## Licenses

The project is under the MIT License. The files in the build directory are under the MIT license but licensed by Damien Nicolet. (except build.rs) The files in src/nadk/storage are under the MIT license but licensed by Yaya.Cout. (except mod.rs)

Numworks is a registered trademark. This project has no association with Numworks.