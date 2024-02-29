## 모듈성과 에러 처리 향상을 위한 리팩터링

프로그램을 개선하기 위해서 프로그램의 구조 및 잠재적 에러를 처리하는
방식과 관련된 네 가지 문제를 고치려고 합니다. 첫 번째로는 `main` 함수가 지금
두 가지 일을 수행한다는 것입니다: 인수 파싱, 파일을 읽는 작업 말입니다. 이 프로그램이
커짐에 따라 `main` 함수에서 처리하는 개별 작업의 개수는 증가할 것입니다.
어떤 함수가 책임 소재를 계속 늘려나가면, 이 함수는 어떤 기능인지 추론하기
어려워지고, 테스트하기도 힘들어지고, 기능 일부분을 깨트리지 않으면서 고치기도
어려워집니다. 기능을 나누어 각각의 함수가 하나의 작업에 대한 책임만 지는
것이 최선입니다.

이 주제는 두 번째 문제와도 엮입니다: `query`와 `file_path`가 프로그램의
설정 변수이지만, `contents` 같은 변수는 프로그램 로직을 수행하기 위해 사용됩니다.
`main`이 점점 길어질수록 필요한 변수들이 더 많이 스코프 안에 있게 되고,
스코프 안에 더 많은 변수가 있을수록 각 변수의 목적을 추적하는 것이
더 어려워집니다. 설정 변수들을 하나의 구조체로 묶어서 목적을 분명히
하는 것이 가장 좋습니다.

세 번째 문제는 파일 읽기 실패 시 에러 메시지 출력을 위해서 `expect`를
사용했는데, 이 에러 메시지가 겨우 `Should have been able to read the
file`이나 출력한다는 것입니다. 파일을 읽는 작업은 여러 가지 방식으로 실패할
수 있습니다: 이를테면 파일을 못 찾았거나, 파일을 열 권한이 없었다든가 하는
식이죠. 현재로서는 상황과는 관계없이 모든 에러에 대해 동일한 에러 메시지를
출력하고 있는데, 이는 사용자에게 어떠한 정보도 제공할 수 없을 것입니다!

네 번째로, `expect`가 서로 다른 에러를 처리하기 위해 반복적으로 사용되는데, 만일
사용자가 실행되기 충분한 인수를 지정하지 않고 프로그램을 실행한다면, 사용자는
러스트의 `index out of bounds` 에러를 얻게 될 것이고 이 에러는 문제를 명확하게
설명하지 못합니다. 모든 에러 처리 코드가 한 곳에 있어서 미래에 코드를 유지보수할
사람이 에러 처리 로직을 변경하기를 원할 경우 찾아봐야 하는 코드가 한 군데에만 있는
것이 가장 좋을 것입니다. 모든 에러 처리 코드를 한 곳에 모아두면 최종 사용자에게
의미 있는 메시지를 출력할 수 있습니다.

이 프로젝트를 리팩터링하여 위의 네 가지 문제를 해결해 봅시다.

### 바이너리 프로젝트에 대한 관심사 분리

여러 작업에 대한 책임을 `main` 함수에 떠넘기는 조직화 문제는 많은 바이너리
프로젝트에서 흔한 일입니다. 이에 따라 러스트 커뮤니티는 `main`이
커지기 시작할 때 이 바이너리 프로그램의 별도 관심사를 나누기 위한
가이드라인을 개발했습니다. 이 프로세스는 다음의 단계로 구성되어
있습니다:

* 프로그램을 *main.rs*와 *lib.rs*로 분리하고 프로그램 로직을
  *lib.rs*로 옮기세요.
* 커맨드 라인 파싱 로직이 작은 동안에는 *main.rs*에 남을 수
  있습니다.
* 커맨드 라인 파싱 로직이 복잡해지기 시작하면, *main.rs*로부터
  추출하여 *lib.rs*로 옮기세요.

이 과정을 거친 후 `main` 함수에 남아있는 책임소재는 다음으로
한정되어야 합니다:

* 인수 값을 가지고 커맨드 라인 파싱 로직 호출하기
* 그 밖의 설정
* *lib.rs*의 `run` 함수 호출
* `run`이 에러를 반환할 때 에러 처리하기

이 패턴은 관심사 분리에 관한 것입니다: *main.rs*는 프로그램의 실행을
다루고, *lib.rs*는 당면한 작업의 모든 로직을 처리합니다. `main` 함수를
직접 테스트할 수 없으므로, 이 구조는 *lib.rs* 내의 함수 형태로 테스트를
옮기게 하여 여러분의 모든 프로그램 로직을 테스트하게끔 합니다. *main.rs*에
남겨진 코드는 정확한지 검증할 때 읽는 것만으로도 충분할 정도로 작아질 것입니다.
이 프로세스를 따르는 것으로 프로그램 작업을 다시 해 봅시다.

#### 인수 파서 추출

커맨드 라인 파싱 로직을 *src/lib.rs*로 옮기기 위한 준비 단계로
인수를 파싱하기 위한 기능을 `main`이 호출할 함수로 추출하겠습니다.
예제 12-5는 새로 시작하는 `main`과 호출되는 새로운 함수 `parse_config`를
보여주는데, 여기서는 잠깐 *src/main.rs*에 정의하겠습니다.

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

<span class="caption">예제 12-5: `main`으로부터 `parse_config` 함수
추출</span>

여전히 커맨드 라인 인수는 벡터로 모으지만, `main` 함수 내에서
인덱스 1번의 인수 값을 `query` 변수에 할당하고 인덱스 2번의
인수 값을 `file_path` 변수에 할당하는 대신, 전체 벡터를
`parse_config` 함수에 넘깁니다. 그러면 `parse_config`
함수는 어떤 인수 값이 어떤 변수에 들어갈지 정하는 로직을
담고 있고 이 값들을 `main`에게 다시 넘겨줍니다. 여전히 `query`와
`file_path` 변수는 `main` 안에서 만들지만, `main`은 더 이상
커맨드 라인 인수와 변수들이 어떻게 대응되는지를 결정할 책임이
없습니다.

이러한 재작업은 우리의 작은 프로그램에 대해서는 지나쳐 보일지도 모르겠으나,
우리는 작게, 점진적인 단계로 리팩터링을 하는 중입니다. 이 변경 후에 프로그램을
다시 실행하여 인수 파싱이 여전히 동작하는지 검증하세요. 진행률을 자주 체크하는
것은 좋은 일이며, 문제가 발생했을 때 그 원인을 식별하는 데 도움이 됩니다.

#### 설정 값 묶기

`parse_config` 함수를 더욱 개선하기 위해 작은 단계를 하나 더 진행할 수
있습니다. 현재는 튜플을 반환하는 중인데, 그런 다음 이 튜플을 개별 부분으로
즉시 다시 쪼개고 있습니다. 이는 아직 적절한 추상화가 이루어지지 않았다는
신호일 수 있습니다.

개선의 여지가 남아있음을 보여주는 또 다른 지표는 `parse_config`의
`config` 부분인데, 이는 반환하는 두 값이 연관되어 있고 둘 모두
하나의 설정 값을 이루는 부분임을 의미합니다. 현재 두 값을 튜플로
묶는 것 말고는 데이터의 구조에서 이러한 의미를 전달하지 못하고
있습니다; 그래서 이 두 값을 하나의 구조체에 넣고 구조체 필드에 각각 의미가
있는 이름을 부여하려고 합니다. 그렇게 하는 것이 미래에 이 코드를 유지보수하는
사람에게 이 서로 다른 값들이 어떻게 연관되어 있고 이 값들의 목적은 무엇인지를
더 쉽게 이해하도록 만들어 줄 것입니다.

예제 12-6은 `parse_config` 함수에 대한 개선을 보여줍니다.

<span class="filename">파일명: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

<span class="caption">예제 12-6: `Config` 구조체의 인스턴스를 반환하도록
하는 `parse_config` 리팩터링</span>

`query`와 `file_path`라는 이름의 필드를 갖도록 정의된 `Config`라는 이름의
구조체를 추가했습니다. `parse_config`의 시그니처는 이제 `Config` 값을
반환함을 나타냅니다. `parse_config` 본문에서는 원래 `args`의 `String` 값들을
참조하는 문자열 슬라이스를 반환했는데, 이제는 `String` 값을 소유한 `Config`를
정의했습니다. `main` 안에 있는 `args` 변수는 인수 값들의 소유자이고
`parse_config` 함수에게는 이 값을 빌려주고 있을 뿐인데, 이는 즉
`Config`가 `args`의 값에 대한 소유권을 가져가려고 하면 러스트의 대여
규칙을 위반하게 된다는 의미입니다.

`String` 데이터를 관리하는 방법은 다양하며, 가장 쉬운 방법은 (다소 비효율적이지만)
그 값에서 `clone` 메서드를 호출하는 것입니다. 이는 데이터의 전체
복사본을 만들어 `Config` 인스턴스가 소유할 수 있게 해주는데, 이는
문자열 데이터에 대한 참조자를 저장하는 것에 비해 더 많은 시간과 메모리를 소비합니다.
그러나 값의 복제는 참조자의 라이프타임을 관리할 필요가 없어지기 때문에
코드를 매우 직관적으로 만들어 주기도 하므로, 이러한 환경에서
약간의 성능을 포기하고 단순함을 얻는 것은 가치 있는 절충안입니다.

> ### `clone`을 사용한 절충안
>
> 러스타시안들 중에서 많은 이들이 런타임 비용의 이유로 `clone`을 사용한
> 소유권 문제 해결을 회피하는 경향을 가지고 있습니다. [13장][ch13]<!-- ignore -->에서
> 이러한 종류의 상황에서 더 효율적인 메서드를 사용하는 방법을 배울 것입니다.
> 하지만 프로젝트를 계속 진행하기 위해 지금으로서는 약간의 문자열을 복사하는
> 정도는 괜찮은데, 이 복사가 딱 한 번만 일어나고 파일 경로와 질의
> 문자열이 매우 작기 때문입니다. 한 번에 매우 최적화된 코드 작성을
> 시도하기보다는 다소 비효율적이라도 동작하는 프로그램을 만드는 편이
> 좋습니다. 여러분이 러스트에 더 경험을 쌓게 되면 가장 효율적인
> 해답을 가지고 시작하기 더 쉽겠으나, 지금으로선 `clone`을 호출하는
> 것도 충분히 허용될만 합니다.

`main`을 업데이트하여 `parse_config`가 반환한 `Config` 인스턴스를
`config`라는 이름의 변수에 위치시켰고, 이전에 별개로 사용된
`query`와 `file_path` 대신 이제는 `Config` 구조체의 필드를
이용합니다.

이제 코드가 `query`와 `file_path`가 서로 연관되어 있고 이들의 목적이
프로그램의 동작 방법을 설정하기 위함임을 더 명료하게 전달합니다. 이러한 값을
사용하는 모든 코드는 `config` 인스턴스에서 목적에 맞게 명명된 필드 값을
찾을 수 있습니다.

#### `Config`를 위한 생성자 만들기

여기까지 해서 `main`으로부터 커맨드 라인 인수 파싱을 책임지는 로직을
추출하여 `parse_config` 함수에 위치시켰습니다. 그렇게 하면
`query`와 `file_path` 값이 연관되어 있고 이 관계가 코드로부터
전달된다는 것을 알기 쉽게 해주었습니다. 그다음 `query`와 `file_path`의
목적에 연관된 이름을 갖고 `parse_config` 함수로부터 반환되는 값을
구조체 필드 값이 되도록 하기 위해 `Config` 구조체를 추가하였습니다.

따라서 이제 `parse_config` 함수의 목적이 `Config` 인스턴스를 생성하는
것이 되었으므로, `parse_config`를 일반 함수에서 `Config` 구조체와 연관된
`new`라는 이름의 함수로 바꿀 수 있겠습니다. 이러한 변경이 코드를 더 자연스럽게
만들어 줄 것입니다. `String` 같은 표준 라이브러리 타입의 인스턴스 생성은
`String::new`를 호출하는 것으로 할 수 있습니다. 비슷하게 `parse_config`를
`Config`와 연관된 함수 `new`로 변경함으로써 `Config` 인스턴스의 생성을
`Config::new`의 호출로 할 수 있을 것입니다. 예제 12-7은 이를 위한
변경점을 보여줍니다.

<span class="filename">파일명: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

<span class="caption">예제 12-7: `parse_config`를 `Config::new`로
변경하기</span>

원래 `parse_config`를 호출하고 있던 `main` 부분을 `Config::new` 호출로
바꿨습니다. `parse_config`의 이름은 `new`로 변경되었고 `impl` 블록에
옮겨졌는데, 이것이 `Config`와 `new` 함수를 연관시켜 줍니다.
이 코드를 다시 한번 컴파일하여 잘 동작하는지 확인하세요.

### 에러 처리 수정

이제부터는 에러 처리 기능을 수정할 겁니다. `args` 벡터에 3개보다 적은 아이템이
들어있는 경우에는 인덱스 1이나 2의 값에 접근을 시도하는 것이 프로그램의
패닉을 일으킬 것이라는 점을 상기합시다. 아무런 인수 없이 프로그램을 실행해
보세요; 아래처럼 나올 것입니다:

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

`index out of bounds: the len is 1 but the index is 1` 줄은 프로그래머를
위한 에러 메시지입니다. 최종 사용자들에게는 무엇을 대신 해야 하는지
이해시키는 데 도움이 안 될 것입니다. 이제 수정해 봅시다.

#### 에러 메시지 개선

예제 12-8에서는 인덱스 1과 2에 접근하기 전에 슬라이스의 길이가 충분한지
검증하는 기능을 `new` 함수에 추가했습니다. 만일 슬라이스가 충분히 길지
않다면, 프로그램은 패닉을 일으키고 더 나은 에러 메시지를 보여줍니다.

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

<span class="caption">예제 12-8: 인수의 개수 검사
추가</span>

이 코드는 [예제 9-13에서 작성했었던 `Guess::new` 함수][ch9-custom-types]<!-- ignore -->와
비슷한데, 거기서는 `value` 인수가 유효한 값의 범위 밖인 경우
`panic!`을 호출했었지요. 여기서는 값의 범위를 검사하는 대신,
`args`의 길이가 최소 3이고 이 조건을 만족하는 가정 아래에서
함수의 나머지 부분이 동작할 수 있음을 검사하고 있습니다.
만일 `args`가 아이템을 세 개보다 적게 가지고 있다면 이 조건은
참이 되고, `panic!` 매크로를 호출하여 프로그램을 즉시 종료시킵니다.

`new`에 이렇게 몇 줄을 추가한 다음, 다시 한번 아무 인수 없이 프로그램을
실행하여 이제 에러가 어떤 식으로 보이는지 살펴봅시다:

```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

이번 출력이 더 좋습니다: 이제는 적절한 에러 메시지가 되었습니다. 하지만
사용자들에게 제공할 필요 없는 추가적인 정보도 제공하고 있습니다. 어쩌면
예제 9-13에서 사용했던 기술을 여기에 써먹는 것이 최선은 아닌가 봅니다:
[9장에서 얘기한 것처럼][ch9-error-guidelines]<!-- ignore --> `panic!`을
호출하는 것은 사용의 문제보다는 프로그램의 문제에 더 적합합니다. 대신에
여러분이 9장에서 배웠던 다른 기술, 즉 성공인지 혹은 에러인지를 나타내는
[`Result`를 반환하는][ch9-result]<!-- ignore --> 기술을 사용해 보겠습니다.

<!-- Old headings. Do not remove or links may break. -->
<a id="returning-a-result-from-new-instead-of-calling-panic"></a>

#### `panic!` 호출 대신 `Result` 반환하기

성공한 경우에는 `Config`를 담고 있고 에러가 난 경우에는 문제를
설명해줄 `Result` 값을 반환시킬 수 있습니다. 또한 `new`라는
함수 이름은 `build`로 변경할 것인데, 이는 많은 프로그래머가 `new`
함수가 절대 실패하지 않으리라 예상하기 때문입니다. `Config::build`가
`main`과 소통하고 있을 때 `Result` 타입을 사용하여 문제에 대한 신호를
줄 수 있습니다. 그러면 `main`을 수정하여 `Err` 배리언트를 사용자에게
더 실용적인 에러 메시지로 변경할 수 있고, 이는 `panic!`의 호출로 인한
`thread 'main'`과 `RUST_BACKTRACE`에 대해 감싸져 있는 텍스트를 없앨 수 있겠습니다.

예제 12-9는 이제 `Config::build`라고 하는 함수의 반환 값과
`Result`를 반환할 필요가 있는 함수 본문을 위해서 필요한 변경점을
보여줍니다. `main`도 마찬가지로 수정하지 않으면 컴파일 되지 않는다는
점을 유의하세요. 이건 다음에 하겠습니다.

<span class="filename">파일명: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

<span class="caption">예제 12-9: `Config::build`로부터
`Result` 반환하기</span>

우리의 `build` 함수는 성공한 경우 `Config`를, 에러가 난 경우 `&'static str`을
갖는 `Result`를 반환합니다. 에러 값은 언제나 `'static` 라이프타임을 갖는
문자열 리터럴일 것입니다.

함수 본문에는 두 가지 변경점이 있었습니다: 사용자가 충분한 인수를 넘기지
않았을 때 `panic!`을 호출하는 대신 이제 `Err` 값을 반환하며, 반환 값
`Config`를 `Ok`로 감쌌습니다. 이러한 변경점이 함수의 새로운 타입 시그니처에
맞도록 합니다.

`Config::build`로부터 `Err` 값을 반환하는 것은 `main` 함수가
`build` 함수로부터 반환된 `Result` 값을 처리하여 에러가 난 경우
프로세스를 더 깔끔하게 종료하도록 해줍니다. 

<!-- Old headings. Do not remove or links may break. -->
<a id="calling-confignew-and-handling-errors"></a>

#### `Config::build` 호출과 에러 처리

에러가 발생한 경우를 처리하여 사용자 친화적인 메시지를 출력하기 위해서는,
예제 12-10처럼 `main`을 수정하여 `Config::build`에 의해 반환되는
`Result`를 처리할 필요가 있습니다. 또한 `panic!`으로부터 벗어나서
직접 0이 아닌 에러 코드로 커맨드 라인 도구를 종료하도록 구현할 것입니다.
0이 아닌 종료 상태값은 프로그램을 호출한 프로세스에게 에러 상태값과
함께 종료되었음을 알려주는 관례입니다.

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

<span class="caption">예제 12-10: `Config` 생성이 실패했을 경우 에러
코드와 함께 종료하기</span>

위의 코드에서는 아직 상세히 다루지 않은 `unwrap_or_else` 메서드를
사용했는데, 이는 표준 라이브러리의 `Result<T, E>`에 구현되어
있습니다. `unwrap_or_else`을 사용하면 커스터마이징된 `panic!`이 아닌 에러 처리를
정의할 수 있습니다. 만일 `Result`가 `Ok` 값이라면 이 메서드의 동작은 `unwrap`과
유사합니다: 즉 `Ok`가 감싸고 있는 안쪽 값을 반환합니다. 하지만 값이 `Err`
값이라면, 이 메서드는 *클로저 (closure)* 안의 코드를 호출하는데, 이는
`unwrap_or_else`의 인수로 넘겨준 우리가 정의한 익명 함수입니다.
클로저에 대해서는 [13장][ch13]<!-- ignore -->에서 더 자세히 다루겠습니다.
지금은 그저 `unwrap_or_else`가 `Err`의 내부 값을 클로저의 세로 파이프 (|)
사이에 있는 `err` 인수로 넘겨주는데, 이번 경우 그 값은 예제 12-9에
추가한 정적 문자열 `"not enough arguments"`이라는 정도만 알면
됩니다. 그러면 실행했을 때 클로저 내의 코드가 `err` 값을 사용할 수
있게 됩니다.

새로 추가된 `use` 줄은 표준 라이브러리로부터 `process`를 스코프 안으로
가져옵니다. 에러가 난 경우 실행될 클로저 내의 코드는 딱 두 줄입니다:
`err` 값을 출력한 다음 `process::exit`를 호출하는 것이지요.
`process::exit` 함수는 프로그램을 즉시 멈추고 넘겨진 숫자를
종료 상태 코드로서 반환하게 될 것입니다. 이는 예제 12-8에서
사용했던 `panic!` 기반의 처리와 비슷하지만, 이제는 추가 출력문들이
사라지게 됩니다. 한번 시도해 봅시다:

```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

훌륭하군요! 이 출력문이 사용자들에게 훨씬 친숙합니다.

### `main`으로부터 로직 추출하기

이제 설정 값 파싱의 리팩터링을 끝냈으니, 프로그램 로직으로 돌아와 봅시다.
[‘바이너리 프로젝트에 대한 관심사 분리’](#separation-of-concerns-for-binary-projects)<!-- ignore -->절에서
기술한 바와 같이, 현재 `main` 함수에 있는 로직 중
설정 값이나 에러 처리와는 관련되지 않은 모든 로직을 `run`이라는
함수로 추출하도록 하겠습니다. 그렇게 하고 나면 `main`은
간결하고 검사하기 쉬워질 것이며, 나머지 모든 로직에 대한
테스트를 작성할 수 있게 될 것입니다.

예제 12-11은 추출된 `run` 함수를 보여줍니다. 지금은 함수 추출에
대한 작고 점진적인 개선만 하고 있습니다. 여전히 함수는 *src/main.rs*에
정의되어 있습니다.

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

<span class="caption">예제 12-11: 나머지 프로그램 로직을 담는
`run` 함수 추출</span>

`run` 함수는 이제 이는 파일을 읽는 부분부터 시작되는, `main`으로부터
남은 모든 로직을 담고 있습니다. `run` 함수는 `Config` 인스턴스를 인수로
취합니다.

#### `run` 함수로부터 에러 반환하기

`run` 함수로 분리된 남은 프로그램 로직에 대하여, 예제 12-9에서
`Config::build`에 했던 것처럼 에러 처리 기능을 개선할 수 있습니다.
`run` 함수는 뭔가 잘못되면 `expect`를 호출하여 프로그램이 패닉이
되도록 하는 대신 `Result<T, E>`를 반환할 것입니다. 이를 통해 에러 처리에
관한 로직을 사용자 친화적인 방식으로 `main` 안에 더욱 통합시킬 수
있습니다. 예제 12-12는 `run`의 시그니처와 본문에 필요한 변경점을
보여줍니다.

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

<span class="caption">예제 12-12: `run` 함수가 `Result`를 반환하도록
변경하기</span>

여기서는 세 가지 중요한 변경점이 있습니다. 첫 번째로, `run` 함수의 반환
타입이 `Result<(), Box<dyn Error>>`으로 변경되었습니다. 이 함수는 원래
유닛 타입 `()`를 반환했었는데, `Ok`인 경우에 반환될 값으로써 계속 유지하고
있습니다.

에러 타입에 대해서는 *트레이트 객체* `Box<dyn Error>`를 사용했습니다 (그리고
상단에 `use` 구문을 사용하여 `std::error::Error`를 스코프로 가져 왔습니다).
트레이트 객체에 대해서는 [17장][ch17]<!-- ignore -->에서 다룰 예정입니다. 지금은
그저 `Box<dyn Error>`는 이 함수가 `Error` 트레이트를 구현한 어떤 타입을 반환하는데,
그 반환 값이 구체적으로 어떤 타입인지는 특정하지 않아도 된다는 것을 의미한다는 정도만
알면 됩니다. 이는 서로 다른 에러의 경우에서 서로 다른 타입이 될지도 모를
에러값을 반환하는 유연성을 제공합니다. `dyn` 키워드는 ‘동적 (dynamic)’의
줄임말입니다.

두 번째로 [9장][ch9-question-mark]<!-- ignore -->에서 이야기했던
`?` 연산자를 활용하여 `expect`의 호출을 제거했습니다. `?`은 에러
상황에서 `panic!` 대신 호출하는 쪽이 처리할 수 있도록 현재의 함수로부터
에러 값을 반환할 것입니다.

세 번째로 `run` 함수는 이제부터 성공한 경우 `Ok` 값을 반환합니다.
`run` 함수의 성공 타입은 시그니처 상에서 `()`로 선언되었는데,
이는 유닛 타입 값을 `Ok` 값으로 감쌀 필요가 있다는 의미입니다.
이 `Ok(())` 문법은 처음엔 좀 이상해 보일런지도 모릅니다만, 이렇게 `()`를
사용하는 것은 `run`을 호출하여 부작용에 대해서만 처리하겠다는 것을
가리키는 자연스러운 방식입니다; 즉 반환 값이 필요 없는 경우입니다.

이 코드를 실행시키면, 컴파일은 되지만 다음과 같은 경고가 나타날 것입니다:

```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

러스트가 우리에게 `Result` 값이 무시되고 있으며 `Result` 값이 에러가
발생했음을 나타낼지도 모른다고 알려주는군요. 그렇지만 에러가 있는지 없는지
알아보는 검사를 하지 않고 있고, 그래서 어떤 에러 처리 코드를 의도했었던
것은 아닌지를 상기시켜 줍니다! 이제 이 문제를 바로잡아 봅시다.

#### `main`에서 `run`으로부터 반환된 에러 처리하기

이제 예제 12-10의 `Config::build`에 사용했던 것과 비슷한 기술을 사용하여
에러를 검사하고 이를 처리해 볼 것인데, 약간 다른 점이 있습니다:

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

`run`이 `Err` 값을 반환했는지 검사하고 만일 그렇다면 `process::exit(1)`를
호출하기 위해 사용한 `unwrap_or_else` 대신 `if let`이 사용되었습니다. `run`
함수가 반환한 값은 `Config` 인스턴스를 반환하는 `Config::build`과 동일한
방식대로 `unwrap`을 하지 않아도 됩니다. `run`이 성공한 경우 `()`를 반환하기
때문에 에러를 찾는 것만 신경 쓰면 되므로, 고작 `()`나 들어있을 값을
반환하기 위해 `unwrap_or_else`를 쓸 필요는 없어집니다.

`if let`과 `unwrap_or_else` 함수의 본문은 둘 모두 동일합니다:
즉, 에러를 출력하고 종료합니다.

### 라이브러리 크레이트로 코드 쪼개기

여기까지의 `minigrep` 프로젝트는 괜찮아 보이는군요! 이제 *src/main.rs*
파일을 쪼개어 코드 일부를 *src/lib.rs* 파일에 넣을 것입니다. 그렇게 하여
코드를 테스트할 수 있고 *src/main.rs* 파일의 책임 소재를 더 적게 할 수 있습니다.

`main` 함수가 아닌 모든 코드를 *src/main.rs*에서 *src/lib.rs*로
옮깁시다:

* `run` 함수 정의 부분
* 이와 관련된 `use` 구문들
* `Config` 정의 부분
* `Config::build` 함수 정의 부분

*src/lib.rs*의 내용은 예제 12-13과 같은 시그니처를 가지고 있어야 합니다.
(간결성을 위해 함수의 본문은 생략하였습니다.) *src/main.rs*를 예제 12-14처럼
수정하기 전까지는 컴파일이 되지 않음을 유의하세요.

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs:here}}
```

<span class="caption">예제 12-13: `Config`와 `run`을 *src/lib.rs*
안으로 옮기기</span>

`pub` 키워드를 자유롭게 사용했습니다: `Config`와 이 구조체의 각 필드 및 `build`
메서드, 그리고 `run` 함수에 대해 사용했지요. 이제 우리는 테스트해 볼 수 있는
공개 API를 갖춘 라이브러리 크레이트를 가지게 되었습니다!

이제는 예제 12-14처럼 *src/lib.rs*로 옮겨진 코드를 *src/main.rs* 내의
바이너리 크레이트 스코프 쪽으로 가져올 필요가 생겼습니다.

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

<span class="caption">예제 12-14: *src/main.rs*에서 `minigrep` 라이브러리
크레이트 사용하기</span>

`use minigrep::Config` 줄을 추가하여 라이브러리 크레이트로부터
바이너리 크레이트 스코프로 `Config` 타입을 가져오고, `run` 함수 앞에는
크레이트 이름을 붙였습니다. 이제 모든 기능이 연결되어 동작해야 합니다.
`cargo run`으로 프로그램을 실행하여 모든 것이 정상적으로 동작하는지
확인하세요.

휴우! 작업이 참 많았습니다만, 우리는 미래의 성공을 위한 기반을 닦았습니다.
이제 에러를 처리하기도 훨씬 쉽고, 코드도 훨씬 모듈화되었습니다. 이제부터는
거의 모든 작업이 *src/lib.rs* 내에서 완료될 것입니다.

이전 코드에서는 어려웠지만 새 코드에서는 쉬운 작업을
수행하여 이 새로운 모듈성의 이점을 활용해 봅시다: 테스트를
작성해 보겠습니다!

[ch13]: ch13-00-functional-features.html
[ch9-custom-types]: ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling
[ch9-result]: ch09-02-recoverable-errors-with-result.html
[ch17]: ch17-00-oop.html
[ch9-question-mark]: ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
