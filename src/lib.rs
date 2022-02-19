//! A growable and shrinkable stack array type.
#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]
extern crate alloc;
use alloc::vec::Vec;
use core::{
    iter::Extend,
    ops::{Deref, DerefMut},
};

/// A growable and shrinkable stack array type.
///
/// # Example
/// ```
/// use hay::Stack;
/// let mut stack = Stack::new();
/// stack.push(1);
/// stack.push(2);
/// assert_eq!(stack.len(), 2);
/// assert_eq!(stack.pop(), Some(2));
/// assert_eq!(stack.pop(), Some(1));
/// assert_eq!(stack.pop(), None);
/// ```
/// <b> ~24 bytes on the stack!!! </b>
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Stack<T> {
    vec: Vec<T>,
}
impl<T> Stack<T> {
    /// Constructs a new, empty `Stack<T>`.
    ///
    /// The stack will not allocate until elements are pushed onto it.
    /// # Example
    /// ```
    /// use hay::Stack;
    /// let mut stack: Stack<i32> = Stack::<i32>::new();
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn new() -> Self {
        Self { vec: Vec::new() }
    }

    /// Returns a reference to the top element in the stack.
    ///
    /// This is the most recently pushed element.
    ///
    /// This element will be removed on a call to `pop()`.
    /// # Example
    /// ```
    /// use hay::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(1);
    /// assert_eq!(stack.top(), Some(&1));
    /// stack.pop();
    /// assert_eq!(stack.top(), None);
    /// ```
    #[inline(always)]
    pub fn top(&self) -> Option<&T> {
        self.vec.last()
    }

    /// Returns a mutable reference to the top element in the stack.
    ///
    /// This is the most recently pushed element.
    ///
    /// This element will be removed on a call to `pop()`.
    /// # Example
    /// ```
    /// use hay::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(1);
    /// assert_eq!(stack.top_mut(), Some(&mut 1));
    /// stack.pop();
    /// assert_eq!(stack.top_mut(), None);
    /// ```
    #[inline(always)]
    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.vec.last_mut()
    }

    /// Appends an element to the top of the stack.
    /// # Panics
    /// Panics if the new capacity exceeds `isize::MAX`.
    /// # Example
    /// ```
    /// use hay::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(1);
    /// assert_eq!(stack.pop(), Some(1));
    /// ```
    #[inline(always)]
    pub fn push(&mut self, value: T) {
        self.vec.push(value);
    }

    /// Removes the element at the top of the stack and returns it, or [None] if it is empty.
    /// # Example
    /// ```
    /// use hay::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(1);
    /// assert_eq!(stack.pop(), Some(1));
    /// ```
    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    /// Clears the stack, popping all values.
    ///
    /// Note that this method has no effect on the allocated capacity of the stack.
    /// # Example
    /// ```
    /// use hay::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(1);
    /// stack.push(2);
    /// stack.push(3);
    /// stack.clear();
    /// assert_eq!(stack.pop(), None);
    /// ```
    #[inline(always)]
    pub fn clear(&mut self) {
        self.vec.clear();
    }

    /// Returns the number of elements on the stack, also referred to as it's 'length'.
    /// # Example
    /// ```
    /// use hay::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(1);
    /// stack.push(2);
    /// assert_eq!(stack.len(), 2);
    /// ```
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Extracts a vector containing the entire stack.
    /// # Example
    /// ```
    /// use hay::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(0);
    /// stack.push(1);
    /// stack.push(2);
    /// let slice = unsafe { stack.as_vec().as_slice() };
    /// assert_eq!(slice, &[0, 1, 2]);
    /// ```
    #[inline(always)]
    pub const fn as_vec(&self) -> &Vec<T> {
        &self.vec
    }

    /// Extracts a mutable vector containing the entire stack.
    /// # Example
    /// ```
    /// use hay::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(2);
    /// unsafe { stack.as_mut_vec().insert(0, 1) };
    /// assert_eq!(stack.len(), 2);
    /// assert_eq!(stack.pop(), Some(2));
    /// assert_eq!(stack.pop(), Some(1));
    /// assert_eq!(stack.pop(), None);
    /// ```
    #[inline(always)]
    pub fn as_mut_vec(&mut self) -> &mut Vec<T> {
        &mut self.vec
    }
}
impl<T> Deref for Stack<T> {
    /// The resulting type when dereferencing `Stack<T>`.
    type Target = [T];

    /// Dereferences a `Stack<T>`.
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}
impl<T> DerefMut for Stack<T> {
    /// Mutably dereferences a `Stack<T>`.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}
impl<T> Extend<T> for Stack<T> {
    /// Pushes a collection of values onto a stack.
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        // Iterating through each new value.
        for value in iter {
            // Pushing the value onto the stack.
            // This works because `value` is moved.
            self.push(value);
        }
    }
}
impl<'a, T: 'a + Copy> Extend<&'a T> for Stack<T> {
    /// Pushes a collection of values onto a stack.
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        // Iterating through each new value.
        for value in iter {
            // Pushing the value onto the stack.
            // This works because `T` implements `Copy`.
            self.push(*value);
        }
    }
}
