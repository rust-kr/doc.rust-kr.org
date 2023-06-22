## 우아한 종료와 정리

예제 20-20의 코드는 의도한 대로 스레드 풀을 사용하여 비동기적으로
요청에 응답하고 있습니다. 직접적인 방식으로 사용하지 않는 `workers`,
`id` 및 `thread` 필드에 대한 경고가 표시되어 아무것도 정리하고 있지
않음을 알려줍니다. 덜 우아한 <span class="keystroke">ctrl-c</span>
방법을 사용하여 메인 스레드를 중지하면 다른 모든 스레드도 마찬가지로
즉시 중지되는데, 심지어 요청을 처리하는 도중에 있더라도 그렇게
됩니다.

그래서 다음으로는 풀의 각 스레드에 대해 `join`을 호출하도록 `Drop`
트레이트를 구현하여 종료 전에 작업 중인 요청을 완료할 수 있도록
하겠습니다. 그런 다음 작업 스레드에게 새 요청 수락을 중단하고 종료해야
한다고 알릴 방법을 구현할 것입니다. 이 코드가 실제로 작동하는지 확인하기
위해 서버를 수정하여 스레드 풀을 정상적으로 종료하기 전에 두 개의 요청만
수락하도록 해보겠습니다.

### `ThreadPool`에 대한 `Drop` 트레이트 구현하기

스레드 풀에 대해 `Drop`을 구현하는 것부터 시작해 보겠습니다. 풀이
버려지면 모든 스레드가 조인되어서 작업 완료를 보장해야 합니다.
예제 20-22는 `Drop` 구현의 첫 번째 시도를 보여줍니다; 이 코드는
아직 제대로 작동하지 않습니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/listing-20-22/src/lib.rs:here}}
```

<span class="caption">예제 20-22: 스레드 풀이 스코프 밖으로 벗어날
때 각 스레드 조인하기</span>

먼저 스레드 풀 `workers`의 각각에 대한 반복을 수행합니다. 이를 위해
`&mut`을 사용하는데, `self`는 가변 참조자고 `worker`도 변경할 수
있어야 하기 때문입니다. 각 워커에 대해서 지금의 특정한 워커가 종료된다는
메시지를 출력한 다음, 해당 워커의 스레드에 대해 `join`을 호출합니다.
`join` 호출이 실패하면 `unwrap`을 사용하여 러스트가 패닉 상태에 빠지게
하고 우아하지 않은 종료로 들어갑니다.

아래는 위 코드를 컴파일했을때 나오는 에러입니다:

```console
{{#include ../listings/ch20-web-server/listing-20-22/output.txt}}
```

이 에러는 각 `worker`의 가변 대여만 있고 `join`이 인수의 소유권을 가져가기
때문에 `join`을 호출할 수 없음을 알려줍니다. 이 문제를 해결하려면 `thread`를
소유한 `Worker` 인스턴스로부터 스레드를 밖으로 옮겨서 `join`이 스레드를
써버릴 수 있도록 할 필요가 있습니다. 예제 17-15에서 이 작업을 해봤었지요:
`Worker`가 대신 `Option<thread::JoinHandle<()>>`을 가지고 있다면,
`Option`의 `take` 메서드를 호출하여 `Some` 배리언트에서 값을 빼내고
그 자리에는 `None` 배리언트를 남길 수 있습니다. 바꿔 말하면, 실행 중인
`Worker`는 `thread`에 `Some` 배리언트를 갖도록 하고, `Worker`를 정리하고
싶을 때는 `Some`을 `None`으로 대체하여 `Worker`에 실행 스레드가 없도록
할 것입니다.

따라서 `Worker`의 정의를 다음과 같이 업데이트하고 싶습니다:

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/no-listing-04-update-worker-definition/src/lib.rs:here}}
```

이제 컴파일러를 활용하여 변경이 필요한 다른 위치를 찾아봅시다.
이 코드를 검사하면 두 가지 오류를 얻습니다:

```console
{{#include ../listings/ch20-web-server/no-listing-04-update-worker-definition/output.txt}}
```

`Worker::new` 끝에 있는 코드를 지적하는 두 번째 에러를 해결해
보겠습니다; 새 `Worker`를 생성할 때 `thread` 값을 `Some`으로
감싸야 합니다. 이 에러를 해결하려면 다음과 같이 변경하세요:

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/no-listing-05-fix-worker-new/src/lib.rs:here}}
```

첫 번째 에러는 `Drop` 구현체 안에 있습니다. 앞서 `Option` 값의 `take`를
호출하여 `thread`를 `worker` 밖으로 빼내려고 했음을 언급했습니다. 다음과
같이 변경하면 그렇게 됩니다:

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch20-web-server/no-listing-06-fix-threadpool-drop/src/lib.rs:here}}
```

17장에서 논의한 것처럼 `Option`의 `take` 메서드는 `Some` 배리언트를
제거하고 그 자리에 `None`을 남깁니다. 여기서는 `if let`을 사용해 `Some`을
해체하고 스레드를 가져옵니다; 그런 다음 그 스레드에서 `join`을 호출합니다.
워커의 스레드가 이미 `None`이면 해당 워커의 스레드가 이미 정리되었음을 알 수
있으므로, 그런 경우에는 아무 일도 일어나지 않습니다.

### 작업을 기다리는 스레드에게 정지 신호 보내기

모든 변경 사항을 적용하면 경고 없이 코드가 컴파일됩니다. 하지만 나쁜
소식이 있는데, 그것은 이 코드가 아직 우리가 원하는 방식으로 작동하지
않는다는 것입니다. 핵심은 `Worker` 인스턴스의 스레드에 의해 실행되는
클로저의 로직입니다. 현재로서는 `join`이 호출되지만 스레드가 작업을
찾기 위해 계속 `loop`를 돌기 때문에 스레드는 종료되지 않습니다. 현재
구현된 `drop`으로 `ThreadPool`을 버리려고 하면, 메인 스레드는 첫 번째
스레드가 완료될 때까지 영원히 블록됩니다.

이 문제를 해결하기 위해서 `ThreadPool` `drop` 구현을 변경한 다음
`Worker` 루프를 변경할 필요가 있겠습니다.

먼저 스레드가 완료될 때까지 기다리기 전에 명시적으로 `sender`를
버리도록 `ThreadPool` `drop` 구현을 변경하겠습니다. 예제 20-23은
명시적으로 `sender`를 버리도록 `ThreadPool`을 변경한 내용을 보여줍니다.
스레드에서 했던 것과 동일한 `Option` 및 `take` 기법을 사용하여
`ThreadPool`로부터 `sender`를 빼낼 수 있습니다:

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground,not_desired_behavior
{{#rustdoc_include ../listings/ch20-web-server/listing-20-23/src/lib.rs:here}}
```

<span class="caption">예제 20-23: 워커 스레드를 조인하기 전에
명시적으로 `sender` 버리기</span>

`sender`를 버리면 채널이 닫히며, 이는 더 이상 아무 메시지도 보내지지
않음을 나타냅니다. 이 경우 무한 루프에서 워커가 수행하는 모든 `recv`
호출은 에러를 반환할 것입니다. 예제 20-24에서는 그런 경우 `Worker`
루프가 정상적으로 종료되도록 변경하여 `ThreadPool` `drop` 구현이
`join`을 호출할 때 스레드가 종료되도록 합니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-24/src/lib.rs:here}}
```

<span class="caption">예제 20-24: `recv`가 에러를 반환한 경우
명시적으로 루프 벗어나기</span>

이 코드가 실제로 작동하는 것을 보기 위해서, 예제 20-25에 나온 것처럼 `main`을
수정하여 서버를 정상적으로 종료하기 전에 두 개의 요청만 수락하도록 해봅시다.

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/listing-20-25/src/main.rs:here}}
```

<span class="caption">예제 20-25: 두 개의 요청을 처리한 후 루프를
벗어나는 것으로 서버 종료하기</span>

실제 웹 서버가 두 개의 요청만 처리한 후 종료되는 것을 원하지는 않을
것입니다. 이 코드는 그저 정상 종료 및 정리가 정상적으로 작동하고 있음을
보여줄 따름입니다.

`take` 메서드는 `Iterator` 트레이트에 정의되어 있으며 반복을 최대 첫
두 개의 아이템으로 제한합니다. `ThreadPool`은 `main`이 끝날 때 스코프를
벗어나고, `drop` 구현이 실행될 것입니다.

`cargo run`으로 서버를 시작하고 요청을 세 번 해보세요. 세 번째 요청은
에러가 발생하고 터미널에 아래와 유사한 출력이 표시되어야 합니다:

<!-- manual-regeneration
cd listings/ch20-web-server/listing-20-25
cargo run
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
third request will error because server will have shut down
copy output below
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.0s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 3 got a job; executing.
Worker 1 disconnected; shutting down.
Worker 2 disconnected; shutting down.
Worker 3 disconnected; shutting down.
Worker 0 disconnected; shutting down.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

인쇄되는 워커와 메시지의 순서가 달라질 수 있습니다. 메시지에서
이 코드가 어떻게 작동되는지 알 수 있습니다: 워커 0과 3이 처음 두 개의
요청을 받았습니다. 서버는 두 번째 연결 이후 연결 수락을 중단했고, 워커
3이 작업을 시작하기도 전에 `ThreadPool`의 `Drop` 구현이 실행되기
시작합니다. `sender`를 버리는 것이 모든 워커의 연결을 끊고 종료되도록
지시합니다. 워커는 연결을 끊을 때 각각 메시지를 출력하고, 스레드 풀은
`join`을 호출하여 각 워커 스레드가 완료될 때까지 기다립니다.

이 특정한 실행에서 한 가지 흥미로운 측면을 주목하세요: `ThreadPool`이
`sender`를 버리고, 어떤 워커도 에러를 받기 전에 워커 0을 조인하려고
시도했습니다. 워커 0은 아직 `recv`에서 에러를 받지 않았기 때문에 메인
스레드는 워커 0이 완료될 때까지 기다리는 식으로 블록되었습니다. 그동안
워커 3이 작업을 받은 후 모든 스레드가 오류를 수신했습니다. 워커 0이
완료되면 메인 스레드는 나머지 워커가 완료될 때까지 기다렸습니다. 그 시점에서,
모든 워커가 루프를 종료하고 멈췄습니다.

축하합니다! 이제 우리의 프로젝트를 끝냈습니다; 스레드 풀을 사용하여
비동기식으로 응답하는 기본 웹 서버가 생겼습니다. 이제 서버를 우아하게
종료하여 풀의 모든 스레드를 정리할 수 있습니다.

여기 참고를 위한 전체 코드가 있습니다:

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/no-listing-07-final-code/src/main.rs}}
```

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-07-final-code/src/lib.rs}}
```

여기에 더 많은 작업을 할 수도 있습니다! 이 프로젝트를 계속 개선하고 싶으시다면,
몇 가지 아이디어를 소개합니다:

* `ThreadPool`과 그 공개 메서드에 문서를 더 추가해 보세요.
* 라이브러리 기능에 대한 테스트를 추가해 보세요.
* `unwrap` 호출을 좀 더 견고한 에러 처리 형태로 바꿔보세요.
* `ThreadPool`을 사용하여 웹 요청을 처리하는 것 말고 다른 작업을 수행해 보세요.
* [crates.io](https://crates.io/)에서 스레드 풀 크레이트를 찾아보고 이
  크레이트를 사용하여 비슷한 웹 서버를 구현해 보세요. 그런 다음 이 API와
  견고함을 우리가 구현한 스레드 풀과 비교해 보세요.

## 정리

수고하셨습니다! 이 책의 마지막까지 읽으셨어요! 이번 러스트 여정에 참여해 주신
여러분께 감사를 표합니다. 여러분은 이제 자신만의 러스트 프로젝트를 구현하고
다른 사람들의 프로젝트를 도울 준비가 되었습니다. 여러분의 러스트 여정에서
마주치는 어떠한 문제라도 기꺼이 도와줄 러스타시안 커뮤니티가 있다는 사실을
기억해 주시면 좋겠습니다.
