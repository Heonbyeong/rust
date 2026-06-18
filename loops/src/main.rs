fn main() {
    // 반복문 값 반환
    {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2; // break 표현식 뒤에 반환하고자 하는 값 
            }
        };

        println!("The result is {result}"); // 20

    }

    // loop label
    {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up; // 바깥쪽 루프를 탈출
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {count}");
    }

    // while
    {
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;

        while index < 5 {
            println!("the value is: {}", a[index]);
            index += 1;
        }
    }

    // for 
    {
        let a = [10, 20, 30, 40, 50];

        for element in a {
            println!("the value is: {element}");
        }

        for number in (1..4).rev() { // 1~3, reverse range
            println!("{number}!");
        }

        println!("LIFTOFF!!!");
    }
}
