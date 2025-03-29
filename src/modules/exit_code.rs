use std::{env, marker::PhantomData};

use super::Module;
use crate::{R, powerline::Segment, terminal::Color};

pub struct ExitCode<S: ExitCodeScheme> {
    scheme: PhantomData<S>,
}

pub trait ExitCodeScheme {
    const EXIT_CODE_BG: Color;
    const EXIT_CODE_FG: Color;
}

impl<S: ExitCodeScheme> ExitCode<S> {
    pub fn new() -> ExitCode<S> {
        ExitCode { scheme: PhantomData }
    }
}

impl<S: ExitCodeScheme> Module for ExitCode<S> {
    fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
        let exit_string = env::args().nth(1).unwrap_or_else(|| "1".to_string());

        if exit_string != "0" {
            let (fg, bg) = (S::EXIT_CODE_FG, S::EXIT_CODE_BG);
            segments.push(Segment::simple(
                format!(" {} ", exit_string),
                fg,
                bg,
            ));
        }

        Ok(())
    }
}
