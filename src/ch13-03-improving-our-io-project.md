## I/O 프로젝트 개선하기

반복자에 대한 새로운 지식을 가지고 12장의 I/O 프로젝트에 반복자를
사용하여 코드들을 더 명확하고 간결하게 개선할 수 있습니다. 반복자가
어떻게 `Config::build` 함수와 `search` 함수의 구현을 개선할 수
있는지 살펴봅시다.

### 반복자를 사용하여 `clone` 제거하기

예제 12-6에서는 `String` 값의 슬라이스를 받아서 슬라이스에 인덱스로
접근하고 복사하는 방식으로 `Config` 구조체의 인스턴스를 생성하는 코드를
넣었고, `Config` 구조체가 이 값들을 소유하도록 했습니다. 예제 13-17은
예제 12-23에 있던 `Config::build` 함수의 구현체를 재현한
것입니다:

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/lib.rs:ch13}}
```

<span class="caption">예제 13-17: 예제 12-23의 `Config::build`
함수 재현</span>

그때는 비효율적인 `clone` 호출에 대해서, 나중에 제거할 테니 걱정하지 말라고
이야기했었지요. 자, 그때가 되었습니다!

`String` 요소들의 슬라이스를 `args` 매개변수로 받았지만, `build` 함수는
`args`를 소유하지 않기 때문에 `clone`이 필요했습니다. `Config` 인스턴스의
소유권을 반환하기 위해서는 `Config`의 `query`와 `file_path` 필드로 값을
복제하는 것으로 `Config` 인스턴스가 그 값들을 소유하게 할 필요가 있었습니다.

반복자에 대한 새로운 지식을 사용하면, 인수로써 슬라이스를 빌리는 대신
반복자의 소유권을 갖도록 `build` 함수를 변경할 수 있습니다. 슬라이스의
길이를 체크하고 특정 위치로 인덱싱하는 코드 대신 반복자의 기능을 사용할
것입니다. 이렇게 하면 반복자가 값에 접근하기 때문에 `Config::build` 함수가
수행하는 작업이 명확해집니다.

`Config::build`가 반복자의 소유권을 가져오고 빌린 값에 대한 인덱싱 연산을
사용하지 않게 되면, `clone`을 호출하여 새로 할당하는 대신 반복자의 `String`
값을 `Config`로 이동시킬 수 있습니다.

#### 반환된 반복자를 직접 사용하기

여러분의 I/O 프로젝트에 있는 *src/main.rs* 파일을 열어보면, 아래와 같이 생겼을 것입니다:

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

먼저 예제 12-24에 있던 `main` 함수의 시작점을 수정하여 예제 13-18의
코드로 바꾸려고 하는데, 이번에는 반복자를 사용합니다.
`Config::build`도 마찬가지로 업데이트하기 전에는 컴파일 되지 않습니다.

<span class="filename">파일명: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

<span class="caption">예제 13-18: `env::args`의 반환 값을 `Config::build`로
넘기기</span>

`env::args` 함수는 반복자를 반환합니다! 반복자의 값들을 벡터로 모아서
`Config::build`에 슬라이스를 넘기는 대신, 이번에는 `env::args`로부터
반환된 반복자의 소유권을 `Config::build`로 직접 전달하고
있습니다.

다음으로는 `Config::build`의 정의를 업데이트할 필요가 있습니다. 여러분의
I/O 프로젝트에 있는 *src/lib.rs* 파일에서, 예제 13-19와 같이
`Config::build`의 시그니처를 변경합시다. 함수 본문을 업데이트해야 하기
때문이 여전히 컴파일 되지 않습니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/lib.rs:here}}
```

<span class="caption">예제 13-19: 반복자를 받도록 `Config::build`의 시그니처
업데이트하기</span>

`env::args` 함수에 대한 표준 라이브러리 문서에는 반환되는 반복자의
타입이 `std::env::Args`이며, 이 타입은 `Iterator` 트레이트를 구현하고
`String` 값을 반환함을 명시하고 있습니다.

`Config::build` 함수의 시그니처를 업데이트해서 `args` 매개변수가
`&[String]` 대신 트레이트 바운드 `impl Iterator<Item = String>`를 갖는
제네릭 타입이 되도록 하였습니다. 10장의 [‘매개변수로서의 트레이트’][impl-trait]<!-- ignore -->절에서
논의했었던 이러한 `impl Trait` 문법을 사용하면 `args`가 `Iterator`
타입을 구현하면서 `String` 아이템을 반환하는 모든 종류의 타입을
사용할 수 있습니다.

`args`의 소유권을 가져와서 이를 순회하면서 `args`를 변경할 것이기
때문에, `args` 매개변수의 명세 부분에 `mut` 키워드를 추가하여 가변이
되도록 합니다.

#### 인덱싱 대신 `Iterator` 트레이트 메서드 사용하기

다음으로 `Config::build`의 본문을 수정하겠습니다. `args`가 `Iterator` 트레이트를
구현하고 있으므로, 여기에 `next` 메서드를 호출할 수 있다는 것을 알고 있지요!
예제 13-20은 예제 12-23의 코드를 `next` 메서드를 사용하여 업데이트한 것입니다:

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/lib.rs:here}}
```

<span class="caption">예제 13-20: 반복자 메서드를 사용하여
`Config::build`의 본문 변경하기</span>

`env::args` 반환 값의 첫 번째 값이 프로그램의 이름이라는 점을 기억해 둡시다.
이 첫 번째 값은 무시하고 그다음 값을 얻고자 하므로, 우선 `next`를 호출한
뒤 그 반환 값으로 아무것도 하지 않았습니다. 두 번째로, `next`를 호출하여
`Config`의 `query` 필드에 원하는 값을 집어넣었습니다. `next`가 `Some`을
반환하면, `match`를 사용하여 값을 추출합니다. 만약 `None`을 반환한다면,
이는 충분한 인수가 넘어오지 않았음을 의미하고, `Err` 값과 함께 일찍 반환합니다.
`file_path` 값도 동일하게 처리합니다.

### 반복자 어댑터로 더 간결한 코드 만들기

I/O 프로젝트의 `search` 함수에도 반복자의 장점을 활용할 수 있는데,
예제 12-19의 코드가 예제 13-21에 재현되어 있습니다:

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

<span class="caption">예제 13-21: 예제 12-19의 `search`
함수 구현</span>

반복자 어댑터 메서드를 사용하면 이 코드를 더 간결한 방식으로 작성할 수
있습니다. 이렇게 하면 중간에 가변 `results` 벡터를 만들지 않아도 됩니다.
함수형 프로그래밍 스타일은 더 명확한 코드를 만들기 위해 변경 가능한 상태의 양을
최소화하는 편을 선호합니다. 가변 상태를 제거하면 `results` 벡터에 대한
동시 접근을 관리하지 않아도 되기 때문에, 차후에 검색을 병렬로 수행하도록 하는
향상이 가능해집니다. 예제 13-22는 이러한 변경을 보여줍니다:

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

<span class="caption">예제 13-22: `search` 함수 구현에서 반복자 어댑터
메서드 사용하기</span>

`search` 함수의 목적은 `query`를 포함하는 `contents`의 모든 라인을 반환하는
것임을 상기합시다. 예제 13-16의 `filter` 예제와 유사하게, 이 코드는
`line.contains(query)`이 `true`를 반환하는 라인들만 유지하기 위해서
`filter` 어댑터를 사용합니다. 그런 다음 `collect`를 사용하여 매칭된 라인들을
모아 새로운 벡터로 만듭니다. 훨씬 단순하군요! 마찬가지로
`search_case_insensitive`도 반복자 메서드들을 사용하도록 동일한 변경을
해보셔도 좋습니다.

### 루프와 반복자 중 선택하기

그렇다면 여러분의 코드에서 어떤 스타일을 선택하는 것이 좋은지와 그 이유에 대한
질문이 논리적으로 뒤따르겠지요: 예제 13-21에 있는 원래 구현과 예제 13-29에
있는 반복자를 사용하는 버전 중 어떤 것이 좋을까요? 대부분의 러스트 프로그래머는
반복자 스타일을 선호합니다. 처음 사용하기는 다소 어렵습니다만, 다양한 반복자
어댑터와 어떤 일을 하는지에 대해 일단 감을 잡으면 반복자들을 이해하기
쉬워질 것입니다. 루프를 만들고 새 벡터를 만드는 등 다양한 것들을
만지작거리는 대신, 이 코드는 루프의 고수준의 목표에 집중합니다.
이는 몇몇 아주 흔한 코드를 추상화해서 제거하므로, 반복자의 각 요소가
반드시 통과해야 하는 필터링 조건과 같이 이 코드에 유일한 개념을 더 알기
쉽게끔 합니다.

그런데 이 두 가지 구현은 정말 동일할까요? 직관적으로는 더
저수준의 루프가 더 빨라 보입니다. 그러면 성능에 대해서 얘기해
봅시다.

[impl-trait]: ch10-02-traits.html#traits-as-parameters
