# wn2

[weechat-notify](https://git.karx.xyz/karx/weechat-notify), implemented as a native plugin instead of a script with a library.

requires: rust, notify-send

to build, first run `cargo build --release` then move the generated .so (`target/release/libweechat_notify.so`) to your plugin directory (probably `~/.local/share/weechat/plugins` but could also be `~/.weechat/plugins`). then rename the .so to `weechat_notify.so`

next, run `/plugin load weechat_notify` and enjoy your notifications!

note: changes to the plugin are not reflected until you restart WeeChat.

