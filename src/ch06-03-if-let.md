## `if let`을 사용한 간결한 제어 흐름

`if let` 문법은 `if`와 `let`을 조합하여 하나의 패턴만 매칭시키고
나머지 경우는 무시하도록 값을 처리하는 간결한 방법을 제공합니다.
예제 6-6의 프로그램은 `config_max` 변수의 어떤 `Option<u8>`
값을 매칭하지만 그 값이 `Some` 배리언트일 경우에만 코드를 실행시키고
싶어 하는 예제를 보여줍니다:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

<span class="caption">예제 6-6: 어떤 값이 `Some` 일 때에만 코드를 실행하도록
하는 `match`</span>

이 값이 `Some`이면 패턴 내에 있는 `max`에 `Some` 배리언트의 값을
바인딩하고 출력합니다. `None` 값에 대해서는 아무 처리도 하지 않으려고
합니다. `match` 표현식을 만족시키려면 딱 하나의 배리언트 처리 후
`_ => ()`를 붙여야 하는데, 이는 다소 성가신 보일러 플레이트
코드입니다.

그 대신, `if let`을 이용하여 이 코드를 더 짧게 쓸 수 있습니다. 아래의 코드는
예제 6-6에서의 `match`와 동일하게 동작합니다:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

`if let`은 `=`로 구분된 패턴과 표현식을 입력받습니다. 이는
`match`와 동일한 방식으로 작동하는데, 여기서 표현식은 `match`에
주어지는 것이고 패턴은 이 `match`의 첫 번째 갈래와 같습니다.
위의 경우 패턴은 `Some(max)`이고 `max`는 `Some` 내에 있는
값에 바인딩됩니다. 그렇게 되면 `match`의 갈래 안에서 `max`를
사용했던 것과 같은 방식으로 `if let` 본문 블록 내에서 `max`를
사용할 수 있습니다.

`if let`을 이용하면 여러분이 덜 타이핑하고, 덜 들여 쓰기 하고, 보일러
플레이트 코드를 덜 쓰게 됩니다. 하지만, `match`가 강제했던 철저한
검사를 안하게 되었습니다. `match`와 `if let` 사이에서 선택하는 것은
여러분의 특정 상황에서 여러분이 하고 있는 것에 따라, 그리고 간결함을 얻는
것이 철저한 검사를 안하게 되는 것에 대한 적절한 거래인지에 따라 달린 문제입니다.

즉, `if let`은 한 패턴에 매칭될 때만 코드를 실행하고 다른 경우는 무시하는
`match` 문을 작성할 때 사용하는 문법 설탕 (syntax sugar) 이라고 생각하시면 됩니다.

`if let`과 함께 `else`를 포함시킬 수 있습니다. `else` 뒤에 나오는
코드 블록은 `match` 표현식에서 `_` 케이스 뒤에 나오는 코드 블록과
동일합니다. 예제 6-4에서 `Quarter` 배리언트가 `UsState` 값도 들고
있었던 `Coin` 열거형 정의부를 상기해 보세요.
만일 쿼터가 아닌 모든 동전을 세고 싶은 동시에 쿼터 동전일
경우도 알려주고 싶다면, 아래와 같이 `match`문을 쓸 수도 있을
겁니다:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

혹은 아래와 같이 `if let`과 `else` 표현식을 이용할 수도 있겠지요:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

만일 여러분의 프로그램이 `match`로 표현하기에는 너무 장황한 로직을 가지고 있는 경우라면,
러스트 도구 상자에 `if let`도 있음을 기억하세요.

## 정리

지금까지 열거형을 사용하여 열거한 값의 집합 중에서 하나가 될 수 있는 커스텀 타입을
만드는 방법에 대해 알아보았습니다. 표준 라이브러리의 `Option<T>` 타입이 타입
시스템을 사용하여 에러를 방지하는 데 어떻게 도움이 되는지도 살펴봤습니다.
열거형 값에 데이터가 있는 경우, 처리해야 하는 경우의 수에 따라 `match`나
`if let`을 사용하여 해당 값을 추출하여 사용할 수 있습니다.

여러분은 이제 구조체와 열거형을 이용해 원하는 개념을 표현할 수 있습니다.
또한, 여러분의 API 내에 커스텀 타입을 만들어서 사용하면, 작성한 함수가
원치 않는 값으로 작동하는 것을 컴파일러가 막아주기 때문에 타입 안정성도
보장받을 수 있습니다.

여러분의 사용자에게 사용하기 직관적이고 필요로 하는 것만 정확하게
노출된, 잘 조직된 API를 제공하기 위해서, 이제 러스트의 모듈로
넘어갑시다.

