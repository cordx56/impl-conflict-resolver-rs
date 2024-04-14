struct u32;
struct i32;
struct A;
trait From<T> {}
trait IntoIterator<T> {}
impl<T: IntoIterator<u32>> From<T> for A {}
impl<T: IntoIterator<i32>> From<T> for A {}
impl<T: IntoIterator<i32> + !IntoIterator<u32>> From<T> for A {}
