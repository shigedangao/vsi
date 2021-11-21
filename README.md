# VSI 🛬 (W I P)

<p align="center">
    <iframe src="https://giphy.com/embed/RjBZI0nO3Hk6aprlMz" width="480" height="270" frameBorder="0" class="giphy-embed" allowFullScreen></iframe><p><a href="https://giphy.com/gifs/SafranGroup-landing-airbus-a350-RjBZI0nO3Hk6aprlMz">via GIPHY</a></p>
</p>

A small Windows Tray app which send a notification when your aircraft touchdown on the FS2020 simulator

## Requirements

- Rust
- Windows
- FS2020
- FS2020 SDK
- FS2020 SDK sample
- Visual studio

## Local dev

1. Clone this repository
2. Comment the `main.rs` by commenting the line below

```diff
- #![windows_subsystem = "windows"]
+ // #![windows_subsystem = "windows"]
```

3. Run the project with the command

```shell
cargo run
```

## Issues

❌ Setting the *AppUserModelID* work with the window-rs crate. See the branch *try-notif*. However no notification appear for some reasons... It could be required that the app need to be installed with something ? maybe an exe in a specified path with registration key

## What is currently displayed during a touchdown ?

- G Force
- Touchdown velocity in fpm
- Touchdown pitch deg (so far it's negative might need to invert the value)
- Touchdown heading
- Touchdown bank deg
