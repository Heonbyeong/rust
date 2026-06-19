fn main() {
    {
        let mut s1 = String::from("hello");

        let len = calculate_length(&s1); // s1값을 참조하지만 소유하지 않는 참조자 생성. 대여(borrow)

        println!("The length of '{}' is {}.", s1, len);
    }

    // mutable borrow value
    {
        let mut s1 = String::from("hello");

        mutable_change(&mut s1);

        println!("s1 is {}", s1)
    }

    /*
    * compile error! 동일 데이터에 대해 여러 가변 참조자를 생성할 수 없음
    * 이런 특성 덕분에 데이터 race condition을 방지함
    */
    {
        let mut s = String::from("hello");

        let r1 = &mut s;
        let r2 = &mut s; // 두 번째 !!

        println!("{}, {}", r1, r2);
    }
    {
        let mut s = String::from("hello");

        {
            let r1 = &mut s; // 새로운 스코프를 생성해 여러 가변 참조자를 만드는 것은 가능
        }

        let r2 = &mut s;
    }

    /*
    * 가변/불변 참조자 혼용
    */
    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        let r3 = &mut s; // compile error ! 가변/불변 참조자를 혼용할 수 없음. 여러 불변 참조자를 만드는 것은 가능

        println!("{}, {}, and {}", r1, r2, r3);
    }
    {
        let mut s = String::from("hello");

        let r1 = &s; // 문제없음
        let r2 = &s; // 문제없음
        println!("{} and {}", r1, r2);
        // 이 지점 이후로 변수 r1과 r2는 사용되지 않음 
    
        let r3 = &mut s; // 문제없음
        println!("{}", r3);
    }

    /*
    * 댕글링 참조: 어떤 메모리를 가리키는 포인터가 남아있는 상황에서 일부 메로리를 해제해, 다른 개체가 할당받았을지도 모르는 메모리를 참조하게 된 포인터
    */
    {
        let reference_to_nothing = dangle(); // compile error! 해제된 포인터 참조
        let reference_to_value = no_dangle(); // ok
    }
}


fn calculate_length(s: &String) -> usize {
    s.len()
} // s가 스코프 밖으로 벗어나지만, 소유하지는 않으므로 버려지지 않음


fn change(some_string: &String) {
    some_string.push_str(", world"); // compile error! 빌린 값은 immutable
}

fn mutable_change(some_string: &mut String) {
    some_string.push_str(", world");
}


fn dangle() -> &String {
    let s = String::from("hello");

    &s
} // s는 스코프 밖으로 벗어나고 메모리가 해제됨

fn no_dangle() -> String {
    let s = String::from("hello");

    s // String을 직접 반환하여 댕글링 방지
}