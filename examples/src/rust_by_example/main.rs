use std::fmt::{self, Formatter, Display};
use std::mem;

// 유닛 구조체
struct Nil;

// 튜플 구조체
struct Pair(i32, f32);

// 두 필드를 갖는 구조체
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

fn rect_area(r: Rectangle) -> f32 {
    let x = r.p2.x - r.p1.x;
    let y = r.p2.y - r.p1.y;
    x*y
}

fn main() {
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
}
// point coordinates: (0.3, 0.4)
// pair contains 1 and 0.1
// pair contains 1 and 0.1
//------------------------------------------------------------------------------------------------//
// 이 함수는 slice를 대여한다.
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

#[allow(dead_code)]
fn main_array_slice() {
    // 고정된 크기의 배열 (타입 선언은 불필요하다.)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 모든 요소들이 같은 값으로 초기화 될 수 있다.
    let ys: [i32; 500] = [0; 500];

    // 색인은 0부터 시작한다.
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` 배열의 길이를 반환한다.
    println!("array size: {}", xs.len());

    // 배열은 스택에 할당된다.
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 배열은 자동적으로 조각으로 변환하여 대여할 수 있다.
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // 조각들은 배열의 부분을 가르킬 수 있다.
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // 색인이 범위를 넘어가면 panic으로 넘어간다.
    // println!("{}", xs[5]);
}
// first element of the array: 1
// second element of the array: 2
// array size: 5
// array occupies 20 bytes
// borrow the whole array as a slice
// first element of the slice: 1
// the slice has 5 elements
// borrow a section of the array as a slice
// first element of the slice: 0
// the slice has 3 elements
//------------------------------------------------------------------------------------------------//
// 튜플은 함수 인자로도 반환 값으로도 사용될 수 있다.
#[allow(dead_code)]
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let`은 튜플의 멤버를 변수에 바인드 할 때 사용된다.
    let (integer, boolean) = pair;

    (boolean, integer)
}

// 다음 구조체는 activity 용.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"({0} {1})\n({2} {3})", self.0, self.1, self.2, self.3)
    }
}
#[allow(dead_code)]
fn transpose(src: Matrix) -> Matrix {
    let dst:Matrix=Matrix(src.3, src.0, src.2, src.1);
    dst
}
#[allow(dead_code)]
fn main_tuples() {
    // 서로 다른 타입 무리의 튜플
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // tuple에서 색인으로 값을 추출 할 수 있다.
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // 튜플이 튜플의 멤버가 될 수 있다.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // 튜플은 출력 가능
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    // 하나의 요소인 튜플을 만드려면, 괄호와는 별도로 쉼표를 통해 알리는게 필요하다.
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // 튜플은 바인딩을 생성해서 역구조화 할 수 있다.
    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matri\n:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}
// long tuple first value: 1
// long tuple second value: 2
// tuple of tuples: ((1, 2, 2), (4, -1), -2)
// pair is (1, true)
// the reversed pair is (true, 1)
// one element tuple: (5,)
// just an integer: 5
// 1, "hello", 4.5, true
// Matrix(1.1, 1.2, 2.1, 2.2)
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_literais_operator() {
    // 정수 더하기
    println!("1 + 2 = {}", 1u32 + 2);

    // 정수 빼기
    println!("1 - 2 = {}", 1u32 - 2);

    // 짧은 boolean 논리 연산
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // 비트 연산
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011  OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 밑줄을 사용하여 가독성 올려버리기~
    println!("One million is written as {}", 1_000_000u32);
}
// 1 + 2 = 3
// 1 - 2 = -1
// true AND false is false
// true OR false is true
// NOT true is false
// 0011 AND 0101 is 0001
// 0011  OR 0101 is 0111
// 0011 XOR 0101 is 0110
// 1 << 5 is 32
// 0x80 >> 2 is 0x20
// One million is written as 1000000
//------------------------------------------------------------------------------------------------//
// fn main_primitives() {
//     // 변수는 타입 주해가 달릴 수 있다.
//     let logical: bool = true;
//
//     let a_float: f64 = 1.0;  // 일반적인 주해
//     let an_integer   = 5i32; // 접미사 주해
//
//     // 아니면 기본형이 사용될 것이다.
//     let default_float   = 3.0; // `f64`
//     let default_integer = 7;   // `i32`
//
//     let mut mutable = 12; // 가변형 `i32`.
//
//     // 에러! 변수의 형은 변경될 수 없다!
//     //mutable = true;
// }
//------------------------------------------------------------------------------------------------//
struct City {
    name: &'static str,
    // 위도
    lat: f32,
    // 경도
    lon: f32,
}

impl Display for City {
    // `f`는 버퍼로 이 메소드는 반드시 여기에 형식화된 문자를 써야한다.
    // `f` is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!`는 `format!`과 유사하지만 버퍼(첫번째 인수)에 형식화 문자를 쓴다.
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}", self.red, self.green, self.blue )
    }
}

#[allow(dead_code)]
fn main_formatting() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display
        println!("{}", *color)
    }
}
// Dublin: 53.348°N 6.260°W
// Oslo: 59.950°N 10.750°E
// Vancouver: 49.250°N 123.100°W
// RGB (128, 255, 90) 0x80FF5A
// RGB (0, 3, 254) 0x0003FE
// RGB (0, 0, 0) 0x000000
//------------------------------------------------------------------------------------------------//
// `Vec`를 보관하는 `List`란 이름의 구조체를 정의
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `self`를 역참조하고, 역구조화를 통해 vec에 대한 참조를 생성.
        let List(ref vec) = *self;

        (write!(f, "["))?;

        // `vec`을 `v`로 반복 순차를 `count`로 반복 수행한다.
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { (write!(f, ", "))?; }
            (write!(f, "{}: {}", count, v))?;
        }

        // 열린 괄호를 닫고 fmt::Result 값을 반환.
        write!(f, "]")
    }
}

#[allow(dead_code)]
fn main_tests_list() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
// [0: 1, 1: 2, 2: 3]
//------------------------------------------------------------------------------------------------//
// 구조체는 두 숫자를 보관한다. `Debug`가 파생되어 `Display`의 결과와 대조된다.
#[derive(Debug)]
struct MinMax(i64, i64);

// `Display`를 `MinMax`에 구현한다.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "([{}~{}])", self.0, self.1)
    }
}

// 비교용으로 필드가 이름을 갖는 구조체를 정의한다.
#[derive(Debug)]
struct Point2 {
    x: f64,
    y: f64,
}

// 비슷하게 Point2를 위해 구현한다.
impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 사용자 정의를 통해 `x`와 `y`만 표시되도록 한다.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Binary for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 사용자 정의를 통해 `x`와 `y`만 표시되도록 한다.
        write!(f, "binary x: {}, y: {}", self.x, self.y)
    }
}

#[allow(dead_code)]
fn main_display() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2 { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // 에러. `Debug`와 `Display` 둘이 구현되었지만 `{:b}`는
    // `fmt::Binary`의 구현이 요구된다. 이는 동작하지 않는다.
    println!("What does Point2D look like in binary: {:b}?", point);
}
// Compare structures:
// Display: ([0~14])
// Debug: MinMax(0, 14)
// The big range is ([-300~300]) and the small is ([-3~3])
// Compare points:
// Display: x: 3.3, y: 7.2
// Debug: Point2 { x: 3.3, y: 7.2 }
// What does Point2D look like in binary: binary x: 3.3, y: 7.2?
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_formatted_print() {
    // 일반적으로 `{}`는 인자에 따라 자동으로 변환된다.
    // 다음은 String으로 변환될 것이다.
    println!("{} days", 31);

    // 접미사가 없으면, 31은 i31이 된다.
    // 접미사를 추가해 31의 type을 변경할 수 있다.

    // 출력을 위한 다양한 옵션들이 있다.
    // 위치지정 인자도 사용될 수 있다.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 이름을 인자로 사용할 수 있다.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // `:`; 뒤에 특수 형식 지정자를 사용할 수 있다..
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 넓이를 지정하여 오른쪽 정렬을 사용할 수 있다. 이는 다음과 같이 출력될 것이다.
    // "     1". 5칸의 공백과 "1".
    println!("{number:>width$}", number=1, width=6);

    // 여분의 공간을 0으로 채운 숫자도 사용할 수 있다.
    // 이는 "000001"을 출력할 것.
    println!("{number:>0width$}", number=1, width=6);

    // 위치지정 인자 사용시 정확한 수의 인자들이 왔는데 검증받게 될 것이다.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // `i32` 내장한 구조체를 만들자. 그리고 `Structure`라고 이름지었다.
    // #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    // 하지만, Structure와 같이 당신이 만든 형태는 좀더 복잡해진다.
    // 다음 문장은 실행되지 않을 것이다.
    println!("This struct `{:?}` won't print...", Structure(3));
}
// 31 days
// Alice, this is Bob. Bob, this is Alice
// the quick brown fox jumps over the lazy dog
// 1 of 10 people know binary, the other half don't
// 1
// 000001
// My name is Bond, James Bond
// This struct `Structure(3)` won't print...
//------------------------------------------------------------------------------------------------//
// 이는 주석이고, 컴파일러가 무시할 것이다.
// "Run" 버튼을 클릭하면 이 코드를 테스트 할 수 있다.->
// 키보드로는 "Ctrl + Enter" 단축키를 누르면 된다.

// 이 코드는 수정 가능하니, 자유롭게 수정해봐라!
// 항상 "Reset" 버튼을 클릭하면 원래 코드로 돌아간다.->

#[allow(dead_code)]
fn main_hello_world() {
    // 줄 주석의 예제
    // 줄의 시작 부분에 두개의 슬래쉬들을 주목하라
    // 여기에 쓰인 내용은 컴파일러가 읽지 않는다

    // println!("Hello, world!");
    // 실행해봐라. 결과를 봤으면 두 슬래쉬를 지우고 다시 실행해보도록.

    /*
     * 이번에는 다른 형식인 블럭 커멘트이다. 일반적으로
     * 줄 주석이 추천하는 방식이지만 블록 주석은
     * 임시적으로 많은 양의 코드를 비활성화 할 때
     * 유용하게 사용된다. /* 블록 주석은 /* 중첩 사용이 가능하고, */ */
     * 그래서 단 몇 키 입력으로 모든 라인을 주석 처리 할 수 있다.
     * 이 main() 함수에서 /*/*/* 시험해봐라!  */*/*/
     */

    /*
    참고, 이전 문단에서 '*'은 그저 스타일을 위해서 사용한 것.
    실제로는 필요하지 않다.
    */

    // 블록 주석을 통해 쉽게 표현을 조작할 수 있음을 보라.
    // 줄 주석으론 못함. 주석 식별자를 제거하면
    // 결과가 달라질 것이다:
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}
//Is `x` 10 or 100? x = 10