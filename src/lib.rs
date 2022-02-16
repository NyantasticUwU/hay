//! A growable and shrinkable stack array type.
#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]
extern crate alloc;
use alloc::vec::Vec;

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
    #[inline(always)]
    pub const fn new() -> Self {
        Self { vec: Vec::new() }
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
