use super::Module;
use crate::{R, Segment, terminal::Color};
use std::{env, marker::PhantomData, path};

pub struct Cwd<S: CwdScheme> {
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
    pub fn new(resolve_symlinks: bool) -> Cwd<S> {
        // Cwd { max_length, wanted_seg_num, resolve_symlinks, scheme: PhantomData }
        Cwd { resolve_symlinks, scheme: PhantomData }
    }
}

macro_rules! append_cwd_segments {
    ($self:ident,
   $segments: ident,
   $iter: expr) => {
        let mut some_iter = $iter;

        while let Some(val) = some_iter.next() {
            let mut s_val = val.to_os_string().into_string().unwrap();
            if let Some(_) = some_iter.peek() {
                if s_val.len() >= 6 {
                    s_val = format!("{}\u{2026}", &s_val[0..5]);
                }

                $segments.push(Segment::special(
                    format!("{}", s_val),
                    S::PATH_FG,
                    S::PATH_BG,
                    '/',
                    S::SEPARATOR_FG,
                ));
            } else {
                $segments.push(Segment::special(
                    format!("{} ", s_val),
                    S::PATH_FG,
                    S::PATH_BG,
                    '\u{E0B0}',
                    S::PATH_BG,
                ));
            }
        }
    };
}

impl<S: CwdScheme> Module for Cwd<S> {
    fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
        // skip the root of the FS .
        let mut skip_cwd_components = 1;
        let current_dir = if self.resolve_symlinks {
            env::current_dir()?
        } else {
            path::PathBuf::from(env::var("PWD")?)
        };
        let cwd = current_dir.to_str().unwrap();
        if let Some(home_path) = env::home_dir() {
            let home_str = home_path.to_str().unwrap();

            if cwd.starts_with(home_str) {
                segments.push(Segment::simple(
                    format!(" {} ", S::CWD_HOME_SYMBOL),
                    S::HOME_FG,
                    S::HOME_BG,
                ));
                if cwd == home_str {
                    return Ok(());
                }
                skip_cwd_components += 2;
            } else if cwd == "/" {
                segments.push(Segment::simple(
                    format!(" {} ", "/"),
                    S::HOME_FG,
                    S::HOME_BG,
                ));
                return Ok(());
            } else {
                segments.push(Segment::simple(
                    format!(" {} ", "/"),
                    S::HOME_FG,
                    S::HOME_BG,
                ));
            }
        }

        segments.push(Segment::special(
            "",
            S::PATH_FG,
            S::PATH_BG,
            ' ',
            S::SEPARATOR_FG,
        ));

        append_cwd_segments!(
            self,
            segments,
            current_dir.iter().skip(skip_cwd_components).peekable()
        );

        Ok(())
    }
}
