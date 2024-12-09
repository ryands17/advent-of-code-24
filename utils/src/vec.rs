use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
pub struct MyVec<T>(pub Vec<T>);

impl<T> Deref for MyVec<T> {
  type Target = Vec<T>;
  fn deref(&self) -> &Vec<T> {
    &self.0
  }
}

impl<T> DerefMut for MyVec<T> {
  fn deref_mut(&mut self) -> &mut Vec<T> {
    &mut self.0
  }
}

impl<T> Index<isize> for MyVec<T> {
  type Output = T;
  fn index(&self, index: isize) -> &T {
    if index < 0 || (index as usize) >= self.len() {
      panic!("Index out of bound!");
    }
    &self.0[index as usize]
  }
}

impl<T> IndexMut<isize> for MyVec<T> {
  fn index_mut(&mut self, index: isize) -> &mut T {
    if index < 0 || (index as usize) >= self.len() {
      panic!("Index out of bound!");
    }
    &mut self.0[index as usize]
  }
}

impl<T> IntoIterator for MyVec<T> {
  type Item = T;
  type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<T> From<Vec<T>> for MyVec<T> {
  fn from(value: Vec<T>) -> Self {
    Self(value)
  }
}
