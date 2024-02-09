struct A;
struct str;
struct u8;
trait From<T> {}
trait IntoIterator<I> {}
trait AsRef<T> {}

impl<S: AsRef<str>, I: IntoIterator<S>> From<I> for A {}
impl<S: AsRef<str>, I: IntoIterator<str> - IntoIterator<S>> From<I> for A {}
impl<S: AsRef<u8> - AsRef<str>, I: IntoIterator<S>> From<I> for A {}
