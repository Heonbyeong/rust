/**
* Rust는 정적 타입의 언어 (컴파일 시점에 모든 변수의 타입은 정해져야 함)
**/
fn main() {
    // 여러 타입이 가능한 경우 타입 명시가 필요
    {
        // ok
        let guess: u32 = "42".parse().expect("Not a number!");

        // error!: type annitations needed
        let guess = "42".parse().expect("Not a number!");
    }

    /**
    * 정수형 타입
    *
    * 부호 있음: i / 부호 없음: u
    * length(bit): 8, 16, 32, 64, 128, arch(64-bit 아키텍처: 64, 32-bit 아키텍처: 32)
    * e.g. i8, u32, isize
    **/
    {
        let signed: i8 = 127;
        let unsigned: u8 = 255;

        println!("signed: {}, unsigned: {}", signed, unsigned) // signed: 127, unsigned: 255
    }

    /**
    * 정수형 리터럴
    *
    * Decimal: 98_222
    * Hex: 0xff
    * Octal: 0o77
    * Binary: 0b1111_0000
    * Byte(u8 only): b'A'
    **/
    {
        let decimal = 98_222;
        let hex = 0xff;
        let octal = 0o77;
        let binary = 0b1111_0000;
        let byte = b'A';

        println!("decimal: {}", decimal); // decimal: 98222
        println!("hex: {}", hex); // hex: 255
        println!("octal: {}", octal); // octal: 63
        println!("binary: {}", binary); // binary: 240
        println!("byte: {}", byte); // byte: 65
    }

    /**
    * 부동 소수점
    **/
    {
        let x = 2.0; // f64

        let y: f32 = 3.0; // f32
    }

    /**
    * 부울 타입
    **/
    {
        let t = true;

        let f: bool = false; // 명시적 타입 어노테이션
    }

    /**
    * 문자 타입
    * 4 Byte size, 유니코드 스칼라 값 표현 (U+0000~U+D7FF, U+E000~U+10FFFF)
    **/
    {
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';
    }
}
