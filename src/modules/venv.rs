use std::{env, marker::PhantomData, path::Path};

use super::Module;
use crate::{R, powerline::Segment, terminal::Color};

pub struct VirtualEnv<S: VirtualEnvScheme> {
    scheme: PhantomData<S>,
}

pub trait VirtualEnvScheme {
    const PYVENV_FG: Color;
    const PYVENV_BG: Color;
}

impl<S: VirtualEnvScheme> VirtualEnv<S> {
    pub fn new() -> VirtualEnv<S> {
        VirtualEnv { scheme: PhantomData }
    }
}

impl<S: VirtualEnvScheme> Module for VirtualEnv<S> {
    fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
        let venv = env::var("VIRTUAL_ENV")
            .or(env::var("CONDA_ENV_PATH"))
            .or(env::var("CONDA_DEFAULT_ENV"));

        if let Ok(venv_path) = venv {
            //  file_name is always some, because env variable is a valid directory path.
            let venv_name =
                Path::new(&venv_path).file_name().unwrap().to_string_lossy();

            segments.push(Segment::simple(
                format!(" {} ", venv_name),
                S::PYVENV_FG,
                S::PYVENV_BG,
            ));
        }

        Ok(())
    }
}
