// fn main() {
//     println!("3.3.3.enums>linked_list ~ 7.expressions");
// }


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
}
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