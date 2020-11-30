use std::{env, marker::PhantomData, path};
use super::Module;
use crate::{terminal::Color, terminal::Reset, Segment, utils, R};

pub struct Cwd<S: CwdScheme> {
  starting_pos: usize,
  //wanted_seg_num: usize,
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
  //pub fn new(max_length: usize, wanted_seg_num: usize, resolve_symlinks: bool) -> Cwd<S> {
  pub fn new(resolve_symlinks: bool, starting_pos: usize) -> Cwd<S> {
   // Cwd { max_length, wanted_seg_num, resolve_symlinks, scheme: PhantomData }
    Cwd { starting_pos , resolve_symlinks, scheme: PhantomData }
  }
}

macro_rules! append_cwd_segments {
  ($self:ident,
   $segments: ident,
   $last: ident,
   $cols: ident,
   $start: ident,
   $iter: expr) => {
    let mut some_iter = $iter;
    let mut cwd_len = 0 ;
    let mut starting_pos = $self.$start;
    let mut length_ahead: usize ;

    while let Some(val) = some_iter.next() {

      //look ahead one ..
      if let Some(next_val) = some_iter.peek() {
        let path_ahead = next_val.to_os_string().into_string().unwrap();
        // Since we are shortening path components to 7 chars
        length_ahead = match path_ahead.len() {
          x if x >= 6 => 7,
          x if x < 6 => x,
          _ => 0
        };
        //Ugly . Whatever. Have to look ahead to CR into the next line
        if cwd_len + starting_pos + 6 + length_ahead + 2 >= $cols {
          //println!("brk at {} {}", cwd_len + $self.$start + 4, $cols);
          $segments.push(Segment::special(
            format!("{}\\n", Reset),
            S::PATH_FG,
            S::PATH_BG,
            //'\u{E0B1}',
            ' ',
            S::SEPARATOR_FG,
          ));
          cwd_len = 0;
          // set to 0 after 1st iteration
          starting_pos = 0;
        }
      } else  {
        //ugly AAF . Whatever. Last segment must be preceeded by a
        //CR if it happens to fall at the column boundry ..
        if cwd_len + starting_pos + 6 + val.len() + 2 >= $cols {
          //println!("last brk at {} {}", cwd_len + $self.$start + 4, $cols);
          $segments.push(Segment::special(
            format!("{}\\n", Reset),
            S::PATH_FG,
            S::PATH_BG,
            //'\u{E0B1}',
            ' ',
            S::SEPARATOR_FG,
          ));
        }

        $last = &val.to_str().unwrap();
        continue;
      }

      let mut s_val = val.to_os_string().into_string().unwrap();

      if s_val.len() >= 6 {
        s_val = format!("{}\u{2026}",&s_val[0..5]);
        cwd_len += 7;
      }
      else { cwd_len += (s_val.len() + 1);}
        //println!("cwd len {} {}", cwd_len + $self.$start + 4, $cols);
        //println!("cwd len {} {}", $self.$start , $cols);

      $segments.push(Segment::special(
        format!("{}", s_val),
        S::PATH_FG,
        S::PATH_BG,
        //'\u{E0B1}',
        '/',
        S::SEPARATOR_FG,
      ));
   }}
}

impl<S: CwdScheme> Module for Cwd<S> {
  fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
    let mut skip_cwd_components = 1;
    let current_dir = if self.resolve_symlinks { env::current_dir()? }
      else { path::PathBuf::from(env::var("PWD")?) };
    let cwd = current_dir.to_str().unwrap();
    if let Some(home_path) = env::home_dir() {
      let home_str = home_path.to_str().unwrap();

      if cwd.starts_with(home_str) {
        segments.push(Segment::simple(format!(" {} ", S::CWD_HOME_SYMBOL), S::HOME_FG, S::HOME_BG));
        if cwd == home_str {return Ok(());}
        skip_cwd_components += 2;
      } else if cwd == "/" {
        segments.push(Segment::simple(format!(" {} ", "/"), S::HOME_FG, S::HOME_BG));
        return Ok(());
      } else {
        segments.push(Segment::simple(format!(" {} ", "/"), S::HOME_FG, S::HOME_BG));
      }
    }

    let mut end = "";
    segments.push(Segment::special(
      "",
      S::PATH_FG,
      S::PATH_BG,
      ' ',
      S::SEPARATOR_FG,
    ));

    let columns = utils::get_cols();

    append_cwd_segments!(self,
                         segments,
                         end,
                         columns,
                         starting_pos,
                         current_dir.iter().skip(skip_cwd_components).peekable());

    segments.push(Segment::special(
      format!("{}", end),
      S::PATH_FG,
      S::PATH_BG,
      ' ',
      S::SEPARATOR_FG,
    ));

    segments.push(Segment::special(
      "",
      S::PATH_FG,
      S::PATH_BG,
      ' ',
      S::SEPARATOR_FG,
    ));

    // todo get rid of me
    if let Some(last) = segments.last_mut() {
      last.fg = S::CWD_FG.into_fg();
      last.sep = '\u{E0B0}';
      last.sep_col = last.bg.transpose();
    }

    Ok(())
  }
}






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
