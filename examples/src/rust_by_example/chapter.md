## Introduction
1. Hello World
   1.1. Comments
   1.2. Formatted print
     1.2.1. Debug
     1.2.2. Display
       1.2.2.1. Testcase: List
     1.2.3. Formatting
2. Primitives '기초 요소'
   2.1. Literals and operators '리터럴(고정된 값을 나타내는 표기법)과 연산자'
   2.2. Tuples
   2.3. Arrays and Slices
3. Custom Types
   3.1. Structures
   3.2. Enums
     3.2.1. use
     3.2.2. C-like
     3.2.3. Testcase: linked-list
   3.3. constants '상수'
4. Variable Bindings '변수 바인딩'
   4.1. Mutability '가변성'
   4.2. Scope and Shadowing '범위 및 섀도잉(그림자처럼 따라다닌다)'
   4.3. Declare first '먼저 선언'
   4.4. Freezing
5. Types
   5.1. Casting '형변환'
   5.2. Literals '리터럴(고정된 값을 나타내는 표기법)'
   5.3. Inference
   5.4. Aliasing '별칭'
6. Conversion
   6.1. From and Into
   6.2. TryFrom and TryInto
   6.3. To and from Strings
7. Expressions '표현'
8. Flow of Control
   8.1. if/else
   8.2. loop
     8.2.1. Nesting and labels 
     8.2.2. Returning from loops
   8.3. while
   8.4. for and range
   8.5. match
     8.5.1. Destructuring '역구조화' 
       8.5.1.1. tuples
       8.5.1.2. arrays/slices
       8.5.1.3. enums
       8.5.1.4. pointers/ref
       8.5.1.5. structs
     8.5.2. Guards
     8.5.3. Binding
   8.6. if let
   8.7. while let
9. Functions
   9.1. Methods
   9.2. Closures '클로저'
     9.2.1. Capturing
     9.2.2. As input parameters
     9.2.3. Type anonymity 'Type 익명'
     9.2.4. Input functions
     9.2.5. As output parameters
     9.2.6. Examples in std
       9.2.6.1. Iterator::any
       9.2.6.2. Searching through iterators
   9.3. Higher Order Functions
   9.4. Diverging functions
10. Modules
    10.1. Visibility
    10.2. Struct visibility
    10.3. The use declaration
    10.4. super and self
    10.5. File hierarchy
11. Crates
    11.1. Creating a Library
    11.2. Using a Library
12. Cargo
    12.1. Dependencies
    12.2. Conventions
    12.3. Tests
    12.4. Build Scripts
13. Attributes
    13.1. dead_code
    13.2. Crates
    13.3. cfg
      13.3.1. Custom