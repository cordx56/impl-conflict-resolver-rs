struct A;
trait TP<P> {}

trait TA {}
trait TB {}
trait TC: TA {}
impl<P: TC> TP<P> for A {}
impl<P: TB - TA> TP<P> for A {}
