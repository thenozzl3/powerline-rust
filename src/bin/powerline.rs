use powerline::{modules::*, theme::SimpleTheme};

#[cfg(feature = "time")]
//use powerline::modules::Time;
use powerline::modules::Time;

fn main() -> powerline::R<()> {
    let mut prompt = powerline::Powerline::new();
    prompt.add_module(User::<SimpleTheme>::new())?;
    prompt.add_module(Host::<SimpleTheme>::new())?;

    //println!("{}", ps1_length);
    prompt.add_module(Cwd::<SimpleTheme>::new(false))?;
    prompt.add_module(Git::<SimpleTheme>::new())?;
    //prompt.add_module(ReadOnly::<SimpleTheme>::new())?;
    prompt.add_module(VirtualEnv::<SimpleTheme>::new())?;
    prompt.add_module(ExitCode::<SimpleTheme>::new())?;
    prompt.add_module(Jobs::<SimpleTheme>::new())?;

    println!("{}", prompt);
    Ok(())
}
