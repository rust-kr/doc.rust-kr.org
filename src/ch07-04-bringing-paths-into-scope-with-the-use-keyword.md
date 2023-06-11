## `use` 키워드로 경로를 스코프 안으로 가져오기

함수 호출을 위해서 경로를 작성하는 것은 불편하고 반복적인 느낌을 줄 수
있습니다. 예제 7-7에서는 절대 경로를 사용하건 상대 경로를 사용하건,
`add_to_waitlist` 호출할 때마다 `front_of_house`, `hosting` 모듈을 매번
지정해줘야 했죠. 다행히도 이 과정을 단축할 방법이 있습니다: `use` 키워드를
한번 사용하여 어떤 경로의 단축경로 (shortcut) 를 만들 수 있고, 그러면 스코프
안쪽 어디서라도 짧은 이름을 사용할 수 있습니다. 

예제 7-11은 `crate::front_of_house::hosting` 모듈을
`eat_at_restaurant` 함수가 존재하는 스코프로 가져와,
`eat_at_restaurant` 함수 내에서 `add_to_waitlist` 함수를
`hosting::add_to_waitlist` 경로만으로 호출하는 예제입니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

<span class="caption">예제 7-11: `use` 키워드로 모듈을 스코프 안으로
가져오기</span>

스코프에 `use` 키워드와 경로를 작성하는 건
파일 시스템에서 심볼릭 링크 (symbolic link) 를 생성하는 것과 유사합니다.
크레이트 루트에 `use crate::front_of_house::hosting`를 작성하면
해당 스코프에서 `hosting` 모듈을 크레이트 루트에 정의한 것처럼 사용할 수 있습니다.
`use` 키워드로 가져온 경우도 다른 경로와 마찬가지로 비공개 규칙이 적용됩니다.

`use`가 사용된 특정한 스코프에서만 단축경로가 만들어진다는 점을
주의하세요. 예제 7-12에서는 `eat_at_restaurant` 함수를 새로운
자식 모듈 `customer`로 옮겼는데, 이러면 `use` 구문과 다른 스코프가
되므로, 이 함수는 컴파일 되지 않습니다:

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

<span class="caption">예제 7-12: `use` 구문은 사용된 스코프 내에서만
적용됩니다</span>

컴파일러는 `customer` 모듈 내에 더 이상 단축경로가 적용되지 않음을
알려줍니다:

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

`use`가 해당 스코프 안에서 더 이상 사용되지 않는다는 경고도 있음을 주목하세요!
이 문제를 해결하려면 `use`도 `customer` 모듈 안쪽으로 옮기거나, `customer`
모듈 내에서 `super::hosting`를 써서 부모 모듈로의 단축경로를 참조하면
됩니다.

### 보편적인 `use` 경로 작성법

예제 7-11에서 `add_to_waitlist` 함수까지 경로를 전부 작성하지 않고,
`use crate::front_of_house::hosting` 까지만 작성한 뒤
`hosting::add_to_waitlist` 코드로 함수를 호출하는 점이 의아하실 수도 있습니다.
예제 7-13처럼 작성하면 안 되는 걸까요?

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

<span class="caption">예제 7-13: `use` 키워드로 `add_to_waitlist` 함수를 직접 가져오기
(보편적이지 않은 작성 방식)</span>

예제 7-11과 7-13의 동작은 동일하지만, 예제 7-11 코드가
`use` 키워드로 스코프에 함수를 가져올 때의 관용적인 코드입니다.
함수의 부모 모듈을 `use` 키워드로 가져오면 함수를 호출할 때 부모
모듈을 특정해야 합니다. 함수 호출 시 부모 모듈을 특정하면
전체 경로를 반복하는 것을 최소화하면서도 함수가 로컬에 정의되어 있지
않음을 명백히 보여주게 됩니다. 예제 7-13의 코드는 `add_to_waitlist`가
어디에 정의되어 있는지 불분명합니다.

한편, `use` 키워드로 구조체나 열거형 등의 타 아이템을
가져올 시에는 전체 경로를 작성하는 것이 보편적입니다.
예제 7-14는 `HashMap` 표준 라이브러리 구조체를
바이너리 크레이트의 스코프로 가져오는 관용적인 코드 예시입니다.

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

<span class="caption">예제 7-14: 보편적인 방식으로 `HashMap`을
스코프로 가져오기</span>

이러한 관용이 탄생하게 된 명확한 이유는 없습니다.
어쩌다 보니 관습이 생겼고, 사람들이 이 방식대로 러스트 코드를 읽고 쓰는 데에 익숙해졌을 뿐입니다.

하지만, 동일한 이름의 아이템을 여럿 가져오는 경우는 이 방식을 사용하지 않습니다.
러스트가 허용하지 않기 때문이죠.
예제 7-15는 각각 다른 모듈 내에 위치하지만 이름이 같은 두 개의
`Result` 타입을 스코프로 가져와 사용하는 예시입니다.

<span class="filename">파일명: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

<span class="caption">예제 7-15: 이름이 같은 두 개의 타입을 동일한 스코프에 가져오려면
부모 모듈을 반드시 명시해야 합니다.</span>

보시다시피 부모 모듈을 명시하여 두 개의 `Result` 타입을 구별하고 있습니다.
만약 `use std::fmt::Result`, `use std::io::Result`로 작성한다면,
동일한 스코프 내에 두 개의 `Result` 타입이 존재하므로
러스트는 우리가 어떤 `Result` 타입을 사용했는지 알 수 없습니다.

### `as` 키워드로 새로운 이름 제공하기

`use` 키워드로 동일한 이름의 타입을 스코프로 여러 개 가져올 경우의 또 다른 해결 방법이 있습니다.
경로 뒤에 `as` 키워드를 작성하고, 새로운 이름이나 타입 별칭을 작성하면 됩니다.
예제 7-16은 `as` 키워드를 이용해 예제 7-15 코드의
`Result` 타입 이름 중 하나를 변경한 예제입니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

<span class="caption">예제 7-16: 스코프 안으로 가져온 타입의 이름을
`as` 키워드로 변경하기</span>

두 번째 `use` 구문에서는, 앞서 스코프 안으로 가져온
`std::fmt`의 `Result`와 충돌을 방지하기 위해
`std::io::Result` 타입의 이름을 `IoResult`로 새롭게 지정합니다.
예제 7-15, 예제 7-16은 둘 다 관용적인 방식이므로, 원하는 방식을 선택하시면 됩니다!

### `pub use`로 다시 내보내기

`use` 키워드로 이름을 가져올 경우,
해당 이름은 새 위치의 스코프에서 비공개가 됩니다.
`pub`과 `use`를 결합하면 우리 코드를 호출하는 코드가
해당 스코프에 정의된 것처럼 해당 이름을 참조할 수 있습니다.
이 기법은 아이템을 스코프로 가져오는 동시에 다른 곳에서 아이템을 가져갈 수 있도록 만들기 때문에,
*다시 내보내기 (re-exporting)* 라고 합니다.

예제 7-17은 예제 7-11 코드의 `use`를 `pub use`로
변경한 예제입니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

<span class="caption">예제 7-17: 다른 스코프의 코드에서 사용할 수 있도록
`pub use` 사용</span>

위와 같이 변경하기 전이라면 외부 코드에서는 `add_to_waitlist` 함수를 호출하기
위해 `restaurant::front_of_house::hosting::add_to_waitlist()`라는
경로를 사용해야 할 것입니다. 위의 `pub use`가 루트 모듈로부터 `hosting`
모듈을 다시 내보냈으므로, 이제 외부 코드는 `restaurant::hosting::add_to_waitlist()`
경로를 대신 사용할 수 있습니다.

다시 내보내기 기법은 작성한 코드의 구조 내부와,
그 코드를 사용할 프로그래머들이 예상할법한 해당 분야의 구조가 서로 다를 때 유용합니다.
레스토랑 비유 예제를 예로 들어보죠. 레스토랑을 운영하는
직원들의 머릿속에서는 ‘접객 부서’와 ‘지원 부서’가 나뉘어 있습니다.
하지만 레스토랑을 방문하는 고객들은 레스토랑의 부서를 그런 용어로 나누어 생각하지 않겠죠.
`pub use`를 사용하면 코드를 작성할 때의 구조와, 노출할 때의 구조를 다르게 만들 수 있습니다.
라이브러리를 제작하는 프로그래머와, 라이브러리를 사용하는 프로그래머
모두를 위한 라이브러리를 구성하는 데 큰 도움이 되죠. `pub use`에 대한
또 다른 예제, 그리고 이것이 여러분의 크레이트 문서에 어떤 영향을 주는지에 대해서는
14장의 [‘`pub use`를 사용하여 편리한 공개 API 내보내기’][ch14-pub-use]<!-- ignore -->절에서
살펴보겠습니다.

### 외부 패키지 사용하기

2장에서는 난수 생성을 위해 `rand`라는 외부 패키지를
사용하는 추리 게임 프로젝트를 만들었습니다. `rand` 패키지를
프로젝트에서 사용하기 위해서 *Cargo.toml*에 다음 줄을 추가했었죠:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">파일명: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

*Cargo.toml*에 `rand`를 의존성으로 추가하면 카고가
[crates.io](https://crates.io/)에서 `rand` 패키지를 비롯한 모든 의존성을
다운로드하고 프로젝트에서 `rand` 패키지를 사용할 수 있게 됩니다.

그 후, 구현하고 있는 패키지의 스코프로 `rand` 정의를 가져오기 위해
`use` 키워드와 크레이트 이름인
`rand`를 쓰고 가져올 아이템을 나열했습니다.
2장 [‘임의의 숫자 생성하기’][rand]<!-- ignore --> 절을 다시 떠올려 보죠.
`Rng` 트레잇을 스코프로 가져오고 `rand::thread_rng` 함수를 호출했었습니다.

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

러스트 커뮤니티 구성원들이 [crates.io](https://crates.io/)에서 이용 가능한
다양한 패키지를 만들어왔고, 이들 모두 같은 단계를 거쳐서 여러분 패키지에
가져올 수 있습니다: 패키지의 *Cargo.toml* 파일에 추가하고,
`use` 키워드를 사용해 스코프로 가져오면 됩니다.

알아 두어야 할 것이 있다면
`std` 표준 라이브러리도 마찬가지로 외부 크레이트라는 겁니다.
러스트 언어에 포함되어 있기 때문에 *Cargo.toml* 에 추가할 필요는 없지만,
표준 라이브러리에서 우리가 만든 패키지의 스코프로 가져오려면 `use` 문을 작성해야 합니다.
예를 들어, `HashMap`을 가져오는 코드는 다음과 같습니다.

```rust
use std::collections::HashMap;
```

위는 표준 라이브러리 크레이트의 이름인 `std` 로 시작하는
절대 경로입니다.

### 중첩 경로를 사용하여 대량의 `use` 나열을 정리하기

동일한 크레이트나 동일한 모듈 내에 정의된 아이템을 여럿 사용할 경우,
각 아이템 당 한 줄씩 코드를 나열하면 수직 방향으로 너무 많은 영역을 차지합니다.
예시를 살펴봅시다. 추리 게임의 예제 2-4에서 작성했던
다음 두 `use` 문은 `std` 내 아이템을 스코프로 가져옵니다.

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

그대신 중첩 경로를 사용하여 동일한 아이템을 한 줄로 가져올 수 있습니다.
경로의 공통된 부분을 작성하고 콜론 두 개를 붙인 다음, 중괄호 내에
경로가 다른 부분을 나열합니다.
예시는 예제 7-18과 같습니다.

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

<span class="caption">예제 7-18: 중첩 경로를 사용해, 경로의 앞부분이 같은
여러 아이템을 스코프로 가져오기</span>

규모가 큰 프로그램이라면, 동일한 크레이트나 모듈에서 여러 아이템을 가져올 때
중첩 경로를 사용함으로써 많은 `use` 구문을
줄일 수 있습니다!

중첩 경로는 경로의 아무 단계에서 사용할 수 있으며,
하위 경로가 동일한 `use` 구문이 많을 때 특히 빛을 발합니다.
다음 예제 7-19는 두 `use` 구문의 예시입니다. 하나는 `std::io`를 스코프로 가져오고,
다른 하나는 `std::io::Write`를 스코프로 가져옵니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

<span class="caption">예제 7-19: 하위 경로가 같은
두 `use` 구문</span>

두 경로에서 중복되는 부분은 `std::io`입니다.
또한 `std::io`는 첫 번째 경로 그 자체이기도 합니다.
중첩 경로에 `self`를 작성하면 두 경로를 하나의 `use` 구문으로 합칠 수 있습니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

<span class="caption">예제 7-20: 예제 7-19의 두 경로를
`use` 구문 하나로 합치기</span>

이 한 줄로 `std::io`, `std::io::Write` 둘 다 스코프로 가져올 수 있습니다.

### 글롭 연산자

경로에 글롭 (glob) 연산자 `*`를 붙이면 경로 안에 정의된
*모든* 공개 아이템을 가져올 수 있습니다.

```rust
use std::collections::*;
```

이 `use` 구문은 `std::collections` 내에 정의된
모든 공개 아이템을 현재 스코프로 가져옵니다.
하지만 글롭 연산자는 코드에 사용된 어떤 이름이 어느 곳에 정의되어 있는지
파악하기 어렵게 만들 수 있으므로, 사용에 주의해야 합니다.

글롭 연산자는 테스트할 모든 아이템을 `tests` 모듈로
가져오는 용도로 자주 사용됩니다.
(11장 [‘테스트 작성 방법’][writing-tests]<!-- ignore -->에서 다룰 예정입니다.)
또한 프렐루드 패턴의 일부로 사용되기도 하며, 자세한 내용은
[표준 라이브러리 문서](https://doc.rust-lang.org/std/prelude/index.html#other-preludes)<!-- ignore -->를
참고 바랍니다.

[ch14-pub-use]: ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
[writing-tests]: ch11-01-writing-tests.html#how-to-write-tests
