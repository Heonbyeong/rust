/**
* Shadowing
* x 스스로를 다시 섀도잉하거나 스코프가 끝날 때까지 변수명의 사용을 가져감
**/
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // x = 12
    }

    println!("The value of x is: {x}"); // x = 6

    // let을 통해 새로운 변수를 만들기 때문에 같은 변수명으로 다른 타입의 값을 저장할 수 있음
    let spaces = "    ";
    println!("spaces: {spaces}"); // "" empty
    let spaces = spaces.len();
    println!("spaces: {spaces}"); // 4

    // compile error!
    let mut spaces = "    ";
    spaces = spaces.len();
}
