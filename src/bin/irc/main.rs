extern crate eq;
extern crate irc;

pub use irc::client::prelude as mirc;
use irc::client::Client;
use irc::client::ext::ClientExt;

fn irc_config() -> mirc::Config {
  let server = std::env::var(
    "EQ_IRC_SERVER").expect("server").to_owned();
  let nickname = std::env::var(
    "EQ_IRC_NICKNAME").expect("nickname").to_owned();
  let password = std::env::var(
    "EQ_IRC_PASSWORD").expect("password").to_owned();
  let channel = std::env::var(
    "EQ_IRC_CHANNEL").expect("channel").to_owned();
  mirc::Config {
    nickname: Some(nickname.clone()),
    password: Some(password.clone()),
    server: Some(server),
    channels: Some(vec![channel]),
    ..mirc::Config::default()
  }
}

fn main() {
  let mut heap = eq::heap::Heap::with_capacity(4096);
  let mut container = eq::container::Container::with_heap(heap);
  let mut reactor = mirc::IrcReactor::new().expect("reactor");
  let config = irc_config();
  let client = reactor.prepare_client_and_connect(&config).expect("connect");
  client.identify().expect("identify");
  let command_prefix = format!("{}: ", &config.nickname.unwrap());
  reactor.register_client_with_handler(client, move |client, message| {
    print!("{}", message);
    let response_target = message.response_target().map(|x| x.to_string());
    match message.command {
      mirc::Command::PRIVMSG(_, message_body) => {
        let response_target = response_target.expect("response");
        if message_body.starts_with(&command_prefix) {
          let source = message_body
            .trim_left_matches(&command_prefix).trim();
          match container.eval(&source, 1024) {
            Ok(target) => {
              let command = mirc::Command::PRIVMSG(response_target, target);
              client.send(command)?;
            }
            Err(error) => {
              let target = format!("error: {:?}", &error);
              let command = mirc::Command::PRIVMSG(response_target, target);
              client.send(command)?;
            }
          }
        }
      }
      _ => {}
    }
    return Ok(());
  });
  reactor.run().expect("run");
}
