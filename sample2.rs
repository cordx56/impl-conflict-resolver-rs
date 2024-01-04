trait TP<P> {}

trait TA {}
trait TB {}
struct A;
impl<P: TA> TP<P> for A {}
impl<P: TB - TA> TP<P> for A {}
