## 패턴이 사용될 수 있는 모든 곳

패턴은 러스트 코드 곳곳에 튀어나오며, 여러분도 모르는 사이에 이미 많이 사용하고
있었을 겁니다! 이번 절에서는 패턴을 사용할 수 있는 모든 코드상의 위치에 대해
설명합니다.

### `match` 갈래

6장에서 설명한 것처럼 패턴은 `match` 표현식의 갈래에서 사용됩니다.
공식적으로 `match` 표현식은 다음과 같이 `match` 키워드, 매칭시킬 값,
그리고 패턴 및 그 패턴과 값이 매칭될 경우 실행될 표현식으로 구성된
하나 이상의 갈래로 정의됩니다:

```text
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

예를 들면 아래는 Listing 6-5에서 변수 `x`에 있는 `Option<i32>` 값을
매칭시키는 `match` 표현식입니다:

```rust,ignore
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

이 `match` 표현식에 있는 패턴은 각 화살표 왼쪽에 위치한 `None`과
`Some(i)`입니다.

`match` 표현식에 대한 한 가지 요건은 `match` 표현식의 값에 대한 모든 경우의
수를 고려해야 한다는 의미에서 *빠짐없어야 (exhaustive)* 한다는 것입니다.
모든 가능성을 포괄하는 것을 보장하는 방법 한가지는 마지막 갈래에 캐치올
(catchall) 패턴을 사용하는 것입니다: 예를 들면 어떤 값에 매칭되는 변수명은
절대 실패할 수 없게 되어 나머지 모든 경우를 포괄합니다.

`_`라는 특정 패턴은 어떤 값에라도 매칭되지만 변수에 값을
묶지 않으므로 마지막 매치 갈래에 자주 사용됩니다.
`_` 패턴은 예를 들면 지정되지 않은 값들을 무시하고
싶을때 유용할 수 있습니다. 이 장의
[“패턴에서 값 무시하기”][ignoring-values-in-a-pattern]<!-- ignore -->절에서
`_` 패턴에 대해 더 자세히 다루겠습니다.

### `if let` 조건 표현식

6장에서 주로 하나의 경우에만 매칭하는 `match`를 더 짧게 작성하는
방법으로써 `if let` 표현식을 사용하는 법을 다루었습니다. 추가적으로
`if let`은 `if let`의 패턴에 값이 매칭되지 않을 때 실행되는
코드가 포함된 `else`를 가질 수 있습니다.

Listing 18-1은 `if let`, `else if`, `else if let` 표현식을 섞어서
매칭할 수 있음을 알려주는 코드입니다. 그렇게 하면 패턴과 비교할
값을 하나만 표현할 수 있는 `match` 표현식보다 더 유연하게 사용할
수 있습니다. 또한 러스트에서는 일련의 `if let`, `else if`,
`else if let` 갈래들의 조건식이 서로 관련될 필요도 없습니다.

Listing 18-1의 코드는 여러 개의 조건을 연속적으로 검사하여
배경의 색상을 결정합니다. 이 예제에서는 실제 프로그램에서
사용자 입력으로 받을 수 있는 하드코딩된 값이 들어있는 변수를
만들었습니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-01/src/main.rs}}
```

<span class="caption">Listing 18-1: `if let`, `else if`, `else if let`, `else`의
혼합</span>

사용자가 즐겨찾는 색을 지정한 경우 그 색상이 배경으로 사용됩니다.
즐겨찾는 색상이 지정되지 않았고 오늘이 화요일이면, 배경색은 녹색이
됩니다. 그렇지 않은 경우, 사용자가 자신의 나이를 문자열으로 지정했고
이를 성공적으로 숫자로 파싱할 수 있다면, 색상은 숫자값에 따라 보라
혹은 주황이 됩니다. 이 조건 중 어디에도 해당되지 않으면 배경색은
파란색이 됩니다.

이러한 조건부 구조는 복잡한 요구사항을 지원할 수 있게 해줍니다. 위의
하드코딩된 값을 사용하면 이 예제는 `Using purple as the background color`를
출력할 것입니다.

`if let` 또한 `match` 갈래와 같은 방식으로 쉐도잉 변수를 도입할 수
있다는 것을 알 수 있습니다: 라인 `if let Ok(age) = age`는 `Ok` variant
내의 값을 추출한 새로운 쉐도잉된 `age` 변수를 도입합니다. 이는
`if age > 30`을 그 블록 안에 위치시켜야 함을 뜻합니다: 이 두 조건을
`if let Ok(age) = age && age > 30`로 조합할 수는 없습니다.
30과 비교하려는 쉐도잉된 `age`는 새로운 스코프가 중괄호로 시작되기
전에는 유효하지 않습니다.

`if let` 표현식의 단점은 `match` 표현식과는 다르게 컴파일러가 해당 구문이
모든 경우를 빠짐없이 포괄하는지 검사하지 않는다는 점입니다. 예제의 마지막
`else`절을 생략하여 처리되지 않는 경우가 생기더라도 컴파일러는 이에 따라 발생할
수 있는 논리적 버그를 경고해주지 않습니다.

### `while let` 조건 루프

`if let`과 구조가 비슷한 `while let` 조건 루프는 패턴이 계속
매칭되는 동안 `while` 루프를 실행할 수 있게 해줍니다. Listing
18-2에서는 벡터를 스택처럼 사용하여 벡터의 값을 푸시된 역순으로
출력하는 `while let` 루프를 코딩했습니다.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-02/src/main.rs:here}}
```

<span class="caption">Listing 18-2: `while let`을 사용하여 `stack.pop()`이
`Some`을 반환하는 한 값을 출력하기</span>

이 예제는 3, 2, 1을 출력합니다. `pop` 메서드는 벡터에서 마지막 요소를
가져와서 `Some(value)`를 반환합니다. 벡터가 비어있다면 `pop`은 `None`을
반환합니다. `while` 루프는 `pop`이 `Some`을 반환하는 한 블록의 코드를
계속 실행됩니다. `pop`이 `None`을 반환하면 루프는 멈춥니다. `while let`을
사용하여 스택의 모든 요소를 팝할 수 있습니다.

### `for` 루프

`for` 루프에서 키워드 `for` 바로 뒤에 오는 값은 패턴입니다.
예를 들어 `for x in y`에서는 `x`가 패턴입니다. Listing 18-3은
`for` 루프에서 패턴을 사용하여 `for` 루프의 일부로서 튜플을
해체 혹은 분해하는 방법을 보여줍니다.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-03/src/main.rs:here}}
```

<span class="caption">Listing 18-3: `for` 루프에서 패턴을 사용하여
튜플 분해하기</span>

Listing 18-3의 코드는 다음을 출력할 것입니다:

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-03/output.txt}}
```

`enumerate` 메서드를 사용하여 반복자를 조정하여 값과 해당 값에 대한
인덱스를 생성하여 튜플에 배치합니다. 생성된 첫 번째 값은 튜플
`(0, 'a')`입니다. 이 값을 패턴 `(index, value)`에 매칭시키면
`index`는 `0`이 되고 `value`는 `'a'`가 되어 출력 결과의 첫 번째
줄을 출력합니다.

### `let` 구문

이번 장의 전까지는 `match`나 `if let`과 함께 패턴을 사용하는 것에 대해서만
명시적으로 설명했지만, 실은 다른 곳에서도 패턴을 사용해 왔는데 `let` 구문도
여기 해당합니다. 예를 들어, 아래와 같이 `let`을 사용한 간단한 변수 할당문을
살펴봅시다:

```rust
let x = 5;
```

눈치채셨을지 모르겠지만, 이와 같은 `let` 구문을 사용할 때마다
여러분은 패턴을 사용했던 것입니다! 좀 더 공식적으로 `let` 구문은
다음과 같이 생겼습니다:

```text
let PATTERN = EXPRESSION;
```

`let x = 5;`처럼 `PATTERN` 자리에 변수명이 있는 구문에서, 이
변수명이 패턴의 매우 단순한 형태일 뿐입니다. 러스트는 표현식을
패턴과 비교하여 찾은 이름을 할당합니다. 따라서 `let x = 5;`
예제에서 `x`는 “이 패턴에 매칭되는 값을 변수 `x`에 대입해라”라는
의미의 패턴입니다. `x`라는 이름이 전체 패턴이므로 이 패턴은 사실상
“값이 무엇이든간에 전부 변수 `x`에 바인딩해라”라는 뜻이 됩니다.

패턴 매칭의 관점에서 `let` 좀 더 명확하게 보기 위해서, `let`으로
튜플을 분해하는 패턴을 사용하는 Listing 18-4을 살펴봅시다.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-04/src/main.rs:here}}
```

<span class="caption">Listing 18-4: 패턴을 사용해 튜플을 분해하여
세 변수를 한번에 만들기</span>

여기서는 튜플을 패턴에 매칭합니다. 러스트는 값 `(1, 2, 3)`을 패턴
`(x, y, z)`와 비교하고 이 값이 패턴과 매치되는지 확인하고, `1`을
`x`에, `2`를 `y`에, 그리고 `3`을 `z`에 바인딩합니다. 이 튜플 패턴을
이 안에 있는 세 개의 개별적인 변수 패턴이 중첩된 것으로 생각할 수 있습니다.

패턴의 요소 개수가 주어진 튜플의 요소 개수와 다르면, 전체
타입이 일치하지 않아서 컴파일러 에러가 발생합니다. 예를 들어
Listing 18-5는 세 개의 요소가 있는 튜플을 두 개의 변수로 해체하는
시도를 보여주는데, 이는 작동하지 않을 것입니다.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-05/src/main.rs:here}}
```

<span class="caption">Listing 18-5: 변수 개수가 튜플의 요소 개수와
맞지 않는 부정확한 패턴 구성</span>

이 코드의 컴파일을 시도하면 아래와 같은 타입 에러가 발생합니다:

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-05/output.txt}}
```

이 에러를 고치기 위해서는 `_`나 `..`를 사용하여 튜플의 값을
하나 혹은 그 이상 무시할 수 있는데, 이에 대해서는
[“패턴에서 값 무시하기”][ignoring-values-in-a-pattern]<!-- ignore -->절에서
살펴볼 것입니다. 패턴에 너무 많은 변수가 있는 것이 문제라면, 해결책은
변수를 제거하여 변수 수가 튜플의 요소 개수와 같도록 타입을 일치시키는
것입니다.

### 함수 매개변수

함수 매개변수도 패턴이 될 수 있습니다. Listing 18-6의 코드는
`foo`라는 이름의 함수를 선언하고 타입 `i32`인 `x`라는 매개변수 하나를
받는데, 이제 친숙하게 보일 것입니다.

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-06/src/main.rs:here}}
```

<span class="caption">Listing 18-6: 매개변수에서 패턴을 사용하는
함수 시그니처</span>

`x` 부분이 패턴입니다! `let`에서 했던 것처럼 함수 인자의 튜플을 패턴과
매치시킬 수 있습니다. Listing 18-7은 함수에 값을 넘길때 튜플의 값을
분할합니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-07/src/main.rs}}
```

<span class="caption">Listing 18-7: 튜플을 분해하는 매개변수를 가진
함수</span>

이 코드는 `Current location: (3, 5)`를 출력합니다. 값 `&(3, 5)`는
패턴 `&(x, y)`에 매치되어 `x`는 `3`이 되고 `y`는 `5`가 됩니다.

13장에서 설명한 것처럼 클로저는 함수와 유사하기 때문에,
클로저 매개변수 리스트에서도 함수 매개변수 리스트와 동일한
방식으로 패턴을 사용할 수 있습니다.

지금까지 패턴을 사용하는 여러 가지 방법을 살펴보았지만, 패턴을 사용할 수 있는
모든 곳에서 패턴이 동일하게 작동하는 것은 아닙니다. 어떤 곳에서는 패턴이
*반박이 불가능 (irrefutable)* 해야 합니다; 다른 곳에서는 *반박이 가능 (refutable)*
할 수 있습니다. 다음에는 이 두 가지 개념에 대해 설명하겠습니다.

[ignoring-values-in-a-pattern]:
ch18-03-pattern-syntax.html#ignoring-values-in-a-pattern
