trait TP<P> {}

trait TA {}
trait TB {}
struct A;
impl<P: TA> TP<P> for A {}
impl<P: TB> TP<P> for A {}
