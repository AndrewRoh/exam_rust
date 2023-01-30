fn main() {
    println!("3. Custom Types ~ 7. Expressions");
}

//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_expressions() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 이 표현은  `y` 에 값을 대입한다.
        x_cube + x_squared + x
    };

    let z = {
        // 이 세미콜론 문은 해당 문장을 끝맺고 `()`을 `z`에 대입한다.
        let _ = 2 * x; //2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

// x is 5
// y is 155
// z is ()
//------------------------------------------------------------------------------------------------//
use std::fmt;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

#[allow(dead_code)]
fn main_conversion_to_from_strings() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

// Circle of radius 6
// Sum: 15
//------------------------------------------------------------------------------------------------//
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

#[allow(dead_code)]
fn main_conversion_tryfrom_tryinfo() {
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

// 오류 없음
//------------------------------------------------------------------------------------------------//
use std::convert::From;

#[derive(Debug)]
#[allow(dead_code)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[allow(dead_code)]
fn main_conversion_from_into() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

// My number is Number { value: 30 }
// My number is Number { value: 5 }
//------------------------------------------------------------------------------------------------//
// `NanoSecond`, `Inch`, and `U64` are new names for `u64`.
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

#[allow(dead_code)]
fn main_types_alias() {
    // `NanoSecond` = `Inch` = `U64` = `u64`.
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    // 주의할 점은, 타입 별칭은 어떤 추가 타입 안전성을 제공하지 *않는데*
    // 이는 별칭이 새로운 타입들이 *아니기* 때문이다.
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}

// 5 nanoseconds + 2 inches = 7 unit?
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_types_inference() {
    // 주해가 달려, 컴파일러는 `elem`이 u8 타입임을 알았다.
    let elem = 5u8;

    // 비어있는 벡터를 생성한다. (확장 가능한 배열).
    let mut vec = Vec::new();
    // 이 시점에 컴파일러는 벡터의 정확한 타입을 알지 못하고
    // 그저 어떤 것들의 벡터라고만 알고 있다. (`Vec<_>`).

    // `elem`를 벡터에 추가한다.
    vec.push(elem);
    // 아하! 이제 컴파일러는 `vec` 가 `u8`s의 벡터임을 알았다. (`Vec<u8>`)
    // 해당 줄  `vec.push(elem)` 를 주석처리 하면 오류 발생
    println!("{:?}", vec);
}

// [5]
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_types_literals() {
    // 접두어가 붙은 리터럴, 이들의 타입은 초기화될 때 알려 수 있다.
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 접두어 없는 리터럴, 이들의 타입은 그들의 사용처에 달렸다.
    let i = 1;
    let f = 1.0;

    // `size_of_val` 변수 사이즈를 바이트 단위로 리턴.
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

// size of `x` in bytes: 1
// size of `y` in bytes: 4
// size of `z` in bytes: 4
// size of `i` in bytes: 4
// size of `f` in bytes: 8
//------------------------------------------------------------------------------------------------//
// 변환으로 인한 오버플로우에 관련 경고 모두 무시하기.
#[allow(overflowing_literals)]
#[allow(dead_code)]
fn main_types_casting() {
    let decimal = 65.4321_f32;

    // 에러! 암시적 변환은 안됨!
    //let integer: u8 = decimal;

    // 명시적 변환
    let integer = decimal as u8;
    let character = integer as char;

    // 오류! 변환 규칙에는 제한이 있습니다.
    // float는 char로 직접 변환할 수 없습니다.
    //let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // 1000은 항상 u16에 맞는다.
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // 여기서는, 첫 째 8 최하위비트(LSB)는 유지되고,
    // 남은 쪽에서 최상위비트(MSB)는 잘려나간다.
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // 양수의 경우 모듈러스와 동일합니다.
    println!("1000 mod 256 is : {}", 1000 % 256);

    // 부호 있는 유형으로 캐스팅할 때 (비트) 결과는 다음과 같습니다.
    // 먼저 해당 부호 없는 유형으로 캐스팅합니다. 가장 중요한 경우
    // 해당 값의 비트가 1이면 값은 음수입니다.

    // 물론 이미 맞지 않는 한.
    println!(" 128 as a i16 is: {}", 128 as i16);

    // 128 as u8 -> 128, 8비트 2의 보수 표현의 값은 다음과 같습니다.
    println!(" 128 as a i8 is : {}", 128 as i8);

    // 위의 예 반복
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // 8비트 2의 보수 표현에서 232의 값은 -24입니다.
    println!(" 232 as a i8 is : {}", 232 as i8);

    // Rust 1.45부터 `as` 키워드는 *포화 캐스트*를 수행합니다.
    // float에서 int로 변환할 때. 부동 소수점 값이 다음을 초과하는 경우
    // 상한 또는 하한보다 작음, 반환된 값
    // 바운드 교차와 동일합니다.

    // 300.0 as u8 is 255
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("   nan as u8 is : {}", f32::NAN as u8);

    // 이 동작은 약간의 런타임 비용이 발생하므로 피할 수 있습니다.
    // 안전하지 않은 메서드를 사용하지만 결과가 오버플로될 수 있으며
    // **unsound values(불건전한 값)**을 반환합니다. 다음 방법을 현명하게 사용하십시오.
    unsafe {
        // 300.0 as u8 is 44
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}

// Casting: 65.4321 -> 65 -> A
// 1000 as a u16 is: 1000
// 1000 as a u8 is : 232
// -1 as a u8 is : 255
// 1000 mod 256 is : 232
// 128 as a i16 is: 128
// 128 as a i8 is : -128
// 1000 as a u8 is : 232
// 232 as a i8 is : -24
// 300.0 as u8 is : 255
// -100.0 as u8 is : 0
// nan as u8 is : 0
// 300.0 as u8 is : 44
// -100.0 as u8 is : 156
// nan as u8 is : 0
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_variable_bindings_freezing() {
    let mut _mutable_integer = 7i32;

    {
        // 불변 `_mutable_integer`에 의한 섀도잉
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` 는 이 범위에서 고정됩니다
        //_mutable_integer = 50;

        // `_mutable_integer` 가 범위를 벗어납니다.
    }

    // Ok! `_mutable_integer` 는 이 범위에서 고정되지 않습니다.
    _mutable_integer = 3;
}

//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_variable_bindings_declare_first() {
    // 변수 바인딩 선언
    let a_binding;

    {
        let x = 2;

        // 바인딩 초기화
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;
    // 오류! 초기화되지 않은 바인딩 사용
    // println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}

//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_variable_bindings_scope_shadowing() {
    // 이 바인딩은 메인 함수에 있습니다.
    let long_lived_binding = 1;

    // 이것은 블록이고 메인 함수보다 작은 범위를 가집니다.
    {
        // 이 바인딩은 이 블록에만 존재합니다.
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // 블록의 끝

    // 오류! 이 범위에 `short_lived_binding`이 없습니다.
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    let shadowed_binding = 1;
    {
        println!("before being shadowed: {}", shadowed_binding);

        // 이 바인딩은 외부 바인딩을 *그림자*로 만듭니다.
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // 이 바인딩은 이전 바인딩을 *가립니다*
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}

// inner short: 2
// outer long: 1
// before being shadowed: 1
// shadowed in inner block: abc
// outside inner block: 1
// shadowed in outer block: 2
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_variable_bindings_mutability() {
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
            }
            Nil => {
                format!("Nil")
            }
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

//------------------------------------------------------------------------------------------------//
// 암시적으로 식별되는 enum (0에서 시작)
#[allow(dead_code)]
enum Number {
    Zero,
    One,
    Two,
}

// 명시적으로 식별 가능한 enum
#[allow(dead_code)]
enum Color2 {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

#[allow(dead_code)]
fn main_enums_clike() {
    // `enums` 은 정수형으로 변환 사용 가능.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color2::Red as i32);
    println!("violets are #{:06x}", Color2::Blue as i32);
}

// zero is 0
// one is 1
// roses are #ff0000
// violets are #0000ff
//------------------------------------------------------------------------------------------------//
// 사용하지 않는 코드에 대한 경고를 숨기는 속성입니다.
#[allow(dead_code)]
enum Status {
    Rich,
    Poor,
}

#[allow(dead_code)]
enum Work {
    Civilian,
    Soldier,
}

#[allow(dead_code)]
fn main_enums_use() {
    // 각 이름을 명시적으로 'use'하여 없이도 사용할 수 있도록 합니다.
    // 수동 범위 지정.
    use crate::Status::{Poor, Rich};
    // `Work` 내부의 각 이름을 자동으로 `use`합니다.
    use crate::Work::*;

    // `Status::Poor`와 동일합니다.
    let status = Poor;
    // `Work::Civilian`와 동일합니다.
    let work = Civilian;

    match status {
        // 위의 명시적인 `use`로 인해 범위가 부족하다는 점에 유의하십시오.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // 범위 지정이 없다는 점을 다시 한 번 확인하십시오.
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}

// The poor have no money...
// Civilians work!
//------------------------------------------------------------------------------------------------//
// 웹 이벤트를 분류하기 위해 `enum`을 만듭니다. 둘 다
// 이름과 유형 정보가 함께 변형을 지정합니다.
// `PageLoad != PageUnload` 및 `KeyPress(char) != Paste(String)`.
// 각각은 다르고 독립적입니다.
#[allow(dead_code)]
enum WebEvent {
    // `enum`은 `unit-like`이거나,
    PageLoad,
    PageUnload,
    // 튜플 구조체처럼,
    KeyPress(char),
    Paste(String),
    // 또는 c와 같은 구조
    Click { x: i64, y: i64 },
}


// `WebEvent` 열거형을 인수로 사용하는 함수 및
// 아무것도 반환하지 않습니다.
#[allow(dead_code)]
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // `enum` 내부에서 `c`를 분해합니다.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // `Click`을 `x`와 `y`로 분해합니다.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

#[allow(dead_code)]
fn main_enums() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()`는 스트링 슬라이스에서 소유된 `String`을 생성합니다.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
// pressed 'x'.
// pasted "my text".
// clicked at x=20, y=80.
// page loaded
// page unloaded
//------------------------------------------------------------------------------------------------//

// 유닛 구조체
#[allow(dead_code)]
struct Nil;

// 튜플 구조체
#[allow(dead_code)]
struct Pair(i32, f32);

// 두 필드를 갖는 구조체
#[allow(dead_code)]
struct Point {
    x: f32,
    y: f32,
}

// 구조체는 다른 구조체의 필드로 사용될 수 있다.
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

#[allow(dead_code)]
fn rect_area(r: Rectangle) -> f32 {
    let x = r.p2.x - r.p1.x;
    let y = r.p2.y - r.p1.y;
    x * y
}

#[allow(dead_code)]
fn square(po: Point, f: f32) -> Rectangle {
    //let Point { x: po.x, y: my_y } = point;

    Rectangle {
        p1: Point { x: po.x, y: po.y },
        p2: Point { x: po.x + f, y: po.y + f },
    }
}

#[allow(dead_code)]
fn main_structures() {
    // `Point` 초기화;
    let point: Point = Point { x: 0.3, y: 0.4 };

    // 포인트의 필드에 접근하는 방식.
    println!("point coordinates: ({}, {})", point.x, point.y);

    // `let` 바인딩을 통해 재구조화.
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // 구조체 초기화는 표현문이기도 하다.
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // 단위 구조체 초기화
    let _nil = Nil;

    // 튜플 구조체 초기화
    let pair = Pair(1, 0.1);

    // 튜플 구조체에 접근하는 방식.
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 튜플 구조체의 재구조화
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let area = rect_area(_rectangle);
    println!("rect area {}", area);

    let po: Point = Point { x: 0.3, y: 0.4 };
    let rect = square(po, 4f32);

    println!("rect area {},", rect.p1.x);
}

// point coordinates: (0.3, 0.4)
// pair contains 1 and 0.1
// pair contains 1 and 0.1
// rect area -0.009999999
// rect area 0.3,