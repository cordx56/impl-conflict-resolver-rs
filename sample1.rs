struct u8;
struct u16;
struct A;
trait From<I> {}
trait IntoIterator<I> {}

impl<I: IntoIterator<u8>> From<I> for A {}
impl<I: IntoIterator<u16>> From<I> for A {}
impl<I: IntoIterator<u8> - IntoIterator<u16>> From<I> for A {}
impl<I: IntoIterator<u16> - IntoIterator<u8>> From<I> for A {}
