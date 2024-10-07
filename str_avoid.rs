struct A;
struct str;
struct andstr;
trait From<S> {}
trait AsRef<T> {}

impl<S: AsRef<str>>
    From<S> for A {}
impl<S: AsRef<andstr>>
    From<S> for A {}
impl<S: AsRef<str> - AsRef<andstr>>
    From<S> for A {}
