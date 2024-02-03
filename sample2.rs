struct u8;
struct u16;
struct A;
trait From<I> {}
trait IntoIterator<I> {}

impl<I: IntoIterator<u8>> From<I> for A {}
impl<I: IntoIterator<u16>, T: From<u8>> From<I> for T {}
impl<I: IntoIterator<u16>, T: From<u16> - From<u8>> From<I> for T {}
