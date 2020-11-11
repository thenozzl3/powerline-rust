use std::env;
use std::str;

pub fn is_remote_shell() -> bool {
  env::var_os("SSH_CLIENT").is_some()
    || env::var_os("SSH_TTY").is_some()
    || env::var_os("SSH_CONNECTION").is_some()
}

pub fn get_jobs() -> usize {

  match str::parse::<usize>(match env::var("JOBS"){
     Ok(job_string) => job_string,
     Err(_) => "0".to_string()

   }.trim()) {

     Ok(job_num) => job_num,
     Err(_) => 0
  }

}
