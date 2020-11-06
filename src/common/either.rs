#[derive(Debug,Copy,Clone,PartialOrd,PartialEq)]
pub enum Either<L,R>{
    Left(L),
    Right(R)
}

impl<L,R> Either<L,R>{
}

#[test]
fn either_test(){
    let e : Either<i32,bool> = Either::Right(true);
    println!("{:?}",e);
    match e{
        Either::Left(i) => {
            println!("{}",i)
        }
        Either::Right(b) => {
            println!("{}",b)
        }
    }
}