## 싱글스레드 서버를 멀티스레드 서버로 바꾸기

현재의 서버는 각각의 요청을 차례대로 처리하므로, 첫 번째 요청
처리가 완료될 때까지 두 번째 연결은 처리되지 않습니다. 서버가
점점 더 많은 요청을 수신하면, 이 순차 실행 방식은 점점 더 최적화되지
않을 것입니다. 서버가 처리하는 데 시간이 오래 걸리는 요청을 받으면,
새로운 요청을 빠르게 처리할 수 있더라도 후속 요청은 긴 요청이
완료될 때까지 기다려야 할 것입니다. 이를 해결할 필요가 있지만,
그전에 먼저 이 문제를 실제로 살펴보겠습니다.

### 현재의 서버 구현에서 느린 요청 시뮬레이션

현재의 서버 구현에서 느리게 처리되는 요청이 다른 요청들에 어떤 영향을
미칠 수 있는지 살펴보겠습니다. 예제 20-10은 */sleep* 요청에 대해
서버가 응답하기 전에 5초 동안 서버를 슬립시키는 시뮬레이션된 느린 응답으로
처리하는 것을 구현한 것입니다.

<span class="filename">파일명: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-10/src/main.rs:here}}
```

<span class="caption">예제 20-10: 5초 동안 슬립하여 느린 요청
시뮬레이션하기</span>

이제 세 가지 경우가 있으므로 `if`에서 `match`로 전환했습니다. 문자열
리터럴 값에 대해 패턴 매치를 하려면 `request_line` 슬라이스에 대한
명시적인 매칭을 할 필요가 있습니다; `match`는 동등 메서드가 작동하는 것과
같은 자동 참조 및 역참조를 수행하지 않습니다.

첫 번째 갈래는 예제 20-9의 `if` 블록과 동일합니다. 두 번째
갈래는 */sleep*에 대한 요청과 매칭됩니다. 해당 요청이 수신되면,
서버는 5초 동안 슬립한 후 성공 HTML 페이지를 렌더링합니다. 세 번째
갈래는 예제 20-9의 `else` 블록과 동일합니다.

우리 서버가 얼마나 원시적인지 알 수 있습니다: 실제 라이브러리라면 훨씬 덜 장황한
방식으로 여러 가지 요청에 대한 인식을 처리할 것입니다!

`cargo run`을 사용하여 서버를 시작하세요. 그런 다음 두 개의 브라우저 창을
여세요: 하나는 *http://127.0.0.1:7878/*용, 다른 하나는 *http://127.0.0.1:7878/sleep*용입니다.
이전처럼 */* URI를 몇 번 입력해 보면, 빠르게 응답하는 것을 볼 수 있습니다.
하지만 */sleep*을 입력한 다음 */* 을 로드하면, `sleep`이 5초 동안 완전히
슬립을 끝낼 때까지 */* 는 기다렸다가 로드되는 것을 볼 수 있습니다.

느린 요청 뒤의 요청들이 뒤로 밀리는 것을 방지하기 위해 사용할 수 있는 기술은
여러 가지 있습니다; 우리가 구현해 볼 것은 스레드 풀입니다.

### 스레드 풀로 처리량 개선하기

*스레드 풀 (thread pool)* 은 작업 처리가 준비된 대기 중인
생성된 스레드의 그룹입니다. 프로그램이 새 작업을 받으면, 해당
작업을 풀에 있는 스레드 중 하나에게 할당하고, 해당 스레드가
작업을 처리합니다. 풀의 나머지 스레드는 첫 번째 스레드가 처리하는
동안 들어오는 다른 작업을 처리하는 데 사용될 수 있습니다.
첫 번째 스레드가 작업 처리를 완료하면, 유휴 스레드 풀로 돌아가서
새 작업을 처리할 준비가 됩니다. 스레드 풀을 사용하면 연결을 동시에
처리할 수 있으므로 서버의 처리량이 증가합니다.

풀의 스레드 개수를 적은 수로 제한하여 서비스 거부 (Denial of Services, DoS)
공격으로부터 보호하겠습니다; 만일 요청이 들어올 때마다 프로그램이 새
스레드를 생성하도록 하면, 누군가 서버에 천만 건의 요청을 했을 때
서버의 모든 리소스를 사용하고 요청 처리가 중단되어 혼란을 일으킬
수 있습니다.

그래서 무제한 스레드를 생성하는 대신, 풀에 고정된 수의 스레드가
대기하도록 할 것입니다. 들어오는 요청은 처리를 위해 풀로 전송됩니다.
풀은 들어오는 요청의 대기열을 관리합니다. 풀의 각 스레드는 이
대기열에서 요청을 꺼내서, 요청을 처리한 다음, 대기열에게 다른
요청을 달라고 합니다. 이 설계를 사용하면 최대 `N` 개의 요청을
동시에 처리할 수 있으며, 여기서 `N`은 스레드 개수입니다. 각 스레드가
길게 실행되는 요청에 응답하는 경우에는 후속 요청이 여전히 대기열
뒤편에 있을 수 있지만, 그 지점에 도달하기 전에 처리 가능한 장기간
실행되는 요청의 개수를 늘렸습니다.

이 기술은 웹 서버의 처리량을 개선하는 여러 가지 방법 중 하나일
뿐입니다. 탐구해 볼만한 다른 옵션으로는 *포크/조인 (fork/join) 모델*,
*싱글스레드 비동기 (single-threaded async) I/O 모델* 또는
*멀티스레드 비동기 (multi-threaded async) I/O 모델* 이 있습니다.
이 주제에 관심이 있다면, 다른 솔루션에 대한 자세한 내용을 읽고 구현을 시도해
볼 수 있습니다; 러스트와 같은 저수준 언어를 사용하면 이 모든 옵션이 가능합니다.

스레드 풀 구현을 시작하기 전에, 풀을 사용하는 것이 어떤 모습이어야
하는지에 대해 이야기해 봅시다. 코드를 설계할 때 클라이언트 인터페이스를
먼저 작성하면 설계에 도움이 될 수 있습니다. 코드의 API를 호출하고자
하는 방식대로 구조화되도록 작성하세요; 그런 다음 그 구조를 지키면서
기능을 구현하는 것이지요. 기능을 구현한 다음에 공개 API를 설계하는
방식이 아니고요.

12장의 프로젝트에서 테스트 중심 개발을 사용한 것과 유사하게,
여기서는 컴파일러 중심 개발을 사용하겠습니다. 원하는 함수를 호출하는
코드를 작성한 다음, 컴파일러의 에러를 살펴보고 코드가 작동하도록
하기 위해서는 다음에 무엇을 변경해야 하는지 결정하는 것입니다.
하지만 착수 단계에서는 사용하지 않을 기법 먼저 살펴보겠습니다.

<!-- Old headings. Do not remove or links may break. -->
<a id="code-structure-if-we-could-spawn-a-thread-for-each-request"></a>

#### 각 요청마다 스레드 생성하기

먼저 모든 연결에 대해 새 스레드를 생성하면 코드가 어떻게 생기게
되는지 살펴봅시다. 앞서 언급했듯이, 이 방법은 스레드 수가 무제한으로
생성될 수 있는 문제 때문에 우리의 최종 계획은 아니지만, 먼저 멀티스레드
서버를 작동시키기 위한 출발점은 될 수 있습니다. 그런 다음 스레드
풀을 개선 사항으로 추가하면 두 솔루션을 비교하기가 더 쉬워질 것입니다.
예제 20-11은 `for` 루프 내에서 새 스레드를 생성하여 각 스트림을
처리하기 위해 `main`에 변경해야 할 사항을 보여줍니다.

<span class="filename">파일명: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-11/src/main.rs:here}}
```

<span class="caption">예제 20-11: 각 스트림마다 새 스레드
생성하기</span>

16장에서 배웠던 것처럼, `thread::spawn`은 새 스레드를 생성한 다음 새 스레드의
클로저에서 코드를 실행합니다. 이 코드를 실행하고 브라우저에서 */sleep*을 로드한
다음, 두 개의 브라우저 탭에서 */* 을 추가로 로드하면, */* 에 대한 요청이 */sleep*이
완료될 때까지 기다릴 필요가 없다는 것을 실제로 확인할 수 있습니다. 하지만 앞서
언급했듯이, 이렇게 하면 아무런 제한 없이 새 스레드를 만들게 되므로 결국 시스템에
과부하가 걸리게 됩니다.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-a-similar-interface-for-a-finite-number-of-threads"></a>

#### 유한한 개수의 스레드 생성하기

여기서는 스레드 풀이 유사하고 익숙한 방식으로 작동하여 스레드에서 스레드
풀로 전환할 때 API를 사용하는 코드가 크게 변경될 필요는 없도록 하려고
합니다. 예제 20-12는 `thread::spawn` 대신 사용하고자 하는 `ThreadPool`
구조체에 대한 가상 인터페이스를 보여줍니다.

<span class="filename">파일명: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/listing-20-12/src/main.rs:here}}
```

<span class="caption">예제 20-12: 이상적인 `ThreadPool` 인터페이스</span>

`ThreadPool::new`를 사용하여 설정 가능한 스레드 수(위의 경우 4개)를 넣어
새로운 스레드 풀을 생성합니다. 그런 다음 `for` 루프 안에서는 `pool.execute`가
각 스트림에 대해 풀이 실행해야 하는 클로저를 취한다는 점에서 `thread::spawn`과
유사한 인터페이스를 가집니다. 따라서 이 클로저를 받은 다음 풀의 스레드에 전달하여
실행되도록 `pool.execute`를 구현할 필요가 있습니다. 이 코드는 아직 컴파일되지
않지만, 컴파일러가 문제를 해결하는 방법을 안내할 수 있도록 시도해 보겠습니다.

<!-- Old headings. Do not remove or links may break. -->
<a id="building-the-threadpool-struct-using-compiler-driven-development"></a>

#### 컴파일러 주도 개발을 사용하여 `ThreadPool` 구현하기

예제 20-12에서 *src/main.rs*를 변경한 다음, `cargo check`이 주는
컴파일러 에러를 사용하여 개발을 진행해 보겠습니다. 다음은 우리가 얻는
첫 번째 에러입니다:

```console
{{#include ../listings/ch20-web-server/listing-20-12/output.txt}}
```

훌륭합니다! 이 에러는 `ThreadPool` 타입 또는 모듈이 필요하다는 것을
알려주므로, 지금 구축해 보겠습니다. `ThreadPool` 구현은 웹 서버가
수행하는 작업의 종류와 무관합니다. 따라서, `hello` 상자를 바이너리
크레이트에서 라이브러리 크레이트로 전환하여 `ThreadPool` 구현을 담아봅시다.
라이브러리 크레이트로 변경한 후에는 웹 요청을 처리하는 것뿐만 아니라,
스레드 풀을 사용하여 수행하려는 어떤 작업에 대해서라도 분리된 스레드 풀
라이브러리를 사용할 수 있습니다.

현재 우리가 가질 수 있는 가장 간단한 `ThreadPool` 구조체의 정의에
해당하는 다음의 내용이 포함된 *src/lib.rs*를 생성하세요:

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-01-define-threadpool-struct/src/lib.rs}}
```

그런 다음 *src/main.rs*의 상단에 아래 코드를 추가하도록 *main.rs* 파일을 수정하여
라이브러리 크레이트에서 `ThreadPool`을 스코프에 가져오도록 합니다:

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/no-listing-01-define-threadpool-struct/src/main.rs:here}}
```

이 코드는 여전히 작동하지 않겠지만, 다음으로 해결해야 할 에러를
확인해 봅시다:

```console
{{#include ../listings/ch20-web-server/no-listing-01-define-threadpool-struct/output.txt}}
```

이 에러는 다음으로 `ThreadPool`에 대해 `new`라는 이름의 연관 함수를
만들어야 함을 나타냅니다. 또한 `new`에는 `4`를 인수로 받을 수 있는 하나의
매개변수가 있어야 하며 `ThreadPool` 인스턴스를 반환해야 한다는 것을 알고
있습니다. 이러한 특성을 갖는 가장 간단한 `new` 함수를 구현해
봅시다:

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-02-impl-threadpool-new/src/lib.rs}}
```

`size` 매개변수의 타입으로 `usize`를 선택한 이유는 음수 개수의
스레드가 의미 없음을 알고 있기 때문입니다. 또한 3장의
[‘정수형’][integer-types]<!-- ignore -->절에서 설명한 것처럼,
이 4를 스레드 컬렉션의 요소 개수로 사용할 것임을 알고 있으며,
이것이 바로 `usize` 타입의 용도입니다.

다시 한번 코드를 검사해 봅시다:

```console
{{#include ../listings/ch20-web-server/no-listing-02-impl-threadpool-new/output.txt}}
```

이번에는 `ThreadPool`에 `execute` 메서드가 없어서 에러가 발생합니다.
[‘유한한 개수의 스레드 생성하기’](#creating-a-finite-number-of-threads)<!-- ignore -->절에서
스레드 풀이 `thread::spawn`과 유사한 인터페이스를 가져야
한다고 결정했던 것을 상기하세요. 또한, 주어진 클로저를
받아 풀의 유휴 스레드에 전달하여 실행되도록 `execute`
함수를 구현하겠습니다.

클로저를 매개변수로 받도록 `ThreadPool`에 `execute` 메서드를
정의하겠습니다. 13장의
[‘캡처된 값을 클로저 밖으로 이동하기와 `Fn` 트레이트’][fn-traits]<!-- ignore -->절에서
클로저를 매개변수로 받기 위해 세 가지 트레이트를 사용할 수 있다고 한 것을 상기하세요:
바로 `Fn`, `FnMut`, `FnOnce` 였지요. 여기서는 어떤 종류의 클로저를 사용할지
결정할 필요가 있습니다. 표준 라이브러리 `thread::spawn` 구현과 비슷한 작업을
하게 될 것이므로, `thread::spawn`의 시그니처가 매개변수에 어떤 트레이트 바운드가
있는지 살펴볼 수 있겠습니다. 문서에는 다음과 같은 내용이 나와 있습니다:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

`F` 타입 매개변수가 여기서 고려하는 그것입니다; `T` 타입 매개변수는 반환 값과
관련이 있으며, 여기서는 고려하지 않고 있습니다. `spawn`이 `F`의 트레이트
바운드로 `FnOnce`를 사용하는 것을 볼 수 있습니다. 이것이 아마도 우리가
원하는 것일 텐데, `execute`에서 얻은 인수는 결국 `spawn`에 전달될 것이기
때문입니다. 요청을 실행하는 스레드는 해당 요청의 클로저를 딱 *한 번*만
실행하기 때문에 `FnOnce`가 여기서 사용하고자 하는 트레이트라는 것을 더욱
확신할 수 있으며, 이는 `FnOnce`의 `Once`와도 일치합니다.

`F` 타입 매개변수에는 또한 트레이트 바운드 `Send`와 라이프타임 바운드 `'static`이
있는데, 이는 지금의 상황에서 유용합니다: 한 스레드에서 다른 스레드로 클로저를
전송하기 위해서는 `Send`가 필요하고, 스레드가 실행되는 데 얼마나 오래 걸릴지
모르므로 `'static`이 필요합니다. 이러한 바운드를 사용하여 `ThreadPool`에
`F` 타입의 일반 매개변수를 받는 `execute` 메서드를 만들어 보겠습니다:

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-03-define-execute/src/lib.rs:here}}
```

여기에서 `FnOnce`는 매개변수가 없고 유닛 타입 `()`를 반환하는 클로저를
나타내므로 `FnOnce` 뒤에는 여전히 `()`가 사용됩니다. 함수 정의와
마찬가지로 반환 타입은 시그니처에서 생략이 가능하지만, 매개변수가
없더라도 괄호는 여전히 필요합니다.

다시 말하지만, 이것은 `execute` 메서드의 가장 간단한 구현입니다: 아무 일도 하지
않지만, 코드가 컴파일되도록 하는 것만 시도하는 중입니다. 다시 한번 검사해 봅시다:

```console
{{#include ../listings/ch20-web-server/no-listing-03-define-execute/output.txt}}
```

컴파일 되는군요! 하지만 `cargo run`을 실행한 다음 브라우저에서 요청을
날리면, 이 장의 시작 부분에서 보았던 에러가 브라우저에 표시될 것입니다.
우리 라이브러리는 아직 실제로 `execute`로 전달된 클로저를 호출하지
않았거든요!

> Note: 하스켈이나 러스트와 같은 엄격한 컴파일러가 있는 언어에 대해서 ‘코드가
> 컴파일되면 작동한다’는 말을 흔히 들을 수 있습니다. 하지만 이 말이 보편적인
> 사실은 아닙니다. 우리 프로젝트는 컴파일은 되지만, 아무것도 하지 않습니다!
> 실제 완전한 프로젝트를 구축하고 있다면, 코드가 컴파일되고 *그러면서도*
> 원하는 동작을 하는지 확인하기 위해서 유닛 테스트의 작성을 시작하기 좋은
> 시점입니다.

#### `new`에서 스레드 개수 검증하기

아직 매개변수 `new`와 `execute`로 아무것도 하지 않고 있습니다.
이제 원하는 동작이 되도록 이 함수들의 본문을 구현해 봅시다. 먼저
`new`에 대해 생각해 봅시다. 앞서 `size` 매개변수에 부호 없는 타입을
선택했는데, 그 이유는 스레드 수가 음수인 풀은 의미가 없기 때문입니다.
그러나 스레드 수가 0인 풀도 의미가 없지만, 0은 완벽하게 유효한
`usize`입니다. 예제 20-13에 나온 것처럼 `ThreadPool` 인스턴스를
반환하기 전에 `size`가 0보다 큰지 확인하는 코드를 추가하고, `assert!`
매크로를 사용하여 0을 수신하면 프로그램이 패닉 상태에 빠지도록 하겠습니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-13/src/lib.rs:here}}
```

<span class="caption">예제 20-13: `size`가 0이면 패닉을 일으키도록
`ThreadPool::new` 구현하기</span>

문서화 주석을 써서 `ThreadPool`에 대한 문서도 약간 추가했습니다.
14장에서 설명한 것처럼 함수가 패닉에 빠질 수 있는 상황을
설명하는 구절을 추가하여 좋은 문서화 관행을 따랐음을 주목하세요.
`cargo doc --open`을 실행하고 `ThreadPool` 구조체를 클릭하여
`new`에 대해 생성된 문서가 어떻게 보이는지 확인해 보세요!

여기처럼 `assert!` 매크로를 추가하는 대신, 예제 12-9의 I/O
프로젝트에서 `Config::build`를 구현할 때처럼 `new`를 `build`로
변경하고 `Result`를 반환할 수도 있습니다. 하지만 지금의 경우에는
스레드 없이 스레드 풀을 생성하려고 하면 복구할 수 없는 에러가 발생한다고
결정했습니다. 도전할 마음이 있다면 다음과 같은 시그니처를 가진 `build`라는
이름의 함수를 작성해서 `new` 함수와 비교해 보세요:

```rust,ignore
pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
```

#### 스레드를 저장할 공간 만들기

이제 풀에 저장할 스레드의 유효한 개수가 입력된 것을 알 방법이 생겼으므로,
`ThreadPool` 구조체를 반환하기 전에 해당 스레드를 생성하고 이를 구조체에
저장할 수 있습니다. 그런데 스레드를 어떻게 ‘저장’할까요? `thread::spawn`
시그니처를 다시 한번 살펴봅시다:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

`spawn` 함수는 `JoinHandle<T>`를 반환하는데, 여기서 `T`는 클로저가
반환하는 타입입니다. `JoinHandle`도 사용해 보고 어떤 일이 일어나는지
살펴봅시다. 지금의 경우 스레드 풀에 전달하는 클로저는 연결을 처리하고
아무것도 반환하지 않으므로, `T`는 유닛 타입 `()`가 됩니다.

예제 20-14의 코드는 컴파일되지만 아직 스레드를 생성하지 않습니다.
`ThreadPool`의 정의를 변경하여 `thread::JoinHandle<()>` 인스턴스의
벡터를 보관하고, `size` 용량으로 벡터를 초기화하고, 스레드를 생성하기
위해 어떤 코드를 실행하는 `for` 루프를 설정한 다음, 이들을 담고 있는
`ThreadPool` 인스턴스를 반환했습니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch20-web-server/listing-20-14/src/lib.rs:here}}
```

<span class="caption">예제 20-14: `ThreadPool`가 스레드를 담아둘
벡터 생성하기</span>

`ThreadPool` 내 벡터의 아이템 타입으로 `thread::JoinHandle`을
사용하기 때문에, 라이브러리 크레이트로부터 `std::thread`를 스코프로
가져왔습니다.

일단 유효한 크기를 받으면 `ThreadPool`은 아이템을 `size`만큼 담을 수
있는 새 벡터를 생성합니다. `with_capacity` 함수는 `Vec::new`와 동일한
작업을 수행하지만 중요한 차이점이 있습니다: 벡터에 공간을 미리 할당한다는
것입니다. 벡터에 `size` 개의 요소들을 저장해야 한다는 것을 알고 있기 때문에,
요소가 삽입될 때 크기가 조정되는 `Vec::new`를 사용하는 것보다는, 이렇게
할당을 미리 수행하는 것이 약간 더 효율적입니다.

`cargo check`를 다시 실행해 보면 성공할 것입니다.

#### `ThreadPool`에서 스레드로 코드의 전송을 담당하는 `Worker` 구조체

예제 20-14의 `for` 루프에 스레드 생성에 관한 주석을 남겼습니다.
여기서는 실제로 스레드를 생성하는 방법을 살펴보겠습니다. 표준
라이브러리는 스레드를 생성하는 방법으로 `thread::spawn`을 제공하며,
`thread::spawn`은 스레드가 생성되는 즉시 스레드가 실행해야 하는 코드를
가져올 것으로 예상합니다. 그러나 지금의 경우는 스레드를 생성한 후
나중에 전송될 코드를 *대기*하도록 하고 싶습니다. 표준 라이브러리의
스레드 구현에는 이를 수행하는 방법이 포함되어 있지 않습니다; 우리가
수동으로 구현해야 합니다.

`ThreadPool`과 스레드 사이에 이러한 새로운 동작을 관리하게 될 새로운
데이터 구조를 도입하여 이 동작을 구현하겠습니다. 이 데이터 구조를
*워커 (worker)* 라고 부를 건데, 이는 풀링 구현에서 일반적으로 사용되는
용어입니다. 워커는 실행해야 하는 코드를 집어 들어서 이 코드를 워커의
스레드에서 실행합니다. 식당의 주방에서 일하는 사람들을 생각해 보세요:
워커는 고객으로부터 주문이 들어올 때까지 기다렸다가, 주문을 받고
주문을 이행하는 일을 담당합니다.

스레드 풀에 `JoinHandle<()>` 인스턴스의 벡터를 저장하는 대신, `Worker`
구조체의 인스턴스를 저장하겠습니다. 각 `Worker`는 하나의 `JoinHandle<()>`
인스턴스를 저장하게 될 겁니다. 그런 다음 실행할 코드의 클로저를 가져와서
이미 실행 중인 스레드로 전송하여 이를 실행하는 메서드를 `Worker`에
구현하겠습니다. 또한 각 워커에 `id`를 부여하여 로깅이나 디버깅을 할 때
풀의 워커들을 서로 구별할 수 있도록 하겠습니다.

다음은 `ThreadPool`을 생성할 때 일어날 새로운 과정입니다.
이런 식으로 `Worker`를 설정한 다음 클로저를 스레드로 보내는
코드를 구현하겠습니다:

1. `id`와 `JoinHandle<()>`를 가지고 있는 `Worker` 구조체를 정의합니다.
2. `ThreadPool`이 `Worker` 인스턴스의 벡터를 갖도록 변경합니다.
3. `id` 숫자를 받아서 `id`와 빈 클로저로 생성된 스레드를
   가진 `Worker` 인스턴스를 반환하는 `Worker::new` 함수를
   정의합니다.
4. `ThreadPool::new`에서 `for` 루프 카운터를 사용하여 `id`를 생성하고, 해당
   `id`로 새 `Worker`를 생성한 다음 벡터에 워커를 저장합니다.

도전해 보고 싶다면 예제 20-15의 코드를 살펴보기 전에 이러한 변경
사항을 직접 구현해 보세요.

준비됐나요? 여기 예제 20-15가 앞서 설명한 변경 사항을 구현한 방법 중 하나를 보여줍니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-15/src/lib.rs:here}}
```

<span class="caption">예제 20-15: 스레드를 직접 가지는 대신 `Worker`
인스턴스를 가지도록 `ThreadPool` 수정하기</span>

`ThreadPool`의 필드 이름을 `threads`에서 `workers`로 변경했는데,
이제는 `JoinHandle<()>` 인스턴스 대신 `Worker` 인스턴스를 가지게 되기
때문입니다. `for` 루프의 카운터를 `Worker::new`의 인수로 사용하고,
각각의 새로운 `Worker`를 `workers`라는 벡터에 저장합니다.

(*src/main.rs* 서버 같은) 외부 코드는 `ThreadPool` 내에서 `Worker`
구조체를 사용하는 것과 관련된 구현 세부 사항을 알 필요가 없으므로,
`Worker` 구조체와 그 `new` 함수를 비공개로 설정합니다. `Worker::new`
함수는 우리가 제공한 `id`를 써서 빈 클로저를 사용하는 새 스레드를 생성하는
방식으로 만들어진 `JoinHandle<()>` 인스턴스를 저장합니다.

> Note: 시스템 리소스가 충분하지 않아 운영체제가 스레드를 생성할
> 수 없는 경우 `thread::spawn`은 패닉을 일으키게 됩니다. 그러면
> 스레드 생성이 일부 성공하더라도 전체 서버가 패닉에 빠지게 됩니다.
> 단순화를 위해서라면 이 동작은 괜찮지만, 프로덕션에서의 스레드 풀
> 구현이라면 패닉 대신 `Result`를 반환하는
> [`std::thread::Builder`][builder]<!-- ignore -->와 여기서 제공하는
> [`spawn`][builder-spawn]<!-- ignore --> 메서드를 사용하고 싶을 수도 있겠습니다.

이 코드는 컴파일되고 우리가 `ThreadPool::new`에 인수로 지정한 개수만큼
`Worker` 인스턴스를 저장합니다. 하지만 `execute`에서 얻은 클로저는 *여전히*
처리하지 않고 있습니다. 다음에는 이를 처리하는 방법을 살펴보겠습니다.

#### 채널을 통해 스레드에 요청 보내기

다음으로 다룰 문제는 `thread::spawn`에 주어진 클로저가 아무 일도
하지 않는다는 것입니다. 현재는 `execute` 메서드에서 실행하고자 하는
클로저를 얻습니다. 그러나 `ThreadPool`의 생성 중에 각 `Worker`를
생성할 때 실행할 클로저를 `thread::spawn`에 제공해야 합니다.

앞에서 만들어 둔 `Worker` 구조체가 `ThreadPool`에 보관된 대기열에서 실행할
코드를 가져온 다음 그 코드를 자신의 스레드로 전송하여 실행하기를 원합니다.

16장에서 배운 채널, 즉 두 개의 스레드 간에 통신하는 간단한 방법은 지금의
사용 사례에 완벽히 들어맞을 것입니다. 채널을 사용하여 작업의 대기열로 작동하도록
하고, `execute`가 `ThreadPool`에서 `Worker` 인스턴스로 작업을 보내면,
이 인스턴스는 자신의 스레드로 작업을 보내게 됩니다. 계획은 다음과 같습니다:

1. `ThreadPool`은 채널을 생성하고 송신자를 대기시킵니다.
2. 각 `Worker`는 수신자를 보관합니다.
3. 채널을 통해 보내려는 클로저를 가진 새로운 구조체
   `Job`을 만듭니다.
4. `execute` 메서드는 송신자를 통하여 실행하려는 작업을
   보냅니다.
5. `Worker`는 자신의 스레드에서 수신자에 대한 반복을 수행하고
   자신이 받은 작업의 클로저를 실행합니다.

예제 20-16에 나온 것처럼 `ThreadPool::new`에 채널을 생성하고
`ThreadPool` 인스턴스가 송신자를 갖도록 하는 것으로 시작하겠습니다.
지금은 `Job` 구조체에 아무것도 없지만 이것이 채널을 통해 전송될 아이템
타입이 될 것입니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-16/src/lib.rs:here}}
```

<span class="caption">예제 20-16: `Job` 인스턴스를 보내는 채널의
송신자를 저장하도록 `ThreadPool` 수정하기</span>

`ThreadPool::new`에서 새 채널을 생성하고 풀이 송신자를 보유하도록 합니다.
이 코드는 성공적으로 컴파일됩니다.

스레드 풀이 채널을 생성할 때 채널의 수신자를 각 워커에 전달해
봅시다. 워커가 생성하는 스레드에서 수신자를 사용하고자 함을 알고
있으므로, 클로저에서 `receiver` 매개변수를 참조하겠습니다. 예제 20-17의
코드는 아직 컴파일되지 않습니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/listing-20-17/src/lib.rs:here}}
```

<span class="caption">예제 20-17: 워커에게 수신자 넘기기</span>

약간의 직관적인 변경 사항을 적용했습니다: 수신자를 `Worker::new`로
전달한 다음, 클로저 내부에서 사용하였습니다.

이 코드를 검사하면 아래와 같은 에러가 발생합니다:

```console
{{#include ../listings/ch20-web-server/listing-20-17/output.txt}}
```

이 코드는 여러 개의 `Worker` 인스턴스에게 `receiver`를 전달하는 시도를
하고 있습니다. 16장에서 상기하실 수 있듯, 이는 동작하지 않습니다: 러스트가
제공하는 채널 구현체는 여러 개의 *생산자*, 하나의 *소비자*입니다. 즉, 이
코드를 수정하기 위해 채널의 소비자 쪽만 복제할 수는 없습니다. 또한 여러
소비자에게 메시지를 여러 번 보내고 싶지도 않습니다; 각 메시지가 한 번씩
처리되도록 여러 워커가 있는 하나의 메시지 리스트가 되기를 원합니다.

또한, 채널 대기열에서 작업을 빼내려면 `receiver`를 변경해야 하므로,
스레드가 `receiver`를 안전하게 공유하고 수정할 수 있는 방법이 필요합니다;
그렇지 않으면 (16장에서 다룬 것처럼) 데이터 경합이 발생할 수 있습니다.

16장에서 설명한 스레드 안전 스마트 포인터를 상기해 봅시다: 여러 스레드에서
소유권을 공유하고 스레드가 값을 변경할 수 있도록 하려면, `Arc<Mutex<T>>`를
사용할 필요가 있습니다. `Arc` 타입은 여러 워커가 수신자를 소유할 수 있도록
하고, `Mutex`는 한 번에 한 워커만 수신자로부터 작업을 가져올 수 있도록
합니다. 예제 20-18은 변경해야 할 사항을 보여줍니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-18/src/lib.rs:here}}
```

<span class="caption">예제 20-18: `Arc`와 `Mutex`를 사용하여
여러 워커 간에 수신자 공유하기</span>

`ThreadPool::new`에서 수신자를 `Arc`와 `Mutex`에 넣습니다. 각각의
새 워커에 대해 `Arc`를 복제하여 참조 카운트를 늘려 워커가 수신자의
소유권을 공유할 수 있도록 합니다.

이 변경 사항을 적용하면 코드가 컴파일됩니다! 이제 끝이 보입니다!

#### `execute` 메서드 구현하기

마지막으로 `ThreadPool`에서 `execute` 메서드를 구현해 봅시다.
또한 `Job`을 구조체가 아니라 `execute`가 수신하는 클로저 타입을
갖는 트레이트 객체의 타입 별칭으로 변경하겠습니다. 19장의
[‘타입 별칭으로 타입의 동의어 만들기’][creating-type-synonyms-with-type-aliases]<!-- ignore -->
절에서 설명한 것처럼, 타입 별칭을 사용하면 긴 타입을 사용하기 쉽도록
짧게 만들 수 있습니다. 예제 20-19를 봅시다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-19/src/lib.rs:here}}
```

<span class="caption">예제 20-19: 각 클로저를 담는 `Box`에 대한 `Job`
타입 별칭을 만들어서 이 작업을 채널로 보내기</span>

`execute`에서 얻은 클로저를 사용하여 새 `Job` 인스턴스를 생성한
후, 해당 작업을 채널 단말로 보냅니다. 전송이 실패할 경우를 위하여
`send`에서 `unwrap`을 호출하고 있습니다. 이를테면 모든 스레드의
실행이 중지되어 수신 측에서 새 메시지의 수신을 중단한 경우에는
전송이 실패할 수 있습니다. 현재로서는 스레드 실행을 중지할 수 없습니다:
풀이 존재하는 한 스레드는 계속 실행됩니다. 여기서 `unwrap`을 사용하는
이유는 실패 사례가 발생하지 않을 것이라는 것을 우리는 알고 있지만,
컴파일러는 이를 알지 못하기 때문입니다.

하지만 아직 끝나지 않았습니다! 워커에서 `thread::spawn`으로 전달된
클로저는 여전히 채널의 수신 단말을 *참조만* 하고 있습니다. 그 대신
클로저가 영원히 반복되도록 하여, 채널의 수신 단말에 작업을 요청하고
작업을 받으면 해당 작업을 실행하도록 할 필요가 있습니다. 예제 20-20에
나온 변경 사항을 `Worker::new`에 적용해 봅시다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-20/src/lib.rs:here}}
```

<span class="caption">예제 20-20: 워커의 스레드에서 작업을 받아서
실행하기</span>

여기서는 먼저 `receiver`에서 `lock`을 호출하여 뮤텍스를 획득한 다음, `unwrap`을
호출하여 에러가 발생하면 패닉을 일으키도록 합니다. 뮤텍스가 *독성 (poisoned)*
상태인 경우라면 락이 실패할 수 있는데, 이는 다른 스레드가 락을 가지고 있는
상태에서 락을 해제하지 않고 패닉에 빠졌을 때 발생할 수 있는 일입니다. 이
상황에서는 `unwrap`을 호출하여 이 스레드를 패닉 상태에 빠뜨리는 것이 올바른
조치입니다. 이 `unwrap`을 의미 있는 에러 메시지와 함께 `expect`로 편하게 변경하셔도
좋습니다.

뮤텍스에서 락을 얻으면 `recv`를 호출하여 채널로부터 `Job`을 받습니다.
여기서도 마지막 `unwrap`이 모든 에러를 지나치게 되는데, 이는 수신자가
종료되면 `send` 메서드가 `Err`을 반환하는 것과 유사하게 송신자를 가지고
있는 스레드가 종료된 경우 발생할 수 있습니다.

`recv` 호출은 스레드 실행을 차단하므로, 아직 작업이 없는 경우 현재 스레드는
작업이 들어올 때까지 기다리게 될 것입니다. `Mutex<T>`는 한 번에 하나의
`Worker` 스레드만 작업을 요청하도록 하는 것을 보장합니다.

이제 스레드 풀이 작동할 수 있는 상태가 되었습니다! `cargo run`을 실행하고
요청을 몇 가지 해보세요:

<!-- manual-regeneration
cd listings/ch20-web-server/listing-20-20
cargo run
make some requests to 127.0.0.1:7878
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field is never read: `workers`
 --> src/lib.rs:7:5
  |
7 |     workers: Vec<Worker>,
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: field is never read: `id`
  --> src/lib.rs:48:5
   |
48 |     id: usize,
   |     ^^^^^^^^^

warning: field is never read: `thread`
  --> src/lib.rs:49:5
   |
49 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `hello` (lib) generated 3 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 1.40s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
```

성공이군요! 이제 연결을 비동기적으로 실행하는 스레드 풀이 생겼습니다.
스레드가 4개 이상 생성되지 않으므로 서버가 많은 요청을 받더라도
시스템에 과부하가 발생하지 않습니다. */sleep* 요청을 보내면, 서버는
다른 요청에 대해 다른 스레드에서 이를 처리하는 방식으로 요청을 처리할
수 있습니다.

> Note: 여러 브라우저 창에서 */sleep*을 동시에 열면, 5초 간격으로
> 한 번에 하나씩 로드될 수도 있습니다. 몇몇 웹 브라우저는 캐싱을
> 위해 동일한 요청의 여러 인스턴스를 순차적으로 실행합니다. 이 제한
> 사항은 우리의 웹 서버에 의한 것이 아닙니다.

18장에서 `while let` 루프에 대해 배운 후이니, 예제 20-21에 나온 것처럼
워커 스레드 코드를 작성하지 않은 이유가 궁금할 수도 있겠습니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch20-web-server/listing-20-21/src/lib.rs:here}}
```

<span class="caption">예제 20-21: `while let`을 사용한 `Worker::new`의
대체 구현</span>

이 코드는 컴파일되고 실행되지만 결과적으로는 원하는 대로 스레드가 동작하지
않습니다: 느린 요청은 여전히 다른 요청이 처리될 때까지 대기하게 됩니다.
그 이유는 다소 미묘합니다: `Mutex` 구조체에는 공개 `unlock`
메서드가 없는데, 이는 락의 소유권이 `lock` 메서드가 반환하는
`LockResult<MutexGuard<T>>` 내의 `MutexGuard<T>`의 수명에
기반하기 때문입니다. 그러면 컴파일 타임에 대여 검사기는 락을
보유하지 않은 경우 `Mutex`에 의해 보호되는 리소스에 접근할 수
없다는 규칙을 적용할 수 있습니다. 그러나 위 구현은 `MutexGuard<T>`의
수명을 염두에 두지 않은 경우 락이 의도한 것보다 더 오래 유지될 수
있습니다.

예제 20-20의 `let job = receiver.lock().unwrap().recv().unwrap();`을
사용하는 코드는 작동하는데, `let`을 사용하면 등호 기호 오른쪽의
표현식에 사용된 모든 임시 값이 `let` 문이 끝날 때 즉시
버려지기 때문입니다. 그러나 `while let` (그리고 `if let`과
`match`) 은 연관된 블록이 끝날 때까지 임시 값을 버리지 않습니다.
예제 20-21에서는 `job()`을 호출하는 동안 락이 유지되므로
다른 워커가 작업을 받을 수 없습니다.

[creating-type-synonyms-with-type-aliases]:
ch19-04-advanced-types.html#creating-type-synonyms-with-type-aliases
[integer-types]: ch03-02-data-types.html#integer-types
[fn-traits]:
ch13-01-closures.html#moving-captured-values-out-of-the-closure-and-the-fn-traits
[builder]: https://doc.rust-lang.org/std/thread/struct.Builder.html
[builder-spawn]: https://doc.rust-lang.org/std/thread/struct.Builder.html#method.spawn
