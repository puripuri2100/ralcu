pub type Env<T> = Vec<(String, T)>;

pub fn empty<T>() -> Env<T> {
  Vec::new()
}

pub fn extend<T>(x: String, v: T, env: Env<T>) -> Env<T> {
  let mut mut_env = env;
  mut_env.push((x, v));
  mut_env
}

pub fn lookup<T>(x: String, env: Env<T>) -> Option<T>
where
  T: Clone,
{
  let mut env_mut = env;
  env_mut.reverse();
  let a = env_mut.iter().find(|(y, _)| &x == y);
  match a {
    None => None,
    Some((_, v)) => Some(v.clone()),
  }
}
