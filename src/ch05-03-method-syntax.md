## 메소드 문법

*메소드(method)* 는 함수와 유사합니다.
`fn` 키워드와 함수명으로 선언하고, 매개변수와 반환 값을 가지며,
다른 어딘가로부터 호출될 때 실행됩니다.
하지만 메소드는 함수와 달리 구조체 문맥상에 정의되고 (열거형이나
트레잇 객체 안에 정의되기도 하며, 이는 각각 [6장][enums]<!-- ignore -->,
[17장][trait-objects]<!-- ignore -->에서 알아보겠습니다),
첫 번째 매개변수가 항상 `self` 라는 차이점이 있습니다.
`self` 매개변수는 메소드가 호출된 구조체 인스턴스를 나타냅니다.

### 메소드 정의

기존의 `Rectangle` 매개변수를 갖던 `area` 함수를 수정하여
Listing 5-13 처럼 `Rectangle` 구조체에 정의된
`area` 메소드로 바꿔봅시다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

<span class="caption">Listing 5-13: `Rectangle` 구조체에
`area` 메소드 정의</span>

`Rectangle` 내에 함수를 정의하기 위해서, `Rectangle`에 대한 `impl`
(구현, implementation) 블록을 만드는 것으로 시작합니다. 이 `impl` 블록 내의
모든 것은 `Rectangle` 타입에 관련되게 됩니다. 그런 다음 `area` 함수를 `impl`의
중괄호 안으로 옮기고 함수 시그니처의 첫번째 매개변수를 (이번 경우는 유일한
매개변수군요) `self`로 변경하고 본문 내의 나머지 모든 부분도 변경합니다. 
그리고 `main` 함수 내에선 `rect1` 을 인수로 넘겨 `area` 함수를 호출했었는데,
그 대신 *메소드 문법(method syntax)* 을 사용해 `Rectangle` 인스턴스의 `area`
메소드를 호출할 수 있습니다. 메소드 문법은 차례대로 인스턴스, 점,
메소드명, 괄호 및 인수로 구성됩니다.

`area` 시그니처에서는 `rectangle: &Rectangle` 대신 `&self`를 사용했습니다.
`&self`는 실제로는 `self: &Self`를 줄인 것입니다. `impl` 블록 내에서
`Self`는 `impl` 블록의 대상이 되는 타입의 별명입니다. 메소드는
`Self` 타입의 `self`라는 이름의 매개변수를 첫번째 파라미터로 가져야
하는데, 그렇게 해야 첫번째 매개변수 자리에 적어넣은 `self` 형태의 축약형을
사용할 수 있습니다. `rectangle: &Rectangle`에서 그랬던 것처럼 `Self` 이 메소드가
`Self`의 인슨스를 빌려온다는 것을 나타내기 위해서는 `self` 축약형 앞에 계속
`&`를 붙일 필요가 있음을 주목하세요. 메소드는 다른 파라미터가 그런 것처럼
`self`의 소유권을 가져올 수도, 지금처럼 `self`를 불변으로 빌려올 수도,
가변으로 빌려올 수도 있습니다.

여기서 `&self`를 선택한 이유는 기존의 함수 버전에서 `&Rectangle`을
사용했던 이유와 같습니다: 우리가 원하는 건 소유권을 가져오는 것도,
데이터를 쓰는 것도 아닌, 데이터를 읽는 것뿐이니까요. 만약 메소드에서
작업 중 호출된 인스턴스를 변경하고 싶다면, 첫번째 매개변수로
`&mut self`를 사용하면 됩니다. `self`라고만 작성하여 인스턴스의 소유권을
가져오도록 만드는 일은 거의 없습니다; 이러한 테크닉은 보통 해당 메소드가
`self`를 다른 무언가로 변형시고 그 이후에는 원본 인스턴스 사용을 막고자
할 때 사용됩니다.

함수 대신 메소드를 이용했을 때의 주된 이유는 메소드 시그니처 내에서
`self` 의 타입을 반복해서 입력하지 않아도 된다는 것 정도만이 아닙니다.
메소드는 코드를 더 조직적으로 만드는데 도움을 줍니다. 향후 우리가 제공한
라이브러리를 사용할 사람들이 `Rectangle`의 기능과 관련된 코드를 라이브러리
곳곳에서 찾아내야 하는 것보다는, 하나의 `impl` 블록 내에 이 타입의 인스턴스로
할 수 있는 모든 것들을 모아두는 것이죠.

구조체의 필드 이름과 동일한 이름의 메소드를 만들 수도 있습니다.
예를 들면, `width`라는 중복된 이름의 메소드를 `Rectangle`상에 정의할 수
있지요:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

여기서는 인스턴스의 `width` 필드가 `0`보다 크면 `true`를 반환하고
`0`이면 `false`를 리턴하는 메소드의 이름으로 `width`를 선택했습니다:
같은 이름의 메소드 내에서 필드를 어떤 목적으로든 사용할 수 있습니다.
`main`에서 `rect1.width` 뒤에 괄호를 붙이면 러스트는 `width` 메소드를
의도한다는 것을 인지합니다. 괄호를 사용하지 않으면 러스트는 `width` 필드를
의미한다는 것으로 봅니다.

필드와 동일한 이름의 메소드를 만드는 경우는 해당 필드의 값을 얻어오는 것
말고는 안하는 경우가 거의 대부분이긴 합니다. 이러한 메소드를
*게터 (getter)* 라고 부르는데, 러스트는 다른 언어들처럼 구조체 필드에 대한
게터를 자동으로 만들지 않습니다. 필드를 비공개(private)로 하고 메소드는
공개(public)로 만들 수 있기 때문에 게터는 어떤 타입의 공개 API로서 어떤
필드의 읽기 전용 접근만을 허용하고자 하는 경우 유용합니다. 공개와 비공개가
무엇이고, 필드 혹은 메소드를 공개 혹은 비공개로 만드는 방법에 대해서는
[7장][public]<!-- ignore -->에서 다루겠습니다.

> ### `->` 연산자는 없나요?
>
> C 나 C++ 언어에서는 메소드 호출에 두 종류의 연산자가 쓰입니다.
> 어떤 객체의 메소드를 직접 호출할 땐 `.` 를 사용하고,
> 어떤 객체의 포인터를 이용해 메소드를 호출하는 중이라서 역참조가 필요할 땐 `->` 를 사용하죠.
> 예를 들어서 `object` 라는 포인터가 있다면,
> `object->something()` 는 `(*object).something()` 로 나타낼 수 있습니다.
>
> 이 `->` 연산자와 동일한 기능을 하는 연산자는 러스트에 없습니다.
> 러스트에는 *자동 참조 및 역참조(automatic referencing and dereferencing)* 라는 기능이 있고,
> 메소드 호출에 이 기능이 포함돼있기 때문입니다.
>
> 여러분이 `object.something()` 코드로 메소드를 호출하면,
> 러스트에서 자동으로 해당 메소드의 시그니처에 맞도록 `&`, `&mut`, `*` 를 추가합니다.
> 즉, 다음 두 표현은 서로 같은 표현입니다:
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> 첫 번째 표현이 더 깔끔하죠?
> 이런 자동 참조 동작은 메소드의 수신자(`self` 의 타입을 말합니다)가 명확하기 때문에 가능합니다.
> 수신자와 메소드명을 알면 해당 메소드가 인스턴스를 읽기만 하는지(`&self`),
> 변경하는지(`&mut self`), 소비하는지(`self`) 러스트가 알아낼 수 있거든요.
> 또한 메소드의 수신자를 러스트에서 암묵적으로 빌린다는 점은,
> 실사용 환경에서 소유권을 인간공학적으로 만드는 중요한 부분입니다.

### 더 많은 파라미터를 가진 메소드

`Rectangle` 구조체의 두 번째 메소드를 구현하여 메소드 사용법을 연습해 봅시다.
이번에 만들 새로운 메소드는 다른 `Rectangle` 인스턴스를 받아서,
`self` 사각형 (첫번째 `Rectangle`) 면적 내에 두 번째 사각형 `Rectangle`
인스턴스가 완전히 들어갈 수 있다면 `true`를 반환하고, 못 들어가면 `false`를
반환할 겁니다. 즉, `can_hold` 메소드를 정의하여 다음 Listing 5-14에 나오는
프로그램이 작동하도록 만들겠습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

<span class="caption">Listing 5-14: `can_hold` 메소드를 작성하고 나면
작동할 코드</span>

`rect2`는 너비와 높이 둘 다 `rect1`보다 작지만,
`rect3` 는 `rect1` 보다 너비가 넓으므로
출력은 다음과 같을 겁니다:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

메소드의 정의는 `impl Rectangle` 블록 내에 위치할 것이고,
메소드명은 `can_hold`, 매개변수는 `Rectangle` 을 불변 참조자로 받겠죠.
이때 매개변수 타입은 메소드를 호출하는 코드를 보면 알아낼 수 있습니다.
`rect1.can_hold(&rect2)` 에서 `Rectangle` 인스턴스
`rect2` 의 불변 참조자인 `&rect2` 를 전달했으니까요.
`rect2` 를 읽을 수만 있으면 되기 때문에
변경 가능하게 빌려올 필요도 없으며,
`rect2` 의 소유권을 `main` 에 남겨두지 않을 이유도 없으니,
논리적으로도 불변 참조자가 가장 적합합니다.
반환값은 Boolean 타입이 될 거고, `self` 의 너비, 높이가
다른 `Rectangle` 의 너비, 높이보다 큰지 검사하는 식으로 구현될 겁니다.
그럼 이제 Listing 5-13의 `impl` 블록에 `can_hold` 메소드를 새로 추가해보죠!
추가하고 난 모습은 다음 Listing 5-15와 같습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

<span class="caption">Listing 5-15: 다른 `Rectangle` 인스턴스를
매개변수로 갖는 `can_hold` 메소드를 `Rectangle` 에 구현</span>

이제 Listing 5-14 에서 본
`main` 함수를 실행하면 원했던 결과가 나올 겁니다.
이처럼 메소드는 `self` 매개변수 뒤에 여러 매개변수를 가질 수 있으며,
이 매개변수는 함수에서의 매개변수와 동일하게 기능합니다.

### 연관 함수

`impl` 블록 내에 구현된 모든 함수를 *연관 함수 (associated function)*이라고
부르는데, 이는 `impl` 뒤에 나오는 타입과 모두 연관된 함수이기 때문입니다.
동작하는데 해당 타입의 인스턴스가 필요하지 않다면 `self`를 첫 매개변수로
갖지 않는 (따라서 메소드가 아닌) 연관 함수를 정의할 수도 있습니다.
우리는 이미 `String` 타입에 정의되어 있는 `String::from` 함수처럼 이런
종류의 함수를 사용해 봤습니다.

메소드가 아닌 연관 함수는 구조체의 새 인스턴스를 반환하는 생성자로
자주 활용됩니다. 이 함수들은 보통 `new`라고 명명되는데, `new`는
이 언어에서 특별한 이름 혹은 키워드가 아닙니다. 생성자의 예시로서
`Rectangle`로 정사각형을 만들 때 너비, 높이에 같은 값을 두 번 명시하는 대신,
치수 하나를 매개변수로 받고 해당 치수로 너비 높이를 설정하는 연관함수를 만들어,
더 간단하게 정사각형을 만드는 연관 함수 `square`를
제공해보겠습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

반환 타입 및 함수 본문의 `Self` 키워드는 `impl` 키워드
뒤에 적혀있는 타입의 별명으로서, 여기서는 `Rectangle`이
되겠습니다.

연관 함수를 호출할 땐
`let sq=Rectangle::square(3);` 처럼 구조체 명에 `::` 구문을 붙여서 호출합니다.
연관 함수는 구조체의 네임스페이스 내에 위치하기 때문이죠.
`::` 구문은 [7장][modules]<!-- ignore -->에서 알아볼 모듈에 의해 생성되는
네임스페이스에도 사용됩니다.

### `impl` 블록은 여러 개일 수 있습니다

각 구조체는 여러 개의 `impl` 블록을 가질 수 있습니다.
다음 Listing 5-16은 Listing 5-15에 나온 코드를 변경해
`impl` 블록을 여러 개로 만든 모습입니다:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

<span class="caption">Listing 5-16: Listing 5-15 를 여러 `impl`
블록을 사용하도록 재작성</span>

위 코드에서는 `impl` 블록을 여러 개로 나눠야 할 이유가 전혀 없지만,
`impl` 블록을 반드시 하나만 작성해야 할 필요는 없음을 보여드리기 위해 예시로 작성했습니다.
여러 `impl` 블록을 유용하게 사용하는 모습은 제네릭 타입 및 트레잇 내용을 다루는 10장에서 보실 수 있습니다.

## 요약

구조체를 사용하면 우리에게 필요한 의미를 갖는 타입을 직접 만들 수 있습니다.
또한, 구조체를 사용함으로써 서로 관련 있는 데이터들을 하나로 묶어 관리할 수
있으며, 각 데이터 조각에 이름을 붙여 코드를 더 명확하게 만들 수 있습니다.
`impl` 블록 내에서는 여러분의 타입과 연관된 함수들, 그리고 연관 함수의
일종인 메소드를 정의하여 여러분의 구조체 인스턴스가 가질 동작들을 명시할 수
있습니다.

하지만 구조체로만 커스텀 타입을 만들 수 있는 건 아닙니다.
다음엔 열거형을 배워서 여러분이 쓸 수 있는 도구를 하나 더 늘려보도록 합시다.

[enums]: ch06-00-enums.html
[trait-objects]: ch17-02-trait-objects.md
[public]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
