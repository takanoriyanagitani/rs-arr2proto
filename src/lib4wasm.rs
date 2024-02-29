use std::sync::RwLock;

pub fn read_opt<T, U, F>(o: &RwLock<Option<T>>, f: F) -> Result<U, &'static str>
where
    F: Fn(&T) -> Result<U, &'static str>,
{
    let guard = o.try_read().map_err(|_| "unable to read lock")?;
    let ot: &Option<T> = &guard;
    let t: &T = ot.as_ref().ok_or("no data")?;
    f(t)
}

pub fn write_opt<T, U, F>(o: &RwLock<Option<T>>, f: F) -> Result<U, &'static str>
where
    F: Fn(&mut T) -> Result<U, &'static str>,
{
    let mut guard = o.try_write().map_err(|_| "unable to write lock")?;
    let ot: &mut Option<T> = &mut guard;
    let t: &mut T = ot.as_mut().ok_or("no data")?;
    f(t)
}

pub fn write_opt_init<T, U, F, I>(o: &RwLock<Option<T>>, f: F, init: I) -> Result<U, &'static str>
where
    F: Fn(&mut T) -> Result<U, &'static str>,
    I: Fn() -> T,
{
    let mut guard = o.try_write().map_err(|_| "unable to write lock")?;
    let ot: &mut Option<T> = &mut guard;
    match ot {
        None => {
            let mut t: T = init();
            let u: U = f(&mut t)?;
            ot.replace(t);
            Ok(u)
        }
        Some(t) => {
            let mt: &mut T = t;
            f(mt)
        }
    }
}
