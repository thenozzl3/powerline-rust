use std::marker::PhantomData;

use super::Module;
use crate::{terminal::Color, utils, Segment, R};

pub struct Jobs<S: JobsScheme> {
  scheme: PhantomData<S>,
}

pub trait JobsScheme {
  const HOSTNAME_FG: Color;
  const HOSTNAME_BG: Color;
}

impl<S: JobsScheme> Jobs<S> {
  pub fn new() -> Jobs<S> {
    Job { scheme: PhantomData }
  }
}

impl<S: JobsScheme> Module for Jobs<S> {
  fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
    let job_count = utils::get_jobs();
    if  job_count >= 1
      segments.push(Segment::simple(
        format!(" {} ", job_count),
        S::HOSTNAME_FG,
        S::HOSTNAME_BG,
      ));
  }
  Ok(())
}
