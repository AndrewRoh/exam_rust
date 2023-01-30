// fn main() {
//     println!("8.Flow of Control ~ 8.Flow of Control");
// }

fn main() {
    use std::mem;

    let color = String::from("green");

    // `color`를 출력하는 클로저는 `color`를 즉각적 대여(`&`)하고
    // `print` 변수에 대여와 클로저를 저장한다. `print`가 범위에서 벗어날 때까지
    // 대여는 유지된다. `println!`은 `참조에 의해`서만 요구되므로
    // 더 제한적인 것을 강요하지 않는다.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`.
    let _reborrow = &color;
    print();

    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;


    let mut count = 0;
    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow.
    inc();

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    // let _reborrow = &count;
    // ^ TODO: try uncommenting this line.
    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;


    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    // consume();
    // ^ TODO: Try uncommenting this line.
}

//------------------------------------------------------------------------------------------------//
#[allow(dead_code)]
fn main_functions_Closures() {
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
