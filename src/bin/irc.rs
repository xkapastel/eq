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
  let server = std::env::var("SUNDIAL_IRC_SERVER").unwrap();
  let nickname = std::env::var("SUNDIAL_IRC_NICKNAME").unwrap();
  let password = std::env::var("SUNDIAL_IRC_PASSWORD").unwrap();
  let channel = std::env::var("SUNDIAL_IRC_CHANNEL").unwrap();
  mirc::Config {
    nickname: Some(nickname),
    password: Some(password),
    server: Some(server),
    channels: Some(vec![channel]),
    ..mirc::Config::default()
  }
}

fn main() {
  let space_quota = 65536;
  let time_quota = 4096;
  let mut pod = sundial::Pod::default(space_quota, time_quota).unwrap();
  let mut reactor = mirc::IrcReactor::new().unwrap();
  let config = irc_config();
  let client = reactor.prepare_client_and_connect(&config).unwrap();
  client.identify().unwrap();
  let nickname = config.nickname.unwrap().to_string();
  let command_prefix = format!("{}: ", &nickname);
  reactor.register_client_with_handler(client, move |client, message| {
    print!("{}", message);
    let response_target = message.response_target().map(|x| x.to_string());
    match message.command {
      mirc::Command::PRIVMSG(message_target, message_body) => {
        let flag = &message_target == &nickname;
        let flag = flag || message_body.starts_with(&command_prefix);
        if flag {
          let response_target = response_target.unwrap();
          let source = message_body
            .trim_left_matches(&command_prefix).trim();
          if let Ok(target) = pod.eval(&source, time_quota) {
            let command = mirc::Command::PRIVMSG(response_target, target);
            client.send(command)?;
          }
        }
      }
      _ => {}
    }
    return Ok(());
  });
  reactor.run().unwrap();
}
