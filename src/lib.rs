use std::process::Command;
use weechat::hooks::SignalData;
use weechat::hooks::SignalHook;
use weechat::plugin;
use weechat::Args;
use weechat::Plugin;
use weechat::ReturnCode;
use weechat::Weechat;

struct WeechatNotify {
    // these have underscores to suppress the dead_code warning. they are still dropped when the
    // plugin is unloaded.
    _h1: SignalHook,
    _h2: SignalHook,
}

impl Plugin for WeechatNotify {
    fn init(_: &Weechat, _: Args) -> Result<Self, ()> {
        let h1 = SignalHook::new("weechat_highlight", notify)?;
        let h2 = SignalHook::new("irc_pv", notify)?;
        Ok(Self { _h1: h1, _h2: h2 })
    }
}

fn notify_inner(
    _weechat: &Weechat,
    _signal_name: &str,
    data: Option<SignalData>,
) -> Result<(), String> {
    if let Some(SignalData::String(s)) = data {
        let mut arr: Vec<&str> = s.split('\t').collect();
        if arr.len() == 1 {
            let a = arr[0].split("PRIVMSG").collect::<Vec<&str>>();
            let b = a[0]
                .split('!')
                .next()
                .and_then(|s| s.strip_prefix(':'))
                .ok_or_else(|| "malformed data".to_string())?;
            let c = a[1]
                .split(':')
                .nth(1)
                .ok_or_else(|| "malformed data".to_string())?;
            arr = vec![b, c];
        }

        let nick = arr[0];
        let text = arr[1];

        Command::new("notify-send")
            .arg("-i")
            .arg("/usr/share/icons/Papirus-Dark/128x128/apps/weechat.svg")
            .arg("--")
            .arg(format!("{}:", nick))
            .arg(text)
            .spawn()
            .map_err(|e| e.to_string())?;

        return Ok(());
    }

    Err("No data was provided to the plugin".into())
}

fn notify(weechat: &Weechat, signal_name: &str, data: Option<SignalData>) -> ReturnCode {
    match notify_inner(weechat, signal_name, data) {
        Ok(_) => ReturnCode::Ok,
        Err(e) => {
            Weechat::print(&e);
            ReturnCode::Error
        }
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
