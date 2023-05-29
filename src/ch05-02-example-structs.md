## 구조체를 사용한 예제 프로그램

어떨 때 구조체를 사용하면 좋을지 이해해 보기 위해 사각형 넓이를
계산하는 프로그램을 작성해 봅시다. 단일 변수로만 구성된 프로그램으로
시작해 구조체를 사용하기까지 리팩터링하면서 말이죠.

카고를 사용해 *rectangles*라는 새로운 바이너리 프로젝트를 만들어 줍시다.
이 프로그램은 픽셀 단위로 명시된 사각형의 너비와 높이로 넓이를 계산할 겁니다.
예제 5-8은 *src/main.rs*에 이 기능을 간단하게 구현한
모습입니다:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

<span class="caption">예제 5-8: 각 변수에 지정된 너비와 높이로
사각형 넓이 계산</span>

`cargo run`으로 실행해 보죠:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

위 코드는 `area` 함수에 각 치수 값을 넣어 사각형의 면적을 성공적으로
계산하지만, 코드를 더 명료하고 읽기 쉽게 만들기 위해서 몇 가지를 더
할 수 있습니다.

`area` 함수의 시그니처를 보면 개선해야 할 점이 여실히 드러납니다:

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

`area` 함수는 하나의 사각형의 면적을 계산하는 것을 가정하고
있지만 두 개의 매개변수를 받고 있으며, 이 두 값이 서로 연관되어
있다는 것을 명확하게 표현하는 부분은 찾아볼 수 없군요.
두 값을 하나로 묶어버리면 코드의 가독성도 높아지고 관리하기도 쉬워질 겁니다.
앞서 3장 [“튜플 타입”][the-tuple-type]<!-- ignore --> 절에서
배운 튜플로 해결해 볼까요?

### 튜플로 리팩터링하기

다음 예제 5-9 는 튜플을 사용한 모습입니다.

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

<span class="caption">예제 5-9: 사각형의 너비와 높이를
튜플로 명시하는 코드</span>

튜플을 사용함으로써 더 짜임새 있는 코드가 됐고,
인수도 단 하나만 넘기면 된다는 점에선 프로그램이 발전했다고 볼 수 있습니다.
하지만 각 요소가 이름을 갖지 않는 튜플의 특성 때문에
값을 인덱스로 접근해야 해서 계산식이 불분명해졌네요.

다행히 넓이를 계산할 땐 어떤 값이 너비이고 어떤 값이 높이인지 구분하지 못해도 별 문제가
없습니다. 하지만 만들어야 할 프로그램이 화면에 사각형을 그리는 프로그램이라고 가정해 보면
어떨까요? 너비 값인 `width`가 튜플 인덱스 `0`에 위치하고 높이 값인 `height`는 튜플
인덱스 `1`에 위치한다는 걸 꼭 기억하고 있어야 할 겁니다. 혹여나 다른 사람이 이 코드로
작업할 일이 생기면 그 사람도 이 점을 알아내서 기억해야 하겠죠. 코드 내에 데이터의
의미를 전달하지 못하므로 에러를 만들어 내기 더 쉬워집니다.

### 구조체로 리팩터링하여 코드에 더 많은 의미를 담기

구조체는 데이터에 이름표를 붙여서 의미를 나타낼 수 있습니다.
예제 5-10처럼, 기존에 사용하던 튜플을 구조체로 바꿔
각 구성 요소에 이름을 지어줍시다.

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

<span class="caption">예제 5-10: `Rectangle` 구조체 정의</span>

`Rectangle`이라는 구조체를 정의하고,
중괄호 내에 `width`, `height` 필드를 `u32` 타입으로 정의했습니다.
이후 `main` 내에선 너비 `30`, 높이 `50` 짜리
`Rectangle` 구조체의 인스턴스를 생성했습니다.

`area` 함수의 매개변수는 이제 `rectangle` 하나뿐입니다.
단, 구조체의 소유권을 가져와 버리면 `main` 함수에서
`area` 함수 호출 이후에 `rect1`을 더 사용할 수 없으므로,
`rectangle` 매개변수의 타입을 불변 참조자 타입으로 정하여
소유권을 빌려오기만 하도록 만들었습니다.
불변 참조자 타입이니 함수 시그니처와 호출 시에 `&`를 작성합니다.

`area` 함수는 `Rectangle` 인스턴스의 `width`, `height` 필드에
접근합니다 (빌린 구조체 인스턴스의 필드에 접근하는 것은 필드 값을
이동시키지 않으며, 이것이 구조체의 빌림을 자주 보게 되는 이유임을
기억해 두세요). `area` 함수의 시그니처는 이제 의미하는 바를 정확히
알려줍니다: `Rectangle`의 `width`와 `height` 필드를 사용하여
넓이를 계산하라는 뜻이지요. `width`, `height`가 서로 연관된
값이라는 것도 알 수 있고, `0` 이나 `1` 대신 서술적인 필드명을
제공합니다. 명료성 측면에서 승리입니다.

## 트레잇 파생으로 유용한 기능 추가하기

프로그램을 디버깅하는 동안 `Rectangle` 인스턴스 내 모든 필드 값을
출력해서 확인할 수 있다면 좋을 것 같군요. 예제 5-11는 앞서 다뤄본
[`println!` 매크로][println]<!-- ignore -->를 사용해 본 예시이나,
작동하진 않습니다.

<span class="filename">파일명: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

<span class="caption">예제 5-11: `Rectangle` 인스턴스
출력을 시도해 본 모습</span>

이 코드를 컴파일하면 다음과 같은 메시지가 나타납니다:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

`println!` 매크로에는 여러 출력 형식을 사용할 수 있습니다.
그리고 기본 형식인 `{}` 로 지정할 땐 `Display`라는, 최종 사용자를 위한 출력 형식을 사용하죠.
여태 사용했던 기본 타입들은 `Display`가 기본적으로 구현되어 있었습니다.
`1` 같은 기본 타입들을 사용자에게 보여줄 수 있는 형식은 딱 한 가지뿐이니까요.
하지만 구조체라면 이야기가 달라집니다.
중간중간 쉼표를 사용해야 할 수도 있고, 중괄호도 출력해야 할 수도 있고,
필드 일부를 생략해야 할 수도 있는 등 여러 경우가 있을 수 있습니다.
러스트는 이런 애매한 상황에 우리가 원하는 걸 임의로 예상해서 제공하려 들지 않기 때문에,
구조체에는 `println!` 및 `{}` 자리표시자와 함께 사용하기 위한
`Display` 구현체가 기본 제공되지 않습니다.

에러를 더 읽다 보면 다음과 같은 도움말을 찾을 수 있습니다:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```

`{}` 대신 `{:?}`를 사용해 보라네요. 한번 해보죠.
`println!` 매크로 호출을 `println!("rect1 is {:?}", rect1);`으로 바꿔봅시다.
`{}` 내에 `:?`를 추가하는 건 `println!`에 `Debug`라는 출력 형식을 사용하고 싶다고 전달하는 것과 같습니다.
이 `Debug`라는 트레잇은 최종 사용자가 아닌, 개발자에게 유용한 방식으로 출력하는,
즉 디버깅할 때 값을 볼 수 있게 해주는 트레잇입니다.

변경하고 나서 다시 컴파일해 보면, 어째서인지 여전히 에러가 발생하네요:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

그런데 컴파일러가 또 무언가를 알려주네요:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```

러스트는 디버깅 정보를 출력하는 기능을 *자체적으로 가지고 있습니다*.
하지만 우리가 만든 구조체에 해당 기능을 적용하려면 명시적인 동의가 필요하므로,
예제 5-12처럼 구조체 정의 바로 이전에 `#[derive(Debug)]`
외부 속성 (outer attirbute) 을 작성해주어야 합니다.

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

<span class="caption">예제 5-12: `Rectangle` 인스턴스를 디버그 출력 형식으로
사용하기 위해, 속성을 추가하여 `Debug` 트레잇 파생 (derive) 하기</span>

이제 프로그램을 실행해 보면 더 이상 에러가 나타나지 않고,
다음과 같은 출력이 나타날 겁니다:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

좋습니다! 가장 예쁜 출력 형태라 할 수는 없지만,
인스턴스 내 모든 필드 값을 보여주므로 디버깅하는 동안에는 확실히 유용할 겁니다.
필드가 더 많은 구조체라면 이보다 더 읽기 편한 형태가 필요할 텐데요,
그럴 땐 `println!` 문자열 내에 `{:?}` 대신 `{:#?}`를 사용하면 됩니다.
`{:#?}`를 사용했을 때의 출력 예시는 다음과 같습니다.

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

`Debug` 포맷을 사용하여 값을 출력하는 그 밖의 방법은
[`dbg!` 매크로][dbg]<!-- ignore -->를 사용하는 것인데,
이는 표현식의 소유권을 가져와서, (참조자를 사용하는 `println!`과는 다릅니다)
코드에서 `dbg!` 매크로를 호출한 파일 및 라인 번호를 결괏값과 함께 출력하고
다시 소유권을 반환합니다.

> Note: `dbg!` 매크로의 호출은 표준 에러 콘솔 스트림(`stderr`)에 출력을
> 하는데, 이는 표준 출력 콘솔 스트림(`stdout`)에 출력하는 `println!`과는
> 상반됩니다. `stderr`와 `stdout`에 대한 정보는 [12장의 “표준 출력 대신
> 표준 에러로 에러 메시지 출력하기”][err]<!-- ignore -->절에서 더
> 이야기하겠습니다.

아래는 `rect`의 전체 구조체 값뿐만 아니라 `width` 필드에 대입되는
값에 관심이 있는 경우에 대한 예시입니다:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

표현식 `30 * scale`을 `dbg!`으로 감싸 넣었는데, 이는 `dbg!`가 표현식 값의 소유권을
반환하면서 `dbg!` 호출을 하지 않았을 때와 같은 값이 `width` 필드에 입력되기
때문입니다. `dbg!`가 `rect1`의 소유권을 가져가는 것은 원치 않으므로, 그다음의
호출에서는 `rect1`에 대한 참조자를 사용하였습니다.
위 예제의 출력 결과는 아래와 같습니다:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

첫 번째 출력 결과가 현재 디버깅 중인 `30 * scale` 표현식이 있는 *src/main.rs*의
10번째 라인인 것, 그리고 그 결괏값은 `60`임을 알 수 있습니다 (정수형을 위한
`Debug` 형식은 그냥 그 값을 출력하는 것으로 되어 있습니다). *src/main.rs*의
14번째 라인에 있는 `dbg!` 호출은 `Rectangle` 구조체인 `&rect1`의 결과를
출력합니다. 이 출력 결과는 `Rectangle` 타입에 대한 보기 좋은 `Debug` 포맷을
이용합니다. `dbg!` 매크로는 여러분의 코드가 무슨 짓을 하는지 알아볼 때 매우
유용할 수 있습니다!

러스트에선 이처럼 `Debug` 트레잇 말고도 `derive` 속성으로
우리가 만든 타입에 유용한 동작을 추가할 수 있는 트레잇을 여럿 제공합니다.
이들 목록 및 각각의 동작은 [부록 C][app-c]<!-- ignore -->에서
확인할 수 있으니 참고해 주세요. 또한, 여러분만의 트레잇을 직접 만들고,
이런 트레잇들의 동작을 커스터마이징해서 구현하는 방법은 10장에서 배울 예정입니다.
또한 `device` 외에도 여러 가지 속성들이 있습니다; 더 많은 정보는 [러스트 참고 자료의
“속성”절][attributes]를 살펴보세요.

우리가 만든 `area` 함수는 사각형의 면적만을 계산합니다.
`Rectangle` 구조체를 제외한 다른 타입으로는 작동하지 않으니
`Rectangle` 구조체와 더 밀접하게 묶는 편이 더 유용할 겁니다.
다음에는 `area` 함수를 `Rectangle` 타입 내에 *메서드(method)* 형태로
정의하여 코드를 리팩터링하는 방법을 알아보겠습니다.

[the-tuple-type]: ch03-02-data-types.html#the-tuple-type
[app-c]: appendix-03-derivable-traits.md
[println]: ../std/macro.println.html
[dbg]: ../std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
[attributes]: ../reference/attributes.html
