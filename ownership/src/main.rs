fn main() {
    { // s는 선언되지 않아 아직 유효하지 않음
        let s = "hello"; // 이 시점부터 유효
        // some job
    } // 스코프가 종료되며 s가 유효하지 않음

    {
        let s = String::from("hello"); // s는 이 지점부터 유효

        // some job
    } // 스코프가 종료되며 s가 유효하지 않음
      // 스코프 밖으로 벗어나면 drop이라는 함수 호출. 타입을 개발한 개발자가 직접 넣을 수 있으며, String 타입을 만든 사람이 작성한 해제 코드가 실행됨. }가 나타나는 시점에 자동 호출

    {
        let s1 = String::from("hello");
        // let s2 = s1;

        /**
        * String은 힙 데이터를 소유함
        * - s1: ptr -> "hello", len = 5, capacity = 5
        * - 문자열 데이터는 힙에, ptr, len, capacity는 스택에 저장
        * s2 = s1가 발생하면
        * - 소유권 이동(move) 수행. (before) s1 -> "hello" / (after) s2 -> "hello"
        * - 결과적으로 s1는 사용할 수 없음
        * 왜 무효화를 하는지?
        * - 둘 다 살아있다면 drop(s1), drop(s2)가 호출되어 같은 힙 메모리를 두 번 해제함(double free)
        * - 러스트는 자동으로 deep copy를 통해 데이터를 복사하는 일은 없음
        **/
        println!("{}, world!", s1); // compile error!
    }

    // String deep copy를 원할 땐 clone을 사용
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
    }

    /*
    * 스택에만 저장되는 데이터(단순한 스칼라 값의 묶음): Copy
    * - 컴파일 타임에 크기가 고정되는 타입은 모두 스택에 저장. 빠르게 생성할 수 있어 무효화 할 필요가 없음
    * - 스택 저장 타입에 달아 둘 수 있는 Copy 트레이트 존재. 이 트레이트가 타입에 구현되어 있다면 이동하지 않고 복사됨
    */
    {
        let x = 5;
        let y = x;

        println!("x = {}, y = {}", x, y);

    }

    /*
    * onwership & function
    */
    {
        ownership_and_function();
    }

    /*
    * return & scope
    */
    {
        return_and_scope();
    }
}

fn ownership_and_function() {
    let s = String::from("hello");

    takes_ownership(s); // s 값이 함수로 이동
                        // 더 이상 s는 유효하지 않음

    let x = 5;

    makes_copy(x); // x가 함수로 이동하지만 Copy 트레이트기 때문에 앞으로도 x를 사용할 수 있음
} // x, s 모두 스코프로 벗어나지만, s는 함수로 이동했으므로 아무런 일이 일어나지 않음

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string drop 호출, 메모리 해제

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // 별다른 일이 발생하지 않음


fn return_and_scope() {
    let s1 = gives_ownership(); // gives_ownership이 자신의 반환 값을 s1으로 이동

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2); // s2는 takes_and_gives_back으로 이동, 이 함수 또한 s3로 이동
} // s1, s3는 여기서 버려짐. s2는 이동되어 아무일도 일어나지 않음

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // 호출자 함수로 이동
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // 호출자 함수로 이동
}


