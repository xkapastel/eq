// This file is a part of Sundial.
// Copyright (C) 2018 Matthew Blount

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public
// License along with this program.  If not, see
// <https://www.gnu.org/licenses/.

extern crate sundial;
extern crate irc;

pub use irc::client::prelude as mirc;
use irc::client::Client;
use irc::client::ext::ClientExt;

fn irc_config() -> mirc::Config {
  let server = std::env::var("SUNDIAL_IRC_SERVER").unwrap().to_owned();
  let nickname = std::env::var("SUNDIAL_IRC_NICKNAME").unwrap().to_owned();
  let password = std::env::var("SUNDIAL_IRC_PASSWORD").unwrap().to_owned();
  let channel = std::env::var("SUNDIAL_IRC_CHANNEL").unwrap().to_owned();
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
  let image_path = std::env::var("SUNDIAL_CONTAINER").unwrap();
  let mut container = sundial::Container::from_image(
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
