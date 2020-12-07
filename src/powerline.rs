use std::fmt;
//use std::env;

use crate::{modules::Module, terminal::*, R, utils};

#[derive(Clone)]
pub struct Segment {
  pub val: String,
  pub fg: FgColor,
  pub bg: BgColor,
  pub sep: char,
  pub sep_col: FgColor,
}

impl Segment {
  pub fn simple<S: Into<String>>(val: S, fg: Color, bg: Color) -> Segment {
    Segment {
      val: val.into(),
      fg: fg.into_fg(),
      bg: bg.into_bg(),
      sep: '\u{E0B0}',
      sep_col: bg.into_fg(),
    }
  }

  pub fn special<S: Into<String>>(val: S, fg: Color, bg: Color, sep: char, sep_col: Color) -> Segment {
    Segment { val: val.into(),
               fg: fg.into_fg(),
               bg: bg.into_bg(),
               sep,
               sep_col: sep_col.into_fg() }
  }
}

pub struct Powerline {
  segments: Vec<Segment>,
}

impl Powerline {
  pub fn new() -> Powerline {
    Powerline { segments: Vec::new() }
  }

  pub fn add_module(&mut self, mut part: impl Module) -> R<()> {
    part.append_segments(&mut self.segments)
  }

  pub fn add_segments(&mut self, new_segments: Vec<Segment>) {
    self.segments.extend(new_segments);
  }

  pub fn length(&mut self) -> usize {
    self.segments.iter().fold(0, |acc, segment| acc + segment.val.len())
  }
}

impl fmt::Display for Powerline {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // get the display size
    let cols = utils::get_cols();
    let mut next_sep_col = format!("{}", Reset);
    let mut ps_length = 0;
    let mut iter = self.segments.iter().peekable();

    while let Some(seg) = iter.next() {

      ps_length += seg.val.chars().count() + 1 ;

      if ps_length >= cols {
        write!(f, "{}\n", Reset)?;
        ps_length = 0;
      }

      if let Some(next) = iter.peek() {
        if ps_length + next.val.chars().count() + 1 <  cols {
        next_sep_col = format!("{}",next.bg) ;
        } else {
          if seg.sep != '/'  {
            next_sep_col = format!("{}", Reset) ;
          }
        }
      } else {
        next_sep_col = format!("{}", Reset) ;
      }

      write!(f, "{}{}{}{}{}{}", seg.fg, seg.bg, seg.val, next_sep_col, seg.sep_col, seg.sep)?;

    }
    write!(f, "{} ", Reset)
  }
}
