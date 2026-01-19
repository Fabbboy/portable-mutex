use portable_mutex::prelude::*;
use std::{
  sync::Arc,
  thread,
};

#[test]
fn mutex_concurrent_increment() {
  let mutex = Arc::new(Mutex::new(0));

  let mut handles = Vec::new();

  let workers = 10;
  let tasksper = 10_000;
  let total = workers * tasksper;

  for _ in 0..workers {
    let mutex = Arc::clone(&mutex);
    handles.push(thread::spawn(move || {
      for _ in 0..tasksper {
        let mut guard = mutex.lock();
        *guard += 1;
      }
    }));
  }

  for handle in handles {
    handle.join().unwrap();
  }

  let final_count = *mutex.lock();
  assert_eq!(final_count, total);
}
