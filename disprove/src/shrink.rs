/// Returns an iterator that yields no values.
///
/// Used as the default shrink implementation for types that don't
/// support shrinking.
pub fn empty_shrinker<T: 'static>() -> Box<dyn Iterator<Item = T>> {
    Box::new(std::iter::empty())
}

/// Chain two shrink iterators together.
#[allow(dead_code)]
pub fn chain<T: 'static>(
    a: Box<dyn Iterator<Item = T>>,
    b: Box<dyn Iterator<Item = T>>,
) -> Box<dyn Iterator<Item = T>> {
    Box::new(a.chain(b))
}
