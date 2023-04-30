## 테스트 주도 개발로 라이브러리 기능 개발하기

로직을 *src/lib.rs*로 추출하고 인자 모으기와 에러 처리는
*src/main.rs*에 남겨두었으니, 이제는 우리 코드의 핵심 기능에 대한
테스트를 작성하기 무척 쉽습니다. 커맨드 라인에서 바이너리를 호출할
필요 없이 다양한 인자값을 가지고 함수를 직접 호출하여 반환값들을
검사해볼 수 있습니다.

이 절에서는 아래의 단계를 따르는 테스트 주도 개발 (Test-Driven Development, TDD)
프로레스를 사용하여 `minigrep` 프로그램의 검색 로직을 추가해보도록 하겠습니다:

1. 실패하는 테스트를 착성하고, 실행하여 여러분이 예상한 이유대로 실패하는지
   확인합니다.
2. 이 새로운 테스트를 통과하기 충분한 정도의 코드만 작성하거나 수정하세요. 
3. 추가하거나 변경한 코드를 리팩토링하고 테스트가 계속 통과하는지
   확인하세요.
4. 1단계로 돌아가세요!

그저 소프트웨어 작성의 수많은 방식중 하나일 뿐이지만, TDD는 코드 설계를 주도하는데
도움이 됩니다. 테스트를 통과하도록 해줄 코드를 작성하기 전에 테스트 먼저 작성하는
것은 프로세스 전체에 걸처 높은 테스트 범위를 유지하는데 도움을 줍니다.

우리는 질의 문자열을 파일 내용물 안에서 실제로 찾아보고 질의에
맞는 라인의 리스트를 생성하는 기능의 구현을 테스트 주도적으로
해볼 것입니다. 이 기능을 `search`라는 이름의 함수에
추가해보겠습니다.

### 실패하는 테스트 작성하기

프로그램 동작을 확인하기 위해 사용되었던 *src/lib.rs*와 *src/main.rs*의
`println!` 구문들은 이제 더이상 필요가 없으므로 제거합시다.
그런 다음 [11장][ch11-anatomy]<!-- ignore -->에서 그랬던 것처럼 *src/lib.rs*에
`test` 모듈과 함께 테스트 함수를 추가하세요. 테스트 함수는 `search` 함수가
가져야할 동작을 특정합니다: 즉 질의값와 검색할 텍스트를 입력받아서
텍스트로부터 질의값을 담고 있는 라인들만 반환하는 것이죠. Listing 12-15는
이러한 테스트를 보여주는데, 아직 컴파일되진 않을 것입니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

<span class="caption">Listing 12-15: 구현하고자 하는 `search` 함수를 위한
실패하는 테스트 만들기</span>

이 테스트는 문자열 `"duct"`를 검색합니다. 검색하는 텍스트는 세 줄인데,
그중 한줄만이 `"duct"`를 가지고 있습니다 (앞의 큰 따움표 뒤에 붙은 역슬래시는
이 문자열 리터럴 내용의 앞에 새 줄 글자를 집어넣지 않도록 하라고 러스트에게
알려주는 것임을 유의하세요). `search` 함수가 반환하는 값은 우리가 예상하는
라인만 가지고 있을 것이라고 단언해 두었습니다.

이 테스트는 아직 컴파일도 되지 않을 것이므로 테스트를 실행시켜서 실패하는
걸 지켜볼 수는 없습니다: 아직 `search` 함수가 없으니까요! TDD 원칙에 따라서,
Listing 12-16과 같이 항상 빈 벡터를 반환하는 `search` 함수 정의부를
추가하는 것으로 컴파일과 테스트가 동작하기에 딱 충분한 코드만 집어넣어
보겠습니다. 그러면 테스트는 컴파일되고, 반환된 빈 벡터가 `"safe, fast,
productive."` 라인을 가지고 있는 벡터와 일치하지 않으므로 실패해야
합니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

<span class="caption">Listing 12-16: 테스트가 딱 컴파일만 될 정도의
`search` 함수 정의하기</span>

`search`의 시그니처에는 명시적 라이프타임 `'a`가 정의될 필요가 있고
이 라이프타임이 `contents` 인자와 반환값에 사용되고 있음을 주목하세요.
[10장][ch10-lifetimes]<!-- ignore -->에서 본 것처럼 라이프타임 매개변수는
어떤 인자의 라이프타임이 반환값의 라이프타임과 연결되는지를 특정한다는
점을 상기해봅시다. 위의 경우에는 반환된 벡터에 (인자 `query`쪽이 아니라)
인자 `contents`의 슬라이스를 참조하는 문자열 슬라이스가 들어있음을
나타내고 있습니다.

바꿔 말하면, 우리는 지금 러스트에게 `search` 함수에 의해 반환된 데이터가
`search` 함수의 `contents` 인자로 넘겨진 데이터만큼 오래 살 것이라는
것을 말해준 것입니다. 이것이 중요합니다! 슬라이스에 의해 참조된 데이터는
그 참조자가 유효한 동안 유효할 필요가 있습니다; 만일 컴파일러가 `contents`
대신 `query`의 문자열 슬라이스를 만들고 있다고 가정하면, 안전성 검사를
부정확하게 하게 될 것입니다.

라이프타임 명시를 까먹고 이 함수의 컴파일을 시도하면, 다음과
같은 에러를 얻게됩니다:

```console
{{#include ../listings/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```

러스트는 두 인자 중 어떤 쪽을 우리가 원하는지 알 가능성이 없고, 따라서 이를
명시적으로 말해줄 필요가 있습니다. `contetns`가 모든 텍스트를 가지고 있는
인자이고 이 텍스트에서 일치되는 부분을 반환하고 싶은 것이므로, 우리는
`contents`가 라이프타임 문법을 사용해 반환값과 연결되어야 할 인자라는 사실을
알고 있습니다.

다른 프로그래밍 언어들은 여러분에게 시그니처에다 인자와 반환값을 연결하도록
요구하지 않습니다만, 이 연습은 시간이 지날수록 더 쉬워질 것입니다. 어쩌면
이 예제와 10장의
[“라이프타임으로 참조자 유효화하기”][validating-references-with-lifetimes]<!-- ignore -->절에
있는 예제를 비교하고 싶을지도 모르겠습니다.

이제 테스트를 실행해봅시다:

```console
{{#include ../listings/ch12-an-io-project/listing-12-16/output.txt}}
```

좋습니다. 예상대로 테스트는 실패했습니다. 이제 테스트가 통과되도록 해봅시다!

### 테스트를 통과하도록 코드 작성하기

현재는 언제나 빈 벡터가 반환되기 때문에 테스트가 실패하고 있습니다. 이를 고치고
`search`를 구현하기 위해서, 우리 프로그램은 아래의 단계를 따를 필요가 있습니다:

* 내용물의 각 라인에 대해 반복합니다.
* 해당 라인이 질의 문자열을 담고 있는지 검사합니다.
* 만일 그렇다면, 반환하고자 하는 값의 리스트에 추가합니다.
* 아니라면 아무것도 안합니다.
* 매칭된 결과 리스트를 반환합니다.

라인들에 대한 반복을 시작으로 각 단계 별로 작업해봅시다.

#### `lines` 메서드로 라인들에 대해 반복하기

러스트는 문자열의 라인별 반복을 처리하기 위한 유용한 메서드를 제공하는데,
편리하게도 `lines`라는 이름이고 Listing 12-17에서 보는 바와 같이 동작합니다.
아직 컴파일되지 않음을 주의하세요.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

<span class="caption">Listing 12-17: `contents`의 각 줄에 대한 반복
</span>

`lines` 메서드는 반복자를 반환합니다. 반복자에 대해서는 [13장][ch13-iterators]<!-- ignore -->에서
더 깊이 다루겠습니다만, 여러분은 [Listing 3-5][ch3-iter]<!-- ignore -->에서
이런 방식의 반복자 사용을 봤었음을 상기해봅시다. 그때는 어떤 컬렉션 안의 각
아이템에 대해 어떤 코드를 실행시키기 위해 `for`과 함께 반복자를 사용했었지요.

#### 각 라인에서 질의값 검색하기

다음으로는 현재의 라인에 질의 문자열이 들어있는지 검사해 보겠습니다.
다행히도 이걸 해주는 `contains`라는 이름의 유용한 메서드가 문자열에
있습니다! Listing 12-18처럼 `search` 함수에 `contains` 메서드 호출을
추가하세요. 아직 컴파일되지는 않음을 주의하세요.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

<span class="caption">Listing 12-18: 라인이 `query`의 문자열을 포함하는지
알아보기 위한 기능 추가하기</span>

이 시점에서는 아직 기능을 쌓아가는 중입니다. 컴파일되기 위해서는
함수 시그니처에 표시한 것처럼 함수 본문에서 어떤 값을 반환할
필요가 있습니다.

#### 매치된 라인 저장하기

이 함수를 완성하기 위해서는 반환하고자 하는 매치된 라인들을 저장할 방법이
필요합니다. 이를 위해서 `for` 루프 전에 가변 백터를 만들고 `line`을 이
벡터에 저장하기 위해 `push` 메서드를 호출할 수 있겠습니다. `for` 루프 뒤에는
Listing 12-19와 같이 이 벡터를 반환합니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

<span class="caption">Listing 12-19: 매치된 라인들을 저장하여 반환될 수
있게 하기</span>

이제 `search` 함수는 `query`를 담고 있는 라인들만 반환해야 하고
테스트는 통과되어야 합니다. 테스트를 실행해봅시다:

```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

테스트가 통과되었으므로, 함수가 동작한다는 사실을 알았습니다!

이 지점에서, 동일한 기능을 유지하여 테스트가 계속 통과되도록
유지하면서 이 검색 함수의 구현을 리팩토링할 기회를 고려해볼
수 있겠습니다. 이 검색함수의 코드는 그렇게 나쁘진 않습니다만,
반복자의 몇몇 유용한 기능을 활용하고 있지는 않군요.
[13장][ch13-iterators]<!-- ignore -->에서 이 예제로 돌아올건데, 거기서
반복자에 대해 더 자세히 탐구하고 어떻게 개선할 수 있는지 알아볼 것입니다.

#### `run` 함수에서 `search` 함수 사용하기

이제 `search` 함수가 작동하고 테스트도 되었으니, `run` 함수에서
`search`를 호출할 필요가 있겠습니다. `search` 함수에 `config.query` 값과
`run`이 읽어들인 `contents`를 넘겨줘야 합니다. 그러면 `run`은
`search`가 반환한 각 라인을 출력할 것입니다:


<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/src/lib.rs:here}}
```

`search`로부터 반환된 각 라인에 대해 여전히 `for`를 사용하여 출력하고 있습니다.

이제 전체 프로그램이 동작해야 합니다! 먼저 에밀리 딕킨슨의 시에서 딱 한줄만
반환되도록 “frog”라는 단어를 넣어 시도해봅시다:

```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

멋지군요! 이제 여러 줄과 매칭될 “body” 같은 단어를 시도해봅시다:

```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

마지막으로, “monomorphization” 같이 이 시의 어디에도 없는 단어를 검색하는 경우
아무 줄도 안나오는지 확인해봅시다:

```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```

훌륭하군요! 우리는 고전적인 도구의 우리 자신만의 미니 버전을 만들었고
애플리케이션을 구조화하는 방법에 대해 많이 배웠습니다. 또한 파일 입출력과
라이프타임, 테스트, 커맨드 라인 파싱에 대해서도 약간씩 배웠습니다.

이 프로젝트를 정리하기 위해서, 환경 변수를 가지고 동작시키는
방법과 표준 에러로 출력하는 방법을 간략하게 보려고 하는데, 둘 모두
여러분들이 커맨드 라인 프로그램을 작성할 때 유용합니다.

[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[ch11-anatomy]: ch11-01-writing-tests.html#the-anatomy-of-a-test-function
[ch10-lifetimes]: ch10-03-lifetime-syntax.html
[ch3-iter]: ch03-05-control-flow.html#looping-through-a-collection-with-for
[ch13-iterators]: ch13-02-iterators.html
