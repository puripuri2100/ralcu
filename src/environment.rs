pub type Env<T> = Vec<(String, T)>;

pub fn empty<T>() -> Env<T> {
  Vec::new()
}

pub fn extend<T>(x: String, v: T, mut env: Env<T>) -> Env<T> {
  env.push((x, v));
  env
}

pub fn lookup<T>(x: String, env: Env<T>) -> Option<T>
where
  T: Clone,
{
  let mut env_mut = env;
  env_mut.reverse();
  let a = env_mut.iter().find(|(y, _)| &x == y);
  a.map(|(_, v)| v.clone())
}

pub fn remove<T>(id: &str, env: Env<T>) -> Env<T> {
  let mut l = Vec::new();
  let mut e = env;
  let mut is_find = false;
  while let Some(t) = e.pop() {
    let (t_id, _) = &t;
    if t_id == id && !is_find {
      is_find = true
    } else {
      l.push(t);
    }
  }
  l.reverse();
  l
}
