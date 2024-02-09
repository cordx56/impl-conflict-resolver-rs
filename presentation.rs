struct String;
trait From<T> {}
trait Display {}
trait ToString {}

impl<T: Display> From<T> for String {}
impl<T: ToString> From<T> for String {}
impl<T: ToString - Display> From<T> for String {}
