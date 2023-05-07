<!-- Old heading. Do not remove or links may break. -->
<a id="closures-anonymous-functions-that-can-capture-their-environment"></a>

## 클로저: 자신의 환경을 캡처하는 익명 함수

러스트의 *클로저*는 변수에 저장하거나 다른 함수에 인자로 넘길 수 
있는 익명 함수입니다. 한 곳에서 클로저를 만들고 다른 컨텍스트 내의 다른
곳에서 그것을 평가하기 위해 호출할 수 있습니다. 함수와 다르게 클로저는 
정의된 스코프에서 값을 캡처할 수 있습니다. 앞으로 클로저의 이러한 
기능이 코드 재사용과 동작 사용자 정의를 어떻게 가능케 하는지 살펴볼 
것입니다.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>

### 클로저로 환경 캡처하기

먼저 클로저를 사용하여 나중에 사용하기 위해 클로저가 정의된 환경으로부터
값을 캡처하는 방법을 시험해 보겠습니다. 여기 시나리오가 있습니다: 이따금씩
우리의 티셔츠 회사는 프로모션으로 우리의 메일링 리스트에 있는 사람들에게
독점 공급하는 한정판 티셔츠를 증정합니다. 메일링 리스트에 있는 사람들은
추가적으로 그들의 프로파일에 제일 좋아하는 색상을 추가할 수 있습니다.
만일 무료 티셔츠에 추첨된 사람이 좋아하는 색상을 설정해두었다면, 그 색상의
티셔츠를 받게 됩니다. 만일 그 사람이 좋아하는 색상을 특정하지 않았다면 회사가
현재 제일 많이 가지고 있는 색상을 받게 됩니다.

이를 구현하는 방법은 여러가지가 있습니다. 이번 예제에서는 `Red`와
`Blue` 배리언트를 가진 (단순함을 위해 가능한 색상을 제한했습니다)
`ShirtColor`라는 열거형을 이용해 보겠습니다. 회사의 재고는 `Inventory`
구조체로 표현하는데 여기에는 `shirts`라는 이름의 필드가 있고, 이 필드는
현재 재고에 있는 셔츠 색상을 나타내는 `Vec<ShirtColor>` 타입입니다.
`Inventory` 상에 정의된 `giveaway` 메서드는 무료 티셔츠를 타게 된
사람의 추가 색상 설정값을 얻어와서 그 사람이 받게 될 셔츠 색상을
반환합니다. 이러한 설정이 Listing 13-1에 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

<span class="caption">Listing 13-1: 셔츠 회사 증정 상황</span>

`main`에 정의된 `store`에는 이 한정판 프로모션 배포를 위해 남은 두 개의
파란색 셔츠와 하나의 빨간색 셔츠를 가지고 있습니다. 여기서 빨간색 셔츠로
설정한 고객과 색상 설정이 없는 고객에 대하여 `giveaway` 메서드를 호출하였습니다.

다시 한번 말하지만 이 코드는 여러가지 방법으로 구현될 수 있고, 여기서는
클로저에 초점을 맞추기 위해서 클로저가 사용된 `giveaway` 메서드 본문을
제외하고는 여러분이 이미 배운 개념만 하용했습니다. `giveaway` 메서드에서는
고객의 설정을 `Option<ShirtColor>` 타입의 매개변수 `user_preference`로
`unwrap_or_else` 메서드를 호출합니다.
[`Option<T>`의 `unwrap_or_else` 메서드 `Option<T>`][unwrap-or-else]<!-- ignore -->는
표준 라이브러리에 정의되어 있습니다. 이것은 하나의 인자를 받습니다: 바로 아무런
인자도 없고 `T` 값을 반환하는 클로저 입니다 (이때 `T`는 `Option<T>`의 `Some`
배리언트에 저장되는 타입과 동일하며, 지금의 경우 `ShirtColor`입니다). 만일
`Option<T>`가 `Some` 배리언트라면, `unwrap_or_else`는 그 `Some` 안에 들어있는
값을 반환합니다. 만일 `Option<T>`가 `None` 배리언트라면, `unwrap_or_else`는
이 클로저를 호출하여 클로저가 반환한 값을 반환해 줍니다.

`unwrap_or_else`의 인자로는 `|| self.most_stocked()`이라는 클로저
표현식을 명시했습니다. 이는 아무런 매개변수를 가지지 않는 클로저입니다
(만일 클로저가 매개변수를 갖고 있다면 두 개의 세로 막대 사이에 매개변수가 나올
것입니다). 클로저의 본문은 `self.most_stocked()`를 호출합니다.
여기서는 클로저가 정의되어 있고, 결과값이 필요해진 경우 `unwrap_or_else`의
구현 부분이 이 클로저를 나중에 평가할 것입니다.

이 코드를 실행하면 다음이 출력됩니다:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

여기서 한가지 흥미로운 관점은 현재의 `Inventory` 인스턴스 상에서
`self.most_stocked()`를 호출하는 클로저를 넘겼다는 것입니다. 표준 라이브러리는
우리가 정의한 `Inventory`나 `ShirtColor` 타입이나, 혹은 이 시나리오에서
우리가 사용하고자 하는 로직에 대해 전혀 알 필요가 없습니다. 이 클로저는
`self` `Inventory` 인스턴스의 불변 참조자를 캡처하여  우리가 지정한 코드와
함께 이 값을 `unwrap_or_else` 메서드에 넘겨줍니다. 반면에 함수는 이런 방식으로
자신의 환경을 캡처할 수 없습니다.

### 클로저 타입 추론과 명시

함수와 클로저 간의 차이점은 더 있습니다. 클로저는 보통 매개변수 혹은
반환값의 타입을 `fn` 함수에서처럼 명시하도록 요구하지 않습니다.
함수의 타입 명시는 요구되는데 이는 타입이 여러분의 사용자들에게
노출되는 명시적인 인터페이스의 일부분이기 때문입니다. 이러한 인터페이스를
완고하게 정의하는 것은 함수가 어떤 타입의 값을 사용하고 반환하는지에 대해
모두가 동의하도록 확신시키는 데에 중요합니다. 반면에 클로저는 함수처럼
노출된 인터페이스로 사용되지 않습니다: 클로저는 이름이 지어지거나 우리
라이브러리의 사용자들에게 노출되지 않은 채로 변수에 저장되고 사용됩니다.

클로저는 통상적으로 짧고 임의의 시나리오보다는 짧은 컨텍스트 내에서만
관련됩니다. 이러한 한정된 컨텍스트 내에서, 컴파일러는 대부분의 변수에
대한 타입을 추론할 수 있는 방법과 유사한 식으로 클로저의 매개변수와
반환 타입을 추론합니다 (컴파일러가 클로저 타입을 명시하도록 요구하는
곳 또한 드물게 있습니다).

변수와 마찬가지로, 딱 필요한 것만 적지 않고 더 장황하게 되는 비용을
지불하고서라도 명시성과 명확성을 올리고 싶다면 타입 명시를 추가할 수
있습니다. 클로저에 대한 타입 명시는 Listing 13-2과 같이 생긴 정의처럼
보일 것입니다. 이 예제에서는 Listing 13-1에서처럼 인자로 넘기고 싶은
위치에서 클로저를 정의하기 보다는, 클로저를 정의하여 변수에 저장하고
있습니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

<span class="caption">Listing 13-2: 클로저에 매개변수와 반환값의
타입을 추가적으로 명시하기</span>

타입 명시가 추가되면 클로저의 문법은 함수의 문법과 더욱 유사해
보입니다. 아래는 매개변수의 값에 1을 더하는 함수와, 그와 동일한
동작을 수행하는 클로저를 비교하기 위해 정의해본 것입니다. 관련된
부분들의 열을 맞추기 위해 공백을 좀 추가했습니다. 아래는 파이프의
사용과 부차적인 문법들을 제외하면 클로저의 문법이 함수 문법과 얼마나
비슷한지를 보여줍니다:

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

첫번째 줄은 함수의 정의를 보여주고, 두번째 줄은 모든 것이 명시된
클로저 정의를 보여줍니다. 세번째 줄에서는 타입 명시를 제거했습니다.
네번째 줄에서는 중괄호를 제거했는데, 이 클로저의 본문이 딱 하나의
표현식이기 때문에 그럴 수 있습니다. 위의 모든 방식이 호출했을 때 동일한
동작을 수행하는 유효한 정의법입니다. `add_one_v3`와 `add_one_v4` 줄을
컴파일하기 위해서는 이 클로저들이 평가되는 곳이 필요한데, 그 이유는 이
클로저들이 사용된 곳으로부터 타입이 추론될 것이기 때문입니다. 이는
`let v = Vec::new();`가 러스트에 의해 타입이 추론되기 위해서 타입 명시
혹은 `Vec` 안에 집어넣을 어떤 타입의 값이 필요한 것과 유사합니다.

클로저 정의에 대하여, 컴파일러는 각각의 매개변수와 변환값 마다 하나의
고정 타입을 추론할 것입니다. 예를 들면 Listing 13-3은 자신이
매개변수로 받은 값을 그냥 반환하는 짧은 클로저의 정의를 보여주고
있습니다. 이 클로저는 이 예제 용도 말고는 그다지 유요하진 않습니다.
정의에 아무런 타입 명시를 하지 않았음을 주의하세요. 아무런 타입
명시도 없으므로 아무 타입에 대해서나 이 클로저를 호출할 수 있는데,
여기서는 처음에 `String`에 대해 호출했습니다. 그런 다음 정수에
대해 `example_closure`의 호출을 시도한다면, 에러를 얻게 됩니다.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

<span class="caption">Listing 13-3: 두 개의 다른 타입에 대해 타입이
추론되는 클로저 호출 시도하기</span>

컴파일러는 아래와 같은 에러를 냅니다:

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

처음 `String`을 가지고 `example_closure`를 호출하면, 컴파일러는
클로저의 `x` 타입과 반환 타입이 `String`이라고 추론합니다. 그런 다음
이 타입들은 `example_closure` 클로저에 잠기게 되고, 그다음 동일한 클로저를
가지고 다른 타입에 대해 사용 시도했을 때 타입 에러를 얻게 됩니다.

### 참조자를 캡처하거나 소유권 이전하기

클로저는 세 가지 방식으로 자신의 환경으로부터 값을 캡처할 수 있는데,
이는 함수가 매개변수를 취하는 세 가지 방식과 직접적으로 대응됩니다:
불변으로 빌려오기, 가변으로 빌려오기, 그리고 소유권 이전하기이죠.
클로저는 캡처된 값이 어떻게 쓰이는지에 기초하여 어떤 방법을 쓸 지
결정할 것입니다.

Listing 13-4에서 정의한 클로저는 `list`라는 이름의 벡터에 대한 불변
참조자를 캡처하는데, 이는 그저 값을 출력하기 위한 불변 참조자가 필요한
상태이기 때문입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

<span class="caption">Listing 13-4: 불변 참조자를 캡처하는 클로저의
정의와 호출</span>

또한 이 예제는 어떤 변수가 클로저의 정의에 묶일 수 있고, 이 클로저는
나중에 마치 변수 이름이 함수 이름인 것처럼 변수 이름과 괄호를 사용하여
호출될 수 있음을 보여줍니다.

`list`에 대한 여러 개의 불변 참조자를 동시에 가질 수 있기
때문에, `list`에는 클로저 정의 전이나 후 뿐만 아니라 클로저의
호출 전과 후에도 여전히 접근이 가능합니다. 이 코드는 컴파일되고,
실행되고, 다음을 출력합니다:

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

다음으로 Listing 13-5에서는 클로저의 본문을 바꾸어 `list` 벡터에 요소를
추가하도록 했습니다. 클로저는 이제 가변 참조자를 캡처합니다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

<span class="caption">Listing 13-5: 가변 참조자를 캡처하는 클로저의
정의와 호출</span>

이 코드는 컴파일되고, 실행되고, 다음을 출력합니다:

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

`borrows_mutably` 클로저의 정의와 호출 사이에 더이상 `println!`이 없음을
주목하세요: `borrows_mutably`가 정의된 시점에, 이 클로저가 `list`에 대한
가변 참조자를 캡처합니다. 클로저가 호출된 이후로 다시 클로저를 사용하고
있지 않으므로, 가변 빌림이 그 시점에서 끝납니다. 클로저 정의와 호출
사이에는 출력을 위한 불변 빌림이 허용되지 않는데, 이는 가변 빌림이 있을
때는 다른 빌림이 허용되지 않기 때문입니다. `println!`를 추가해서 어떤
에러가 나오는지 시도해보세요!

엄밀하게는 클로저의 본문에서 사용하고 있는 값의 소유권이 필요하진
않더라도 만약 여러분이 클로저가 소유권을 갖도록 만들고 싶다면,
매개변수 리스트 전에 `move` 키워드를 사용할 수 있습니다.

이 기술은 클로저를 새 스레드에 넘길때 데이터를 이동시켜서 새로운 스레드가
이 데이터를 소유하도록 하는데 대체로 유용합니다. 스레드가 무엇이고 왜 이를
사용하게 되는지에 대한 자세한 내용은 16장에서 동시성에 대한 이야기를 할 때
다루기로 하고, 지금은 `move` 키워드가 필요한 클로저를 사용하는 새 스레드의
생성을 간략하게만 둘러봅시다. Listing 13-6은 Listing 13-4를 수정하여
메인 스레드가 아닌 새 스레드에서 벡터를 출력하는 코드를 보여줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

<span class="caption">Listing 13-6: 스레드에 대한 클로저가 `list`의
소유권을 갖도록 `move` 사용하기</span>

여기서는 새 스레드를 생성하여 여기에 인자로 실행될 클로저를 제공합니다.
클로저의 본문에서는 리스트를 출력합니다. Listing 13-4에서는 클로저가
불변 참조자만 사용하여 `list`를 캡처했는데, 이것이 `list`를 출력하기
위해 필요한 최소한의 접근 수준이기 때문입니다. 이 예제에서는 클로저 본문이
여전히 불변 참조자만 필요할지라도, 클로저 정의의 앞부분에 `move` 키워드를
집어넣어 `list`가 이전되어야 함을 명시할 필요가 있습니다.
새로운 스레드는 메인 스레드의 나머지 부분이 끝나기 전에 끝날 수도
있고, 혹은 메인 스레드가 먼저 끝날 수도 있습니다. 만일 메인 스레드가
`list`의 소유권을 유지하고 있는데 새 스레드가 끝나기 전에 끝나버려서
`list`를 제거한다면, 새 스레드의 불변 참조자는 유하지 않게 될 것입니다.
따라서, 컴파일러는 `list`가 새 스레드에 제공될 클로저로 이전되어
참조자가 유효하도록요구합니다. `move` 키워드를 제거하거나 클로저가
정의된 이후 메인 스레드에서 `list`를 사용하면  어떤 컴파일러 에러를
얻게 되는지 시도해 보세요!

<!-- Old headings. Do not remove or links may break. -->
<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>

### 캡처된 값을 클로저 밖으로 이전하기와 `Fn` 트레잇

어떤 클로저가 자신이 정의된 환경으로부터 참조자 혹은 값의 소유권을 일단
캡처하면 (그래서 클로저의 *안으로* 무언가 이동된 것에 영향을 준다면),
클로저의 본문 내의 코드는 이 클로저가 나중에 평가될 때 그 참조자나 값에
어떤 일이 벌어지는지를 정의합니다 (그래서 클로저의 *밖으로* 무언가
이동되는 것에 영향을 끼칩니다). 클로저 본문은 다음에 제시되는 어떤 것이든
할 수 있습니다: 캡처된 값을 클로저 밖으로 이전시키기, 캡처된 값을 변형하기,
이전시키지도 변형시키지도 않기, 혹은 시작 단계에서부터 환경으로부터 아무
값도 캡처하지 않기.

클로저가 환경으로부터 값을 캡처하고 다루는 방식은 이 클로저가 어떤
트레잇을 구현하지는지에 영향을 주고, 트레잇은 함수와 구조체가 자신이
사용할 수 있는 클로저의 종류를 명시할 수 있게 해주는 방법입니다. 클로저는
클로저의 본문이 값을 어떻게 다루는지에 따라서 이 `Fn` 트레잇들 중 하나,
둘, 혹은 셋 모두를 첨가하는 방식으로 자동적으로 구현할 것입니다:

1. `FnOnce`는 한번만 호출될 수 있는 클로저에게 적용됩니다. 모든 클로저들은
   호출될 수 있으므로, 최소한 이 트레잇은 구현해둡니다. 캡처된 값을 본문
   밖으로 이전시키는 클로저에 대해서는 `FnOnce`만 구현되며 나머지 `Fn` 트레잇은
   구현되지 않는데, 이는 이 클로저가 딱 한번만 호출될 수 있기 때문입니다.
2. `FnMut`은 본문 밖으로 캡처된 값을 이전시키지는 않지만 값을 변경할 수는
   있는 클로저에 대해 적용됩니다. 이러한 클로저는 한번 이상 호출될 수
   있습니다.
3. `Fn`은 캡처된 값을 본문 밖으로 이동시키지 않고 캡처된 값을 변경하지도
   않는 클로저, 뿐만 아니라 환경으로부터 아무런 값도 캡처하지 않는 클로저에
   적용됩니다. 이러한 클로저는 자신의 환경을 변경시키지 않으면서 한번 이상
   호출될 수 있는데, 이는 클로저가 동시에 여러번 호출되는 등의 경우에서
   중요합니다.

Listing 13-1에서 사용했던 `Option<T>`의 `unwrap_or_else` 메서드 정의를
살펴봅시다:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

`T`가 `Option`의 `Some` 배리언트 내 값의 타입을 나타내는 제네릭 타입임을
상기합시다. 이 타입 `T`는 또한 `unwrap_or_else` 함수의 반환 타입이기도
합니다: 예를 들어 `Option<String>` 상에서 `unwrap_or_else`를 호출하면
`String`을 얻을 것입니다.

다음으로, `unwrap_or_else` 함수가 추가로 제네릭 타입 매개변수 `F`를
갖고 있음을 주목하세요. `F` 타입은 `f`라는 이름의 매개변수의 타입인데,
이것이 `unwrap_or_else`를 호출할 때 제공하는 클로저입니다.

제네릭 타입 `F`에 명시된 트레잇 바운드는 `FnOnce() -> T`인데,
이는 `F`가 한번만 호출될 수 있어야 하고, 인자가 없고, `T`를 반환함을
의미합니다. 트레잇 바운드에 `FnOnce`를 사용하는 것은 `unwrap_or_else`가
`f`를 아무리 많아야 한번만 호출할 것이라는 제약 사항을 표현해줍니다.
`unwrap_or_else`의 본문을 보면 `Option`가 `Some`일때 `f`가 호출되지 않을
것임을 알 수 있습니다. 만일 `Option`이 `None`라면 `f`가 한번만 호출될
것입니다. 모든 클로저가 `FnOnce`를 구현하므로 `unwrap_or_else`는 가장
다양한 종류의 클로저를 허용하며 될 수 있는 한 유연하게 동작합니다.

> Note: 함수도 이 세 종류의 `Fn` 트레잇을 모두 구현할 수 있습니다. 만일
> 하고자 하는 것이 환경으로부터 값을 캡처할 필요가 없다면, `Fn` 트레잇 중
> 하나를 구현한 무언가가 필요한 곳에 클로저 대신 함수 이름을 사용할 수
> 있습니다. 예를 들면 `Option<Vec<T>>`의 값 상에서
> `unwrap_or_else(Vec::new)`를 호출하여 이 값이 `None`일 경우
> 비어있는 새 벡터를 얻을 수 있습니다.

이제 표준 라이브러리에서 슬라이스 상에 정의되어 있는 메서드인 `sort_by_key`를
살펴보면서 `unwrap_or_else`와는 어떻게 다르고 `sort_by_key`의 트레잇
바운드는 왜 `FnOnce` 대신 `FnMut`인지를 알아봅시다. 이 클로저는 처리하려고
하는 슬라이스 내에서 현재 아이템에 대한 참조자를 하나의 인자로 받아서,
순서를 메길 수 있는 `K` 타입의 값을 반환합니다. 이 함수는 각 아이템의
특정 속성을 이용하여 슬라이스를 정렬하고 싶을 때 유용합니다. Listing
13-7에는 `Rectangle` 인스턴스의 리스트가 있고 `sort_by_key`를 사용하여
`width` 속성을 낮은 것부터 높은 순으로 정렬합니다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

<span class="caption">Listing 13-7: `sort_by_key`를 사용하여 너비로
사각형 정렬하기</span>

이 코드는 다음을 출력합니다:

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

`sort_by_key`가 `FnMut` 클로저를 갖도록 정의된 이유는 이 함수가 클로저를
여러번 호출하기 때문입니다: 슬라이스 내 각 아이템마다 한번씩요. 클로저 `|r|
r.width`는 자신의 환경으로부터 어떤 것도 캡처나 변형하기, 혹은 이전시키지
않으므로, 트레잇 바운드 요건을 충족합니다.

반면 Listing 13-8은 `FnOnce` 트레잇만 구현한 클로저의 예를
보여주는데, 이 클로저는 환경으로부터 값을 이전시키고 있습니다.
컴파일러는 이 클로저를 `sort_by_key`에 사용할 수 없게 할 것입니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

<span class="caption">Listing 13-8: `FnOnce` 클로저를 `sort_by_key`에 사용
시도하기</span>

이는 `list`를 정렬할 때 `sort_by_key`가 클로저를 호출하는 횟수를 세려고
시도하는 부자연스럽고 대단히 난해한 방식입니다 (동작하지 않습니다). 이 코드는
이 횟수 세기를 `value`라는 클로저 환경으로부터의 `String` 값을
`sort_operations` 벡터에 밀어넣는 형태로 시도하고 있습니다. 클로저는
`value`를 캡처하여 `sort_operations` 벡터로 소유권을 이전시키는 것으로
`value`를 클로저 밖으로 이전시킵니다. 이 클로저는 한번만 호출될 수 있습니다;
두번째 호출 시도에서는 `value`가 더 이상 이 환경에 남아있지 않은데
`sort_operations`로 밀어넣으려고 하므로 동작하지 않을 것입니다! 따라서,
이 클로저는 오직 `FnOnce`만 구현하고 있습니다. 이 코드의 컴파일 시도를 하면,
클로저가 `FnMut`를 구현해야 하기 때문에 `value`가 클로저 밖으로 이전될 수 없음을
지적하는 에러를 얻게 됩니다:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

이 에러는 환경에서 `value` 값을 빼내는 클로저 본문의 라인을 지적합니다.
이를 고치기 위해서는 클로저 본문을 수정하여 환경에서 값을 이전시키지
않도록 할 필요가 있습니다. `sort_by_key`가 호출되는 횟수를 세기
위해서는 환경쪽에 카운터를 유지하면서 클로저 본문에서 이 값을 증가시키는
것이 더 직관적으로 계산하는 방법이겠습니다. Listing 13-9의 클로저는
`sort_by_key`에서 동작하는데, 이는 `num_sort_operation` 카운터에
대한 가변 참조자를 캡처할 뿐이므로 한번 이상 호출이 가능하기
때문입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

<span class="caption">Listing 13-9: `FnMut` 클로저를 `sort_by_key`에 사용하는
것은 허용됩니다</span>

`Fn` 트레잇은 클로저를 사용하는 함수 혹은 타입을 정의하고 사용할 때
중요합니다. 다음 절에서는 반복자를 다루려고 합니다. 많은 반복자들이
클로저 인자를 받으니, 계속하려면 이 클로저에 대한 세부적인 내용을
새겨둡시다!

[unwrap-or-else]: ../std/option/enum.Option.html#method.unwrap_or_else
