use weechat::plugin;
use weechat::Args;
use weechat::Plugin;
use weechat::Weechat;

struct WeechatNotify;

impl Plugin for WeechatNotify {
    fn init(_: &Weechat, _: Args) -> Result<Self, ()> {
        Weechat::print("Hello from weechat-notify!");
        Ok(Self)
    }
}

impl Drop for WeechatNotify {
    fn drop(&mut self) {
        Weechat::print("Bye from weechat-notify!");
    }
}

plugin! {
    WeechatNotify,
    name: "weechat-notify",
    author: "Yash Karandikar <yash@karx.xyz>",
    description: "Notification plugin for weechat written in Rust",
    version: "0.1.0",
    license: "0BSD"
}
