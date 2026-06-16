fn main() {
    /**
    * Tuple
    **/
    {
        let tup: (i32, f64, u8) = (500, 6.4, 1);

        let (x, y, z) = tup; // 구조 해체를 통한 튜플 값 할당

        println!("The vlaue of y is: {y}");

        let five_hundred = tup.0; // 인덱스를 통한 튜플 요소 접근
        let six_point_four = tup.1;

        println!("The value of five_hundred is: {five_hundred}");
        println!("The value of six_point_four is: {six_point_four}");

        let unit: () = (); // 비어있는 튜플(Unit)
    }

    /**
    * Array
    **/
    {
        let a = [1, 2, 3, 4, 5];
        println!("array a is: {a:?}");

        let a: [i32; 5] = [1, 2, 3, 4, 5]; // 배열의 타입과 사이즈 정의
        println!("array a is: {a:?}");

        let a = [3; 5]; // 모든 요소가 동일한 값으로 채워진 배열
        println!("array a is: {a:?}"); // let a = [3, 3, 3, 3, 3];
    }
}
