struct A;
trait TA {}
trait TB {}
trait TC: TA + TB {}
trait TD {}
trait TP<P> {}
impl<P: TA> TP<P> for A {}
impl<P: TA + TB> TP<P> for A {}
impl<P: TC> TP<P> for A {}
impl<P: TD - TA> TP<P> for A {}
