## 함수

러스트 코드는 온통 함수로 가득합니다. 여러분은 이미 이 언어에서 가장
중요한 함수 중 하나를 보셨습니다: 바로 많은 프로그램의 시작점인 `main`
함수를 말이죠. 또한 새로운 함수를 선언하도록 해주는 `fn` 키워드도
보셨습니다.

러스트 코드는 함수나 변수 이름을 위한 관례로 *스네이크 케이스 (snake case)* 방식을
이용하는데, 이는 모든 글자를 소문자로 쓰고 밑줄(underscore)로 단어를 구분하는
방식을 말합니다. 다음은 예시로 함수를 정의한 프로그램입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

러스트에서는 `fn` 뒤에 함수 이름과 괄호를 붙여서 함수를
정의합니다. 중괄호는 함수 본문의 시작과 끝을 컴파일러에게
알려줍니다.

함수의 이름 뒤에 괄호 묶음을 쓰면 우리가 정의해둔 어떤 함수든 호출할 수
있습니다. `another_function`이 프로그램 내에 정의되어 있으므로, `main`
함수에서 해당 함수를 호출할 수 있습니다. 소스 코드 내에서 `another_function`이
`main` 함수 *이후에* 정의되어 있다는 점을 주목하세요. 이 함수를 `main` 함수 앞에서
정의할 수도 있습니다. 러스트는 여러분의 함수 위치를 고려하지 않으며, 호출하는
쪽에서 볼 수 있는 스코프 어딘가에 정의만 되어 있으면 됩니다.

함수에 대해 좀더 알아보기 위해서 *functions* 이라는 이름의 새 바이너리 프로젝트를
시작해 봅시다. `another_function` 예제를 *src/main.rs* 에 넣고 실행해보면
다음과 같은 결과를 보게 될 것입니다:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

`main` 함수 안의 내용이 순서대로 수행됩니다.
먼저 "Hello, world!" 메시지가 출력된 다음, `another_function`이
호출되고 그 안의 메세지가 출력됩니다.

### 매개변수

함수는 *매개변수 (parameter)* 를 갖도록 정의될 수 있으며, 이는 함수
시그니처 (function signiture) 의 일부인 특별한 변수입니다.
함수가 매개변수를 갖고 있으면 이 매개변수를 대한 구체적인 값을
전달할 수 있습니다. 엄밀하게는 이러한 구체적인 값을 *인자 (argument)* 라고
부르지만, 일상적인 대화에서는 보통 함수 정의부 내의 변수나 함수를 호출할
때 전달되는 구체적인 값에 대해 *매개변수*와 *인자*라는 용어를 혼용하는
경향이 있습니다.

아래의 `another_function`에는 매개변수가 추가되었습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

이 프로그램을 실행하면 다음과 같은 결과를 볼 수 있습니다:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

`another_function` 선언을 보면 `x`라는 이름의 매개변수를 하나 가지고 있습니다.
`x`의 타입은 `i32`로 명시되어 있습니다. `5`가 `another_function`으로 전달되면,
`println!` 매크로는 포맷 문자열 내 중괄호 쌍의 위치에 `5`를
집어넣습니다.

함수 시그니처에서는 각 매개변수의 타입을 *반드시* 선언해야 합니다.
이는 러스트를 설계하면서 신중하게 내린 결정사항입니다: 함수의 정의에 타입 명시를
강제하면 이 함수를 다른 코드에서 사용할 때 여러분이 의도한 타입을 컴파일러가
추측하지 않아도 되게 됩니다. 컴파일러는 또한 함수가 기대한 타입이 무엇인지 알고
있을 경우 더욱 유용한 에러 메세지를 제공할 수 있습니다.

여러 매개변수를 정의하려면 아래처럼 쉼표 기호로 매개변수
정의를 구분하세요:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

이 예제에서는 두 개의 매개변수를 갖는 함수 `print_labeled_measurement`를
생성합니다. 첫번째 매개변수 이름은 `value`이고 `i32` 타입입니다. 두번째는
`unit_label`이라는 이름이고 `char` 타입입니다. 이 함수는 `value`와
`unit_label`을 담고 있는 텍스트를 출력합니다.

한번 코드를 실행해봅시다. 여러분의 *function* 프로젝트의
*src/main.rs* 내용을 위의 예제로 변경한 뒤에, `cargo run`으로
실행시키면 됩니다:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

`value`에는 `5`를, `unit_label`에는 `h`를 넣어 함수를 호출했으므로,
이 값들이 프로그램의 출력에 들어갔습니다.

### 구문과 표현식

함수 본문은 필요에 따라 표현식(expression)으로 끝나는 구문(statement)의
나열로 구성됩니다. 지금까지 우리가 다룬 함수들은 표현식으로 끝나지 않았지만,
표현식이 구문의 일부분으로 쓰인건 보셨습니다. 러스트는 표현식 기반의
언어이므로, 구문과 표현식의 구분은 러스트를 이해하는데 중요합니다.
다른 언어들은 이런 구분이 없으므로, 구문과 표현식이 무엇이며
둘 간의 차이가 함수의 본문에 어떤 영향을 주는지
살펴보겠습니다.

* **구문**은 어떤 동작을 수행하고 값을 반환하지 않는
  명령입니다.
* **표현식**은 결과값을 산출합니다. 몇 가지 예제를 살펴봅시다.

우리는 실제로는 이미 구문과 표현식을 사용해 봤습니다.
`let` 키워드로 변수를 만들고 값을 할당하는 것은 구문입니다.
Listing 3-1의 `let y = 6;`은 구문입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

<span class="caption">Listing 3-1: 구문 하나로 되어있는 `main` 함수</span>

또한 함수 정의도 구문입니다; 위 예제는 그 자체로 구문에
해당됩니다.

구문은 값을 반환하지 않습니다. 따라서 아래와 같이 `let` 구문을 다른 변수에
할당하려고 하면 에러가 납니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

이 프로그램을 실행하면 다음과 같은 에러를 보게 됩니다:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

`let y = 6` 구문은 값을 반환하지 않으므로 `x`에 바인딩시킬
것이 없습니다. 이것이 C나 Ruby 같은 다른 언어와의 차이점인데,
이 언어들은 할당문이 할당된 값을 반환하죠. 이런 언어들에서는
`x = y = 6`라고 작성하여 `x`와 `y`에 모두 `6`을 대입할 수 있지만,
러스트에서는 그렇지 않습니다.

여러분이 작성하는 러스트 코드의 대부분은 표현식이며, 이는 어떤 값을 산출합니다.
`5 + 6`과 같은 간단한 수학 연산을 살펴봅시다. 이 수식은 `11`이라는
값을 산출하는 표현식입니다. 표현식은 구문의 일부일 수 있습니다:
Listing 3-1의 `let y = 6;`이라는 구문에서 `6`은 `6`이라는 값을 
산출하는 표현식입니다. 함수를 호출하는 것도, 매크로를 호출하는 것도
표현식입니다. 아래 예제처럼 중괄호로 만들어진 새로운 스코프 블록도
표현식입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

아래의 표현식:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

같은 경우에는 `4`를 산출하는 코드 블록입니다. 이 값이 `let` 구문의
일부로서 `y`에 바인딩됩니다. 여러분이 지금까지 봐온 것과 다르게 `x + 1`
줄의 마지막이 세미콜론으로 끝나지 않은 점을 주목하세요. 표현식은
종결을 나타내는 세미콜론을 쓰지 않습니다. 만약 표현식 끝에 세미콜론을
추가하면, 표현식은 구문으로 변경되고 값을 반환하지 않게 됩니다.
이 점을 상기하면서 이후부터 함수의 반환 값과 표현식을 살펴보길
바랍니다.

### 반환 값을 갖는 함수

함수는 호출한 코드에게 값을 반환할 수 있습니다. 반환되는 값을
명명해야 할 필요는 없지만, 그 값의 타입은 화살표 (`->`) 뒤에
선언되어야 합니다. 러스트에서 함수의 반환 값은 함수 본문의
마지막 표현식의 값과 동일합니다. `return` 키워드와 값을 지정하여
함수로부터 일찍 값을 반환할 수 있지만, 대부분의 함수들은 암묵적으로
마지막 표현식 값을 반환합니다. 값을 반환하는 함수의 예를
보겠습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

`five` 함수에는 함수 호출, 매크로, 심지어 `let` 구문도 없이
그저 `5`란 숫자 하나가 있습니다. 러스트에서는 이게 완벽하게 유효한
함수입니다. 함수 반환 값의 타입도 `-> i32`로 명시되어 있다는 점을
주목하세요. 결과는 아래와 같이 나와야 합니다:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

`5`는 `five` 함수의 반환값이며, 이 때문에 반환 타입을 `i32`으로 한
것이지요. 좀 더 자세히 보자면, 중요한 지점이 두 곳 있습니다: 첫째로,
`let x = five();` 라인은 함수의 반환 값을 변수의 초기 값으로 사용하는
것을 보여줍니다. `five`의 반환 값이 `5`이기 때문에, 해당 라인은 다음과 
동일합니다:

```rust
let x = 5;
```

두번째로, `five` 함수는 매개변수 없이 반환 타입만 정의되어 있지만,
본문에는 `5`만이 세미콜론 없이 외롭게 있는데, 그 이유는 이 값이
반환하려는 값에 대한 표현식이기 때문입니다.

다른 예제도 살펴봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

이 코드를 실행하면 `The value of x is: 6`이 출력됩니다.
만일 `x + 1` 끝에 세미콜론이 추가되어 표현식이 구문으로 변경되면
에러가 발생합니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

이 코드를 컴파일하면 다음과 같은 에러가 나타납니다:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

주 에러 메시지 `mismatched types`는 이 코드의 핵심 문제를 보여줍니다.
`plus_one` 함수의 정의에는 `i32` 값을 반환한다고 되어 있지만,
구문은 값을 산출하지 않기에 `()`로 표현되는 유닛 타입으로 표현됩니다.
따라서 아무 것도 반환되지 않아 함수가 정의된 내용과 상충하게 되어
에러가 발생됩니다. 위의 출력 결과에서 러스트는 이 문제를 바로잡는데
도움될 수 있는 메세지를 제공합니다: 바로 세미콜론을 제거하면 에러가
수정될 것이란 제안이지요.
