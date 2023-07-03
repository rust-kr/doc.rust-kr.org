## 변수와 가변성

[‘변수에 값 저장하기’][storing-values-with-variables]<!-- ignore -->
에서 언급했듯이, 변수는 기본적으로 불변 (immutable) 입니다. 이것은
러스트가 제공하는 안정성과 쉬운 동시성을 활용하는 방식으로 코드를 작성할
수 있도록 하는 넛지 (nudge, 슬며시 선택을 유도하기) 중 하나입니다.
하지만 여러분은 여전히 변수를 가변 (mutable) 으로 만들 수 있습니다.
어떻게 하는지 살펴보고 왜 러스트가 불변성을 권하는지와 어떨 때 가변성을
써야 하는지 알아봅시다.

변수가 불변일 때, 어떤 이름에 한번 값이 묶이면 그 값은 바꿀 수
없습니다. 이를 표현하기 위해, `cargo new variables`로
*projects* 디렉터리 안에 *variables*라는 프로젝트를 만들어 봅시다.

그리고, 새 *variables* 디렉터리의 *src/main.rs* 파일을 열어서
다음의 코드로 교체하세요 (아직은 컴파일되지 않습니다):

<span class="filename">파일명: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

저장하고 `cargo run`으로 프로그램을 실행해 보세요. 다음의 출력처럼
불변성 에러에 관한 에러 메시지를 받을 것입니다:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

이 예시는 컴파일러가 프로그램의 에러 찾기를 어떻게 도와주는지 보여줍니다.
컴파일러 에러가 실망스러울 수도 있겠지만, 컴파일러는 그저 여러분의 프로그램이 아직은 
원하는 대로 안전하게 동작하지 않는다고 할 뿐입니다. 컴파일러는 여러분이 좋은 프로그래머가
아니라고 한 적이 *없습니다!* 경험이 많은 러스타시안들에게조차 컴파일러 에러가 발생합니다.

``불변 변수 `x`에 두 번 값을 할당할 수 없다``라는 내용의 에러 메시지를
받았습니다. 불변 변수 `x`에 두 번째 값을 할당하려고 했기 때문이죠.

불변으로 지정한 값을 변경하려고 하는 바로 이 상황이 버그로 이어질 수
있기 때문에, 컴파일 타임 에러가 발생하는 것은 중요합니다.
만약 코드의 한 부분이 변숫값은 변하지 않는다는 전제 하에
작동하고 코드의 다른 부분이 그 값을 바꾼다면, 앞부분의 코드는
원래 지정된 일을 못할 가능성이 생깁니다. 이런 류의 버그는 발생
후 추적하는 것이 어려운데, 특히 코드의 두 번째 부분이 값을
*가끔씩만* 바꿀 때 그렇습니다. 러스트 컴파일러는 값이 바뀌지 않을
것이라고 여러분이 지정하면 실제로 그렇도록 보증합니다. 이 말은
코드를 읽고 쓸 때 값이 어디서 어떻게 바뀔지 추적할 필요가 없다는
것입니다. 따라서 여러분의 코드는 흐름을 따라가기 쉬워집니다.

하지만 가변성은 아주 유용할 수 있고, 코드 작성을 더 편하게 해 줍니다.
변수는 기본적으로 불변이더라도, 여러분이
[2장][storing-values-with-variables]<!-- ignore -->에서 했던
것처럼 변수명 앞에 `mut`을 붙여서 가변으로 만들 수 있습니다.
`mut`를 추가하는 것은 또한 미래에 코드를 읽는 이들에게 코드의 다른
부분에서 이 변수의 값이 변할 것이라는 의도를 전달합니다.

예를 들어, *src/main.rs*를 다음과 같이 바꿉시다:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

지금 이 프로그램을 실행하면, 다음의 출력을 얻습니다.

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

`mut`를 사용해 `x`의 값을 `5`에서 `6`으로 바꿀 수 있었습니다.
궁극적으로 가변성을 사용할지 말지는 여러분의 몫이고, 특정 상황에서
가장 명확하다고 생각하는 것이 어떤 것이냐에 따라 달라집니다.

### 상수

*상수 (constant)* 는 불변 변수와 비슷한데, 어떤 이름에 묶여 있는
값이고 값을 바꾸는 것이 허용되지 않지만, 변수와는 약간 다른 점들이
있습니다.

먼저, 상수는 `mut`와 함께 사용할 수 없습니다. 상수는
기본적으로 불변인 것이 아니고, 항상 불변입니다. 상수는 `let`
키워드 대신 `const` 키워드로 선언하며, 값의 타입은 *반드시*
명시되어야 합니다. 다음 절 [‘데이터 타입’][data-types]<!-- ignore -->에서
타입과 타입 명시에 대해 다룰 예정이므로, 자세한 사항은 아직 걱정하지
않아도 됩니다. 항상 타입 명시를 해야 한다는 것만 알아두세요.

상수는 전역 스코프를 포함한 어떤 스코프에서도 선언 가능하므로
코드의 많은 부분에서 알 필요가 있는 값에 유용합니다.

마지막 차이점은, 상수는 반드시 상수 표현식으로만 설정될 수 하고
런타임에서만 계산될 수 있는 결괏값으로는 안된다는 것입니다.

아래에 상수 선언의 예제가 있습니다:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

상수의 이름은 `THREE_HOURS_IN_SECONDS`이고 값은
60(분당 초의 개수), 60(시간당 분의 개수), 3(이 프로그램에서
알아둘 필요가 있는 시간의 숫자)를 모두 곱한 값입니다.
러스트의 이름 짓기 관례에서 상수는 단어 사이에 밑줄을 사용하고
모든 글자를 대문자로 쓰는 것입니다. 컴파일러는 컴파일 타임에
제한된 연산을 수행할 수 있는데, 이런 상숫값을 10,800으로
쓰는 대신 이해하고 검사하기 더 쉽게 작성할 방법을 제공해 줍니다.
[상숫값 평가에 대한 러스트 참고 자료 절][const-eval]에서 상수 선언에
사용될 수 있는 연산이 어떤 것이 있는지 더 많은 정보를 찾을 수
있습니다.

상수는 선언된 스코프 내에서 프로그램이 동작하는 전체
시간 동안 유효합니다. 이러한 특성은, 플레이어가 얻을
수 있는 점수의 최곳값이라던가 빛의 속도 같이, 프로그램의
여러 부분에서 알 필요가 있는 값들에
유용합니다.

전체 프로그램에 하드코딩된 값에 상수로써 이름을 붙이는 것은
미래의 코드 관리자에게 그 값의 의미를 전달하는 데 유용합니다.
또한 나중에 업데이트될 하드코딩된 값을
단 한 군데에서 변경할 수 있게 해 줍니다.

### 섀도잉

[2장][comparing-the-guess-to-the-secret-number]<!-- ignore -->에서
다루었던 추리 게임에서 보았듯이, 새 변수를 이전 변수명과 같은 이름으로
선언할 수 있습니다. 러스타시안들은 첫 번째 변수가 두 번째 변수에
의해 *가려졌다 (shadowed)* 라고 표현하며, 이는 해당 변수의
이름을 사용할 때 컴파일러가 두 번째 번수를 보게 될 것이라는
의미입니다. 사실상 두 번째 변수는 첫 번째 것을 가려서, 스스로를 다시
가리거나 스코프가 끝날 때까지 변수명의 사용을 가져가 버립니다.
아래처럼 똑같은 변수명과 let` 키워드의 반복으로 변수를 가릴 수
있습니다:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

이 프로그램은 먼저 `x`에 `5`라는 값을 묶어 넣습니다. 다음으로 `let x = `을
반복해 새로운 변수 `x`를 만들고, 원래 값에 `1`을 더한 값을 대입해서
`x`의 값은 이제 `6`이 됩니다. 그 후 중괄호를 사용하여 만들어진 안쪽 스코프
내에 있는 세 번째 `let` 구문 또한 `x`를 가리고 새로운 변수를 만드는데,
이전 값에 `2`를 곱해 `x`에 할당해서 `x`의 최종값은 `12`가 됩니다.
이 스코프가 끝나면 안쪽의 섀도잉은 끝나서 `x`는 다시 `6`으로 돌아옵니다.
우리가 이 프로그램을 실행하면 다음과 같이 출력될 것입니다:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

섀도잉은 변수를 `mut`로 표시하는 것과는 다릅니다.
실수로 `let` 키워드 없이 변수에 값을 재할당하려고
한다면 컴파일 타임 에러가 발생하기 때문입니다.
`let`을 사용하면, 값을 변형하면서 변형이 완료된 후에는
불변으로 유지할 수 있습니다.

`mut`과 섀도잉의 또 다른 차이점은 다시금 `let` 키워드를 사용하여 새로운
변수를 만드는 것이기 때문에 같은 변수명으로 다른 타입의 값을 저장할
수 있다는 것입니다. 예를 들어, 프로그램이 사용자에게 어떤 텍스트
사이에 몇 개의 공백을 넣고 싶은지 공백문자를 입력하도록 요청하고,
이 값을 숫자로 저장하고 싶다 칩시다:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

첫 번째 `spaces`는 문자열 타입이고 두 번째 `spaces`는 숫자 타입입니다.
따라서 섀도잉은 `spaces_str`과 `spaces_num` 같이 구분되는
변수명을 쓸 필요가 없도록 여유를 줍니다; 즉, 더 간단한 `spaces`라는
이름을 재사용할 수 있게 해 줍니다. 그런데 여기에서 `mut`을 사용하려
한다면, 보시다시피 컴파일 타임 에러가 발생합니다:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

에러는 변수의 타입을 바꿀 수 없다고 알려줍니다.

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

변수가 어떻게 작동하는지 알아보았으니, 변수가 가질 수 있는 더 많은
타입들에 대해 알아봅시다.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#%EB%B9%84%EB%B0%80%EB%B2%88%ED%98%B8%EC%99%80-%EC%B6%94%EB%A6%AC%EA%B0%92%EC%9D%84-%EB%B9%84%EA%B5%90%ED%95%98%EA%B8%B0
[data-types]: ch03-02-data-types.html#data-types
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#storing-values-with-variables
[const-eval]: ../reference/const_eval.html
