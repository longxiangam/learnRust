pub fn add(left: usize, right: usize) -> usize {
    left + right
}
#[macro_export]
macro_rules! nice{
     ($x: ident)=>{
        println!("ident:{}",stringify!($x));
    };
    ($x:expr) => {
        println!("express:{}",$x);
    };

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
