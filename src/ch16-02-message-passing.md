## 메세지 패싱을 사용하여 스레드 간 데이터 전송하기

안전한 동시성을 보장하기 위해 인기가 오르고 있는 접근법 중에는 *메세지 패싱 (message passing)*
이 있는데, 이는 스레드들 혹은 액터들이 서로 데이터를 담은 메세지를 보내서
통신하는 것입니다. [Go 언어 문서](https://golang.org/doc/effective_go.html#concurrency)
의 슬로건에 있는 아이디어는 다음과 같습니다: “메모리를 공유하여 통신하지
마세요; 그대신, 통신하여 메모리를 공유하세요.”

메세지 보내기 동시성을 달성하기 위해서 러스트 표준 라이브러리는 *채널 (channel)*
구현체를 제공합니다. 채널은 한 스레드에서 다른쪽으로 데이터를 보내기 위한
일반적인 프로그래밍 개넘입니다.

프로그래밍에서의 채널은 개울이나 강 같은 물의 방향성 채널과
비슷하다고 상상할 수 있겠습니다. 강에 고무 오리 같은 것을
넣으면, 물길의 끝까지 하류로 여행하게 되겠지요.

채널은 둘로 나뉘어져 있습니다: 바로 송신자(transmitter)와 수신자(receiver)입니다.
송신자 측은 여러분이 강에 고무 오리를 띄우는 상류 위치이고, 수신자
측은 하류에 고무 오리가 도달하는 곳입니다. 여러분의 코드 중 한 곳에서
보내고자 하는 데이터와 함꼐 송신자의 메서드를 호출하면, 다른 곳에서는
도달한 메세지에 대한 수신 종료를 검사합니다. 송신자 혹은 송신자가
버려지면 채널이 *닫혔다 (closed)* 라고 말합니다.

여기서는 값을 생성하여 채널로 내려보내는 한 스레드와, 값을 받아서
이를 출력하는 또다른 스레드로 이루어진 프로그램을 만들어 보겠습니다.
기능을 설명하기 위해서 채널을 사용하여 스레드 간에 단순한 값들을
보내려고 합니다. 여러분이 이 기술에 익숙해지고 나면, 채팅 시스템이나
다수의 스레드가 계산의 일부분을 수행하여 결과를 종합하는 스레드에
이를 보내는 시스템과 같이 서로 통신이 필요한 스레드에 채널을
이용할 수 있습니다.

먼저 Listing 16-6에서는 채널을 만들지만 이걸로 아무것도 하지 않을 것입니다.
채널을 통해 보내려는 값의 타입이 무엇인지 러스트가 알지 못하므로 이 코드가
아직 컴파일되지 않는다는 점을 주의하세요.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-06/src/main.rs}}
```

<span class="caption">Listing 16-6: 채널을 생성하여 각 절반을
`tx`와 `rx`에 할당하기</span>

`mpsc::channel` 함수를 사용하여 새로운 채널을 생성합니다; `mpsc`는
*복수 생산자, 단수 소비자 (multiple producer, single consumer)* 를
나타냅니다. 짧게 줄이면, 러스트의 표준 라이브러리가 채널을 구현한 방법은
한 채널이 값을 생산하는 *송신* 단말을 여러 개 가질 수 있지만 값을 소비하는
*수신* 단말은 단 하나만 가질 수 있음을 의미합니다. 하나의 큰 강으로 함께
흐르는 여러 개울들을 상상해 보세요: 아무 개울에나 흘려보낸 모든 것은 끝내 하나의
강에서 끝날 것입니다. 지금은 단일 생산자를 가지고 시작하겠지만, 이 예제가 동작하기
시작하면 여러 생산자를 추가할 것입니다.

`mpsc::channel` 함수는 튜플을 반환하는데, 첫번째 요소는 송신
단말이고 두번째 요소는 수신 단말입니다. `tx`와 `rx`라는 약어는
많은 분야에서 각각 *송신자 (transmitter)* 와 *수신자 (receiver)* 에
사용되므로, 각각의 단말을 나타내기 위해 그렇게 변수명을 지었습니다.
튜플을 해체하는 패턴과 함께 `let` 구문이 사용되고 있습니다;
`let` 구문 내에서의 패턴의 사용과 해체에 대해서는 18장에서 다룰
것입니다. 지금은 이런 방식으로 `let` 구문을 사용하는 것이
`mpsc::channel`에 의해 반환된 튜플의 조각들을 추출하는데 편리한
접근법이라고만 알아둡시다.

Listing 16-7과 같이 송신 단말을 생성된 스레드로 이동시키고 하나의
문자열을 전송하게 하여 생성된 스레드가 메인 스레드와 통신하도록
해봅시다. 이는 강 상류에 고무 오리를 띄우는 것 혹은 한 스레드에서 다른
스레드로 채팅 메세지를 보내는 것과 비슷합니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-07/src/main.rs}}
```

<span class="caption">Listing 16-7: `tx`를 생성된 스레드로 이동시키고
“hi”를 보내기</span>

다시 한번 `thread::spawn`을 이용하여 새로운 스레드를 생성한 뒤 `move`를
사용하여 `tx`를 클로저로 이동시켜 생성된 스레드가 `tx`를 소유하도록 합니다.
생성된 스레드는 채널을 통해 메세지를 보낼 수 있도록 하기 위해 채널의 송신 단말을
소유할 필요가 있습니다. 송신 단말에는 보내고 싶은 값을 받는 `send` 메서드가
있습니다. `send` 메서드는 `Result<T, E>` 타입을 반환하므로, 수신 단말이
이미 버려져 값을 보낼 곳이 없다면, 송신 연산은 에러를 반환할 것입니다.
이 예제에서는 에러가 나는 경우 패닉을 일으키기 위해 `unwrap`을 호출하는
중입니다. 그러나 실제 애플리케이션에서는 이를 적절히 다뤄야 할 것입니다:
적절한 에러 처리를 위한 전략을 다시 보려면 9장으로 돌아가세요.

Listing 16-8에서는 메인 스레드에 있는 채널의 수신 단말로부터 값을
받을 것입니다. 이는 강의 끝물에서 고무 오리를 건져올리는 것 혹은 채팅
메세지를 받는 것과 비슷합니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-08/src/main.rs}}
```

<span class="caption">Listing 16-8: 메인 스레드에서 “hi” 값을
받아 출력하기</span>

수신자는 두 개의 유용한 메서드를 가지고 있습니다: `recv`와 `try_recv`입니다. 여기서는
*수신 (receive)* 의 줄임말인 `recv`를 사용하고 있는데, 이는 메인 스레드의 실행을
블록시키고 채널로부터 값을 받을 때까지 기다릴 것입니다. 일단 값을 받으면, `recv`는
이것을 `Result<T, E>`로 반환할 것입니다. 채널의 송신 단말이 닫히면, `recv`는 더
이상 어떤 값도 오지 않을 것이란 신호를 주기 위해 에러를 반환할 것입니다.

`try_recv` 메서드는 블록하지 않는 대신 즉시 `Result<T, E>`를
반환할 것입니다: 전달 받은 메세지가 있다면 이를 담고 있는 `Ok` 값을,
지금 시점에서 메세지가 없다면 `Err` 값을 반환합니다. `try_recv`의
사용은 메세지를 기다리는 동안 다른 작업을 해야 할 때 유용합니다:
`try_recv`을 매번 호출하는 루프를 작성하여 메세지가 있으면 이를
처리하고, 그렇지 않으면 다음번 검사때까지 잠시 다른 일을 할 수
있습니다.

이 예제에서는 간소화를 위해 `recv`를 이용했습니다; 이 메인 스레드에서는
메세지를 기다리는 동안 해야 할 다른 일이 없으므로, 메인 스레드를 블록시키는
것이 적절합니다.

Listing 16-8의 코드를 실행하면, 메인 스레드로부터 출력된 값을 보게
될 것입니다:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
```

완벽하군요!

### 채널과 소유권 이전

소유권 규칙은 메세지 전송에서 안전하면서 동시적인 코드를 작성하는데
중요한 역할을 합니다. 동시성 프로그래밍 내에서의 에러 방지는 러스트
프로그램 전체에서 소유권을 고려할 경우 얻을 수 있는 이점입니다. 실험을
통해 채널과 소유권이 함께 동작하는 것이 어떤 식으로 문제를 방지하는지
알아봅시다: 채널로 `val` 값을 보낸 *이후에* 생성된 스레드에서 이 값을
사용하는 시도를 해보겠습니다. Listing 16-9의 코드를 컴파일하여 이 코드가
왜 허용되지 않는지를 보세요:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-09/src/main.rs}}
```

<span class="caption">Listing 16-9: `val`을 채널로 보낸뒤
이에 대한 사용 시도</span>

여기서는 `tx.send`를 통하여 채널에 `val`을 보낸뒤 이를 출력하는 시도를
하였습니다. 이를 허용하는 것은 나쁜 생각입니다: 일단 값이 다른 스레드로 보내지고
나면, 그 값을 다시 사용하려고 하기 전에 값을 받은 스레드에서 수정되거나
버려질 수 있습니다. 잠재적으로, 다른 스레드에서의 수정은 불일치하거나 존재하지 않는
데이터로 인하여 에러 혹은 예상치 못한 결과를 야기할 수 있습니다. 하지만 러스트에서는
Listing 16-9 코드의 컴파일 시도를 하면 에러가 납니다:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-09/output.txt}}
```

우리의 동시성에 관한 실수가 컴파일 타임 에러를 만들었습니다. `send` 함수가
그 매개변수의 소유권을 가져가고, 이 값이 이동되면, 수신자가 이에 대한
소유권을 얻습니다. 이는 값을 보낸 이후에 우발적으로 이 값을 다시 사용하는
것을 방지합니다; 소유권 시스템은 모든 것이 정상인지 확인합니다.

### 여러 값 보내기와 수신자가 기다리는지 알아보기

Listing 16-8의 코드는 컴파일되고 실행도 되지만, 두개의 분리된 스레드가
채널을 통해 서로 대화를 했는지를 우리에게 명확히 보여주진 못했습니다.
Listing 16-10에서는 Listing 16-8의 코드가 동시에 실행됨을 입증해 줄
수정본을 만들었습니다: 이제 생성된 스레드가 여러 메세지를 보내면서
각 메세지 사이에 1초씩 잠깐 멈출 것입니다.

<span class="filename">Filename: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-10/src/main.rs}}
```

<span class="caption">Listing 16-10: 여러 메세지를 보내고
각각마다 멈추기</span>

이번에는 생성된 스레드가 메인 스레드로 보내고 싶은 문자열의 벡터를 
가지고 있습니다. 문자열마다 반복하여 각각의 값을 개별적으로 보내고,
`Duration` 값에 1초를 넣어서 `thread::sleep` 함수를 호출하는 것으로
각각의 사이에 멈춥니다.

메인 스레드에서는 더 이상 `recv` 함수를 명시적으로 호출하지 않고 있습니다:
대신 `rx`를 반복자처럼 다루고 있습니다. 각각의 수신된 값에 대해서 이를
출력합니다. 채널이 닫힐 때는 반복이 종료될 것입니다.

Listing 16-10의 코드를 실행시키면 다음과 같은 출력이 각 줄마다 1초씩
멈추면서 보일 것입니다:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
Got: from
Got: the
Got: thread
```

메인 스레드의 `for` 루프 내에는 어떠한 멈춤 혹은 지연 코드를 넣지 않았으므로,
메인 스레드가 생성된 스레드로부터 값을 전달받는 것을 기다리는 중임을 알 수
있습니다.

### 송신자를 복제하여 여러 생산자 만들기

이전에 `mpsc`가 *복수 생산자, 단일 소비자 (multiple producer, single consumer)*
의 약어라는 것을 언급했었지요. `mpsc`를 Listing 16-10의 코드에 적용하여 모두 동일한
수신자로 값들을 보내는 여러 스레드들을 만들도록 코드를 확장해봅시다. Listing 16-11처럼
채널의 송신자를 복제하면 그렇게 할 수 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-11/src/main.rs:here}}
```

<span class="caption">Listing 16-11: 여러 개의 생산자로부터 여러 메세지
보내기</span>

이번에는 첫번째로 생성된 스레드를 생성하기 전에, 채널의 송신 단말에 대해 `clone`을
호출했습니다. 이는 첫번째로 생성된 스레드로 넘길 수 있는 새로운 송신 핸들을
제공해줄 것입니다. 두번째로 생성된 스레드에게는 원래의 채널 송신 단말을 넘깁니다.
이렇게 다른 메세지를 하나의 수신 단말로 보내주는 두 스레드를 만듭니다.

이 코드를 실행시키면, 다음과 같은 출력과 비슷하게 보여야 합니다:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

값들의 순서는 여러분의 시스템에 따라 다르게 보일 수도 있습니다. 이것이 바로
동시성을 흥미롭게 만들 뿐만 아니라 어렵게 만드는 것입니다. 만일 여러분이
`thread::sleep`을 가지고 실험하면서 서로 다른 스레드마다 다양한 값을 썼다면,
매번의 실행이 더욱 비결정적이고 매번 다른 출력을 생성할 것입니다.

이제 채널이 동작하는 방식을 알아봤으니, 동시성의 다른 방법을
알아봅시다.
