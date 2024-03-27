## 테스트 작성 방법

테스트란, 테스트할 코드가 의도대로 기능하는지 검증하는 함수입니다.
테스트 함수는 보통 본문에서 세 가지 동작을
수행합니다.

1. 필요한 데이터나 상태 설정
2. 테스트할 코드 실행
3. 의도한 결과가 나오는지 확인

`test` 속성 (attribute), 몇 가지 매크로, `should_panic` 속성을 포함하여
위 세 가지 동작을 수행하는 테스트를 위해 러스트가 특별히 제공하는
기능을 살펴봅시다.

### 테스트 함수 파헤치기

간단히 말해서, 러스트에서 테스트란 `test` 속성이 어노테이션된 함수입니다.
속성은 러스트 코드 조각에 대한 메타데이터입니다.
앞서 5장에서 구조체에 사용했던 `derive`도 속성 중 하나입니다.
함수의 `fn` 이전 줄에 `#[test]`를 추가하면 테스트 함수로 변경됩니다.
테스트는 `cargo test` 명령어로 실행되며,
이 명령을 실행하면 러스트는 속성이 표시된 함수를 실행하고
결과를 보고하는 테스트 실행 바이너리를 빌드합니다.

카고로 새 라이브러리 프로젝트를 생성할 때마다
테스트 함수가 포함된 테스트 모듈이 자동 생성됩니다.
이 모듈이 테스트 작성을 위한 템플릿을 제공하므로,
새 프로젝트를 시작할 때마다 정확한 구조 및 테스트 함수 문법을
찾아볼 필요는 없습니다. 테스트 모듈과 테스트 함수는 여러분이
원하는 만큼 추가할 수 있습니다!

어떤 코드를 실제로 테스트해 보기 전에, 먼저 이 템플릿 테스트를 가지고
실험해 보면서 테스트가 어떻게 작동하는지 알아보겠습니다. 그다음 실제로
우리가 작성한 코드가 제대로 작동하는지 확인하는 테스트를 직접 작성해 보겠습니다.

두 숫자를 더하는 `adder`라는 라이브러리 프로젝트를 생성해 봅시다:

```console
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

`adder` 라이브러리의 *src/lib.rs* 파일 내용은
다음과 같습니다.

<span class="filename">파일명: src/lib.rs</span>

<!-- manual-regeneration
cd listings/ch11-writing-automated-tests
rm -rf listing-11-01
cargo new listing-11-01 --lib --name adder
cd listing-11-01
cargo test
git co output.txt
cd ../../..
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

<span class="caption">예제 11-1: `cargo new` 명령어로 자동 생성된
테스트 모듈과 함수</span>

맨 위 두 줄은 무시하고 함수에 집중합시다. `#[test]` 어노테이션을 주목해 주세요:
이 속성은 해당 함수가 테스트 함수임을 표시하며, 테스트 실행기는 이 표시를
보고 해당 함수를 테스트로 다룰 수 있게 됩니다. `tests` 모듈 내에는 테스트
함수뿐만 아니라, 일반적인 시나리오를 설정하거나 자주 쓰이는 연산을 수행하는
일반 함수도 작성하기도 하므로, 어떤 함수가 테스트 함수인지 항상 표시해 줘야 합니다.

예제 함수 본문에서는 `assert_eq!` 매크로를 사용하여 `result`에 대한 단언 (assert) 을
했는데, 이 변수의 내용물이 2와 2를 더한 결과인 4와 같다는 것입니다. 이 단언 코드는
일반적인 테스트 형식 예제로써 제공됩니다. 한번 테스트를 실행해 이 테스트가
통과되는지 확인해 보죠.

`cargo test` 명령어는 프로젝트 내 모든 테스트를 실행합니다.
결과는 예제 11-2처럼 나타납니다.

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

<span class="caption">예제 11-2: 자동 생성된 테스트
실행 결과</span>

카고가 테스트를 컴파일하고 실행했습니다. `running 1 test` 줄이
보입니다. 그다음 줄에는 생성된 테스트 함수의 이름 `it_works`와 테스트
실행 결과 `ok`가 표시됩니다. 전체 요약 `test result: ok.`는 모든
테스트가 통과됐다는 뜻이고, `1 passed; 0 failed`라는 부분은
통과하거나 실패한 테스트 개수를 종합합니다.

어떤 테스트를 무시하도록 표시하여 특정 인스턴스에서는 실행되지 않도록 할 수도 있습니다;
이에 대해서는 이 장의 [‘특별 요청이 없다면 일부 테스트 무시하기’][ignoring]<!-- ignore -->절에서
다루겠습니다. 이번 예제에는 그런 게 없었으므로, 요약에는 `0 ignored`가
표시됩니다. 또한 `cargo test`에 인수를 넘겨서 어떤 문자열과 이름이
일치하는 테스트만 실행하도록 할 수도 있습니다; 이것을 *필터링 (filtering)* 이라고
하고, [‘이름을 지정해 일부 테스트만 실행하기’][subset]<!-- ignore -->절에서
다룰 예정입니다. 지금의 테스트에서는 필터링도 없었으므로, 요약의 끝부분에
`0 filtered out`이 표시됩니다.

`0 measured` 통계는 성능 측정 벤치마크 테스트용입니다. 이 내용이 작성된 시점을
기준으로, 벤치마크 테스트는 러스트 나이틀리 (nightly) 에서만 사용 가능합니다.
자세한 내용은 [벤치마크 테스트 문서][bench]를 참고해 주세요.

테스트 출력 결과 중 `Doc-tests adder`로 시작하는 부분은 문서 테스트 결과를
나타냅니다. 아직 문서 테스트를 작성해 보진 않았지만, 러스트는 API 문서에 작성해
놓은 예제 코드도 컴파일 할 수 있습니다. 러스트의 이 기능은 작성한 코드와 문서의
내용이 달라지지 않도록 유지보수하는 데에 매우 유용하답니다! 문서 테스트 작성 방법은
14장의 [‘테스트로서의 문서화 주석’][doc-comments]<!-- ignore -->절에서 배울 예정입니다.
지금은 일단 `Doc-tests` 출력을 무시하겠습니다.

현재의 요구사항에 맞게 테스트의 커스터마이징을 시작해 봅시다. 먼저 다음과 같이
`it_works` 함수의 이름을 `exploration` 같은 다른 이름으로 변경해 봅시다:

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

`cargo test`를 다시 실행하면 출력 결과에 `it_works` 대신
`exploration`이 나타납니다.

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

이제 다른 테스트를 추가하는데, 이번엔 테스트가 실패하도록 만들어 보죠!
테스트 함수 내에서 패닉이 발생하면 테스트는 실패합니다. 각각의 테스트는
새로운 스레드에서 실행되며, 메인 스레드에서 테스트 스레드가 죽은 것을
알게 되면 해당 테스트는 실패한 것으로 처리됩니다. 9장에서, 가장 쉽게 패닉을
일으키는 방법은 `panic` 매크로를 호출하는 것이라고 이야기했습니다. 예제 11-3처럼
*src/lib.rs* 파일에 `another`라는 테스트를 새로 추가해 봅시다.

<span class="filename">파일명: src/lib.rs</span>

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs:here}}
```

<span class="caption">예제 11-3: `panic!` 매크로를 호출하여 실패하도록 만든
테스트 추가</span>

`cargo test`를 다시 실행해 보죠. 출력 결과는 예제 11-4처럼
`exploration` 테스트는 통과하고 `another` 테스트는 실패했다고 나타날 겁니다.

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

<span class="caption">예제 11-4: 테스트 하나는 통과하고 다른 하나는 실패했을 때의
테스트 결과</span>

`test tests::another` 줄은 `ok`가 아니라 `FAILED`로 표시됩니다.
개별 결과와 요약 사이에 새로운 절이 두 개 나타났네요:
첫 번째 절은 테스트가 실패한 자세한 이유를 보여줍니다.
위의 경우 `another` 테스트는 `panicked at 'Make this test fail'`라는 이유로 실패했으며,
*src/lib.rs* 파일 10번째 줄에서 발생했다는 세부 사항을 알게 되었습니다.
다음 절은 실패한 테스트의 이름을 목록으로 보여줍니다. 이는 테스트가 많아지고
테스트 실패 사유 출력량도 많아졌을 때 유용합니다. 실패한 테스트의 이름을 이용해
해당 테스트만 실행하면 쉽게 디버깅할 수 있습니다. 테스트를 실행하는 각종 방식은
[‘테스트 실행 방법 제어하기’][controlling-how-tests-are-run]<!-- ignore -->절에서
다룰 예정입니다.

요약 줄은 마지막에 출력됩니다. 종합적인 테스트 결과는 `FAILED`군요.
테스트 하나는 통과했지만, 테스트 하나가 실패했습니다.

각 상황에서 테스트 실행 결과가 어떻게 나타나는지 살펴봤으니,
`panic!` 이외에 테스트에서 유용하게 쓰이는 매크로를 알아봅시다.

### `assert!` 매크로로 결과 검사하기

어떤 조건이 `true`임을 보장하는 테스트를 작성할 땐 표준 라이브러리가
제공하는 `assert!` 매크로가 유용합니다. `assert!` 매크로는 부울린 값으로
평가되는 인수를 전달받습니다. `true` 값일 경우, 아무 일도 일어나지 않고
테스트는 통과합니다. `false` 값일 경우, `assert!` 매크로는 `panic!` 매크로를
호출하여 테스트를 실패하도록 만듭니다. `assert!` 매크로를 사용하면
작성한 코드가 의도대로 기능하는지 검사하는 데에 유용합니다.

5장 예제 5-15에서 `Rectangle` 구조체랑 `can_hold` 메서드를 사용했었죠.
(예제 11-5로 다시 보여드립니다.) 이 코드를 *src/lib.rs* 파일에
작성하고, 그다음 `assert!` 매크로로 테스트를 작성해 봅시다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs:here}}
```

<span class="caption">예제 11-5: 5장 `Rectangle` 구조체와
`can_hold` 메서드</span>

`can_hold` 메서드는 부울린 값을 반환하니
`assert` 매크로 사용 예시로 쓰기에 딱 알맞습니다.
예제 11-6은 `can_hold` 메서드를 시험하는 테스트를 작성한 모습입니다.
너비 8, 높이 7 `Rectangle` 인스턴스를 생성하고, 이 인스턴스는
너비 5, 높이 1 `Rectangle` 인스턴스를 포함할 수 있음을 단언합니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

<span class="caption">예제 11-6: 큰 사각형이 작은 사각형을 정말로
포함할 수 있는지 검사하는 `can_hold` 메서드 테스트</span>

`tests` 모듈에 `use super::*;` 줄이 추가되었습니다.
`tests` 모듈 또한 7장
[‘경로를 사용하여 모듈 트리의 아이템 참조하기’][paths-for-referring-to-an-item-in-the-module-tree]<!-- ignore -->절에서 다룬
가시성 규칙을 따르는 평범한 모듈입니다.
따라서, 내부 모듈인 `tests` 모듈에서 외부 모듈의 코드를
테스트하려면 먼저 내부 스코프로 가져와야 합니다.
`tests` 모듈에서는 글롭 (`*`) 을 사용해 외부 모듈에
정의된 걸 전부 사용할 수 있도록 하였습니다.

테스트 이름은 `larger_can_hold_smaller`로 정하고,
필요한 `Rectangle` 인스턴스를 두 개 생성하고,
`larger.can_hold(&smaller)` 호출 결과를 전달하여 `assert!` 매크로를 호출하였습니다.
`larger.can_hold(&smaller)` 표현식은 `true`를 반환할 테니 테스트는 성공하겠죠. 확인해 봅시다!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

통과됐네요! 이번에는 작은 사각형이 큰 사각형을 포함할 수 없음을
단언하는 테스트를 추가해 봅시다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

이번에는 `can_hold` 함수가 `false`를 반환해야 하니,
`assert!` 매크로에 전달하기 전에 논리 부정 연산자를 사용했습니다.
결과적으로, 이 테스트는 `can_hold` 함수에서 `false` 값을 반환하면 성공합니다.

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

두 테스트를 모두 통과했습니다! 그러면 이제 코드에 버그가 있으면
테스트 결과가 어떻게 되는지 알아보죠.
`can_hold` 메서드 구현부 중 너비 비교 부분의
큰 부등호를 작은 부등호로 바꿔보겠습니다:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

테스트 실행 결과는 다음과 같습니다.

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

테스트로 버그를 찾아냈네요! `larger.width`는  8이고 `smaller.width`는  5인데
`can_hold`의 너비 비교 결과는 `false`(`larger.width`가 `smaller.width` 보다 작음)를 반환합니다.
8이 5보다 작진 않죠.

### `assert_eq!`, `assert_ne!` 매크로를 이용한 동등 테스트

기능성 검증의 일반적인 방법은 테스트 코드의 결괏값이
예상한 값과 같은지 확인하는 것입니다. 이는 `assert!`
매크로에 `==` 연산자를 사용한 표현식을 전달하는 식으로도 가능하지만,
러스트는 이런 테스트에 더 알맞은 매크로를 따로 제공합니다.
`assert_eq!`, `assert_ne!` 매크로는 각각 두 인수를 비교하고
동등한지 (equality) 그렇지 않은지 (inequality) 판단합니다.
단언 코드가 실패하면 두 값을 출력하여 테스트의
*실패 사유*를 더 알기 쉽게 보여줍니다.
`assert!` 매크로는 `==` 표현식이 `false` 값임을 알려줄 뿐,
어떤 값으로 인해 `false` 값이 나왔는지 출력하지는 않습니다.

예제 11-7은 매개변수에 `2`를 더하는 `add_two` 함수를 작성한 다음,
`assert_eq!` 매크로를 이용해 테스트하는 예제입니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

<span class="caption">예제 11-7: `assert_eq!` 매크로를 이용한
`add_two` 함수 테스트</span>

테스트를 통과하는지 확인해 봅시다!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

`assert_eq!`에 `4`를 인수로 넘겼는데, 이는 `add_two(2)` 호출 결과와 같습니다.
출력 중 테스트에 해당하는 줄은 `test tests::it_adds_two ... ok`이고,
`ok`는  테스트가 통과했다는 뜻이죠!

코드에 버그를 집어넣어서 `assert_eq!`가 실패했을 때는 어떤 식으로 보이는지 확인해 봅시다.
`add_two` 함수가 `3`을 더하도록 구현을 변경해 봅시다:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

테스트를 다시 실행해 보죠.

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

테스트가 버그를 찾아냈습니다! `it_adds_two` 테스트가 실패하고, 메시지는
`` assertion failed: `(left == right)` `` 메시지와 `left`, `right`가
각각 `4`, `5` 였다는 것을 알려줍니다. 이 메시지로 `assert_eq!`의 `left`
인수는 `4`였는데 `right` 인수(`add_two(2)`)는 `5`였다는 내용을 알 수 있기
때문에, 디버깅을 시작하는 데 도움이 됩니다. 수많은 테스트가 있을 때라면
특히 유용할 것임을 짐작할 수 있습니다.

몇몇 프로그래밍 언어, 프레임워크에서는 동등 단언 함수의 매개변수를
`expected`, `actual`라고 지칭하며, 코드를 작성할 때 인수의 순서를 지켜야 합니다.
하지만 러스트에서는 `left`, `right`라고 지칭할 뿐,
예상값과 테스트 코드로 만들어진 값의 순서는 상관없습니다.
테스트 코드를 `assert_eq!(add_two(2), 4)` 로 작성할 수도 있습니다.
이 경우에도 실패 메시지는 똑같이 `` assertion failed: `(left == right)` ``라고
나타납니다.

`assert_ne!` 매크로는 전달한 두 값이 서로 같지 않으면 통과하고,
동등하면 실패합니다. *어떤 값이 될지*는 확신할 수 없지만, 적어도
*이 값은 되지 않아야 함*을 알고 있는 경우에 유용합니다.
예를 들어, 테스트할 함수가 입력값을 어떤 방식으로든 변경한다는 것은
확실하지만, 테스트를 실행하는 요일에 따라 함수의 입력값이 달라진다면,
입력값과 함수 출력이 동일하면 안 된다고 테스트를 작성하는 게
가장 좋을 겁니다.

내부적으로 `assert_eq!`, `assert_ne!` 매크로는 각각 `==`, `!=`
연산자를 사용합니다. 단언에 실패할 경우, 매크로는 인수를 디버그 형식으로
출력하는데, 즉 `assert_eq!`, `assert_ne!` 매크로로 비교할 값은
`PartialEq`, `Debug` 트레이트를 구현해야 합니다.
모든 기본 타입 및 대부분의 표준 라이브러리 타입은 이 두 트레이트를 구현합니다.
직접 정의한 구조체나 열거형의 경우에는 `PartialEq` 트레이트를 구현하여
해당 타입의 값이 같음을 단언할 수 있도록 할 필요가 있습니다. 또한
단언 실패 시 값이 출력될 수 있도록 `Debug` 트레이트도 구현해야 합니다.
5장 예제 5-12에서 설명했듯 두 트레이트 모두 파생 가능한 트레이트이기 때문에,
구조체, 열거형 정의에 `#[derive(PartialEq, Debug)]`를 어노테이션하는 것이 일반적입니다.
이에 대한 추가 내용 및 파생 가능한 나머지 트레이트는
부록 C [‘파생 가능한 트레이트’][derivable-traits]<!-- ignore -->를 참고해 주세요.

### 커스텀 실패 메시지 추가하기

`assert!`, `assert_eq!`, `assert_ne!` 매크로에
추가 인수로 실패 메시지에 출력될 내용을 추가할 수 있습니다.
필수적인 인수들 이후의 인수는 `format!` 매크로로 전달됩니다.
(`format!` 매크로는 8장의
[‘`+` 연산자나 `format!` 매크로를 이용한 접합’][concatenation-with-the--operator-or-the-format-macro]<!-- ignore -->절에서 다루었습니다.)
따라서 `{}` 자리표시자가 들어있는 포맷 문자열과
자리표시자에 들어갈 값을 전달할 수 있습니다.
커스텀 메시지는 테스트 단언의 의미를 서술하는 데에 유용합니다;
테스트가 실패할 경우 코드의 문제점이
무엇인지 알아내기 더 수월해지죠.

예를 들어 이름을 불러 사람을 환영하는 함수가 있고, 함수에게 전달한
이름이 결과에 나타나는지 확인하는 테스트를 작성한다고 칩시다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

아직 프로그램의 요구 사항이 정해지지 않아서,
분명히 `Hello` 텍스트 부분이 나중에 변경될 거라고 치죠.
프로그램 요구 사항이 바뀔 때 테스트 코드도 고치고 싶지는 않으니
`greeting` 함수의 정확한 반환 값을 검사하는 대신,
출력 값에 입력 매개변수로 전달한 텍스트가 포함되어 있는지만
확인하려고 합니다.

이제 기본 테스트 실패 시 출력을 살펴보기 위해, `greeting` 함수 결괏값에서
`name`이 빠지도록 변경하여 버그를 만들어 보았습니다:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

테스트 결과는 다음과 같습니다.

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

출력 결과는 단언이 실패했다는 것과
몇 번째 줄에서 실패했는지만 표시합니다.
실패 메시지에서 `greeting` 함수의 반환 값을 출력해 주면 더 유용하겠죠.
테스트 함수에 커스텀 실패 메시지를 추가해 봅시다. `greeting` 함수가
반환하는 실제 값으로 채워지게 될 자리표시자가 들어있는 포맷 문자열을 작성해 보죠.

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

이제 에러 메시지를 보고 더 많은 정보를 얻을 수 있습니다. 테스트를 다시 실행해 보죠.

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

실제 테스트 결괏값을 볼 수 있으니 의도했던 것과
무엇이 다른지 알 수 있어, 디버깅하는 데 도움이 됩니다.

### `should_panic` 매크로로 패닉 발생 검사하기

코드의 반환 값을 검사하는 것에 더하여, 예상한대로 에러 조건을
잘 처리하는지 검사하는 것도 중요합니다. 예를 들어 9장의 예제 9-10에서
만들었던 `Guess` 타입을 생각해 보세요. `Guess` 타입을 사용하는 다른 코드는
`Guess` 인스턴스가 1에서 100 사잇값임을 보장하는 기능에 의존합니다.
이런 경우, 범위를 벗어난 값으로 `Guess` 인스턴스를 만들면 패닉이
발생하는지 검사하는 테스트를 작성하면 이를 확실하게 보장할 수 있습니다.

패닉 검사 테스트 함수에는 `should_panic` 속성을 추가합니다.
이 테스트는 내부에서 패닉이 발생해야 통과되고,
패닉이 발생하지 않으면 실패합니다.

예제 11-8은 `Guess::new`의 에러 조건이 의도대로 작동하는지
검사하는 테스트를 보여줍니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

<span class="caption">예제 11-8: `panic!` 발생
테스트</span>

`#[should_panic]` 속성은 `#[test]` 속성과
적용할 함수 사이에 위치시켰습니다.
테스트 성공 시 결과를 살펴봅시다.

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

괜찮아 보이네요! 이제 `new` 함수의 패닉 발생 조건 중 100보다
큰 값일 때의 조건을 지워서 버그를 만들어 보죠.

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

예제 11-8 테스트를 실행하면 다음과 같이 실패합니다.

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

에러 메시지는 그다지 유용하지 않지만,
테스트 함수를 살펴보면 `#[should_panic]`으로 어노테이션된 함수라는 걸 알 수 있습니다.
즉, 테스트 함수에서 패닉이 발생하지 않아서 실패했다는 뜻이죠.

`should_panic`을 사용하는 테스트는 정확하지 않을 수 있습니다.
의도한 것과는 다른 이유로 패닉이 발생하더라도
`should_panic` 테스트는 통과할 것입니다. `should_panic`
속성에 `expected` 매개변수를 추가해, 포함되어야 하는 실패
메시지를 지정하면 더 꼼꼼한 `should_panic` 테스트를 작성할
수 있습니다. 예제 11-9는 `new` 함수에서 값이 너무 작은
경우와 큰 경우에 서로 다른 메시지로 `panic!`을 발생시키도록
수정한 `Guess` 코드입니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

<span class="caption">예제 11-9: 특정한 부분 문자열을 포함하는 패닉 메시지를
사용한 `panic!`에 대한 테스트</span>

`should_panic` 속성의 `expected` 매개변숫값이
`Guess::new` 함수에서 발생한 패닉 메시지 문자열의 일부이므로 테스트는 통과합니다.
발생해야 하는 패닉 메시지 전체를 명시할 수도 있습니다.
이 경우 `Guess value must be less than or equal to 100, got 200.`이 되겠죠.
`expected` 매개변수에 명시할 내용은 패닉 메시지가 얼마나
고유한지 혹은 동적인지, 그리고 테스트에 요구되는 정확성에 따라 달라집니다.
이번 경우에는, 패닉 메시지 문자열 일부만으로도 실행된 함수 코드가
`else if value > 100` 상황에 해당함을 확신할 수 있으니 충분합니다.

`expected` 메시지를 지정한 `should_panic` 테스트가 실패하면 어떻게 되는지 알아보죠.
`if value < 1` 코드 단락과 `else if value > 100` 코드 단락을 서로 바꾸어
버그를 만들어 보았습니다.

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

이번에는 `should_panic` 테스트가 실패합니다.

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

테스트에서 패닉이 발생하긴 했지만,
지정한 `"less than or equal to 100"` 문자열이
패닉 메시지에 포함되어 있지 않다는 것을 알려줍니다.
실제로 발생한 패닉 메시지는 `Guess value must be greater than or equal to 1, got 200.`입니다.
이제 이 메시지를 단서로 버그를 찾아낼 수 있습니다!

### `Result<T, E>`를 이용한 테스트

지금까지는 실패 시 패닉을 발생시키는 테스트만 작성했습니다. 테스트는
`Result<T, E>`를 사용해 작성할 수도 있습니다. 다음은 예제 11-1 테스트를
`Result<T, E>`를 사용하도록 수정한 예시입니다. 패닉을 발생시키는 대신 `Err`을 반환합니다.

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs}}
```

이제 `it_works` 함수는 `Result<(), String>` 타입을 반환합니다.
함수 본문에서는 `assert_eq!` 매크로를 호출하는 대신,
테스트 성공 시에는 `Ok(())`를 반환하고 실패 시에는 `String`을 갖는 `Err`을
반환합니다.

`Result<T, E>`를 반환하는 테스트에서는 `?` 연산자를 사용할 수 있기 때문에,
내부 작업이 `Err`를 반환할 경우 실패해야 하는 테스트를 작성하기
편리합니다.

`Result<T, E>` 테스트에서는 `#[should_panic]` 어노테이션을 사용할 수 없습니다.
연산이 `Err` 배리언트를 반환하는 것을 단언하기 위해서는 `Result<T, E>` 값에
물음표 연산자를 *사용하지 마세요*. 대신 `assert!(value.is_err())`를
사용하세요.

여러 테스트 작성 방법을 배웠으니,
테스트를 실행할 때 어떤 일들이 일어나는지 알아보고
`cargo test` 명령어 옵션을 살펴봅시다.

[concatenation-with-the--operator-or-the-format-macro]:
ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro
[bench]: https://doc.rust-lang.org/unstable-book/library-features/test.html
[ignoring]: ch11-02-running-tests.html#ignoring-some-tests-unless-specifically-requested
[subset]: ch11-02-running-tests.html#running-a-subset-of-tests-by-name
[controlling-how-tests-are-run]:
ch11-02-running-tests.html#controlling-how-tests-are-run
[derivable-traits]: appendix-03-derivable-traits.html
[doc-comments]: ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
