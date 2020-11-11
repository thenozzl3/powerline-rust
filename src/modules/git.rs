use std::{env, marker::PhantomData, path, path::PathBuf};

use super::Module;
use crate::{terminal::Color, Segment, R};

#[cfg(not(feature = "libgit"))]
mod process;
#[cfg(not(feature = "libgit"))]
use process as internal;

#[cfg(feature = "libgit")]
mod libgit;
#[cfg(feature = "libgit")]
use libgit as internal;

pub struct Git<S> {
  scheme: PhantomData<S>,
}

#[derive(Clone)]
pub struct GitStats {
  pub untracked: u32,
  pub conflicted: u32,
  pub non_staged: u32,
  pub ahead: u32,
  pub behind: u32,
  pub staged: u32,
  pub branch_name: String,
  pub stash_count: u32,
}

pub trait GitScheme {
  const GIT_AHEAD_BG: Color;
  const GIT_AHEAD_FG: Color;
  const GIT_BEHIND_BG: Color;
  const GIT_BEHIND_FG: Color;
  const GIT_STAGED_BG: Color;
  const GIT_STAGED_FG: Color;
  const GIT_NOTSTAGED_BG: Color;
  const GIT_NOTSTAGED_FG: Color;
  const GIT_UNTRACKED_BG: Color;
  const GIT_UNTRACKED_FG: Color;
  const GIT_CONFLICTED_BG: Color;
  const GIT_CONFLICTED_FG: Color;
  const GIT_REPO_CLEAN_BG: Color;
  const GIT_REPO_CLEAN_FG: Color;
  const GIT_REPO_DIRTY_BG: Color;
  const GIT_REPO_DIRTY_FG: Color;
}

impl<S: GitScheme> Git<S> {
  pub fn new() -> Git<S> {
    Git { scheme: PhantomData }
  }

  pub fn get_git_data(&mut self, path: PathBuf) -> R<GitStats> {
    internal::run_git(&path)
  }
}

impl GitStats {
  pub fn is_dirty(&self) -> bool {
    (self.untracked + self.conflicted + self.staged + self.non_staged) > 0
  }
}

fn find_git_dir() -> Option<path::PathBuf> {
  let mut git_dir = env::current_dir().unwrap();
  loop {
    git_dir.push(".git/");

    if git_dir.exists() {
      git_dir.pop();
      return Some(git_dir);
    }
    git_dir.pop();

    if !git_dir.pop() {
      return None;
    }
  }
}

impl<S: GitScheme> Module for Git<S> {
  fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
    let git_dir = match find_git_dir() {
      Some(dir) => dir,
      _ => return Ok(()),
    };

    let stats = self.get_git_data(git_dir)?;

    let (_branch_fg, _branch_bg) = if stats.is_dirty() {
      (S::GIT_REPO_DIRTY_FG, S::GIT_REPO_DIRTY_BG)
    } else {
      (S::GIT_REPO_CLEAN_FG, S::GIT_REPO_CLEAN_BG)
    };

    let mut full_str = format!(" \u{E0A0} {} ", stats.branch_name);

    let mut add_elem = |count, symbol| {
      if count >= 1 {
        full_str = format!("{}{}{} ",full_str, count, symbol);
      }
    };

    add_elem(stats.ahead, '\u{2B06}');
    add_elem(stats.behind, '\u{2B07}');
    add_elem(stats.staged, '\u{2714}');
    add_elem(stats.non_staged, '\u{270E}');
    add_elem(stats.untracked, '\u{2026}');
    add_elem(stats.conflicted, '\u{273C}');
    add_elem(stats.stash_count, '\u{2691}');
    segments.push(Segment::simple(full_str, S::GIT_NOTSTAGED_FG, S::GIT_NOTSTAGED_BG));

    Ok(())
  }
}
