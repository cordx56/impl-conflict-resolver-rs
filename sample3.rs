struct A;
struct str;
trait AsRef<P> {}
trait From<P> {}
trait IntoIterator<P> {}

impl<S: AsRef<str>> From<S> for A {}
impl<S: AsRef<str>, I: IntoIterator<S> - AsRef<str>> From<I> for A {}
