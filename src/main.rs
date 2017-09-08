extern crate regex;

mod powerline;
mod segments;
mod part;
mod color;

use segments::*;
use part::*;

fn main() {
    let mut prompt = powerline::Powerline::new();
    prompt.add_segments(user::User::new().get_segments().expect("Failed seg: User"));
    prompt.add_segments(host::Host::new().get_segments().expect("Failed seg: Host"));
    prompt.add_segments(cwd::Cwd::new("~").get_segments().expect("Failed seg: Cwd"));
    prompt.add_segments(git::GitInfo::new().get_segments().expect("Failed seg: Git"));
    prompt.add_segments(cmd::Cmd::new("\\$").get_segments().expect("Failed seg: Cmd"));
    println!("{}", prompt);
}