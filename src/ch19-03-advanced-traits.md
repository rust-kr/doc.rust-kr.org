## 고급 트레이트

10장의 [‘트레이트로 공통된 동작을 정의하기’][traits-defining-shared-behavior]<!-- ignore -->절에서
트레이트에 대해 처음 다뤘지만, 더 자세한 내용은 다루지
않았습니다. 이제 러스트에 대해 더 많이 알게 되었으니,
핵심을 살펴볼 수 있습니다.

### 연관 타입으로 트레이트 정의에서 자리표시자 타입 지정하기

*연관 타입*은 타입 자리표시자와 트레이트를 연결하여 트레이트 메서드 정의를
할 때 이러한 자리표시자 타입을 시그니처에서 사용할 수 있도록 합니다.
트레이트의 구현자는 특정 구현을 위해서 자리표시자 타입 대신 사용할
구체적인 타입을 지정합니다. 이렇게 하면 트레이트가 구현될 때까지 해당
타입이 무엇인지 정확히 알 필요 없이 임의의 타입을 사용하는 트레이트를
정의할 수 있습니다.

이 장에서 설명하는 대부분의 고급 기능은 거의 필요하지 않다고
설명했습니다. 연관 타입은 그 중간 정도에 해당합니다: 이 책의
나머지 부분에서 설명하는 기능보다는 드물게 사용되지만 이 장에서
설명하는 다른 많은 기능보다는 더 자주 사용됩니다.

연관 타입이 있는 트레이트의 한 예로 표준 라이브러리에서 제공하는
`Iterator` 트레이트가 있습니다. 연관 타입의 이름은 `Item`이며
`Iterator` 트레이트를 구현하는 타입이 반복하는 값의 타입을
나타냅니다. `Iterator` 트레이트의 정의는 예제 19-12에 나와
있습니다.

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-12/src/lib.rs}}
```

<span class="caption">예제 19-12: 연관 타입 `Item`이 있는
`Iterator` 트레이트의 정의</span>

타입 `Item`은 자리표시자이고, `next` 메서드의 정의는 `Option<Self::Item>`
타입의 값을 반환할 것임을 보여줍니다. `Iterator` 트레이트의 구현자는
`Item`의 구체적 타입을 지정하고, `next` 메서드는 해당 구체적 타입의 값을
담고 있는 `Option`을 반환합니다.

연관 타입은 제네릭과 비슷한 개념처럼 보일 수 있는데, 제네릭은 처리할
수 있는 타입을 지정하지 않으면서 함수를 정의할 수 있게 해 준다는
점에서 그렇습니다. 두 개념의 차이점을 살펴보기 위해 `Item` 타입이
`u32`로 지정된 `Counter`라는 타입에 대한 `Iterator` 트레이트 구현을
살펴보겠습니다:

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-22-iterator-on-counter/src/lib.rs:ch19}}
```

이 문법은 제네릭과 비슷해 보입니다. 그렇다면 예제 19-13에 나온 것처럼
제네릭으로 `Iterator` 트레이트를 정의하면 되지 않을까요?

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-13/src/lib.rs}}
```

<span class="caption">예제 19-13: 제네릭을 사용한 `Iterator` 트레이트의
가상 정의</span>

차이점은 예제 19-13에서와 같이 제네릭을 사용할 때는 각 구현에서
타입을 명시해야 한다는 점입니다; `Counter`에 대해 `Iterator<String>`
혹은 다른 타입을 구현할 수도 있기 때문에, `Counter`에 대해 `Iterator`의
구현이 여러 개 있을 수 있습니다. 다시 말해, 한 트레이트에 제네릭
매개변수가 있는 경우, 매번 제네릭 타입 매개변수의 구체적 타입을
변경하면서 한 트레이트에 대해 여러 번 구현할 수 있습니다. `Counter`에
`next` 메서드를 사용할 때, 어떤 `Iterator`의 구현을 사용할지를
나타내기 위한 타입 명시를 제공해야 합니다.

연관 타입을 사용하면 타입에 트레이트를 여러 번 구현할 수 없기
때문에 타입 명시를 할 필요도 사라집니다. 연관 타입을 사용하는 정의가
있는 예제 19-12에서는 `Item`의 타입을 한 번만 선택할 수 있는데,
이는 `impl Iterator for Counter`가 하나만 존재할 수 있기 때문입니다.
`Counter`에서 `next`를 호출할 때마다 `u32` 값의 반복자를 원한다고
지정할 필요가 없습니다.

연관 타입도 트레이트 계약의 일부가 됩니다: 트레이트의 구현자는
연관 타입 자리표시자를 대신할 타입을 제공해야 합니다. 연관 타입은
종종 그 타입이 어떻게 사용되는지 설명하는 이름을 갖게 되며,
API 문서에 연관 타입을 문서화하는 것이 좋습니다.

### 기본 제네릭 타입 매개변수와 연산자 오버로딩

제네릭 타입 매개변수를 사용하면 제네릭 타입에 대한 기본 구체적 타입을
지정할 수 있습니다. 이렇게 하면 기본 타입이 작동하는 경우 트레이트의 구현자가
구체적 타입을 지정할 필요가 없습니다. 제네릭 타입을 선언할 때
`<PlaceholderType=ConcreteType>` 문법을 사용하여 기본 타입을 지정합니다.

이 기법이 유용한 경우 중 좋은 예가 특정 상황에서 (`+` 같은) 연산자의
동작을 커스터마이징하는 *연산자 오버로딩 (operator overloading)*
과 함께 쓰이는 경우입니다.

러스트에서는 자체 연산자를 만들거나 임의의 연산자를 오버로딩할 수
없습니다. 그러나 `std::ops`에 나열된 연산자와 연관된 트레이트를 구현하여
연산자 및 해당 트레이트를 오버로딩할 수 있습니다. 예를 들면, 예제 19-14에서는
`+` 연산자를 오버로딩하여 두 `Point` 인스턴스를
더합니다. 이 작업은 `Point` 구조체에 `Add` 트레이트를 구현하여
수행합니다:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-14/src/main.rs}}
```

<span class="caption">예제 19-14: `Add` 트레이트를 구현하여
`Point` 인스턴스에 대한 `+` 연산자 오버로딩하기</span>

`add` 메서드는 두 `Point` 인스턴스의 `x` 값과 `y` 값을 더하여
새로운 `Point`를 생성합니다. `Add` 트레이트에는 `Output`이라는
연관 타입이 있는데, 이는 `add` 메서드에서 반환되는 타입을
결정합니다.

이 코드에서 기본 제네릭 타입은 `Add` 트레이트 안에 있습니다. 아래는
그 정의입니다:

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

이 코드는 일반적으로 익숙해 보일 것입니다: 하나의 메서드와 연관된 타입이 있는
트레이트라는 점이 말이지요. 새로운 부분은 `Rhs=Self`입니다: 이 문법을
*기본 타입 매개변수 (default type parameter)* 라고 합니다. `Rhs` 기본 타입
매개변수는 (‘오른쪽 (right hand side)’의 줄임말) `add` 메서드에서 `rhs`
매개변수의 타입을 정의합니다. `Add` 트레이트를 구현할 때 `Rhs`에 대한 구체적
타입을 지정하지 않으면 `Rhs`의 타입은 `Add`를 구현하고 있는 타입인 `Self`로
기본 지정됩니다.

`Point`에 대해 `Add`를 구현할 때 두 `Point` 인스턴스를 더하고
싶었으므로 `Rhs`에 대한 기본 타입을 사용했습니다. 기본 타입을 사용하지
않고 `Rhs` 타입을 커스터마이징 하려는 경우에서의 `Add` 트레이트를 구현하는
예를 살펴봅시다.

`Millimeters`와 `Meters`라는 두 개의 구조체에는 서로 다른 단위의
값을 담고 있습니다. 기존 타입을 다른 구조체에서 얇게 감싸는 것을
*뉴타입 패턴 (newtype pattern)* 이라고 하며,
[‘뉴타입 패턴을 사용하여 외부 타입에 외부 트레이트 구현하기’][newtype]<!-- ignore -->절에서
더 자세히 설명합니다. 밀리미터 단위의 값을 미터 단위의 값에 더하고 `Add`의
구현이 변환을 올바르게 수행하도록 하고 싶습니다. 예제 19-15에서 보시는 것처럼
`Meters`를 `Rhs`로 사용하여 `Millimeters`에 대한 `Add`를 구현할 수 있습니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-15/src/lib.rs}}
```

<span class="caption">예제 19-15: `Millimeters`와 `Meters`를 더하기
위하여 `Millimeters`에 대한 `Add` 트레이트 구현하기</span>

`Millimeters`와 `Meters`를 더하기 위해서, `impl Add<Meters>`라고 지정하여
기본 타입인 `Self` 대신 `Rhs` 타입 매개변수의 값을 설정합니다.

두 가지 주요한 방법으로 기본 타입 매개변수를 사용합니다:

* 기존 코드를 깨는 일 없이 타입을 확장하기 위해
* 대부분의 사용자가 필요로 하지 않는 특정 상황에 대한 커스터마이징을 허용하기 위해

표준 라이브러리의 `Add` 트레이트는 두 번째 목적의 예입니다: 일반적으로
두 개의 유사한 타입을 더하지만 `Add` 트레이트는 그 이상으로 커스터마이징할
수 있는 기능을 제공합니다. `Add` 트레이트 정의에서 기본 타입 매개변수를
사용하면 대부분의 경우 추가 매개변수를 지정할 필요가 없습니다.
바꿔 말하면, 약간의 구현 보일러 플레이트가 필요 없으므로 트레이트를 더
쉽게 사용하게 해 줍니다.

첫 번째 목적은 두 번째 목적과 비슷하지만 방향이 반대입니다:
기존 트레이트에 타입 매개변수를 추가하려는 경우, 기본값을 지정하여
기존 구현 코드를 손상시키지 않고 트레이트의 기능을 확장할 수
있습니다.

### 모호성 방지를 위한 완전 정규화 문법: 같은 이름의 메서드 호출하기

러스트에서는 어떤 트레이트에 다른 트레이트의 메서드와 같은 이름의 메서드가
있는 것을 막지 않으며, 한 타입에서 두 트레이트를 모두 구현하는 것도 막지
않습니다. 또한 트레이트의 메서드와 이름이 같은 메서드를 타입에 직접
구현하는 것도 가능합니다.

같은 이름의 메서드를 호출할 때는 어떤 메서드를 사용할지 러스트에 알려줘야
합니다. 예제 19-16의 코드에서는 `Pilot`과 `Wizard`라는 두 개의 트레이트를
정의했는데, 두 트레이트 모두 `fly`라는 메서드를 가지고 있다고 가정해 봅시다.
그런 다음 이미 `fly`라는 메서드가 구현된 `Human` 타입에 두 트레이트를 모두
구현합니다. 각각의 `fly` 메서드는 다른 일을 합니다.

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-16/src/main.rs:here}}
```

<span class="caption">예제 19-16: 두 트레이트에는 `fly` 메서드가
있도록 정의되어 `Human` 타입에 대해 구현되었고, `Human`에 직접 `fly`
메서드가 구현되어 있습니다</span>

`Human`의 인스턴스에 `fly`를 호출하면, 예제 19-17에서 보시는 것처럼
컴파일러는 기본적으로 타입에 직접 구현된 메서드를 호출합니다.

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-17/src/main.rs:here}}
```

<span class="caption">예제 19-17: `Human` 인스턴스에 `fly`
호출하기</span>

이 코드를 실행하면 `*waving arms furiously*`가 출력되어 러스트가
`Human`에 직접 구현된 `fly` 메서드를 호출했음을 보여줍니다.

`Pilot` 트레이트나 `Wizard` 트레이트의 `fly` 메서드를 호출하려면, 더 명확한
문법을 사용하여 어떤 `fly` 메서드를 의도한 것인지 지정할 필요가 있습니다.
예제 19-18에서 이 문법을 보여줍니다.

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-18/src/main.rs:here}}
```

<span class="caption">예제 19-18: 호출하고자 하는 트레이트의 `fly`
메서드 지정하기</span>

메서드 이름 앞에 트레이트 이름을 지정하면 어떤 `fly` 구현을 호출할지
러스트에게 명확히 알릴 수 있습니다. 예제 19-18에서 사용한
`person.fly()`와 동일한 `Human::fly(&person)`를 작성할 수도
있지만, 명확히 할 필요가 없는 경우라면 작성 시간이 조금 더
길어집니다.

이 코드를 실행하면 다음이 출력됩니다:

```console
{{#include ../listings/ch19-advanced-features/listing-19-18/output.txt}}
```

`fly` 메서드는 `self` 매개변수를 취하기 때문에, 하나의 *트레이트*를 구현하는
두 개의 *타입*이 있다면, 러스트는 `self`의 타입에 따라 어떤 트레이트의 구현체를
사용하려는 것인지 알아낼 수 있습니다.

그러나, 메서드가 아닌 연관 함수에는 `self` 매개변수가 없습니다.
동일한 함수명을 가진 메서드가 아닌 함수가 정의된 여러 타입 또는
트레이트가 있는 경우, *완전 정규화 문법 (fully qualified syntax)* 을 사용하지 않는 한 러스트는
어떤 타입을 의미하는지 항상 알 수 없습니다. 예를 들어, 예제 19-19에서는
모든 강아지의 이름을 *스팟 (Spot)* 으로 지정하려는 동물 보호소에 대한
트레이트를 생성합니다. 메서드가 아닌 연관 함수 `baby_name`이 있는 `Animal`
트레이트를 만듭니다. `Animal` 트레이트는 구조체 `Dog`에 대해 구현되며, 여기에도
메서드가 아닌 연관 함수 `baby_name`가 직접 제공됩니다.

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-19/src/main.rs}}
```

<span class="caption">예제 19-19: 연관 함수가 있는 트레이트와
이 트레이트를 구현하면서 동시에 같은 이름의 연관 함수가 있는
타입</span>

모든 강아지 이름을 스팟으로 짓는 코드를 `Dog`에 정의된 `baby_name`
연관 함수에 구현합니다. `Dog` 타입은 모든 동물이 가지고 있는
특성을 기술하는 `Animal` 트레이트도 구현합니다. 아기 개는 강아지 (puppy)
라고 불리며, 이는 `Animal` 트레이트와 연관된 `baby_name` 함수에서
`Dog`에 대한 `Animal` 트레이트의 구현으로 표현됩니다.

`main`에서는 `Dog::baby_name` 함수를 호출하는데, 이는 `Dog`에 직접 정의된
연관 함수를 호출합니다. 이 코드는 다음을 출력합니다:

```console
{{#include ../listings/ch19-advanced-features/listing-19-19/output.txt}}
```

이 출력은 우리가 원했던 것이 아닙니다. 우리는 `Dog`에 구현한 `Animal`
트레이트에 속하는 `baby_name` 함수를 호출하여 코드가 `A baby dog is
called a puppy`라고 출력하기를 원합니다. 예제 19-18에서 사용했던
트레이트 이름을 지정하는 기법은 여기서 도움이 되지 않습니다; 예제 19-20의
코드로 `main`을 변경하면 컴파일 에러가 발생합니다.

<span class="filename">파일명: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-20/src/main.rs:here}}
```

<span class="caption">예제 19-20: `Animal` 트레이트의 `baby_name`을
호출하는 시도이지만, 러스트는 어떤 구현체를 사용해야 하는지 알지
못합니다</span>

`Animal::baby_name`은 `self` 매개변수가 없고, `Animal` 트레이트를 구현하는
다른 타입이 있을 수 있기 때문에, 러스트는 우리가 원하는 `Animal::baby_name`의
구현체를 알 수 없습니다. 다음과 같은 컴파일 에러가 발생합니다:

```console
{{#include ../listings/ch19-advanced-features/listing-19-20/output.txt}}
```

다른 타입에 대한 `Animal` 구현체가 아니라 `Dog`에 대한 `Animal`
구현체를 사용하고 싶다는 것을 명확히 하고 러스트에 알려주려면,
완전 정규화 문법을 사용해야 합니다. 예제 19-21은 완전 정규화
문법을 사용하는 방법을 보여줍니다.

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-21/src/main.rs:here}}
```

<span class="caption">예제 19-21: 완전 정규화 문법을 사용하여
`Dog`에 구현된 `Animal` 트레이트의 `baby_name`을 호출하고자 함을
지정하기</span>

꺽쇠 괄호 안에 타입 명시를 제공하고 있는데, 이는 이 함수 호출에
대해 `Dog` 타입을 `Animal`로 취급하고 싶다고 알려줌으로써 `Dog`에
구현된 `Animal` 트레이트의 `baby_name` 메서드를 호출하고 싶음을
나타냅니다. 이제 이 코드는 우리가 원하는 것을 출력합니다:

```console
{{#include ../listings/ch19-advanced-features/listing-19-21/output.txt}}
```

일반적으로 완전 정규화 문법은 다음과 같이 정의됩니다:

```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

메서드가 아닌 연관 함수의 경우 `receiver`가 없습니다: 다른
인수의 목록만 있을 뿐입니다. 함수나 메서드를 호출하는 곳이라면
어디든 완전 정규화 문법을 사용할 수 있습니다. 그러나 이 문법에서
프로그램의 다른 정보로부터 러스트가 알아낼 수 있는 부분은 생략할
수 있습니다. 동일한 이름을 사용하는 구현이 여러 개 있고 러스트가
호출하려는 구현을 식별하는 데 도움이 필요한 경우에만 이렇게 더
자세한 문법을 사용하면 됩니다.

### 슈퍼트레이트를 사용하여 한 트레이트에서 다른 트레이트의 기능을 요구하기

때로는 다른 트레이트에 의존하는 트레이트 정의를 작성할 수 있습니다:
어떤 타입이 첫 번째 트레이트를 구현하려면 해당 타입이 두 번째 트레이트도
구현하도록 요구할 수 있습니다. 이렇게 하면 트레이트 정의가 두 번째
트레이트의 연관 아이템을 활용할 수 있습니다. 트레이트 정의가 의존하고
있는 트레이트를 트레이트의 *슈퍼트레이트 (supertrait)* 이라고 합니다.

예를 들어, 주어진 값을 형식화하여 애스터리스크 `*`로 둘러싸서 출력하는
`outline_print` 메서드가 있는 `OutlinePrint` 트레이트를 만들고
싶다고 가정해 봅시다. 즉, 표준 라이브러리 트레이트 `Display`를 구현하여
`(x, y)`를 출력하는 `Point` 구조체가 주어졌을 때, `x`에 `1`, `y`에
`3`이 있는 `Point` 인스턴스에서 `outline_print`를 호출하면 다음과
같이 출력되어야 합니다:

```text
**********
*        *
* (1, 3) *
*        *
**********
```

`outline_print` 메서드의 구현에서 `Display` 트레이트의 기능을
사용하고자 합니다. 따라서, `Display`를 구현하고 `OutlinePrint`가
요구하는 기능을 제공하는 타입에 대해서만 `OutlinePrint` 트레이트가
작동할 것임을 명시해야 합니다. 트레이트 정의에서 `OutlinePrint: Display`를
지정하는 것으로 그렇게 할 수 있습니다. 이 기법은 트레이트에 트레이트
바운드를 추가하는 것과 유사합니다. 예제 19-22는 `OutlinePrint`
트레이트의 구현을 보여줍니다.

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-22/src/main.rs:here}}
```

<span class="caption">예제 19-22: `Display`의 기능을 요구하는
`OutlinePrint` 트레이트 구현하기</span>

`OutlinePrint`에 `Display` 트레이트가 필요하다고 지정했으므로,
`Display`를 구현하는 모든 타입에 대해 자동으로 구현되는 `to_string`
함수를 사용할 수 있습니다. 트레이트 이름 뒤에 콜론 및 `Display` 트레이트를
추가 지정하지 않고 `to_string`을 사용하려고 하면, 현재 스코프에서
`&Self` 타입에 대해 `to_string`이라는 이름의 메서드를 찾을 수 없다는
에러가 발생합니다.

`Point` 구조체와 같이 `Display`를 구현하지 않는 타입에 `OutlinePrint`를
구현하려고 하면 어떻게 되는지 살펴봅시다:

<span class="filename">파일명: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-02-impl-outlineprint-for-point/src/main.rs:here}}
```

`Display`가 필요하지만 구현되지 않았다는 에러가 발생합니다:

```console
{{#include ../listings/ch19-advanced-features/no-listing-02-impl-outlineprint-for-point/output.txt}}
```

이를 해결하려면, `Point`에 `Display`를 구현하고 `OutlinePrint`가
요구하는 제약 조건을 만족시키면 됩니다:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-03-impl-display-for-point/src/main.rs:here}}
```

그러면 `Point`에 `OutlinePrint` 트레이트를 구현하면 컴파일이 성공적으로
완료되고, `Point` 인스턴스에 대해 `outline_print`를 호출하여 애스터리스크로
둘러싼 형식으로 출력할 수 있습니다.

### 뉴타입 패턴을 사용하여 외부 타입에 외부 트레이트 구현하기

10장 [‘특정 타입에 트레이트 구현하기’][implementing-a-trait-on-a-type]<!-- ignore -->절에서
트레이트나 타입이 우리 크레이트의 것인 경우에만 타입에 트레이트를
구현할 수 있다는 고아 규칙에 대해 설명한 바 있습니다. 튜플 구조체로
새로운 타입을 생성하는 *뉴타입 패턴 (newtype pattern)* 을 사용하면 이 제한을 우회할 수
있습니다. (튜플 구조체에 대해서는 5장의
[‘명명된 필드 없는 튜플 구조체를 사용하여 다른 타입 만들기’][tuple-structs]<!-- ignore -->절에서 
다루었습니다.) 튜플 구조체는 하나의 필드를 가지며 트레이트를
구현하고자 하는 타입을 얇게 감싸는 래퍼 (wrapper) 가
됩니다. 그러면 래퍼 타입은 우리 크레이트 내에 있게 되어
래퍼에 대한 트레이트를 구현할 수 있습니다. *뉴타입*은
하스켈 프로그래밍 언어에서 유래한 용어입니다. 이 패턴을
사용해도 런타임 성능에 대한 불이익은 없으며, 래퍼 타입은
컴파일 시 제거됩니다.

예를 들어, `Vec<T>`에 대해 `Display`를 구현하고 싶다고 가정해 보면,
`Display` 트레이트와 `Vec<T>` 타입이 크레이트 외부에 정의되어 있으므로
고아 규칙에 의해 직접 구현할 수는 없습니다. `Vec<T>`의 인스턴스를 보유하는
`Wrapper` 구조체를 만들 수 있습니다; 그러면 예제 19-23에 나온 것처럼
`Wrapper`에 `Display`를 구현하고 `Vec<T>` 값을 사용할 수 있습니다.

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-23/src/main.rs}}
```

<span class="caption">예제 19-23: `Display` 구현을 위해서 `Vec<String>`을
감싼 `Wrapper` 타입 만들기</span>

`Wrapper`는 튜플 구조체이고 `Vec<T>`는 튜플의 인덱스 0에 있는 아이템이기
때문에, `Display`의 구현체는 `self.0`을 사용하여 내부 `Vec<T>`에 액세스합니다.
그러면 `Wrapper`에서 `Display` 타입의 기능을 사용할 수 있습니다.

이 기법을 사용할 때의 단점은 `Wrapper`가 새로운 타입이기
때문에 보유하고 있는 값의 메서드가 없다는 것입니다.
메서드가 `self.0`에 위임되도록 `Vec<T>`의 모든 메서드를
`Wrapper`에 직접 구현해야 `Wrapper`를 `Vec<T>`와 똑같이
취급할 수 있습니다. 이 뉴타입이 내부 타입의 모든 메서드를
갖기를 원한다면, (15장의
[‘`Deref` 트레이트로 스마트 포인터를 보통의 참조자처럼 취급하기’][smart-pointer-deref]<!-- ignore -->절에서
설명했던) `Deref` 트레이트를 `Wrapper`에 구현해 내부 타입을
반환하는 것이 해결책이 될 수 있습니다. `Wrapper` 타입이 내부
타입의 모든 메서드를 갖지 않도록 하려면 (이를테면 `Wrapper`
타입의 동작을 제한하려면) 원하는 메서드만 수동으로 구현해야 합니다.

이 뉴타입 패턴은 또한 트레이트가 포함되지 않은 경우에도 유용합니다. 이제 초점을
바꿔서 러스트의 타입 시스템과 상호작용하는 몇 가지 고급 방법을 살펴봅시다.

[newtype]: ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
[implementing-a-trait-on-a-type]:
ch10-02-traits.html#implementing-a-trait-on-a-type
[traits-defining-shared-behavior]:
ch10-02-traits.html#traits-defining-shared-behavior
[smart-pointer-deref]: ch15-02-deref.html#treating-smart-pointers-like-regular-references-with-the-deref-trait
[tuple-structs]: ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
