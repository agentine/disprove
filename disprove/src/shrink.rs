/// Returns an iterator that yields no values.
///
/// Used as the default shrink implementation for types that don't
/// support shrinking.
pub fn empty_shrinker<T: 'static>() -> Box<dyn Iterator<Item = T>> {
    Box::new(std::iter::empty())
}
