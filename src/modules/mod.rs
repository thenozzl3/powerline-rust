use crate::{powerline::Segment, R};

mod cmd;
mod cwd;
mod exit_code;
mod git;
mod host;
mod readonly;
mod user;
mod venv;
mod jobs;

#[cfg(feature = "time")]
mod time;

pub use cmd::{Cmd, CmdScheme};
pub use cwd::{Cwd, CwdScheme};
pub use exit_code::{ExitCode, ExitCodeScheme};
pub use git::{Git, GitScheme};
pub use host::{Host, HostScheme};
pub use readonly::{ReadOnly, ReadOnlyScheme};
pub use user::{User, UserScheme};
pub use venv::{VirtualEnv, VirtualEnvScheme};
pub use jobs::{Jobs, JobsScheme};

#[cfg(feature = "time")]
pub use time::{Time, TimeScheme};

pub trait Module: Sized {
  fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()>;

  #[inline]
  fn into_segments(mut self) -> R<Vec<Segment>> {
    self.get_segments()
  }

  #[inline]
  fn get_segments(&mut self) -> R<Vec<Segment>> {
    let mut vec = Vec::new();

    self.append_segments(&mut vec).map(|_| vec)
  }
}
