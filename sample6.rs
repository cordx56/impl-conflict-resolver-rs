struct A;
trait TA {}
trait TB {}
trait TC: TA + TB {}
trait TD {}
trait TE {}
trait TF {}
trait TP<P> {}

impl<P: TA> TP<P> for A {}
impl<P: TA + TB> TP<P> for A {}
impl<P: TC> TP<P> for A {}
impl<P: TD - TA> TP<P> for A {}
impl<P: TE + TA - TB> TP<P> for A {}
impl<P: TA, Q: TP<P>> TP<Q> for A {}
impl<P: TB - TA, Q: TP<P>> TP<Q> for A {}
impl TF for A {}
impl TF for A {}
