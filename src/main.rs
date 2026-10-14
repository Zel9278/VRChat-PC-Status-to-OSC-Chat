//! Shows this PC's status in the VRChat chat box over OSC. The figures come from pcsc-rs's
//! system monitor, read on this PC; no PC Status server or pcsc-rs client is needed.
//! A line typed into the console is added to the chat box; an empty line takes it away.

mod message;

use std::{
    env,
    io::BufRead,
    net::UdpSocket,
    process,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use rosc::{OscMessage, OscPacket, OscType, encoder};

const USAGE: &str = "\
Shows this PC's status in the VRChat chat box (OSC).

Usage: vrc-pc-status [options]

  --host <address>    where VRChat listens for OSC (default 127.0.0.1)
  --port <port>       its port (default 9000)
  --interval <secs>   how often to send, at least 2 (default 3)
  --text <text>       text added to the chat box at the start; type a line to change it
  --format <format>   the chat box text, instead of the usual line. \\n is a line break.
                      Placeholders: {PLACEHOLDERS}
  --sound             play the chat box's notification sound on each update
  --help

Type a line and press Enter to add it to the chat box; an empty line takes it away.";

struct Options {
    host: String,
    port: u16,
    interval: Duration,
    text: String,
    format: Option<String>,
    sound: bool,
}

fn options() -> Result<Options, String> {
    let mut options = Options {
        host: "127.0.0.1".into(),
        port: 9000,
        interval: Duration::from_secs(3),
        text: String::new(),
        format: None,
        sound: false,
    };
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut value = |name: &str| args.next().ok_or_else(|| format!("{name} needs a value"));
        match arg.as_str() {
            "--host" => options.host = value("--host")?,
            "--port" => {
                options.port = value("--port")?
                    .parse()
                    .map_err(|_| "--port takes a number")?;
            }
            "--interval" => {
                let secs: f64 = value("--interval")?
                    .parse()
                    .map_err(|_| "--interval takes seconds")?;
                // VRChat may drop chat box updates that come faster
                options.interval = Duration::from_secs_f64(secs.max(2.0));
            }
            "--text" => options.text = value("--text")?,
            "--format" => options.format = Some(value("--format")?),
            "--sound" => options.sound = true,
            "--help" | "-h" => {
                println!("{}", USAGE.replace("{PLACEHOLDERS}", message::PLACEHOLDERS));
                process::exit(0);
            }
            other => return Err(format!("unknown option {other} (see --help)")),
        }
    }
    Ok(options)
}

/// `/chatbox/input`: the text, sent at once rather than typed into the keyboard, with or
/// without the notification sound.
fn chatbox(text: &str, sound: bool) -> Vec<u8> {
    let packet = OscPacket::Message(OscMessage {
        addr: "/chatbox/input".into(),
        args: vec![
            OscType::String(text.into()),
            OscType::Bool(true),
            OscType::Bool(sound),
        ],
    });
    encoder::encode(&packet).expect("an OSC message always encodes")
}

fn main() {
    let options = match options() {
        Ok(options) => options,
        Err(e) => {
            eprintln!("{e}");
            process::exit(2);
        }
    };
    let socket = UdpSocket::bind(("0.0.0.0", 0)).unwrap_or_else(|e| {
        eprintln!("Could not open a UDP socket: {e}");
        process::exit(1);
    });
    let target = format!("{}:{}", options.host, options.port);

    // The text typed into the console
    let text = Arc::new(Mutex::new(options.text.clone()));
    {
        let text = Arc::clone(&text);
        thread::spawn(move || {
            for line in std::io::stdin().lock().lines().map_while(Result::ok) {
                line.trim().clone_into(
                    &mut text
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner),
                );
            }
        });
    }

    let monitor = pcsc_rs::Monitor::start();
    println!(
        "Sending to VRChat at {target} every {:.0?}. Type a line to add it to the chat box.",
        options.interval
    );
    loop {
        // Disk and CPU figures are the change over a second; let the first sample settle
        thread::sleep(options.interval);
        let figures = message::Figures::read(&monitor.status());
        let current = text
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();
        let line = message::render(&figures, &current, options.format.as_deref());
        match socket.send_to(&chatbox(&line, options.sound), &target) {
            Ok(_) => println!("{line}"),
            Err(e) => eprintln!("Could not send to {target}: {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use rosc::{OscPacket, OscType, decoder};

    #[test]
    fn encodes_the_chatbox_message() {
        let bytes = super::chatbox("CPU: 1%", false);
        let (_, OscPacket::Message(message)) = decoder::decode_udp(&bytes).unwrap() else {
            panic!("not a message");
        };
        assert_eq!(message.addr, "/chatbox/input");
        assert_eq!(
            message.args,
            [
                OscType::String("CPU: 1%".into()),
                OscType::Bool(true),
                OscType::Bool(false)
            ]
        );
    }
}
