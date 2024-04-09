struct u32;
struct i32;
struct A;
trait From<T> {}
trait IntoIterator<T> {}
impl<I: IntoIterator<u32>> From<I> for A {}
impl<I: IntoIterator<i32>> From<I> for A {}
impl<I: IntoIterator<i32> + !IntoIterator<u32>> From<I> for A {}
