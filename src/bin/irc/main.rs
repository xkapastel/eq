extern crate eq;
extern crate irc;

pub use irc::client::prelude as mirc;
use irc::client::Client;
use irc::client::ext::ClientExt;

fn irc_config() -> mirc::Config {
  let server = std::env::var("EQ_IRC_SERVER").unwrap().to_owned();
  let nickname = std::env::var("EQ_IRC_NICKNAME").unwrap().to_owned();
  let password = std::env::var("EQ_IRC_PASSWORD").unwrap().to_owned();
  let channel = std::env::var("EQ_IRC_CHANNEL").unwrap().to_owned();
  mirc::Config {
    nickname: Some(nickname.clone()),
    password: Some(password.clone()),
    server: Some(server),
    channels: Some(vec![channel]),
    ..mirc::Config::default()
  }
}

fn main() {
  let space_quota = 4096;
  let time_quota = 4096;
  let image_path = std::env::var("EQ_CONTAINER").unwrap();
  let mut container = eq::Container::from_image(
    &image_path, space_quota, time_quota).unwrap();
  let mut reactor = mirc::IrcReactor::new().unwrap();
  let config = irc_config();
  let client = reactor.prepare_client_and_connect(&config).unwrap();
  client.identify().unwrap();
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
