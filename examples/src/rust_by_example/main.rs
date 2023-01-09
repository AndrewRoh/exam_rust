fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error!
    //_immutable_binding += 1;
}
// Before mutation: 1
// After mutation: 2
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_variable_bindings() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // `an_integer`를 `copied_integer`로 복사
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 컴파일러는 사용하지 않는 변수 바인딩에 대해 경고합니다. 이러한 경고는
    // 변수 이름 앞에 밑줄을 붙여 침묵
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
}
// An integer: 1
// A boolean: true
// Meet the unit value: ()
//------------------------------------------------------------------------------------------------//
// 전역은 다른 모든 범위 외부에서 선언됩니다.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

#[allow(dead_code)]
fn is_big(n: i32) -> bool {
    // 일부 함수에서 상수에 액세스
    n > THRESHOLD
}

#[allow(dead_code)]
fn main_constants() {
    let n = 16;

    // 메인 스레드에서 상수에 액세스
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    //THRESHOLD = 5;
}
// This is Rust
// The threshold is 10
// 16 is big
//------------------------------------------------------------------------------------------------//
use crate::List::*;
#[allow(dead_code)]
enum List {
    // Cons: 튜플 구조체로 보관하는 것은 요소와 다음 노드 포인터
    Cons(u32, Box<List>),
    // Nil: 노드로 linked list의 끝을 식별
    Nil,
}
#[allow(dead_code)]
// 메소드는 enum에 접목될 수 있다.
impl List {
    // 빈 리스트를 생성.
    fn new() -> List {
        // `Nil`는 `List` 타입
        Nil
    }

    // list를 취하고 동일 list와 새 요소를 전면에 추가해 반환.
    fn prepend(self, elem: u32) -> List {
        // `Cons` 또한 List 타입을 갖는다.
        Cons(elem, Box::new(self))
    }

    // list의 길이 반환.
    fn len(&self) -> u32 {
        // `self`가 일치해야 되는 이유는 이 메소드의 행위가
        // `self`의 변수형에 달려있기 때문이다.
        // `self`는  `&List`타입이고, `*self`는 `List`타입이고,
        // 정확한 타입 `T`이 참조 `&T`보다 match에서 선호된다
        match *self {
            // 꼬리에 대한 소유권을 얻을 수 없는 이유는 `self`가 대여중이기 때문이다;
            // 대신 꼬리에 대한 참조를 빌리자.
            Cons(_, ref tail) => 1 + tail.len(),
            // 기본 상태: 빈 list는 0의 길이를 갖는다.
            Nil => 0
        }
    }

    // 반환하는 것은 list를 string으로 표현한 것(heap 할당된).
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!`은 `print!`와 유사하지만,
                // 반환하는 것은 콘솔 출력 대신 힙에 할당된 string이다.
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

#[allow(dead_code)]
fn main_enums_linked_list() {
    // 빈 linked list를 만든다.
    let mut list = List::new();

    // 요소 몇 개를 추가한다.
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // list의 마지막 상태를 보여준다.
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
// linked list has length: 3
// 3, 2, 1, Nil