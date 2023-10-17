## 데이터 타입

러스트의 모든 값은 특정한 *타입*을 가지며, 이는 러스트가 해당 데이터로 작업하는
방법을 알 수 있도록 어떤 종류의 데이터가 지정되고 있는지 알려줍니다. 여기서는
타입을 스칼라 타입과 복합 타입, 두 가지 부분 집합으로 나누어 보겠습니다.

러스트는 *정적 타입의 (statically typed)* 언어라는 점을 주지하세요.
이게 의미하는 바는 모든 변수의 타입이 컴파일 시점에 반드시 정해져 있어야 한다는 겁니다.
보통 컴파일러는 우리가 값을 어떻게 사용하는지에 따라 타입을 추측할 수 있습니다. 2장의
[‘비밀번호와 추릿값을 비교하기’][comparing-the-guess-to-the-secret-number]<!-- ignore -->에서
`String`에 `parse`를 사용하여 숫자로 변환했던 경우처럼
여러 가지 타입이 가능한 경우에는 다음과 같이 반드시
타입 명시를 추가해야 합니다:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

여기에 `: u32`라는 타입 명시를 하지 않으면 러스트는 아래와 같은 에러를
출력하는데, 이는 컴파일러에게 사용하고자 하는 타입이 무엇인지에 대한
추가적인 정보가 필요하다는 뜻입니다:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

다양한 데이터 타입들의 타입 명시를 살펴보겠습니다.

### 스칼라 타입

*스칼라 (scalar)* 타입은 하나의 값을 표현합니다. 러스트는 정수, 부동 소수점 숫자, 부울린 (boolean),
그리고 문자, 이렇게 네 가지 스칼라 타입을 갖고 있습니다. 아마 다른 프로그래밍 언어에서도 이런 걸 본 적이
있을 겁니다. 러스트에서는 이들이 어떤 식으로 동작하는지 살펴보도록 합시다.

#### 정수형

*정수형 (integer type)* 은 소수점이 없는 숫자입니다. 2장에서
정수형 중 하나인 `u32` 타입을 사용했었죠. 해당 타입의 선언은
부호 없는 32비트 변수임을 나타냅니다 (부호 있는 타입은 `u` 대신
`i`로 시작합니다.) 표 3-1은 러스트에서 사용되는 정수형들을
보여줍니다. 이 변형 중 어떤 것이라도 정숫값의 타입 선언에
사용할 수 있습니다.

<span class="caption">표 3-1: 러스트의 정수형 타입들</span>

| 길이    | 부호 있음 (signed)  | 부호 없음 (unsigned) |
|---------|---------|----------|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

각각의 타입은 부호 있는 (signed) 혹은 부호 없는 (unsigned) 타입이며 명시된 크기를
갖습니다. 부호 혹은 부호 없음의 의미는, 이 타입이 음수를 다룰 수 있는지를
나타냅니다. 다르게 말하면, 숫자가 부호를 가져야 하는 경우인지 (부호 있는)
혹은 오직 양수만을 가질 것이기에 부호 없이 표현 가능한지를 (부호 없는)
나타냅니다. 종이에 숫자 기재하는 것과 같습니다:
부호와 함께 다뤄야 하는 경우 숫자는 덧셈이나 뺄셈 기호와 함께 표시하지요.
하지만 숫자가 양수라고 가정해도 문제없는 상황에는 부호 없이 표시하게 됩니다. 
부호 있는 숫자는 [2의 보수][twos-complement]<!-- ignore -->형태를 사용하여
저장됩니다.

각 부호 있는 타입의 변수는 -(2<sup>n - 1</sup>)부터 2<sup>n - 1</sup> - 1까지의 값을
저장할 수 있습니다. 여기서 *n*은 사용되는 타입의 비트 수입니다. 따라서 `i8`은
-(2<sup>7</sup>)에서 2<sup>7</sup> - 1까지의 값, 즉 -128에서 127 사이의 값을 저장할 수
있습니다. 부호 없는 타입은 0에서 2<sup>n</sup> - 1까지의 값을 저장할 수 있습니다. 그래서
`u8` 타입은 0에서 2<sup>8</sup> - 1 다시 말해, 0에서 255까지의 값을 저장할 수 있습니다. 

추가로, `isize`와 `usize` 타입은 여러분의 프로그램이 동작하는 컴퓨터
환경에 따라 결정되는데, 위 테이블에는 ‘arch’라고 적시되어 있습니다.
64-bit 아키텍처이면 64비트를, 32-bit 아키텍처이면 32비트를 갖게
됩니다.

정수형 리터럴은 표 3-2에서 보시는 것과 같은 형태로 작성할 수 있습니다.
여러 숫자 타입이 될 수 있는 숫자 리터럴에는 `57u8`과 같은 타입 접미사를
사용하여 타입을 지정할 수 있습니다. `1_000`처럼 시각적인 구분으로
읽기 쉽게 하기 위해서 `_`을 사용할 수 있는데, 이는 `1000`이라고 쓴 것과
똑같은 값이 됩니다.

<span class="caption">표 3-2: 러스트의 정수형 리터럴</span>

| 숫자 리터럴       | 예            |
|------------------|---------------|
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |

그러면 어떤 타입의 정수를 사용해야 하는지는 어떻게 알아낼까요? 확실히
정해진 경우가 아니라면 러스트의 기본값인 `i32`가 일반적으로 좋은
시작 지점이 됩니다. `isize`나 `usize`는 주로 어떤 컬렉션 종류의 인덱스에
사용됩니다.

> ##### 정수 오버플로우
>
> 여러분이 0과 255 사이의 값을 담을 수 있는 `u8` 타입의 변수를 갖고 있다고
> 해봅시다. 만약에 이 변수에 256처럼 범위 밖의 값으로 변경하려고 하면
> *정수 오버플로우 (integer overflow)* 가 일어나는데, 이는 둘 중
> 한 가지 동작을 일으킵니다. 코드를 디버그 모드에서 컴파일하는 경우,
> 러스트는 런타임에 정수 오버플로우가 발생했을 때 *패닉 (panic)* 을 발생시키는
> 검사를 포함시킵니다. 러스트에서는 에러가 발생하면서 프로그램이 종료되는 경우
> 패닉이라는 용어를 사용합니다; 9장의
> [‘`panic!`으로 복구 불가능한 에러 처리하기’][unrecoverable-errors-with-panic]<!-- ignore -->절에서
> 패닉에 대해 좀 더 자세히 다루겠습니다.
>
> `--release` 플래그를 사용하여 코드를 릴리즈 모드로 컴파일하는 경우에는
> 패닉을 발생시키는 정수 오버플로우 검사를 실행파일에 포함시키지 않습니다.
> 대신 오버플로우가 발생하면 러스트는 *2의 보수 감싸기 (two's complement wrapping)*
> 을 수행합니다. 짧게 설명하자면, 해당 타입이 가질 수 있는 최댓값보다 더 큰 값은 
> 허용되는 최솟값으로 ‘돌아갑니다 (wrap around)’. `u8`의 경우 256은 0이,
> 257은 1이 되는 식입니다. 프로그램은 패닉을 발생시키지 않으나, 해당 변수는
> 아마도 여러분이 예상치 못했던 값을 갖게 될 겁니다. 정수 오버플로우의
> 감싸기 동작에 의존하는 것은 에러로 간주됩니다.
>
> 명시적으로 오버플로우의 가능성을 다루기 위해서는 표준 라이브러리에서
> 기본 수치 타입에 대해 제공하는 아래 메서드 종류들을 사용할 수 있습니다:
>
> * `wrapping_add`와 같은 `wrapping_*` 메서드로 감싸기 동작 실행하기
> * `checked_*` 메서드를 사용하여 오버플로우가 발생하면 `None` 값 반환하기
> * `overflowing_*` 메서드를 사용하여 값과 함께 오버플로우 발생이 있었는지를
>   알려주는 부울린 값 반환하기
> * `saturating_*` 메서드를 사용하여 값의 최대 혹은 최솟값 사이로
>   제한하기

#### 부동 소수점 타입

러스트에도 소수점을 갖는 숫자인 *부동 소수점 (floating-point) 숫자* 기본
타입이 두 가지 있습니다. 러스트의 부동 소수점 타입은 `f32`와 `f64`로,
각각 32bit와 64bit의 크기를 갖습니다. 기본 타입은 `f64`인데, 그 이유는
현대의 CPU 상에서 `f64`가 `f32`와 대략 비슷한 속도를 내면서도 더 정밀하기
때문입니다. 모든 부동 소수점 타입은 부호가 있습니다.

다음은 부동 소수점 숫자의 용례입니다:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

부동 소수점 숫자는 IEEE-754 표준을 따릅니다. `f32` 타입은 1배 수 정밀도 (single-precision) 인
부동 소수점이고, `f64`는 2배 수 정밀도 (double-precision) 입니다.

#### 수치 연산

러스트는 모든 숫자 타입에 대해서 여러분이 예상할 수 있는 기본 수학 연산 기능을
제공합니다: 더하기, 빼기, 곱하기, 나누기 등등을 말이죠. 정수 나눗셈은 가장
가까운 정숫값으로 버림을 합니다. 아래 코드는 `let` 구문 내에서 각 연산을
어떻게 사용하는지를 보여줍니다:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

위의 구문에서 각 표현식은 수학 연산자를 사용하여 값을 평가한
뒤, 그 값을 변수에 묶어 넣습니다. [부록 B][appendix_b]<!-- ignore -->에
러스트가 제공하는 모든 연산자 목록이
있습니다.

#### 부울린 타입

대부분의 다른 언어들처럼, 러스트에서의 부울린 (boolean) 타입도 `true`와 `false` 두
값을 가질 수 있습니다. 부울린 값은 1바이트 크기입니다. 러스트에서 부울린 타입은
`bool`로 명시됩니다. 예를 들면:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

부울린 값을 사용하는 주요 방식은 `if` 표현식과 같은 조건문에서 사용하는
것입니다. 러스트에서 `if` 표현식이 동작하는 방식에 대해서는
[‘제어 흐름문’][control-flow]<!-- ignore -->절에서 다루겠습니다.

#### 문자 타입

러스트의 `char`는 이 언어의 가장 기본적인 알파벳 타입입니다.
다음 코드는 `char` 값을 선언하는 몇 가지 예시입니다:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

문자열 리터럴이 큰따옴표를 쓰는 반면, `char` 타입은 작은따옴표를
쓰는 점을 주목하세요. 러스트의 `char` 타입은 4바이트 크기이며 유니코드
스칼라 값을 표현하며, 이는 ASCII보다 훨씬 더 많은 값을 표현할 수
있다는 의미입니다. 억양 표시가 있는 문자, 한국어/중국어/일본어 문자,
이모지, 넓이가 0인 공백문자 모두가 러스트에서는 유효한 `char` 값입니다.
유니코드 스칼라 값의 범위는 `U+0000`에서 `U+D7FF`, 그리고 `U+E000`에서
`U+10FFFF`입니다. 하지만 ‘문자’는 유니코드를 위한 개념이 아니기 때문에,
‘문자’에 대한 여러분의 직관은 `char`와 들어맞지 않을지도 모릅니다. 8장의
[‘문자열에 UTF-8 텍스트 저장하기’][strings]<!-- ignore -->
에서 이 주제에 대해 자세히 다루겠습니다.

### 복합 타입

*복합 타입 (compound type)* 은 여러 값을 하나의 타입으로 묶을 수 있습니다.
러스트에는 튜플 (tuple) 과 배열 (array), 두 가지 기본 복합 타입이 있습니다.

#### 튜플 타입

*튜플*은 다양한 타입의 여러 값을 묶어 하나의 복합 타입으로 만드는 일반적인
방법입니다. 튜플은 고정된 길이를 갖습니다. 즉, 한번 선언되면 그 크기를
늘리거나 줄일 수 없습니다.

괄호 안에 쉼표로 구분하여 값들의 목록을 작성하면 튜플을
만들 수 있습니다. 튜플 내의 각 위치는 타입을 갖고,
이 튜플 내의 타입들은 서로 달라도 됩니다. 다음은 (안 써도 괜찮지만)
타입을 명시해 본 예제입니다:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

튜플은 하나의 복합 요소로 취급되므로 변수 `tup`은 튜플 전체가 바인딩됩니다.
튜플로부터 개별 값을 얻어오려면 아래와 같이 패턴 매칭을 하여 튜플 값을
해체하면 사용하면 됩니다:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

이 프로그램은 먼저 튜플을 만든 후 이를 변수 `tup`에 바인딩시킵니다.
그다음 `let`을 이용하는 패턴을 사용하여 `tup`을 세 개의 분리된 변수 `x`, `y`,
그리고 `z`로 바꿉니다. 이것을 *구조 해체 (destructuring)* 라고 부르는 이유는
하나의 튜플을 세 부분으로 나누기 때문입니다. 최종적으로 프로그램은 `y`의 값을
출력할 것이고 이는 `6.4`입니다.

마침표(`.`) 뒤에 접근하고자 하는 값의 인덱스를 쓰는 방식으로도 튜플 요소에
접근할 수 있습니다. 예를 들면:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

위의 프로그램은 튜플 `x`를 만들고, 인덱스를 사용하여 이 튜플의
각 요소에 접근합니다. 대부분의 언어가 그렇듯이 튜플의 첫 번째
인덱스는 0입니다. 

아무 값도 없는 튜플은 *유닛 (unit)* 이라는 특별한 이름을 갖습니다.
이 값과 타입은 모두 `()`로 작성되고 빈 값이나 비어있는 반환 타입을
나타냅니다. 표현식이 어떠한 값도 반환하지 않는다면 암묵적으로 유닛
값을 반환하게 됩니다.

#### 배열 타입

여러 값의 집합체를 만드는 다른 방법으로는 *배열*이 있습니다. 튜플과는 달리
배열의 모든 요소는 모두 같은 타입이여야 합니다. 몇몇 다른 언어들과는 달리
러스트의 배열은 고정된 길이를 갖습니다.

대괄호 안에 쉼표로 구분한 값들을 나열해서 배열을
만들 수 있습니다:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

여러분이 힙보다는 스택에 데이터를 할당하고 싶을 때나
(힙과 스택은 [4장][stack-and-heap]<!-- ignore -->에서
더 다루겠습니다) 항상 고정된 개수의 요소로 이루어진 경우라면
배열이 유용합니다. 하지만 배열은 벡터 타입처럼 유연하지는 않습니다.
벡터는 표준 라이브러리가 제공하는 배열과 유사한 컬렉션 타입인데
크기를 늘리거나 줄일 수 있습니다. 배열을 이용할지 혹은
벡터를 이용할지 잘 모르겠다면, 아마도 벡터를 사용해야 할 겁니다.
[8장][vectors]<!-- ignore -->에서 벡터에 대해 더 자세히 다룰 예정입니다.

그러나 요소의 개수가 바뀔 필요가 없다는 것을 알고 있을 때라면 배열이
더 유용합니다. 한 가지 예로, 프로그램에서 달의 이름을 이용하려고 한다면,
이것이 언제나 12개의 요소만 가지고 있을 것이라는 사실을 알고 있으므로,
아마도 벡터보다는 배열을 사용할 것입니다:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

다음과 같이 대괄호 안에 요소의 타입을 쓰고 세미콜론을 쓴 뒤 요소의
개수를 적는 식으로 배열의 타입을 작성할 수도 있습니다:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

여기서 `i32`는 각 요소의 타입입니다. 세미콜론 뒤의 숫자 `5`는
배열이 5개의 요소를 갖고 있음을 나타냅니다.

또한 다음과 같이 대괄호 안에 초깃값과 세미콜론을 쓴 다음 배열의 길이를
적는 방식을 사용하여 모든 요소가 동일한 값으로 채워진 배열을 초기화할 수도
있습니다:

```rust
let a = [3; 5];
```

`a`라는 이름의 배열은 모두 `3`으로 채워진 `5`개의 요소를 갖게 될
것입니다. 이는 `let a = [3, 3, 3, 3, 3];`이라고 쓴 것과 똑같지만
좀더 편리한 방식입니다.

##### 배열 요소에 접근하기

배열은 스택에 할당될 수 있는 계산 가능한 고정된 크기의 단일 메모리
뭉치입니다. 아래와 같이 인덱스를 통해 배열 요소에 접근할 수
있습니다:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

이 예제에서 `first`로 명명된 변수는 배열에서 인덱스 `[0]`의 값이므로
`1`이 될 것입니다. `second`로 명명된 변수는 배열의 `[1]` 인덱스로부터
얻어진 값 `2`가 되겠죠.

##### 유효하지 않은 배열 요소에 대한 접근

만약 배열의 끝을 넘어선 요소에 접근하려고 하면 어떤 일이 벌어지는지
알아봅시다. 사용자로부터 배열 인덱스를 입력받기 위해서 2장의 추리
게임과 유사한 아래 코드를 실행한다 칩시다:

<span class="filename">파일명: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

이 코드는 성공적으로 컴파일됩니다. `cargo run`으로 코드를 실행한 뒤
`0`, `1`, `2`, `3`, 혹은 `4`를 입력한다면 프로그램은 그 인덱스에 해당하는
배열 값을 출력할 것입니다. 그 대신에 이 배열의 끝을 넘어서는 `10` 같은
숫자를 입력하면, 아래와 같은 출력을 보게 될 것입니다:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

프로그램은 인덱스 연산에서 잘못된 값을 사용한 시점에서 *런타임* 에러를
발생시켰습니다. 이 프로그램은 에러 메시지와 함께 종료되고 마지막 `println!` 구문을
실행하지 못했습니다. 인덱스를 이용하여 요소에 접근을 시도하는 경우,
러스트는 여러분이 명시한 인덱스가 배열 길이보다 작은지 검사할 것입니다.
인덱스가 배열 길이보다 크거나 같을 경우 러스트는 패닉 (panic) 을 일으킵니다.
특히 위의 경우 이러한 검사는 런타임에서 일어나야 하는데, 이는 사용자가
코드를 실행한 뒤에 어떤 값을 입력할지 컴파일러로서는 알 수 없기
때문입니다.

이 예제는 러스트의 안전성 원칙이 동작하는 하나의 예입니다. 많은 저수준 언어들에서는
이러한 검사가 이루어지지 않고, 여러분이 잘못된 인덱스를 제공하면 유효하지 않은
메모리에 접근이 가능합니다. 러스트는 이런 메모리 접근을 허용하고 계속 실행하는
대신 즉시 실행을 종료함으로써 이런 종류의 에러로부터 여러분을 보호합니다. 러스트의
에러 처리 및 패닉을 일으키지 않으면서 유효하지 않은 메모리 접근도 허용하지 않는
읽기 쉽고 안전한 코드를 작성하는 방법에 대해서는 9장에서 더 자세히 다루겠습니다.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[twos-complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[control-flow]: ch03-05-control-flow.html#control-flow
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[stack-and-heap]: ch04-01-what-is-ownership.html#the-stack-and-the-heap
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[appendix_b]: appendix-02-operators.md
