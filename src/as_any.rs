use std::any::Any;

/// This trait unfortunally is necessary because of casting between
/// trait objects and Sized structures/enums.
/// Example:
/// ```
/// use scoolite::AsAny;
/// use std::any::Any;
///
/// trait A: AsAny {
///     fn do_stuff(&self);
/// }
///
/// struct B;
///
/// impl A for B {
///     fn do_stuff(&self) {
///         println!("stuff!!");
///     }
/// }
///
/// impl AsAny for B {
///     fn as_any(&self) -> &Any {
///         self
///     }
/// }
///
///
/// let a: Box<A> = Box::new(B);
/// let any: &Any = &a;
/// let b = any.downcast_ref::<B>();// doesn't work :(
/// assert_eq!(b.is_none(), true);
///
/// let b = a.as_any().downcast_ref::<B>().unwrap();// works! :tada:
/// ```
pub trait AsAny {
    fn as_any(&self) -> &Any;
}
