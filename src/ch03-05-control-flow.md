## 흐름 제어문

어떤 조건이 참인지에 따라 특정 코드를 실행하고, 어떤 조건을 만족하는
동안 특정 코드를 반복 수행하는 기능은 대부분의 프로그래밍 언어에서
기본 재료로 사용됩니다. 러스트 코드의 실행 흐름을 제어하도록 해주는
가장 일반적인 재료는 `if` 표현식과 반복문입니다.

### `if` 표현식

`if` 표현식은 여러분의 코드가 조건에 따라 분기할 수 있도록 해줍니다.
조건을 제공한 뒤 다음과 같이 기술하는 식이죠. “만약 조건을 만족하면,
이 코드 블록을 실행하세요. 그렇지 않다면 코드 블록을 실행하지 마세요.”

`if` 표현식을 알아보기 위해서 여러분의 *projects* 디렉토리에 *branches*라는
새 프로젝트를 생성합시다. *src/main.rs* 파일에 다음을 입력하세요:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

모든 `if` 표현식은 `if`라는 키워드로 시작하고 그 뒤에
조건이 옵니다. 위의 경우 조건은 변수 `number`가 5보다 작은
값인지 검사합니다. 조건이 참일 때 실행하는 코드 블록은
조건 바로 뒤 중괄호로 된 블록에 배치됩니다. `if` 표현식의
조건과 관련된 코드 블록은 2장의
[“비밀번호와 추리값을 비교하기”][comparing-the-guess-to-the-secret-number]<!-- ignore -->절에서
다뤘던 `match`식의 *갈래 (arms)* 와 마찬가지로 갈래로
불리기도 합니다.

추가로 위의 경우처럼 `else` 표현식을 붙일 수도 있는데,
이는 프로그램에게 해당 조건이 거짓일 경우 실행되는 코드 블록을
제공합니다. `else` 표현식이 없고 조건식이 거짓이라면,
프로그램은 `if` 블록을 생략하고 다음 코드로
넘어갑니다.

이 코드를 실행해보면 다음과 같은 결과를 얻을 수 있습니다:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

조건을 `false`로 만드는 값으로 `number`의 값을 변경하면 어떤 일이
일어나는지 봅시다:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

프로그램을 다시 실행시키면 다음과 같은 결과를 보게 됩니다:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

또한 이 코드의 조건식이 *반드시* `bool` 이어야 한다는 점을 주목할
가치가 있습니다. 조건식이 `bool`이 아니면 에러가 발생합니다. 예를 들자면
아래 코드를 실행해보세요:

<span class="filename">파일명: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

이 경우에는 `if` 조건식의 결과가 `3`이고, 러스트는 에러를
발생시킵니다.

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

이 에러는 러스트가 `bool`을 예상했으나 정수값을 받았다는 것을 알려줍니다.
Ruby나 JavaScript 같은 언어와 달리 러스트는 부울린 타입이 아닌 값을 부울린
타입으로 자동 변환하지 않습니다. `if`문에는 항상 명시적으로 부울린 타입의
조건식을 제공해야 합니다. 예를 들어 어떤 숫자가 `0`이 아닌 경우에만
`if` 코드 블록을 실행시키고자 한다면, 다음과 같이 `if` 표현식을 바꾸면
됩니다:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

이 코드를 실행시키면 `number was something other than zero`가 출력될 것입니다.

#### `else if`로 여러 조건식 다루기

`if`와 `else` 사이에 `else if`를 조합하면 여러 조건식을 사용할 수
수 있습니다. 예를 들면:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

이 프로그램은 분기 가능한 4개의 경로가 있습니다. 실행하면 다음과
같은 결과를 보게 됩니다:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

이 프로그램을 실행하면 각각의 `if` 표현식을 순차적으로 검사하고 조건이
참일때의 첫번째 본문을 실행합니다. 6이 2로 나누어 떨어지지만
`number is divisible by 2`나 `number is not divisible by 4, 3, or 2`와
같은 `else` 블록의 텍스트가 출력되지 않는다는 점을 주목하세요.
이는 러스트가 처음으로 `true`인 조건의 본문을 실행하고나면 나머지는
검사도 하지 않기 때문입니다.

`else if` 표현식을 너무 많이 사용하면 코드가 복잡해질 수 있으므로, 표현식이
두 개 이상이면 코드를 리팩토링하는 것이 좋습니다. 6장에서는 이런 경우에
적합한 `match`라는 러스트의 강력한 분기 구조에 대해 설명합니다.

#### `let` 구문에서 `if` 사용하기

`if`는 표현식이기 때문에 예제 3-2처럼 변수에 결과를 할당하기
위하여 `let` 구문의 우변에 사용할 수 있습니다.

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

<span class="caption">예제 3-2: `if` 표현식의 결과를 변수에
할당하기</span>

변수 `number`에는 `if` 표현식을 계산한 결과값이 바인딩될 것입니다.
코드를 실행해서 무슨 일이 벌어지는지 봅시다:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

코드 블록은 블록 안의 마지막 표현식을 계산하고, 숫자는
그 자체로 표현식임을 기억하세요. 위의 경우 전체 `if`
표현식의 값은 실행되는 코드 블록에 따라 결정됩니다.
그렇기에 `if` 표현식의 각 갈래의 결과값은 같은 타입이어야
합니다. 예제 3-2에서 `if` 갈래와 `else` 갈래는 모두
`i32` 정수값입니다. 하지만 만약 다음 예제처럼 타입이
다르면 어떻게 될까요?

<span class="filename">파일명: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

이 코드를 컴파일하려고 하면 에러가 발생합니다. `if`와 `else` 갈래
값의 타입이 호환되지 않고, 러스트는 프로그램의 어느 지점에 문제가
있는지 정확히 알려줍니다:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

`if` 블록은 정수값을 계산하는 표현식이고 `else` 블록은 문자열로
산출되는 표현식입니다. 이런 형태의 코드가 동작하지 않는 이유는 변수가 가질
수 있는 타입이 오직 하나이기 때문입니다. 러스트는 컴파일 시점에 `number`
변수의 타입이 무엇인지 확실히 알 필요가 있습니다. 그래야 컴파일 시점에
`number`가 사용되는 모든 곳에서 해당 타입이 유효한지 검증할 수 있으니까요.
러스트에서는 `number`의 타입이 런타임에 정의되도록 할 수 없습니다;
컴파일러가 어떤 변수에 대해 여러 타입에 대한 가정값을 추적해야 한다면
컴파일러는 더 복잡해지고 보장할 수 있는 것들이 줄어들 것입니다.

### 반복문을 이용한 반복

코드 블록을 한 번 이상 수행하는 일은 자주 쓰입니다. 반복 작업을 위해서,
러스트는 몇 가지 *반복문 (loop)* 을 제공하는데 이는 루프 본문의 시작부터
끝까지 수행한 뒤 다시 처음부터 수행합니다. 반복문을 실험해보기 위해
*loops*라는 이름의 새 프로젝트를 만듭시다.

러스트에는 `loop`, `while`, 그리고 `for`라는 세 종류의 반복문이 있습니다. 하나씩 써봅시다.

#### `loop`로 코드 반복하기

`loop` 키워드는 여러분이 그만두라고 명시적으로 알려주기 전까지, 혹은 영원히
코드 블록을 반복 수행하도록 해줍니다.

일례로 *loops* 디렉토리의 *src/main.rs* 코드를 다음과 같이
바꿔보세요:

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

이 프로그램을 실행시키면, 우리가 프로그램을 강제로 정지시키기 전까지
`again!`이 계속 반복적으로 출력되는 것을 보게 됩니다. 대부분의 터미널은 단축키
<span class="keystroke">ctrl-c</span>를 눌러서 무한루프에 빠진 프로그램을
정지시키는 기능을 지원합니다. 한번 시도해 보세요:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

기호 `^C`는 우리가 <span class="keystroke">ctrl-c</span>를
누른 지점을 표시합니다. 코드가 정지 신호를 받은 시점에 따라
`^C` 이후에 `again!`이 출력될 수도 안될 수도
있습니다. 

다행히 러스트에서는 코드를 사용하여 루프에서 벗어나는 방법도 제공합니다.
루프 안에 `break` 키워드를 집어넣으면 루프를 멈춰야 하는 시점을 프로그램에게
알려줄 수 있습니다. 2장의 [“정답을 맞춘 후 종료하기”][quitting-after-a-correct-guess]<!-- ignore -->절의
추리 게임 코드에서 사용자가 정답을 추리하여 게임에서
이겼을 경우 프로그램을 종료하기 위해 했었던 일을
상기해보세요.

추리 게임에서는 `continue`도 사용했었는데, 이는 프로그램에게
이번 회차에서 루프에 남은 코드를 넘겨버리고 다음 회차로 넘어가라고
알려줍니다.

#### 반복문에서 값 반환하기

`loop`의 용례 중 하나는 어떤 스레드가 실행 완료되었는지 검사하는 등
실패할지도 모르는 연산을 재시도할 때입니다. 여기서 해당 연산의 결과를
이후의 코드에 전달하고 싶을 수도 있습니다. 이를 위해서는 루프 정지를
위해 사용한 `break` 표현식 뒤에 반환하고자 하는 값을 넣으면 됩니다;
해당 값은 아래와 같이 반복문 밖으로 반환되여 사용 가능하게
됩니다:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

반복문 전에 `counter`라는 이름의 변수를 선언하여 `0`으로 초기화
했습니다. 그 다음 `result`라는 변수를 선언하여 반복문으로부터
반환된 값을 저장하도록 했습니다. 반복문의 매 회차 마다
`counter` 변수에 `1`을 더한 후 값이 `10`과 같은지 검사합니다.
그런 경우 `break` 키워드와 `counter * 2` 값을 사용하였습니다.
루프 뒤에는 `result`에 값을 할당하는 구문을 끝내기 위해 세미콜론을
붙였습니다. 결과적으로 `result`의 값이 `20`으로 출력됩니다.

#### 루프 라벨로 여러 반복문 사이에 모호함 없애기

만일 루프 안에 루프가 있다면, `break`와 `continue`는 해당 지점의 바로 바깥쪽
루프에 적용됩니다. 루프에 *루프 라벨 (loop label)* 을 추가적으로 명시하면
`break`나 `continue`와 함께 이 키워드들이 바로 바깥쪽 루프 대신 라벨이 적힌
특정한 루프에 적용되도록 할 수 있습니다. 루프 라벨은 반드시 작은 따옴표로
시작해야 합니다. 아래에 루프가 두 개 중첩된 예제가 있습니다:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

바깥쪽 루프는 `'counting_up` 이라는 라벨이 붙어있고, 0에서부터 2까지
카운트합니다. 라벨이 없는 안쪽 루프는 10에서 9까지 거꾸로 카운트합니다.
라벨이 명시되지 않은 첫번째 `break`는 안쪽 루프만 벗어납니다. `break 'counting_up;`
구문은 바깥쪽 루프를 탈출할 것입니다. 이 코드는 다음을 출력합니다:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

#### `while`을 이용한 조건 반복문

반복문 내에서 조건을 검사하는 작업도 자주 사용됩니다. 조건문이 `true`인
동안에는 계속 반복하는 형태죠. 조건문이 `true`가 아니게 될 때 프로그램은
`break`를 호출하여 반복을 종료합니다. 이러한 반복문 형태는 `loop`, `if`,
`else`와 `break`의 조합으로 구현할 수 있습니다; 여러분이 원하신다면 그렇게
시도해볼 수 있습니다. 하지만 이러한 패턴은 매우 흔하기 때문에 러스트에서는
`while` 반복문이라 일컫는 구조가 내장되어 있습니다. 예제 3-3은 `while`을
사용하여 코드를 3번 반복 실행하면서 매번 카운트를 깎고 난 다음, 반복문 후에
다른 메세지를 출력하고 종료합니다.

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

<span class="caption">예제 3-3: `while` 반복문을 사용하여 조건이 참인
동안 코드를 반복 실행하기</span>

이 구조는 `loop`, `if`, `else` 및 `break`를 사용할 때 필요하게 될
많은 중첩 구조를 제거하고 코드를 더 깔끔하게 만듭니다. 조건식이 `true`로
계산되는 동안 코드가 실행되고, 그렇지 않으면 반복문을 벗어납니다.

#### `for`를 이용한 컬렉션에 대한 반복문

`while`를 사용하여 배열과 같은 컬렉션의 각 요소에 대한 반복문을
작성할 수 있습니다. 한 가지 예로 예제 3-4의 반복문은
`a`라는 배열의 각 요소를 출력합니다.

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

<span class="caption">예제 3-4: `while` 반복문을 사용하여 컬렉션의
각 요소 순회하기</span>

위의 코드는 배열의 요소들을 훑기 위해 숫자를 셉니다. 인덱스 `0`을 시작으로
배열의 마지막 인덱스에 도달할 때까지 반복합니다 (위의 경우 `index < 5`가
`true`가 아닐때까지 입니다). 이 코드를 실행하면 배열의 모든 요소가 출력될
것입니다:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

기대한대로 배열의 다섯 개의 값이 모두 터미널에 출력됩니다 `index`가
어떤 시점에서 `5`에 도달하더라도 반복문은 이 배열로부터 6번째 값을
얻어오기 전에 실행 종료됩니다.

하지만 이런 접근 방식은 에러가 발생하기 쉽습니다; 즉 인덱스의 길이가
부정확하면 패닉을 발생시키는 프로그램이 될 수 있습니다. 예를 들어, `a`
배열이 네 개의 요소를 갖도록 정의 부분을 변경했는데 `while index < 4`의
조건문을 고치는걸 잊어버린다면 코드는 패닉을 일으킬 것입니다. 또한
컴파일러가 루프의 매 반복 회차마다 인덱스가 범위 안에 있는지에 대한
조건문 검사를 수행하는 코드를 붙이기 때문에 느려집니다.

좀 더 간편한 대안으로 `for` 반복문을 사용하여 컬렉션의 각 아이템에 대하여 임의의
코드를 수행시킬 수 있습니다. `for` 반복문은 예제 3-5의 코드처럼 생겼습니다.

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

<span class="caption">예제 3-5: `for` 반복문을 이용하여 콜랙션의
각 요소 순회하기</span>

이 코드를 실행하면 예제 3-4의 결과와 동일한 결과를 보게 됩니다.
그보다 더 중요한 것은 이렇게 함으로써 코드의 안전성이 강화되고
배열의 끝을 넘어서거나 끝까지 가지 못해서 몇 개의 아이템을 놓쳐서
발생할 수도 있는 버그의 가능성을 제거했다는 것입니다.

`for` 루프를 사용하면 여러분이 배열 내 값의 개수를
변경시키더라도 수정해야 할 다른 코드를 기억해둘 필요가
없어질 겁니다.

이러한 안전성과 간편성 덕분에 `for` 반복문은 러스트에서 가장 흔하게
사용되는 반복문 구성요소가 되었습니다. 심지어 `while` 반복문을 사용했던
예제 3-3의 카운트다운 예제처럼 어떤 코드를 몇 번 정도 반복하고
싶은 경우라도, 대부분의 러스타시안들은 `for` 반복문을 이용할 겁니다.
표준 라이브러리가 제공하는 `Range` 타입을 이용하면 특정 횟수만큼의
반복문을 구현할 수 있는데, `Range`는 어떤 숫자에서 시작하여 다른 숫자 종료
전까지의 모든 숫자를 차례로 생성해줍니다.

`for` 반복문을 이용한 카운트다운 구현은 아래처럼 생겼습니다. 여기서 아직 살펴보지
않았던 `rev` 메서드는 범위값을 역순으로 만들어줍니다:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

이 코드가 좀 더 괜찮죠?

## 정리

해냈군요! 정말 긴 장이었습니다: 여러분은 변수, 스칼라 타입 및
복합 타입, 함수, 주석, `if` 표현식, 그리고 루프에 대해 배웠습니다!
이 장에서 다룬 개념들을 연습하고 싶다면 아래 프로그램 만들기에
도전해보세요:

* 화씨 온도와 섭씨 온도 간 변환하기
* *n*번째 피보나치 수 생성하기
* 크리스마시 캐롤 “The Twelve Days of Christmas” 노래의 반복성을
  활용하여 가사 출력해보기

다음으로 넘어갈 준비가 되셨다면, 이번에는 다른 프로그래밍 언어에는
*흔치 않은* 러스트의 개념인 소유권 (ownership) 에 대해 알아보겠습니다.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[quitting-after-a-correct-guess]:
ch02-00-guessing-game-tutorial.html#quitting-after-a-correct-guess
