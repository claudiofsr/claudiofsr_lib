/// Add operations: `x += y`, `x -= y`, `x *= y` and `x /= y`.
pub trait OperationsExtension {
    type Output;

    /**
    Additional assignment method

    `x += y <=> x.addition(y)`
    ```
    use claudiofsr_lib::OperationsExtension;

    let mut count: u64 = 0;
    while count.addition(1) <= 5 {
        println!("do something");
        println!("count = {count}\n");
    }
    let result = 2 + count.addition(4);

    assert_eq!(count, 10);
    assert_eq!(result, 12);
    ```
    */
    fn addition(&mut self, other: Self) -> Self;

    /**
    Subtraction assignment method

    `x -= y <=> x.subtraction(y)`
    ```
    use claudiofsr_lib::OperationsExtension;

    let mut count: i16 = 5;
    while count.subtraction(1) > 0 {
        println!("do something");
        println!("count = {count}\n");
    }
    let result = 8 + count;

    assert_eq!(count, 0);
    assert_eq!(result, 8);
    ```
    */
    fn subtraction(&mut self, other: Self) -> Self;

    /**
    Multiplication assignment method

    `x *= y <=> x.multiply(y)`
    ```
    use claudiofsr_lib::OperationsExtension;

    let mut number = 2;
    let result = 3 + number.multiply(5);

    assert_eq!(number, 10);
    assert_eq!(result, 13);
    ```
    */
    fn multiply(&mut self, other: Self) -> Self;

    /**
    Division assignment method

    `x /= y <=> x.divide(y)`
    ```
    use claudiofsr_lib::OperationsExtension;

    let mut number = 16;
    let result = 7 + number.divide(5);

    assert_eq!(number, 3);
    assert_eq!(result, 10);
    ```
    */
    fn divide(&mut self, other: Self) -> Self;
}

impl<T> OperationsExtension for T
where
    T: Copy + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign,
{
    type Output = Self;

    fn addition(&mut self, other: Self) -> Self {
        *self += other;
        *self
    }

    fn subtraction(&mut self, other: Self) -> Self {
        *self -= other;
        *self
    }

    fn multiply(&mut self, other: Self) -> Self {
        *self *= other;
        *self
    }

    fn divide(&mut self, other: Self) -> Self {
        *self /= other;
        *self
    }
}
