## 고급 타입

러스트의 타입 시스템에는 지금까지 언급은 했지만 아직 논의하지는 않은
몇 가지 기능이 있습니다. 먼저 뉴타입이 타입으로써 유용한 이유를 살펴보면서
뉴타입에 대해 전반적으로 논의하겠습니다. 그런 다음 뉴타입과 비슷하지만 의미는
약간 다른 기능인 타입 별칭 (type alias) 에 대해 살펴보겠습니다. 또한 `!`
타입과 동적 크기 타입 (dynamically sized 쇼ㅔㄷ) 에 대해서도 설명합니다.

### 타입 안전성과 추상화를 위한 뉴타입 패턴 사용하기

> Note: 이 절은 여러분이 이전 절
> [“뉴타입 패턴을 사용하여 외부 타입에 외부 트레잇 구현하기”][using-the-newtype-pattern]<!-- ignore -->을
> 읽었다고 가정합니다.

뉴타입 패턴은 지금까지 설명한 것 이외의 작업에도 유용한데, 여기에는
값이 혼동되지 않도록 정적으로 강제하는 것과 값의 단위를 표시하는
것들이 포함됩니다. Listing 19-15에서 뉴타입을 사용하여 단위를 표시하는
예제를 보았습니다: `Millimeters` 및 `Meters` 구조체가 `u32` 값을
뉴타입으로 감싸고 있었음을 상기하세요. `Millimeters` 타입의
매개변수가 있는 함수를 작성했다면, 실수로 `Meters` 또는 보통의
`u32` 타입의 값으로 해당 함수를 호출 시도하는 프로그램은 컴파일될
수 없습니다.

뉴타입 패턴은 어떤 타입의 구현 세부 사항을 추상화 하는데도
사용 가능합니다: 뉴타입은 비공개 내부 타입의 API와는 다른 공개
API를 노출할 수 있습니다.

뉴타입은 내부 구현을 숨길 수도 있습니다. 예를 들면, 어떤 사람의 ID와
이에 연관된 그 사람의 이름을 저장하는 `HashMap<i32, String>`을
래핑하는 `People` 타입을 만들 수 있습니다. `People`을 사용하는
코드는 `People` 컬렉션에 이름 문자열을 추가하는 메서드처럼
우리가 제공하는 공개 API와만 상호작용할 수 있습니다; 해당
코드는 내부적으로 이름에 `i32` ID를 할당한다는 사실을 알 필요가
없습니다. 뉴타입 패턴은 구현 세부 사항을 숨기는 캡슐화를 달성하는
가벼운 방법으로, 17장의
[“구현 세부 사항을 숨기는 캡슐화”][encapsulation-that-hides-implementation-details]<!-- ignore -->절에서
설명한 바 있습니다.

### 타입 별칭으로 타입의 동의어 만들기

러스트는 *타입 별칭 (type alias)* 을 선언하여 기존 타입에 다른 이름을 부여하는
기능을 제공합니다. 이를 위해서는 `type` 키워드를 사용합니다. 예를 들어, 다음과
같이 `i32`에 대한 `Kilometers`라는 별칭을 만들 수 있습니다:

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-04-kilometers-alias/src/main.rs:here}}
```

이제 별칭 `Kilometers`는 `i32`의 *동의어* 입니다; Listing 19-15에서
만든 `Millimeters` 및 `Meters` 타입과는 달리, `Kilometers`는 별도의
새로운 타입은 아닙니다. `Kilometers` 타입을 가진 값은 `i32` 타입의 값과
동일하게 처리됩니다:

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-04-kilometers-alias/src/main.rs:there}}
```

`Kilometers`와 `i32`는 동일한 타입이므로 두 타입의 값을 모두 더할
수 있고 `Kilometers` 값을 `i32` 매개변수를 받는 함수에 전달할 수
있습니다. 그러나, 이 방법을 사용하면 이전에 설명한 뉴타입 패턴에서
얻을 수 있는 타입 검사 이점을 얻을 수 없습니다. 다시 말해, 어딘가에서
`Kilometers`와 `i32` 값을 혼용하면 컴파일러는 에러를 표시하지
않습니다.

타입 동의어의 주요 사용 사례는 반복을 줄이는 것입니다. 예를 들어,
다음과 같은 긴 타입이 있을 수 있습니다:

```rust,ignore
Box<dyn Fn() + Send + 'static>
```

Writing this lengthy type in function signatures and as type annotations all
over the code can be tiresome and error prone. Imagine having a project full of
code like that in Listing 19-24.
이 긴 타입을 함수 시그니처 및 코드의 모든 곳에 타입 명시로 작성하는
것은 지루하고 에러가 발생하기 쉽습니다. Listing 19-24와 같은 코드로
가득찬 프로젝트가 있다고 상상해보세요.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-24/src/main.rs:here}}
```

<span class="caption">Listing 19-24: 수많은 곳에 긴 타입 사용하기</span>

타입 별칭은 반복을 줄여 이 코드를 관리하기 쉽게 만듭니다. Listing
19-25에서는 이 장황한 타입에 대해 `Thunk`라는 별칭을 만들고 이 타입이
사용된 모든 곳을 짧은 별칭 `Thunk`으로 대체했습니다.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-25/src/main.rs:here}}
```

<span class="caption">Listing 19-25: 타입 별칭 `Thunk`을 도입하여
반복 줄이기</span>

이 코드는 읽고 작성하기 훨씬 쉽습니다! 또한 타입 별칭에 의미 있는
이름을 선택하면 의도를 전달하는 데 도움이 됩니다. (*thunk*는
나중에 평가될 코드를 나타내는 단어이므로, 저장되는 클로저에 적합한
이름입니다.)

타입 별칭은 또한 `Result<T, E>` 타입의 반복을 줄이기 위해 사용되기도
합니다. 표준 라이브러리의 `std::io` 모듈을 생각해 보세요. I/O 연산은
종종 연산이 작동하지 않을 때의 상황을 처리하기 위해 `Result<T, E>`를
반환합니다. 이 라이브러리에는 가능한 모든 I/O 에러를 나타내는
`std::io::Error` 구조체가 있습니다. `std::io`의 많은 함수는 `Write`
트레잇의 함수와 같이 `E`가 `std::io::Error`인 `Result<T, E>`를
반환합니다:

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-05-write-trait/src/lib.rs}}
```

`Result<..., Error>`가 많이 반복됩니다. 이러한 이유로 `std::io`에는
이러한 타입 별칭 선언이 있습니다:

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-06-result-alias/src/lib.rs:here}}
```

이 선언이 `std::io` 모듈에 있으므로, 완전 정규화된 별칭
`std::io::Result<T>`를 사용할 수 있습니다; 즉, `E`가
`std::io::Error`로 채워진 `Result<T, E>`입니다. `Write` 트레잇
함수 시그니처는 결국 다음과 같이 생기게 됩니다:

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-06-result-alias/src/lib.rs:there}}
```

타입 별칭은 두 가지 방법으로 도움을 줍니다: 코드를 쉽게 작성할 수 있게 해주고,
*그러면서도* `std::io` 전체에 일관된 인터페이스를 제공합니다. 이것은 별칭이기
때문에 그저 또다른 `Result<T, E>`일 뿐이고, 이는 `Result<T, E>`에서 작동하는
모든 메서드는 물론, `?` 연산자와 같은 특별 문법도 사용할 수 있음을 뜻합니다.

### 결코 반환하지 않는 부정 타입

러스트에는 `!`라는 특수한 타입이 있는데, 이 타입은 값이 없기 때문에 타입
이론 용어로는 *빈 타입 (empty type)* 이라고 알려져 있습니다. 함수가 절대
반환하지 않을 때 반환 타입을 대신하기 때문에 *부정 타입 (never type)* 이라고
부르는 쪽이 선호됩니다. 다음은 예시입니다:

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-07-never-type/src/lib.rs:here}}
```

이 코드는 “함수 `bar`는 결코 반환하지 않습니다”라고 읽습니다. 결코 반환하지
않는 함수는 *발산 함수 (diverging functions)* 라고 합니다. `!` 타입의 값은
만들 수 없으므로, `bar`는 결코 반환할 수 없습니다.

그런데 값을 결코 만들 수 없는 타입은 어디에 뜨는거죠? 숫자 추리 게임의
부분인 Listing 2-5의 코드를 기억해 보세요; 여기에 Listing 19-26에서
다시 일부를 재현해 두었습니다.

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:ch19}}
```

<span class="caption">Listing 19-26: `continue`로 끝나는 갈래가
있는 `match`</span>

이 시점에서는 이 코드에서 몇 가지 세부 사항을 건너 뛰었습니다. 6장의
[“`match` 제어 흐름 연산자”][the-match-control-flow-operator]<!-- ignore -->절에서
`match` 갈래가 모두 같은 타입을 반환해야 한다는 것을 논의했습니다. 예를 들어,
다음 코드는 작동하지 않습니다:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-08-match-arms-different-types/src/main.rs:here}}
```

`guess`의 타입은 정수 *그리고* 문자열이어야 하며, 러스트는
`guess`가 하나의 타입만 가져야 한다고 요구합니다. 그럼 `continue`가
무엇을 반환할까요? 어떻게 Listing 19-26에서 한 쪽 갈래는 `u32`를
반환하면서 다른 갈래는 `continue`로 끝나는 것이 허용되었을까요?

짐작하셨겠지만, `continue`는 `!` 값을 가집니다. 즉, 러스트가 `guess`의
타입을 계산할 때, 두 개의 매치 갈래를 모두 살펴보게 되는데, 전자는 `u32` 값을
가지고 후자는 `!` 값을 가집니다. `!`는 결코 값을 가질 수 없으므로, 러스트는
`guess`의 타입이 `u32`라고 결정합니다.

이 동작을 설명하는 정식 방법은 `!` 타입의 표현식이 다른 모든 타입으로
강제 변환될 수 있다는 것입니다. `continue`가 값을 반환하지 않기 때문에,
이 `match` 갈래가 `continue`로 끝나도 괜찮습니다; `continue`는 제어를
반복문의 맨 위로 이동시키기 때문에, `Err` 케이스에서는 `guess`에 값을
할당하지 않습니다.

부정 타입은 `panic!` 매크로와 함께 유용하게 쓰입니다. 값을 생성하거나
패닉을 일으키기 위해 `Option<T>` 값에서 호출한 `unwrap` 함수를 기억해
보시면, 여기 그 정의가 있습니다:

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-09-unwrap-definition/src/lib.rs:here}}
```

이 코드에서도 Listing 19-26의 `match`에서와 같은 일이 발생합니다: `val`이
`T` 타입을 가지고 있고 `panic!`이 `!` 타입을 가지고 있으므로, 러스트는 전체
`match` 표현식의 결과가 `T`라는 것을 알 수 있습니다. 이 코드는 `panic!`이
값을 생성하지 않기 때문에 작동합니다; 패닉은 프로그램을 종료하니까요. `None`의
경우 `unwrap`에서 값을 반환하지 않으므로, 이 코드는 유효합니다.

`!` 타입을 가지는 마지막 표현식은 `loop`입니다:

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-10-loop-returns-never/src/main.rs:here}}
```

여기서 루프는 결코 끝나지 않으므로, `!`가 이 표현식의 값이 됩니다. 하지만
`break`를 포함시키면, 루프는 `break`에 도달했을 때 종료되므로, 이는
참이 아니게 될 것입니다.

### 동적 크기 타입과 `Sized` 트레잇

러스트는 특정 타입의 값에 할당할 공간의 크기 등 타입에 대한 특정 세부
사항을 알아야 합니다. 이로 인해 처음에는 타입 시스템의 한 구석이 약간
혼란스럽습니다: 바로 *동적 크기 타입*의 개념이 그렇습니다. *DTS* 또는
*크기가 지정되지 않은 타입 (unsized type)* 이라고도 하는 이러한 타입을
사용하면 런타임에만 크기를 알 수 있는 값을 사용하여 코드를 작성할 수 있습니다.

이 책 전체에 걸쳐 사용했던 `str`이라는 동적 크기 타입에 대해 자세히
알아보겠습니다. 그렇습니다. `&str`이 아니라 `str` 자체는 DST입니다.
런타임이 될 때까지 문자열의 길이를 알 수 없으므로 `str` 타입의 변수를
만들 수도 없고, `str` 타입의 인수를 받을 수도 없습니다. 아래의
작동하지 않는 코드를 고려해 보세요:


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-11-cant-create-str/src/main.rs:here}}
```

러스트는 특정 타입의 값에 할당할 메모리의 크기을 알아야 하며,
타입의 모든 값은 동일한 크기의 메모리를 사용해야 합니다. 러스트에서
이 코드를 작성할 수 있다면 이 두 `str` 값은 같은 양의 공간을 차지해야
합니다. 그러나 이들은 길이가 다릅니다: `s1`은 12바이트의 저장 공간이
필요하고 `s2`는 15바이트가 필요하기 때문입니다. 이것이 바로 동적 크기를
갖는 변수를 생성할 수 없는 이유입니다.

그럼 어떻게 해야 할까요? 이 경우에는 이미 답을 알고 있습니다: `s1`과
`s2`의 타입을 `str`이 아닌 `&str`으로 만듭니다. 4장의
[“문자열 슬라이스”][string-slice]<!-- ignore -->절에서 슬라이스
데이터 구조는 슬라이스의 시작 위치와 길이만 저장한다는 것을 기억하세요.
따라서 `&T`는 `T`가 위치한 메모리 주소를 저장하는 단일 값이지만,
`&str`은 *두 개의* 값입니다: `str`의 주소와 길이 말이지요. 따라서
컴파일 타임에 `&str` 값의 크기를 알 수 있습니다: `usize` 길이의
두 배입니다. 즉, `&str`이 참조하는 문자열의 길이가 아무리 길어도
항상 `&str`의 크기를 알 수 있습니다. 일반적으로 이것이 러스트에서
동적 크기 타입이 사용되는 방식입니다: 이들은 동적 정보의 크기를
저장하는 추가 메타데이터를 가지고 있습니다. 동적 크기 타입의
황금률은 동적 크기 타입의 값을 항상 어떤 종류의 포인터 뒤에 넣어야
한다는 것입니다.

`str`은 모든 종류의 포인터와 결합될 수 있습니다: 예를 들면, `Box<str>`나 `Rc<str>`
같은 것들이지요. 사실, 여러분은 이전에도 다른 종류의 동적 크기 타입이지만
이런 것을 본적이 있습니다: 바로 트레잇입니다. 모든 트레잇은 그 트레잇의 이름을
사용하여 참조할 수 있는 동적 크기 타입입니다. 17장의
[“트레잇 객체를 사용하여 다른 타입의 값
허용하기”][using-trait-objects-that-allow-for-values-of-different-types]<!-- 무시 -->절에서,
트레잇을 트레잇 객체로 사용하려면 `&dyn Trait` 또는 `Box<dyn Trait>`와
같은 포인터 뒤에 넣어야 한다고 언급했습니다 (`Rc<dyn Trait>`도
가능합니다).

DST로 작업하기 위해 러스트에서는 컴파일 시점에 타입의 크기를 알 수
있는지 여부를 결정하는 `Sized` 트레잇을 제공합니다. 이 트레잇은
컴파일 시 크기가 알려진 모든 것에 대해 자동으로 구현됩니다. 또한
러스트는 암묵적으로 모든 제네릭 함수에 `Sized` 바운드를 추가합니다.
즉, 다음과 같은 제네릭 함수 정의는:

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-12-generic-fn-definition/src/lib.rs}}
```

실제로는 아래와 같이 작성한 것처럼 취급됩니다:

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-13-generic-implicit-sized-bound/src/lib.rs}}
```

기본적으로 제네릭 함수는 컴파일 시점에 크기가 알려진 타입에 대해서만
작동합니다. 그러나 다음과 같은 특별 문법을 사용하여 이 제한을 완화할
수 있습니다:

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-14-generic-maybe-sized/src/lib.rs}}
```

`?Size` 트레잇 바운드는 “`T`는 `Sized`일 수도 있고 아닐 수도 있다”는 의미를
가지며 이 문법은 제네릭 타입이 컴파일 시점에 크기가 알려진 타입이어야 한다는
기본값을 덮어씁니다. 이런 의미의 `?Trait` 문법은 `Sized`에만 사용할 수 있고
다른 어떤 트레잇에도 사용할 수 없습니다.

또한 `t` 매개변수의 타입을 `T`에서 `&T`로 바꾸었음을 주목하세요.
타입이 `Sized`가 아닐 수 있기 때문에 어떤 종류의 포인터 뒤에 놓고
사용해야 합니다. 이 경우에는 참조를 선택했습니다.

다음으로는 함수와 클로저에 대해 이야기해 보겠습니다!

[encapsulation-that-hides-implementation-details]:
ch17-01-what-is-oo.html#encapsulation-that-hides-implementation-details
[string-slices]: ch04-03-slices.html#string-slices
[the-match-control-flow-operator]:
ch06-02-match.html#the-match-control-flow-operator
[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[using-the-newtype-pattern]: ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
