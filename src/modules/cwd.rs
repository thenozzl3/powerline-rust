use std::{env, marker::PhantomData, path};
use super::Module;
use crate::{terminal::Color, Segment, R};

pub struct Cwd<S: CwdScheme> {
  max_length: usize,
  wanted_seg_num: usize,
  resolve_symlinks: bool,
  scheme: PhantomData<S>,
}

pub trait CwdScheme {
  const CWD_FG: Color;
  const PATH_FG: Color;
  const PATH_BG: Color;
  const HOME_FG: Color;
  const HOME_BG: Color;
  const SEPARATOR_FG: Color;
  const CWD_HOME_SYMBOL: &'static str = "\u{2302}";
}

impl<S: CwdScheme> Cwd<S> {
  pub fn new(max_length: usize, wanted_seg_num: usize, resolve_symlinks: bool) -> Cwd<S> {
    Cwd { max_length, wanted_seg_num, resolve_symlinks, scheme: PhantomData }
  }
}

macro_rules! append_cwd_segments {
  ($segments: ident, $last: ident, $iter: expr) => {
    let mut s_val=String::new();

    while let Some(val) = $iter.next() {

      if  $iter.peek().is_none() {
        $last = val ;
        continue;
      }

        s_val = val.to_string();
        if val.len() >= 6 {s_val = format!("{}\u{2026}",&val[0..5]);}
      $segments.push(Segment::special(
        format!("{}", s_val),
        S::PATH_FG,
        S::PATH_BG,
        //'\u{E0B1}',
        '/',
        S::SEPARATOR_FG,
      ));
    }

  };
}

impl<S: CwdScheme> Module for Cwd<S> {
  fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
    let current_dir =
      if self.resolve_symlinks { env::current_dir()? } else { path::PathBuf::from(env::var("PWD")?) };

    let mut cwd = current_dir.to_str().unwrap();


    if let Some(home_path) = env::home_dir() {
      let home_str = home_path.to_str().unwrap();

      if cwd.starts_with(home_str) {
        segments.push(Segment::simple(format!(" {} ", S::CWD_HOME_SYMBOL), S::HOME_FG, S::HOME_BG));
        if cwd == home_str {return Ok(());}
        cwd = &cwd[home_str.len() + 1 ..];
      }

      if cwd == "/" {
        segments.push(Segment::simple(format!(" {} ", "/"), S::HOME_FG, S::HOME_BG));
        return Ok(());
      }
    }

    //let short_cwd = String::new();
    let mut end = "";


    segments.push(Segment::special(
        "",
        S::PATH_FG,
        S::PATH_BG,
        ' ',
        S::SEPARATOR_FG,
      ));
    append_cwd_segments!(segments, end,  cwd.split("/").peekable());

    segments.push(Segment::simple(format!("{}", end), S::PATH_FG, S::PATH_BG));
    segments.push(Segment::special(
        "",
        S::PATH_FG,
        S::PATH_BG,
        ' ',
        S::SEPARATOR_FG,
      ));

/*
    let depth = cwd.matches('/').count();
    if (cwd.len() > self.max_length as usize) && (depth > self.wanted_seg_num) {
      let left = self.wanted_seg_num / 2;
      let right = self.wanted_seg_num - left;

      let start = cwd.split('/').skip(1).take(left);
      let end = cwd.split('/').skip(depth - right + 1);

      append_cwd_segments!(segments, start);
      segments.push(Segment::special(
        " \u{2026} ",
        S::PATH_FG,
        S::PATH_BG,
        '\u{E0B1}',
        S::SEPARATOR_FG,
      ));
      append_cwd_segments!(segments, end);
    } else {
      append_cwd_segments!(segments, cwd.split('/').skip(1));
    };*/

    // todo get rid of me
    if let Some(last) = segments.last_mut() {
      last.fg = S::CWD_FG.into_fg();
      last.sep = '\u{E0B0}';
      last.sep_col = last.bg.transpose();
    }

    Ok(())
  }
}
