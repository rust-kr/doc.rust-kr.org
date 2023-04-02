## 고급 함수와 클로저

이 절에서는 함수 포인터와 클로저를 반환하는 기능을 포함, 함수와 클로저와
관련된 고급 기능에 대해 살펴봅니다.

### 함수 포인터

지금까지는 함수에 클로저를 전달하는 방법에 대해 설명했는데, 일반
함수를 함수에 전달할 수도 있습니다! 이 기법은 새로운 클로저를 정의하는
대신 이미 정의한 함수를 전달하고 싶을 때 유용합니다. 함수는 (소문자
f를 쓰는) `fn` 타입으로 강제되는데, `Fn` 클로저 트레잇과 혼동하면
안됩니다. `fn` 타입을 *함수 포인터 (function pointer)* 라고 합니다.
함수 포인터로 함수를 전달하면 함수를 다른 함수에 대한 인수로 사용할
수 있습니다.

매개변수가 함수 포인터임을 지정하는 문법은 Listing 19-27에 나온
것처럼 클로저의 문법과 유사하며, 여기서는 매개변수에 1을 더하는
함수 `add_one`을 정의했습니다. `do_twice` 함수는 두 개의 매개변수를
받습니다: `i32` 매개변수를 받아 `i32`를 반환하는 함수를 가리키는 함수
포인터와 하나의 `i32 value`지요. `do_twice` 함수는 `f` 함수를
두 번 호출하여 `arg` 값을 전달한 다음 두 함수 호출 결과를
합산합니다. `main` 함수는 `add_one` 및 `5`를 인수로 사용하여
`do_twice`를 호출합니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-27/src/main.rs}}
```

<span class="caption">Listing 19-27: `fn` 타입을 사용하여 함수 포인터를
인자로 허용하기</span>

이 코드는 `The answer is: 12`를 출력합니다. 여기서는 `do_twice`의 매개변수
`f`가 `i32` 타입의 매개변수 하나를 받아 `i32`를 반환하는 `fn`임을 지정합니다.
그러면 `do_twice`의 본문에서 `f`를 호출할 수 있습니다. `main`에서는 함수 이름
`add_one`을 `do_twice`의 첫 번째 인수로 전달할 수 있습니다.

클로저와 달리 `fn`은 트레잇이 아닌 타입이므로, `Fn` 트레잇 중 하나를
트레잇 바운드로 사용한 제네릭 타입 매개변수를 선언하는 대신에 `fn`을
매개변수 타입으로 직접 지정합니다.

함수 포인터는 세 가지 클로저 트레잇 (`Fn`, `FnMut`, `FnOnce`) 을 모두
구현하므로, 클로저를 기대하는 함수에 대한 인수로 함수 포인터를 언제나
전달할 수 있습니다. 제네릭 타입과 클로저 트레잇 중 하나를 사용하는
함수를 작성하여 함수나 클로저 중 하나를 받아들일 수 있도록 하는 것이
가장 좋습니다.

즉, 클로저가 아닌 `fn`만 허용하고 싶은 경우의 한 가지 예로는
클로저가 없는 외부 코드와 상호작용할 때입니다: C 함수는 함수를
인수로 받을 수 있지만, C에는 클로저가 없습니다.

인라인으로 정의된 클로저나 명명된 함수를 사용할 수 있는 예시로,
표준 라이브러리의 `Iterator` 트레잇이 제공하는 `map` 메소드의
사용을 살펴봅시다. `map` 함수를 사용하여 숫자 벡터를 스트링
벡터로 바꾸려면 다음과 같이 클로저를 사용할 수 있습니다:

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-15-map-closure/src/main.rs:here}}
```

혹은 아래와 같이 클로저 대신 `map`의 인자로 함수 이름을 지정할
수도 있습니다:

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-16-map-function/src/main.rs:here}}
```

`to_string`이라는 이름의 함수가 여러 개 있을 수 있기 때문에, 앞서
[“고급 트레잇”][advanced-traits]<!-- ignore --> 절에서 설명했던
완전 정규화 문법을 사용해야 하는 점을 유의하세요. 여기서는 `ToString`
트레잇에 정의된 `to_string` 함수를 사용하고 있는데, 이는 표준
라이브러리에서 `Display`를 구현하는 모든 타입에 대해 구현되어 있습니다.

6장의 ["열거형 값"][enum-values]<!-- ignore -->절에서 우리가 정의하는
각 열거형 variant의 이름도 이니셜라이저 함수가 된다는 것을 기억해
두세요. 이러한 이니셜라이저 함수는 클로저 트레잇을 구현하는 함수
포인터로써 사용될수 있는데, 이는 다음과 같이 클로저를 취하는 메소드의
인자로 이니셜라이저 함수를 지정할 수 있음을 뜻합니다:

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-17-map-initializer/src/main.rs:here}}
```

여기서는 `Status::Value`의 이니셜라이저 함수를 사용하여 `map`이 호출되는
범위의 각 `u32` 값을 사용한 `Status::Value` 인스턴스를 생성합니다. 어떤
사람들은 이 스타일을 선호하고, 어떤 사람들은 클로저를 사용하는 것을 선호합니다.
둘 다 동일한 코드로 컴파일되므로 여러분에게 더 명확한 스타일을 사용하세요.

### 클로저 반환하기

클로저는 트레잇으로 표현되므로, 클로저를 직접 반환할 수 없습니다.
트레잇을 반환해야 하는 대부분의 경우, 대신 트레잇을 구현하는
구체적 타입을 함수의 반환 값으로 사용할 수 있습니다. 그러나,
클로저에는 반환할 수 있는 구체적 타입이 없기 때문에 그렇게 할
수 없습니다; 예를 들면 함수 포인터 `fn`은 반환 타입으로 사용될
수 없습니다.

다음 코드는 클로저를 직접 반환하려고 시도하지만, 컴파일되지 않습니다:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-18-returns-closure/src/lib.rs}}
```

컴파일 에러는 다음과 같습니다:

```console
{{#include ../listings/ch19-advanced-features/no-listing-18-returns-closure/output.txt}}
```

이 에러는 `Sized` 트레잇을 다시 언급합니다! 러스트는 클로저를 저장하기 위해
얼마나 많은 공간이 필요한지 알 수 없습니다. 이 문제에 대한 해결책을 이전에
살펴봤었지요. 바로 트레잇 객체가 사용될 수 있습니다:

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-19-returns-closure-trait-object/src/lib.rs}}
```

이 코드는 잘 컴파일됩니다. 트레잇 객체에 대한 자세한 내용은 17장의
[“트레잇 객체를 사용하여 다른 타입의 값
허용하기”][using-trait-objects-that-allow-for-values-of-different-types]<!-- ignore -->절을
참고하세요.

다음으로, 매크로를 살펴봅시다!

[advanced-traits]:
ch19-03-advanced-traits.html#advanced-traits
[enum-values]: ch06-01-defining-an-enum.html#enum-values
[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
