extern crate eq;
extern crate irc;

pub use irc::client::prelude as mirc;
use irc::client::Client;
use irc::client::ext::ClientExt;

fn main() {
  let server = std::env::var(
    "EQ_IRC_SERVER").expect("server").to_owned();
  let nickname = std::env::var(
    "EQ_IRC_NICKNAME").expect("nickname").to_owned();
  let password = std::env::var(
    "EQ_IRC_PASSWORD").expect("password").to_owned();
  let channel = std::env::var(
    "EQ_IRC_CHANNEL").expect("channel").to_owned();
  let command_prefix = format!("{}: ", &nickname);
  let config = mirc::Config {
    nickname: Some(nickname.clone()),
    password: Some(password.clone()),
    server: Some(server),
    channels: Some(vec![channel]),
    ..mirc::Config::default()
  };
  let mut reactor = mirc::IrcReactor::new().expect("reactor");
  let client = reactor.prepare_client_and_connect(&config).expect("connect");
  client.identify().expect("identify");
  reactor.register_client_with_handler(client, move |client, message| {
    print!("{}", message);
    let response_target = message.response_target().map(|x| x.to_string());
    match message.command {
      mirc::Command::PRIVMSG(_, message_body) => {
        if message_body.starts_with(&command_prefix) {
          let source = message_body.trim_left_matches(&command_prefix);
          let mut target = String::from("=> ");
          let space_quota = 65536;
          let time_quota = 65536;
          let command;
          let response_target = response_target.expect("response");
          println!("[EVAL] {}", source);
          if eq::eval(
            &source, &mut target, space_quota, time_quota).is_ok() {
            println!("[EVAL] {}", &target);
            command = mirc::Command::PRIVMSG(response_target, target);
          } else {
            command = mirc::Command::PRIVMSG(
              response_target,
              "there is no knowledge that is not power".to_string());
          }
          client.send(command)?;
        }
      }
      _ => {}
    }
    return Ok(());
  });
  reactor.run().expect("run");
}
