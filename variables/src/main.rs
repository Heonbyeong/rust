fn main() {
    // 컴파일 에러 발생: let은 불변(Immutable)
    {
        let x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
    }

    // 컴파일 성공: mut은 가변(Mutable)
    {
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
    }
}
