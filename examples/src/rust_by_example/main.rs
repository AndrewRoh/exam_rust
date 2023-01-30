// fn main() {
//     println!("8.Flow of Control ~ 8.Flow of Control");
// }
pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `any` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn any<F>(&mut self, f: F) -> bool where
    // `FnMut` meaning any captured variable may at most be
    // modified, not consumed. `Self::Item` states it takes
    // arguments to the closure by value.
        F: FnMut(Self::Item) -> bool;
}

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

    // `iter()` only borrows `vec1` and its elements, so they can be used again
    println!("vec1 len: {}", vec1.len());
    println!("First element of vec1 is: {}", vec1[0]);
    // `into_iter()` does move `vec2` and its elements, so they cannot be used again
    // println!("First element of vec2 is: {}", vec2[0]);
    // println!("vec2 len: {}", vec2.len());
    // TODO: uncomment two lines above and see compiler errors.

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`.
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // `into_iter()` for arrays yields `i32`.
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
}
// 2 in vec1: true
// 2 in vec2: false
// vec1 len: 3
// First element of vec1 is: 1
// 2 in array1: true
// 2 in array2: false
//------------------------------------------------------------------------------------------------//
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

#[allow(dead_code)]
fn main_functions_closures_output_parameters() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}
// This is a: Fn
// This is a: FnMut
// This is a: FnOnce
//------------------------------------------------------------------------------------------------//
// 제네릭 `F`를 인자로 취하는 함수를 정의하고
// `Fn`으로 바인딩하고 그를 호출한다.
fn call_me<F: Fn()>(f: F) {
    f();
}

// 입력으로 사용될 `Fn` 바인딩을 만족시키는 함수.
fn function() {
    println!("I'm a function!");
}

#[allow(dead_code)]
fn main_functions_closures_input_functions() {
    // `Fn` 바인딩을 만족시키는 클로저를 정의.
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
// I'm a closure!
// I'm a function!
//------------------------------------------------------------------------------------------------//
// `F`는 반드시 입력 이나 반환하지 않는 클로저에 대해 `Fn`을 구현해야 한다.
// 정확히 이는 `print`를 위해 필요하다.
fn apply_2<F>(f: F) where
    F: Fn() {
    f();
}

#[allow(dead_code)]
fn main_functions_closures_type_anonymity() {
    let x = 7;

    // `x`를 익명 타입으로 캡쳐하고 `Fn`을 구현한다. 이를 `print`에 저장.
    let print = || println!("{}", x);

    apply_2(print);
}
// 7
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
// 클로저를 인자로 받아서 호출하는 함수.
// <F>는 F가 "일반 유형 매개변수"임을 나타냅니다.
fn apply<F>(f: F) where
// 클로저는 입력을 받지 않고 아무것도 반환하지 않는다.
    F: FnOnce() {   // Fn() or FnMut()의 경우 에러 발생

    f();
}

// 클로저를 취하고 `i32`를 반환하는 함수
fn apply_to_3<F>(f: F) -> i32 where
// 클로저를 취하고 `i32`를 반환하는 함수
    F: Fn(i32) -> i32 {

    f(3)
}

#[allow(dead_code)]
fn main_functions_closures_input_parameters() {
    use std::mem;

    let greeting = "hello";
    // 비-복사 유형.
    // `to_owned`는 빌린 데이터에서 소유 데이터를 생성합니다.
    let mut farewell = "goodbye".to_owned();

    // 2개의 변수를 캡쳐한다 : `greeting`은 참조로 `farewell`은 값으로.
    let diary = || {
        // `greeting`은 참조에 의하므로 `Fn`이 필요하다.
        println!("I said {}.", greeting);

        // 변경은 `farewell`을 가변참조로 캡쳐되게 강제한다.
        // 여기서는 `FnMut`이 필요하다.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 수동으로 drop을 호출하면 `farewell`을 값으로 캡쳐되도록 강제한다.
        // 여기서는 `FnOnce`가 요구된다.
        mem::drop(farewell);
    };

    // 클로저를 적용하는 함수를 호출한다.
    apply(diary);

    // `double`은 `apply_to_3` trait의 범위를 만족시킨다.
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
// I said hello.
// Then I screamed goodbye!!!.
// Now I can sleep. zzzzz
// 3 doubled: 6
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_functions_closures_capturing() {
    use std::mem;

    let color = String::from("green");

    // `color`를 출력하는 클로저는 `color`를 즉각적 대여(`&`)하고
    // `print` 변수에 대여와 클로저를 저장한다. `print`가 범위에서 벗어날 때까지
    // 대여는 유지된다. `println!`은 `참조에 의해`서만 요구되므로
    // 더 제한적인 것을 강요하지 않는다.
    let print = || println!("`color`: {}", color);

    // closure를 사용 빌려온 print문을 호출한다.
    print();

    // `color`는 불변적으로 다시 빌릴 수 있습니다. 클로저는 `color`.borrow에 대한 불변 참조만 보유하기 때문입니다.
    let _reborrow = &color;
    print();

    // `print`를 마지막으로 사용한 후 이동 또는 다시 대여가 허용됩니다.
    let _color_moved = color;

    let mut count = 0;
    // `count`를 증가시키는 클로저는 `&mut count` 또는 `count` 중 하나를
    // 취할 수 있지만 `&mut count`이 덜 제한적이므로 이를 취한다.
    // 즉각적 대여한 `count`.
    //
    // `mut`가 `inc`에 필요한 이유는 `&mut`이 내부에
    // 저장되기 때문이다. 따라서 클로저의 호출은 `mut`을 요구하는 클로저를 변경한다.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 'mut'로 빌린 클로저 호출.
    inc();

    // 클로저는 나중에 호출되기 때문에 여전히 가변적으로 `count`를 빌립니다.
    // reborrow를 시도하면 오류가 발생합니다.
    // let _reborrow = &count;

    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;


    // 복사가 아닌 타입.
    let movable = Box::new(3);

    // `mem::drop`은 `T`를 요구하므로 이는 값을 취해야 한다. 복사 타입은
    // 클로저로 복사되어 원본은 변경되지 않는다. 복사가 아니면 이동되야 하고
    // 그래서 `movable`이 즉시 클로저로 이동된다.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `comsume`은 변수를 소비하므로 한번만 호출 될 수 있다.
    consume();

    // consume(); 한번 더 호출 하면 에러 발생
}
// `color`: green
// `color`: green
// `count`: 1
// `count`: 2
// `movable`: 3
//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_functions_closures() {
    let outer_var = 42;

    // 일반 함수는 주변 환경의 변수를 참조할 수 없습니다.
    // fn function(i: i32) -> i32 { i + outer_var }
    // 대신 클로저를 정의할 것을 제안합니다.

    // 클로저는 익명입니다. 여기서는 클로저를 참조에 바인딩합니다.
    // 주석은 함수 주석과 동일하지만 선택적입니다.
    // 본문을 감싸는 `{}`도 마찬가지입니다. 이 이름 없는 함수
    // 적절하게 명명된 변수에 할당됩니다.
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;

    // 클로저를 호출
    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {:?}", closure_inferred(1));
    // 한번 유추된 클로저의 유형은 다른 유형으로 다시 유추할 수 없습니다.
    println!("다른 유형으로 closure_inferred를 재사용할 수 없음: {:?}", closure_inferred(42i32));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
}
// closure_annotated: 43
// closure_inferred: 43
// 다른 유형으로 closure_inferred를 재사용할 수 없음: 84
// closure returning one: 1
//------------------------------------------------------------------------------------------------//
struct Point {
    x: f64,
    y: f64,
}

// 구현 블럭, 모든 `Point` 메소드는 여기로.
impl Point {
    // 이는 전역 메소드
    // 전역 메소드는 인스턴스를 통한 호출이 필요하지 않다
    // 이 메소드들은 일반적으로 생성자로 사용된다.
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    // 두 인자를 취하는 다른 전역 메소드:
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // 이것은 인스턴스 메소드이다.
    // `&self`는 문법적으로 `self: &Self`이고, `Self`는 호출되는 객체의 타입이다.
    // 여기서 `Self` = `Rectangle` 이다.
    fn area(&self) -> f64 {
        // `self`는 점 연산자를 통해 구조체 필드에 접근권을 준다.
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        // `abs`는 호출자의 절대 값을 `f64`로 반환하는 메소드.
        ((x1 - x2) * (y1 - y2)).abs()
    }
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }
    // 이 메소드는 호출하는 객체가 가변적일 것을 요구한다.
    // `&mut self`는 `self: &mut Self`로 번역된다.
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair`이 소유하는 자원: 두 힙에 할당된 정수
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // 이 메소드는 호출한 객체 `self: Self`로 번역되는 `self`의
    // 자원을 "소비한다."
    fn destroy(self) {
        // `self` 역구조화
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
        // `first`와 `second`는 범위 밖으로 나가 해제된다.
    }
}

// C/C++와 달리 함수 정의 순서에 제한이 없다.
#[allow(dead_code)]
fn main_functions_methods() {
    let rectangle = Rectangle {
        // 전역 메소드는 더블 콜론을 사용해 호출된다.
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // 인스턴스 메소드는 점 연산자를 사용해 호출된다.
    // 주의할 점은 첫 인자 `&self`는 암시적으로 전달된다, 예.
    // `rectangle.perimeter()` === `rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // 에러! `rectangle`은 불가변적이지만, 이 메소드는 가변한 객체를 요구한다.
    // rectangle.translate(1.0, 0.0);

    // 좋다! 가변 객체는 가변 메소드를 호출할 수 있다.
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // 에러! 이전 `destroy`의 호출은 `pair`을 "소비한다."
    //pair.destroy();
}

// Rectangle perimeter: 14
// Rectangle area: 12
// Destroying Pair(1, 2)
//------------------------------------------------------------------------------------------------//
// C/C++와 달리 함수 정의 순서에 제한이 없다.
#[allow(dead_code)]
fn main_functions() {
    // 여기서 이 함수를 사용하고 나중에 정의할 수 있습니다.
    fizzbuzz_to(10);
}

// boolean 값을 반환하는 함수
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // 코너 케이스, 조기 복귀
    if rhs == 0 {
        return false;
    }
    // 이것은 표현식입니다. 여기서는 `return` 키워드가 필요하지 않습니다.
    lhs % rhs == 0
}

// 값을 반환하지 않는 함수는 실제로는 단위 유형 `()`을 반환합니다.
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// 함수가 `()`를 반환할 때 반환 유형을 서명에서 생략할 수 있습니다.
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
// 1
// 2
// fizz
// 4
// buzz
// fizz
// 7
// 8
// fizz
// buzz
