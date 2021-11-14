# VSI 🛬 (W I P)

<p align="center">
    <img src="https://i.makeagif.com/media/1-10-2021/CCtgdM.gif">
</p>

A small Windows Tray app which send a notification when your aircraft touchdown on the FS2020 simulator

## Requirements

- Rust
- Windows
- FS2020

## Issues

✔️ There is an ongoing issue with the windows crate which required to bundle a manifest for the common controls. Might need to check for further update on that matter. Check the comment on the build.rs there is a link to the window-rs repo about the issue.

❌ Setting the *AppUserModelID* work with the window-rs crate. See the branch *try-notif*. However no notification appear for some reasons... It could be required that the app need to be installed with something ? maybe an exe in a specified path with registration key

## What is currently scrap ?

- G Force
- Touchdown velocity in fpm
- Touchdown pitch deg (so far it's negative might need to invert the value)
- Touchdown heading
- Touchdown bank deg
- On Ground (flag used internally)
