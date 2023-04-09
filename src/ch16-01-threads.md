## 스레드를 이용하여 코드를 동시에 실행하기

대부분의 최신 운영 체제에서, 실행된 프로그램의 코드는 *프로세스*
내에서 실행되고, 운영 체제는 한번에 여러 개의 프로세스들을 관리하게
됩니다. 프로그램 내에서도 동시에 실행되는 독립적인 부분들을 가질 수
있습니다. 이러한 독립적인 부분들을 실행하는 기능을 *스레드*라고 부릅니다.
예를 들어 웹 서버는 여러 스레드를 가지고 동시에 하나 이상의 요청에 대한
응답을 할 수 있습니다.

여러분의 프로그램 내의 연산을 여러 스레드로 쪼개서 동시에 여러 일을 수행하게
하면 성능을 향상시킬 수 있지만, 프로그램을 복잡하게 만들기도 합니다.
스레드가 동시에 실행될 수 있기 때문에, 서로 다른 스레드에서 실행될 코드
조각들의 실행 순서에 대한 내재적인 보증이 없습니다. 이는 다음과 같은 문제들을
야기할 수 있습니다:

* 여러 스레드들이 일관성 없는 순서로 데이터 혹은 리소스에 접근하게 되는,
  경쟁 조건 (race condition)
* 두 스레드가 서로를 기다려서 양쪽 스레드 모두 계속 실행되는 것을 막아버리는,
  데드록 (deadlock)
* 특정한 상황에서만 발생되어 안정적으로 재현하고 수정하기 힘든
  버그들

러스트는 스레드 사용의 부정적인 효과를 완화하는 시도를 하지만,
멀티스레드 콘텍스트에서의 프로그래밍은 여전히 신중하게 생각해야
하고 싱글스레드로 실행되는 프로그램의 것과는 다른 코드 구조가
필요합니다.

프로그래밍 언어들은 몇가지 다른 방식으로 스레드를 구현하고, 많은
운영 체제들이 새로운 스레드를 생성하기 위해 해당 언어가 호출할 수
있는 API를 제공합니다. 러스트 표준 라이브러리는 스레드 구현에 대해
*1:1* 모델을 사용하는데, 이에 따라 프로그램은 하나의 언어 스레드당
하나의 운영 체제 스레드를 사용합니다. 1:1 모델과는 다른 트레이드오프가
있는 그밖의 스레드 모델을 구현한 크레이트도 있습니다.

### `spawn`으로 새로운 스레드 생성하기

새로운 스레드를 생성하기 위해서는 `thread::spawn` 함수를 호출하고 여기에
새로운 스레드에서 실행하고 싶은 코드가 담긴 클로저를 넘깁니다 (클로저에
대해서는 13장에서 다뤘습니다). Listing 16-1의 예제는 메인 스레드에서 어떤
텍스트를 출력하고 새로운 스레드에서는 다른 텍스트를 출력합니다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-01/src/main.rs}}
```

<span class="caption">Listing 16-1: 메인 스레드에서 무언가를 출력하는 동안
다른 것을 출력하는 새로운 스레드 생성하기</span>

러스트 프로그램의 메인 스레드가 완료되면 생성된 모든 스레드는 실행이
종료되었든 혹은 그렇지 않든 멈추게 될 것이라는 점을 주의하세요. 이
프로그램의 출력은 매번 약간씩 다를 수 있으나, 아래와 비슷하게 보일
것입니다:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

`thread::sleep`의 호출은 강제로 스레드가 잠깐 동안 실행을 멈추게 하는데,
다른 스레드는 실행될 수 있도록 합니다. 스레드들은 아마도 교대로 실행될 것이지만,
그게 보장되지는 않습니다: 여러분의 운영 체제가 스레드를 어떻게 스케줄링 하는지에
따라 다른 문제입니다. 위의 실행 예에서는 생성된 스레드로부터의 출력 구문이
먼저 나왔음에도 불구하고 메인 스레드가 먼저 출력하였습니다. 그리고 생성된
스레드에게 `i`가 9일때까지 출력하라고 했음에도 불구하고, 메인 스레드가
멈추기 전까지 고작 5에 도달했습니다.

만일 이 코드를 실행하고 메인 스레드의 출력만 보았다면, 혹은 어떠한 겹침도
보지 못했다면, 범위의 숫자값을 늘려서 운영 체제에게 스레드간 전환에 대한
더 많은 기회를 주도록 시도해 보세용.

### `join` 핸들을 사용하여 모든 스레드들이 끝날때까지 기다리기

Listing 16-1의 코드는 메인 스레드의 종료 때문에 대개의 경우 생성된
스레드를 조기에 멈출게 할 뿐만 아니라, 스레드들이 실행되는 순서에 대한
보장이 없기 때문에 생성된 스레드가 모든 코드를 실행할 것임을 보장해
줄수도 없습니다!

생성된 스레드가 실행되지 않거나, 전부 실행되지 않는 문제는 `thread::spawn`의
반환값을 변수에 저장함으로서 해결할 수 있습니다. `thread::spawn`의 반환
타입은 `JoinHandle`입니다. `JoinHandle`은 이것이 가지고 있는 `join` 메소드를
호출했을 때 그 스레드가 끝날 때까지 기다리는 소유된 값입니다. Listing 16-2는
Listing 16-1에서 만들었던 스레드의 `JoinHandle`을 이용해서 `join`을 호출하여
`main`이 끝나기 전에 생성된 스레드가 종료됨을 보장하는 방법을 보여줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-02/src/main.rs}}
```

<span class="caption">Listing 16-2: `thread::spawn`으로부터 `JoinHandle`을
저장하여 스레드가 완전히 실행되는 것을 보장하기</span>

핸들에 대해 `join`을 호출하면 핸들에 대한 스레드가 종료될 때까지 현재
실행중인 스레드를 블록합니다. 스레드를 *블록 (Block)* 한다는 것은 그 스레드의
작업을 수행하거나 종료되는 것이 방지된다는 뜻입니다. 메인 스레드의 `for`
루프 이후에 `join`의 호출을 넣었으므로, Listing 16-2의 실행은 아래와 비슷한
출력을 만들어야 합니다:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

두 스레드가 계속하여 교차하지만, `handle.join()`의 호출로 인하여 메인
스레드는 기다리고 생성된 스레드가 종료되기 전까지 끝나지 않습니다.

그런데 만일 아래와 같이 `main`의 `for` 루프 이전으로 `handle.join()`을
이동시키면 어떤 일이 생기는지 봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/no-listing-01-join-too-early/src/main.rs}}
```

메인 스레드는 생성된 스레드가 종료될 때까지 기다릴 것이고 그 다음 자신의 `for`
루프를 실행하게 되어, 아래처럼 출력값이 더 이상 교차되지 않을 것입니다:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

`join`이 호출되는 위치와 같은 작은 디테일들이 여러분의 스레드가 동시에 실행되는지
혹은 아닌지에 대해 영향을 미칠 수 있습니다.

### 스레드에 `move` 클로저 사용하기

`move` 클로저는 `thread::spawn`에 넘겨지는 클로저와 함께 자주 사용되는데,
그렇게하면 클로저가 환경으로부터 사용하는 값의 소유권을 갖게 되어 한
스레드에서 다른 스레드로 소유권이 이전될 것이기 때문입니다. 13장의
[“참조자를 캡처하거나 소유권 이전하기”][capture]<!-- ignore -->절에서
클로저의 콘텍스트에서의 `move`에 대해 다루었습니다. 지금은
`move`와 `thread::spawn` 사이의 상호작용에 더 집중해 보겠습니다.

Listing 16-1에서 'thread::spawn'에 넘겨진 클로저는 아무런 인자도 갖지
않음을 주목하세요: 생성된 스레드의 코드에서는 메인 스레드로부터 온 어떤 데이터도
이용하고 있지 않습니다. 메인 스레드로부터의 데이터를 생성된 스레드에서 사용하기
위해, 생성된 스레드의 클로저는 자신이 필요로 하는 값을 캡처해야 합니다. Listing 16-3은
메인 스레드에서 백터를 생성하여 이를 생성된 스레드 내에서 사용하는 시도를 보여주고
있습니다. 그러나 잠시 후에 보시게 될 것처럼 아직은 동작하지 않습니다.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-03/src/main.rs}}
```

<span class="caption">Listing 16-3: 메인 스레드에서 생성된 벡터에 대한 다른
스레드에서의 사용 시도</span>

클로저가 `v`를 사용하므로, `v`는 캡처되어 클로저 환경의 일부가 됩니다.
`thread::spawn`이 이 클로저를 새로운 스레드에서 실행하므로, `v`는
새로운 스레드 내에서 접근 가능해야 합니다. 하지만 이 예제를 컴파일하면
아래와 같은 에러를 얻게 됩니다:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-03/output.txt}}
```

러스트는 `v`를 어떻게 캡처할지 *추론하고*, `println!`이 `v`의 참조자만
필요로 하기 때문에, 클로저는 `v`를 빌리려고 합니다. 하지만 문제가 있습니다:
러스트는 생성된 스레드가 얼마나 오랫동안 실행될지 알 수 없으므로, `v`에 대한
참조자가 항상 유효할 것인지 알지 못합니다.

Listing 16-4는 유효하지 않은 `v`의 참조자가 있을 가능성이 더 높은
시나리오를 제공합니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-04/src/main.rs}}
```

<span class="caption">Listing 16-4: `v`를 버리는 메인 스레드로부터 `v`에
대한 참조자를 캡처하려 하는 클로저를 갖는 스레드</span>

만약 러스트가 이 코드의 실행을 허용했다면, 생성된 스레드가 전혀 실행되지
않고 즉시 백그라운드에 들어갔을 가능성이 있습니다. 생성된 스레드는 내부에
`v`의 참조자를 가지고 있지만, 메인 스레드는 15장에서 다루었던 `drop`
함수를 사용하여 `v`를 즉시 버립니다. 그러면 생성된 스레드가 실행되기
시작할 때 `v`가 더 이상 유효하지 않게 되어, 그에 대한 참조자 또한 유효하지
않게 됩니다. 이런!

Listing 16-3의 컴파일 에러를 고치기 위해서 에러 메세지의 조언을 이용할
수 있습니다:

<!-- manual-regeneration
after automatic regeneration, look at listings/ch16-fearless-concurrency/listing-16-03/output.txt and copy the relevant part
-->

```text
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++
```

`move` 키워드를 클로저 앞에 추가함으로써 러스트가 값을 빌려와야
된다고 추론하도록 하는 것이 아니라 사용중인 값의 소유권을 강제로
가지도록 합니다. Listing 16-3을 Listing 16-5처럼 수정하면 컴파일되어
의도한대로 실행됩니다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-05/src/main.rs}}
```

<span class="caption">Listing 16-5: `move` 키워드를 이용하여 클로저가
사용하는 값의 소유권을 갖도록 강제하기</span>

`move` 클로저를 사용하여 메인 스레드에서 `drop`을 호출하는 Listing 16-4의 코드를
고치려고 시도해보고 싶을 수도 있습니다. 하지만 이 수정은 동작하지 않는데,
그 이유는 Listing 16-4이 시도하고자 하는 것이 다른 이유로 허용되지 않기
때문입니다. 만일 클로저에 `move`를 추가하면, `v`를 클로저의 환경으로 이동시킬
것이고, 더이상 메인 스레드에서 이것에 대한 `drop` 호출을 할 수 없게 됩니다.
대신 아래와 같은 컴파일 에러를 얻게 됩니다:

```console
{{#include ../listings/ch16-fearless-concurrency/output-only-01-move-drop/output.txt}}
```

러스트의 소유권 규칙이 우리를 또다시 구해주었습니다! Listing 16-3의
코드로부터 에러를 받은 이유는 러스트가 보수적이려 하고 스레드를 위해 `v`를
빌리려고만 했기 때문이었는데, 이는 메인 스레드가 이론적으로 생성된 스레드의
참조자를 무효화할 수 있음을 의미합니다. 러스트에게 `v`의 소유권을 생성된 스레드로
이동시키라고 함으로써, 메인 스레드가 `v`를 더 이상 이용하지 않음을 러스트에게
보장하고 있습니다. 만일 Listing 16-4를 같은 방식으로 바꾸면, `v`를 메인
스레드에서 사용하려고 할 때 소유권 규칙을 위반하게 됩니다. `move` 키워드는
러스트의 빌림에 대한 보수적인 기본 기준을 무효화합니다; 즉 소유권 규칙을
위반하지 않도록 해줍니다.

스레드와 스레드 API에 대한 기본적인 이해를 바탕으로, 스레드로 *할 수 있는*
것들을 살펴봅시다.

[capture]: ch13-01-closures.html#capturing-references-or-moving-ownership
