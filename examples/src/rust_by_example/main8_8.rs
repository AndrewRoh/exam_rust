fn main() {
    println!("8.Flow of Control ~ 8.Flow of Control");
}
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_flow_control_while_let() {
    // `Option<i32>` 타입의 `optional` 생성
    let mut optional = Some(0);

    // 반복해서 이 테스트를 수행.
    loop {
        match optional {
            // `optional`이 역구조화 되면, 블럭을 수행.
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
                // ^ 3 들여쓰기 필요!
            },
            // 역구조화에 실패하면 루프 탈출:
            _ => { break; }
            // ^ 이게 필요한 이유가 뭐지? 더 나은 방법이 있을거야!
        }
    }
    println!("-------------------------");
    // `Option` 타입의 `optional` 생성
    let mut optional = Some(0);

    // 읽는 방법: "`let`으로 `optional`을 `Some(i)`로 역구조화 되는 동안
    // 블럭을 수행 (`{}`). 아니면 `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ 더 적은 오른쪽 들여쓰기, 실폐의 경우에 대한 명시적 처리가 필요하지 않다.
    }
    // ^ `if let`은 추가로 부수적 `else`/`else if`절을 갖지만
    // `while let`은 필요치 않음.
}
// `i` is `0`. Try again.
// `i` is `1`. Try again.
// `i` is `2`. Try again.
// `i` is `3`. Try again.
// `i` is `4`. Try again.
// `i` is `5`. Try again.
// `i` is `6`. Try again.
// `i` is `7`. Try again.
// `i` is `8`. Try again.
// `i` is `9`. Try again.
// Greater than 9, quit!
// -------------------------
// `i` is `0`. Try again.
// `i` is `1`. Try again.
// `i` is `2`. Try again.
// `i` is `3`. Try again.
// `i` is `4`. Try again.
// `i` is `5`. Try again.
// `i` is `6`. Try again.
// `i` is `7`. Try again.
// `i` is `8`. Try again.
// `i` is `9`. Try again.
// Greater than 9, quit!
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}
#[allow(dead_code)]
fn main_flow_control_if_let() {
    // 모든 타입이 `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let` 구문을 읽는 법:"if(만약) `let`으로 `number`를
    // 역구조화해 `Some(i)`에 넣고, 그에 따른 블럭 (`{}`) 수행.
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // 실패에 대한 기재가 필요할 경우, else를 사용하라:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // 역구조화가 실패했다. 실패 사례로 변경.
        println!("Didn't match a number. Let's go with a letter!");
    }

    // 변경된 실패 상태를 제공.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
        // 역구조화에 실패했다. `else if` 조건의 평가에 따라
        // 대체 실패 분기가 취해질 것이다.
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // 조건이 false로 판단됐다. 이 분기가 기본이다:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // 변수 a는 Foo::Bar와 일치합니다.
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // 변수 b는 Foo::Bar와 일치하지 않습니다.
    // 그래서 이것은 아무것도 출력하지 않을 것입니다.
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // 변수 c는 값이 있는 Foo::Qux와 일치합니다.
    // 이전 예제의 Some()과 유사
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // 바인딩은 `if let`에서도 작동합니다.
    if let Foo::Qux(_value @ 100) = c {
        println!("c is one hundred");
    }
}
// Matched 7!
// Didn't match a number. Let's go with a letter!
// I don't like letters. Let's go with an emoticon :)!
// a is foobar
// c is 100
// c is one hundred
//------------------------------------------------------------------------------------------------//
// `age` 함수는 `u32`를 반환한다.
fn age() -> u32 {
    15
}
fn some_number() -> Option<u32> {
    Some(42)
}
#[allow(dead_code)]
fn main_flow_control_match_binding() {
    println!("Tell me what type of person you are");

    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        // `match`를 1 ... 12 까지 직접적으로 할 수 있지만
        // 그러면 그 하위에서는 몇 age인지 알 수가 없으니 `n`으로
        // 1 ... 12 순차를 바인드한다. 이제 age를 알 수 있게 된다.
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // 바인드 없이 결과를 반환.
        n             => println!("I'm an old person of age {:?}", n),
    }

    match some_number() {
        // `Some` 변형이 있고, 값이 `n`에 바인딩된 경우 일치합니다.
        // 42와 같습니다.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // 다른 숫자와 일치합니다.
        Some(n)      => println!("Not interesting... {}", n),
        // 다른 항목과 일치합니다(`None` 변형).
        _            => (),
    }
}
// Tell me what type of person you are
// I'm a teen of age 15
// The Answer: 42!
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}
#[allow(dead_code)]
fn main_flow_control_match_guards() {
    let temperature = Temperature::Fahrenheit (99);
    // 다른 값을 입력 해 보세요. `temperature`

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        //`if 조건문`이 guard 부분.
        Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
    }

    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("Should never happen."),
    }
}
// 35C is above 30 Celsius
// 99F is above 86 Fahrenheit
// Greater than zero
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_flow_control_match_structs() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // 구조체의 값을 변경하여 어떤 일이 발생하는지 확인하십시오
    let foo = Foo { x: (4, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // 구조체를 분해하고 변수의 이름을 바꿀 수 있습니다.
        // 순서는 중요하지 않다
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // 일부 변수를 무시할 수도 있습니다.
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // 이렇게 하면 오류가 발생합니다. 패턴이 `x` 필드를 언급하지 않습니다.
        //Foo { y } => println!("y = {}", y),
    }
}
// First of x is 1, b = 2,  y = 3
// y is 2, i = (4, 2)
// y = 3, we don't care about x
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_flow_control_match_pointers() {
    // `i32` 유형의 참조를 할당합니다. `&`는 다음을 의미합니다.
    // 할당되는 참조입니다.
    let reference = &4;

    match reference {
        // `reference`가 `&val`과 일치하는 패턴인 경우 결과
        // 다음과 같은 비교에서:
        // `&i32`
        // `&값`
        // ^ 일치하는 `&`가 삭제되면 `i32`
        // `val`에 할당되어야 합니다.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // `&`를 피하려면 일치하기 전에 역참조합니다.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // 참조로 시작하지 않으면 어떻게 될까요? `참조`는 `&`였습니다.
    // 오른쪽이 이미 참조였기 때문입니다. 이것은 아니다
    // 오른쪽이 1이 아니기 때문에 참조.
    let _not_a_reference = 3;

    // Rust는 정확히 이 목적을 위해 `ref`를 제공합니다. 그것은 수정
    // 요소에 대한 참조가 생성되도록 할당; 이것
    // 참조가 할당됩니다.
    let ref _is_a_reference = 3;

    // 따라서 참조가 없는 2개의 값을 정의하여 참조
    // `ref` 및 `ref mut`을 통해 검색할 수 있습니다.v
    let value = 5;
    let mut mut_value = 6;

    // `ref` 키워드를 사용하여 참조를 생성합니다.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // 유사하게 `ref mut`을 사용합니다.
    match mut_value {
        ref mut m => {
            // 참조를 얻었습니다. 우리가 할 수 있기 전에 역참조해야 해
            // 아무거나 추가합니다.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}
// Got a value via destructuring: 4
// Got a value via dereferencing: 4
// Got a reference to a value: 5
// We added 10. `mut_value`: 16
//------------------------------------------------------------------------------------------------//
// `allow`는 하나의 변수만 사용되는 것에 대한 경고를 생략시키는데 필요하다.
#[allow(dead_code)]
enum Color {
    // 이 3개는 그들의 이름 자체로 열거됩니다.
    Red,
    Blue,
    Green,
    // 이들도 유사하게 `u32`튜플을 다른 이름들:컬러 모델로 결집한다.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

#[allow(dead_code)]
fn main_flow_control_match_enums() {
    let color_m = Color::RGB(122, 17, 40);

    println!("What color is it?");
    // `enum`은 `match`를 사용해 역구조화 할 수 있다.
    match color_m {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                     c, m, y, k),
        // 모든 변수가 검토되었기 때문에 이 외에 범위가 필요하지 않다.
    }
}
// What color is it?
// Red: 122, green: 17, and blue: 40!
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_flow_control_match_arrays() {
    // 배열의 값을 변경하거나 슬라이스로 만드십시오!
    let array = [4, -2, 6];

    match array {
        // 두 번째 및 세 번째 요소를 각각의 변수에 바인딩합니다.
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // 단일 값은 _로 무시할 수 있습니다.
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // 일부를 바인딩하고 나머지는 무시할 수도 있습니다.
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // 아래 코드는 컴파일되지 않습니다.
        // [-1, second] => ...

        // 또는 다른 배열/슬라이스에 저장합니다(유형은
        // 일치되는 값의 값)
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // 이러한 패턴을 결합하여 예를 들어 첫 번째 및
        // 마지막 값, 나머지는 단일 배열에 저장
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}
// array[0] = 3, array[1] = -2 and the other elements were [6]
// array[0] = 4, middle = [-2], array[2] = 6
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_flow_control_match_tuples() {
    let triple = (3, -2.5, 4);

    println!("Tell me about {:?}", triple);
    // 매치는 튜플의 역 구조화 하는데 사용될 수 있다.
    match triple {
        // 두번째 세번째 엘리먼트 역 구조화
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        (.., 2)  => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        // `..` 나머지 튜플을 무시하는 데 사용할 수 있습니다.
        _      => println!("It doesn't matter what they are"),
        // `_`의 의미는 값을 변수에 바인드하지 않는
    }
}
// Tell me about (3, -2.5, 4)
// First is `3`, last is `4`, and the rest doesn't matter
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_flow_control_match() {
    let number = 4;

    println!("Tell me about {}", number);
    match number {
        // 하나의 값에 매치.
        1 => println!("One!"),
        // 다수의 값에 매치.
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        // 범위에 포함되는 매치.
        13..=19 => println!("A teen"),
        // 나머지 경우의 처리.
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // 매치는 표현문이기도 함.
    let binary = match boolean {
        // 매치는 모든 가능한 값들을 범주에 포괄해야 함.
        false => 0,
        true => 1,
        // true => todo!()
    };

    println!("{} -> {}", boolean, binary);
}
// Tell me about 4
// Ain't special
// true -> 1
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_flow_control_range() {
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);

    // Hello Bob
    // Hello Frank
    // There is a rustacean among us!
    // names: ["Bob", "Frank", "Ferris"]

    // let names = vec!["Bob", "Frank", "Ferris"];
    // for name in names.into_iter() {
    //     match name {
    //         "Ferris" => println!("There is a rustacean among us!"),
    //         _ => println!("Hello {}", name),
    //     }
    // }
    // //println!("names: {:?}", names);

    // Hello Bob
    // Hello Frank
    // There is a rustacean among us!

    // let mut names = vec!["Bob", "Frank", "Ferris"];
    // for name in names.iter_mut() {
    //     *name = match name {
    //         &mut "Ferris" => "There is a rustacean among us!",
    //         _ => "Hello",
    //     }
    // }
    // println!("names: {:?}", names);

    // names: ["Hello", "Hello", "There is a rustacean among us!"]
}
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_flow_control_for() {
    // `n`은 각 반복에서 1, 2, ..., 100의 값을 갖습니다
    for n in 1..101 {   // 같은 동작 for n in 1..=100
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

// 1
// 2
// fizz
// 4
// buzz
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_flow_control_while() {
    // 숫자 세기용 변수
    let mut n = 1;

    // `n` 이 101보다 작을 동안 반복
    if n % 15 == 0 {
        println!("fizzbuzz");
    } else if n % 3 == 0 {
        println!("fizz");
    } else if n % 5 == 0 {
        println!("buzz");
    } else {
        println!("{}", n);
    }

    // 숫자 증가
    n += 1;
    println!("{}", n);
}

// 1
// 2
// fizz
// 4
// buzz
// fizz
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_flow_control_loop_returning_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

// 결과없음
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
#[allow(unreachable_code)]
fn main_flow_control_loop_nesting_labels() {
    'outer: loop {
        println!("Entered the outer loop");
        #[allow(unused_labels)]
        'inner: loop {
            println!("Entered the inner loop");

            // 아래 주석을 제거하면 내부 루프를 종결한다.
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

// Entered the outer loop
// Entered the inner loop
// Exited the outer loop
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_flow_control_loop() {
    let mut count = 0u32;
    println!("Let's count until infinity!");

    // 무한 루프
    loop {
        count += 1;

        if count == 3 {
            println!("three");
            // 이 순차의 나머지를 생략한다.
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            // 루프를 벗어난다.
            break;
        }
    }
}

// Let's count until infinity!
// 1
// 2
// three
// 4
// 5
// OK, that's enough
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_flow_control_ifelse() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // 이 표현은 `i32`를 반환한다.
            10 * n
        } else {
            println!(", and is a big number, reduce by two");

            // 이 표현 역시 반드시 `i32`를 반환해야 한다.
            n / 2
            // 해당 문장에 세미콜론을 붙혀 금지해보자.
        };
    //   ^ 여기의 세미콜론을 잊지마세요! 모든 `let` 바인딩은 이게 필요함.

    println!("{} -> {}", n, big_n);
}
// 5 is positive, and is a small number, increase ten-fold
// 5 -> 50