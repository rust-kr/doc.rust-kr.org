## 부록 A: 키워드

다음 목록은 러스트 언어에서 현재 또는 미래에 사용하기 위해 예약된
키워드입니다. 따라서 키워드는 식별자로 사용할 수 없습니다.
([“원시 식별자”][raw-identifiers]<!-- ignore --> 절에서
설명할 원시 식별자는 예외입니다.) 식별자는 함수, 변수, 매개변수,
구조체 필드, 모듈, 크레이트, 상수, 매크로, 정적 값, 속성, 타입,
트레잇, 또는 라이프타임의 이름입니다.

[raw-identifiers]: #raw-identifiers

### 현재 사용중인 키워드

다음은 현재 사용중인 키워드의 목록과 각 키워드의 기능을 설명한
것입니다.

* `as` - 기초 타입 변환하기, 어떤 아이템을 포함하는 특정 트레잇을 구분하기,
  `use` 문에서 아이템의 이름을 변경하기
* `async` - 현재 스레드를 블록하지 않고 `Future`를 반환하기
* `await` - `Future`의 결과가 준비될 때까지 실행을 중단하기
* `break` - 즉시 루프를 빠져나오기
* `const` - 상수 아이템이나 상수 원시 포인터 정의하기
* `continue` - 다음 루프 반복으로 계속하기
* `crate` - 모듈 경로에서 크레이트 루트를 가리키기
* `dyn` - 트레잇 객체에 대한 동적 디스패치하기
* `else` - `if`와 `if let` 제어 흐름 구조의 대안으로 사용하기
* `enum` - 열거형 정의하기
* `extern` - 외부 함수나 변수를 연결하기
* `false` - 부울린 거짓값 리터럴
* `fn` - 함수나 함수 포인터 타입 정의하기
* `for` - 반복자로부터 아이템을 반복하기, 트레잇 구현하기,
  고수준 (higher-ranked) 라이프타임 지정하기  
* `if` - 조건식의 결과에 따라 분기하기
* `impl` - 고유 기능 또는 트레잇 기능 구현하기
* `in` - `for` 루프 구문의 일부
* `let` - 변수를 바인딩하기
* `loop` - 조건 없이 무한 반복하기
* `match` - 값과 패턴을 매칭하기
* `mod` - 모듈 정의하기
* `move` - 클로저가 캡처된 모든 것을 소유하도록 만들기
* `mut` - 참조, 원시 포인터, 패턴 바인딩에서 가변성을 나타내기
* `pub` - 구조체 필드, `impl` 블록, 또는 모듈에서 공개 가시성을 나타내기
* `ref` - 참조로 바인딩하기
* `return` - 함수로부터 반환하기
* `Self` - 현재 정의하거나 구현하고 있는 타입에 대한 별칭
* `self` - 메서드의 주체 혹은 현재 모듈
* `static` - 전역 변수 또는 프로그램 실행 동안 지속되는 라이프타임
* `struct` - 구조체 정의하기
* `super` - 현재 모듈의 부모 모듈
* `trait` - 트레잇 정의하기
* `true` - 부울린 참값 리터럴
* `type` - 타입 별칭 또는 연관 타입 정의하기
* `union` - [유니온][union] 정의하기; 유니온 선언에서만
  키워드로 사용됩니다
* `unsafe` - 안전하지 않은 코드, 함수, 트레잇, 또는 구현을 나타내기
* `use` - 심볼을 스코프 내로 가져오기
* `where` - 어떤 타입을 제한하는 구절 나타내기
* `while` - 표현식의 결과에 따라 조건 반복하기

[union]: ../reference/items/unions.html

### 미래에 사용하기 위해 예약된 키워드

다음 키워드들은 아직 어떤 기능도 가지고 있지 않지만, 나중에 러스트에서
사용할 가능성이 있어 예약되어 있습니다.

* `abstract`
* `become`
* `box`
* `do`
* `final`
* `macro`
* `override`
* `priv`
* `try`
* `typeof`
* `unsized`
* `virtual`
* `yield`

### 원시 식별자

*원시 식별자 (raw identifiers)* 는 일반적으로 키워드가 허용되지 않는 곳에 키워드를 사용하도록
해주는 문법입니다. `r#` 접두사를 키워드 앞에 붙이는 식으로 원시 식별자를 사용합니다.

예를 들면, `match`는 키워드죠. `match`를 함수 이름으로 사용하는 다음
함수를 컴파일 시도하면:

<span class="filename">파일명: src/main.rs</span>

```rust,ignore,does_not_compile
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

다음과 같은 에러가 발생합니다:

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

이 에러는 `match` 키워드를 함수 식별자로 사용할 수 없다는 것을
보여줍니다. `match`를 함수 이름으로 사용하려면, 다음과 같이 원시
식별자 문법을 사용해야 합니다:

<span class="filename">파일명: src/main.rs</span>

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

이 코드는 에러 없이 컴파일됩니다. 함수 정의에서 함수 이름 앞에 `r#` 접두사를 붙인
것 뿐만 아니라 `main`에서 함수를 호출할 때도 `r#` 접두사를 붙인 것에 주목하세요.

원시 식별자를 사용하면 해당 단어가 예약된 키워드일지라도 원하는 단어를
식별자로 사용할 수 있습니다. 이를 통해 식별자 이름을 더 자유롭게 선택할
수 있을 뿐만 아니라, 해당 단어들이 키워드가 아닌 언어로 작성된 프로그램과
통합할 수 있게끔 해줍니다. 또한, 원시 식별자를 사용하면 크레이트에서
사용하는 것과 다른 러스트 버전으로 작성된 라이브러리를 사용할 수 있습니다.
예를 들어 `try`는 2015 에디션에서는 키워드가 아니지만 2018 에디션에서는
키워드입니다. 2015 에디션을 사용하여 작성된 라이브러리에 의존하고 `try`
함수가 있는 경우, 2018 에디션 코드에서 해당 함수를 호출하려면 원시 식별자
구문(이 경우 `r#try`)을 사용해야 합니다. 에디션에 대한 자세한 내용은
[부록 E][부록 E]<!-- ignore -->을 참조하세요.

[appendix-e]: appendix-05-editions.html
