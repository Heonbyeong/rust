fn main() {
    println!("Hello, world!");

    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {x}");

    let x = six();
    println!("The value of x is: {x}");

    expression_function();
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    return 5;
}

fn six() -> i32 {
    6
}

/**
* Rust는 표현식 기반 언어
* 구문(statement): 어떤 동작을 수행하고 값을 반환하지 않는 명령
* 식(expression): 결과값을 평가함
**/
fn statement_function() { // statement (함수 정의)
    let y = 6; // statement
    
    // let x = (let y = 6); // error! statement
}

fn expression_function() {
    let y = { // expression
        let x = 3;
        x + 1 // 표현식은 세미콜론을 쓰지 않음. 세미콜론이 추가되면 구문으로 변경됨
    };
    println!("The value of y is: {y}"); // 4

    let y = {
        let x = 3;
        x + 1;
    };
    println!("The value of y is: {y}"); // error! y is statement
}