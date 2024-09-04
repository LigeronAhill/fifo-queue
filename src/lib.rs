use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
#[derive(Debug)]
pub struct Queue<T> {
    data: Arc<Mutex<Vec<T>>>,
}
impl<T> Default for Queue<T> {
    fn default() -> Self {
        Queue {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
impl<T> Queue<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&self, item: T) -> Result<(), Error> {
        let arc = self.data.clone();
        let mut temp = arc.lock()?;
        temp.push(item);
        Ok(())
    }
    pub fn pop(&self) -> Result<Option<T>, Error> {
        let arc = self.data.clone();
        let mut temp = arc.lock()?;
        if temp.is_empty() {
            return Ok(None);
        }
        let result = temp.remove(0);
        Ok(Some(result))
    }
    pub fn is_empty(&self) -> Result<bool, Error> {
        let arc = self.data.clone();
        let temp = arc.lock()?;
        Ok(temp.is_empty())
    }
    pub fn clear(&self) -> Result<(), Error> {
        let arc = self.data.clone();
        let mut temp = arc.lock()?;
        temp.clear();
        Ok(())
    }
}
#[derive(Debug)]
pub enum Error {
    LockError,
}
impl<T> From<PoisonError<MutexGuard<'_, Vec<T>>>> for Error {
    fn from(_value: PoisonError<MutexGuard<Vec<T>>>) -> Self {
        Self::LockError
    }
}
#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;
    #[test]
    fn push_test() -> Result<(), Error> {
        let q = Queue::new();
        q.push("first")?;
        q.push("second")?;
        let want = vec!["first", "second"];
        let got = q.data.deref().lock()?.deref().clone();
        assert_eq!(want, got);
        Ok(())
    }
    #[test]
    fn pop_test() -> Result<(), Error> {
        let q = Queue::new();
        q.push("first")?;
        q.push("second")?;
        q.push("third")?;
        q.push("fourth")?;
        let want = Some("first");
        let got = q.pop()?;
        assert_eq!(want, got);
        let want = vec!["second", "third", "fourth"];
        let got = q.data.deref().lock()?.deref().clone();
        assert_eq!(want, got);
        Ok(())
    }
    #[test]
    fn is_empty_test() -> Result<(), Error> {
        let q = Queue::new();
        q.push("test")?;
        let _ = q.pop()?;
        assert!(q.is_empty()?);
        Ok(())
    }
    #[test]
    fn clear_test() -> Result<(), Error> {
        let q = Queue::new();
        q.push("first")?;
        q.push("second")?;
        q.clear()?;
        assert!(q.is_empty()?);
        Ok(())
    }
}
