struct A;

trait TA {}
trait From<T> {}

impl From<A> for A {}
impl<T: TA> From<T> for A {}
