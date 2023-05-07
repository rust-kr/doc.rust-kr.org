## 환경 변수 사용하기

`minigrep`에 추가 기능을 넣어서 개선시켜보겠습니다: 바로 환경
변수를 통해 사용자가 켤 수 있는 대소문자를 구분하지 않는 검색
옵션입니다. 이 기능을 커맨드 라인 옵션으로 만들어서 필요한 경우
사용자가 매번 입력하도록 요구할 수도 있겠으나, 환경 변수로 만듦으로서
사용자는 이 환경 변수를 한번만 설정하고 난 다음 그 터미널 세션
동안에는 모든 검색을 대소문자 구분없이 할 수 있게 됩니다.

### 대소문자를 구분하지 않는 `search` 함수에 대한 실패하는 테스트 작성하기

먼저 환경 변수값이 있을 때 호출될 새로운 함수 `search_case_insensitive`를
추가하겠습니다. 계속하여 TDD 프로세스를 따를 것이므로, 첫번째 단계는
다시 한번 실패하는 테스트를 작성하는 것입니다. 새로운 함수
`search_case_insensitive`를 위한 추가하고 이전 테스트 이름은
Listing 12-20처럼 두 테스트 간의 차이를 명화학하게 하기 위해 `one_result`에서
`case_sensitive`로 바꾸겠습니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-20/src/lib.rs:here}}
```

<span class="caption">Listing 12-20: 추가하려는 대소문자 구분없는 함수를 위한
새로운 실패 테스트 추가하기</span>

예전 테스트의 `contents`도 수정되었음을 유의하세요. 대문자 D를 사용한
`"Duct tape."` 라인을 추가였고 이는 대소문자를 구분하는 방식으로 검색할
때는 질의어 `"duct"`에 매칭되지 않아야 합니다. 이렇게 예전 테스트를 변경하는
것은 이미 구현된 대소문자를 구분하는 검색을 우발적으로 깨트리지 않도록
확인하는데 도움을 줍니다. 이 테스트는 지금 통과되어야 하며 우리가 대소문자를
구문하지 않는 검색에 대해 작업을 하는 중에도 계속해서 통과되어야 합니다.

대소문자를 *구문하지 않는* 검색을 위한 새로운 테스트에서는 질의어로 `"rUsT"`를
사용합니다. 우리가 추가하고자 하는 `search_case_insensitive` 함수에서,
질의어 `"rUsT"`는 대소문자 구분이 질의어와 다르더라도 대문자 R로 시작하는
`"Rust:"를 포함하는 라인 및 `"Trust me."` 라인과 매칭되어야 합니다.
이것이 우리의 실패하는 테스트고, 아직 `search_case_insensitive` 함수를
정의하지 않았으므로 컴파일에 실패할 것입니다. Listing 12-16에서 테스트가
컴파일되고 실패하는 것을 지켜보기 위해 했었던 것과 마찬가지로, 편하게
언제나 빈 벡터를 반환하는 뼈대 구현을 추가해 봅시다.

### `search_case_insensitive` 함수 구현하기

Listing 12-21에서 보시는 `search_case_insensitive` 함수는 `search`
함수와 거의 똑같이 생겼을 것입니다. 유일한 차이점은 `query`와 각 `line`을
소문자로 만들어서 입력된 인자의 대소문자가 어떻든 간에 질의어가 라인에
포함되어 있는지 확인할 때는 언제나 같은 소문일 것이란 점입니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-21/src/lib.rs:here}}
```

<span class="caption">Listing 12-21: 질의어와 라인을 비교하기 전에 소문자로
만드는 `search_case_insensitive` 함수 정의하기</span>

먼저 `query` 문자열을 소문자로 만들어서 같은 이름의 변수를 가리는
방식으로 저장합니다. 질의어에 대해 `to_lowercase`가 호출되므로
사용자의 질의어가 `"rust"`, `"RUST"`, `"Rust"`, 혹은 `"rUsT"`이든
상관없이 이 질의어를 `"rust"`로 취급하여 대소문자를 구분하지 않게 될
것입니다. `to_lowercase`가 기본적인 유니코드를 다루겠지만, 100% 정확하지는
않을 것입니다. 실제 애플리케이션을 작성중이었다면 여기에 약간의 작업을
추가할 필요가 있겠지만, 이 절은 유니코드가 아니라 환경변수에 대한 것이므로,
여기서는 그대로 두겠습니다.

`to_lowercase`의 호출이 존재하는 데이터를 참조하지 않고 새로운 데이터를
만들기 때문에, `query`가 이제 문자열 슬라이스가 아니라 `String`이 되었음을
주의하세요. 예를 들어 질의어가 `"rUsT"`라고 해봅시다: 이 문자열 슬라이스는
우리가 사용하려는 소문자 `u`나 `t`가 들어있지 않으므로, `"rust"`를 담고 있는
새로운 `String`을 할당해야 합니다. 이제 `query`를 `contains`의 인자로
넘길때는 앰퍼센드를 붙여줄 필요가 있는데 이는 `contains`의 시그니처가 문자열
슬라이스를 받도록 정의되어 있기 때문입니다.

다음으로 `line`의 모든 글자를 소문자로 만들기 위해 `to_lowercase` 호출을
추가합니다. 이제 `line`과 `query`를 소문자로 변환했으니 질의어의 대소문자에
상관없이 매치된 라인들을 찾아낼 것입니다.

이 구현이 테스트를 통과하는지 살펴봅시다:

```console
{{#include ../listings/ch12-an-io-project/listing-12-21/output.txt}}
```

훌륭하군요! 테스트가 통과되었습니다. 이제 `search_case_insensitive`
함수를 `run` 함수에서 호출해봅시다. 먼저 대소분자 구분 여부를 전환하기
위한 옵션을 `Config` 구조체에 추가하겠습니다. 아직 이 필드를 어디서도
초기화하고 있지 않기 때문에 필드를 추가하는 것만으로는 컴파일 에러가
날 것입니다:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:here}}
```

부울린 값을 갖는 `ignore_case` 필드를 추가했습니다. 다음으로, Listing
12-22에서 보는 것처럼 `run` 함수가 `ignore_case` 필드의 값을 검사하여
`search` 함수 혹은 `search_case_insensitive` 함수 중 어느 쪽을 호출할
지 결정하는 것이 필요합니다. 아직은 컴파일되지 않을 것입니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:there}}
```

<span class="caption">Listing 12-22: `config.ignore_case`의 값에
기초하여 `search`나 `search_case_insensitive`를 호출하기</span>

마지막으로 환경 변수의 검사가 필요합니다. 환경 변수 사용을
위한 함수는 표준 라이브러리의 `env` 모듈에 있으므로,
*src/lib.rs* 상단에서 이 모듈을 스코프로 가져옵니다.
그런 다음 Listing 12-23처럼 `env` 모듈의 `var` 함수를
사용하여 `IGNORE_CASE`라는 이름의 환경 변수에 어떤 값이
설정되었는지 확인해 보겠습니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-23/src/lib.rs:here}}
```

<span class="caption">Listing 12-23: `IGNORE_CASE`라는 이름의 환경 변수의
값을 검사하기</span>

여기서 `ignore_case`라는 새 변수를 만들었습니다. 이 값을 설정하기 위해서
`env::var` 함수를 호출하고 여기에 환경 변수의 이름 `INGORE_CASE`를
넘겼습니다. `env::var` 함수는 `Result`를 반환하는데 여기에는 해당
환경 변수에 어떤 값이 설정되어 있을 경우 그 값을 담은 `Ok` 배리언트가
될 것입니다. 만일 환경 변수가 설정되어 있지 않다면 `Err` 배리언트가
반환될 것입니다.

환경 변수가 설정되었는지 확인하기 위해서 `Result`의 `is_ok` 메서드를
사용중인데, 이는 프로그램이 대소문자를 구분하지 않는 검색을 해야함을
뜻합니다. 만일 `IGNORE_CASE` 환경 변수가 아무 값도 설정되어 있지 않다면,
`is_ok`는 거짓값을 반환하고 프로그램은 대소문자를 구분하는 검색을 수행할
것입니다. 이 환경 변수의 *값*에 대해서는 고려하지 않고 그저 값이 설정되어
있는지 아닌지만 고려하므로, 여기서는 `unwrap`이나 `expect` 혹은 `Result`에서
사용했던 다른 메서드들 대신 `ik_ok`을 사용하고 있습니다.

이 `ignore_case` 변수의 값을 `Config` 인스턴스에게 넘겼으므로, `run` 함수는
Listing 12-22에 구현된 것처럼 이 값을 읽어서 `search_case_insensitive` 혹은
`search`의 호출 여부를 결정할 수 있습니다.

한번 시도해봅시다! 먼저 환경 변수 설정 없이 질의어 `to`를 넣어 프로그램을
실행시킬 것인데, 이는 모두 소문자인 단어 “to”가 포함된 어떤 라인과
매치되어야 합니다:

```console
{{#include ../listings/ch12-an-io-project/listing-12-23/output.txt}}
```

아직 잘 동작하는 것처럼 보이는군요! 이제 `IGNORE_CASE`를 `1`로 설정하고
동일한 질의어 `to`를 넣어서 프로그램을 실행해 봅시다.

```console
$ IGNORE_CASE=1 cargo run -- to poem.txt
```

여러분이 파워쉘을 사용중이라면, 별도의 커맨드로 환경 변수 설정과 프로그램 실행을
할 필요가 있을 것입니다:

```console
PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

이는 남은 쉘 세션에 대해 `IGNORE_CASE`가 영구적으로 설정되게 할
것입니다. `Remove_Item` cmdlet으로 설정을 해제할 수 있습니다:

```console
PS> Remove-Item Env:IGNORE_CASE
```

이제 대문자일 수도 있는 “to”를 담고 있는 라인들을 얻어야 합니다:

<!-- manual-regeneration
cd listings/ch12-an-io-project/listing-12-23
IGNORE_CASE=1 cargo run -- to poem.txt
can't extract because of the environment variable
-->

```console
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

“To”를 담고 있는 라인도 얻었으니, 훌륭합니다! 우리 `minigrep` 프로그램은 지금부터
환경 변수에 의해 제어되는 대소문자 구별없는 검색기능을 사용할 수 있게 되었습니다.
이제 여러분은 커맨드 라인 인자 혹은 환경 변수를 통한 옵션 설정을 관리하는 방법을
알게 되었습니다.

어떤 프로그램들은 같은 환경값에 대해 인자와 환경 변수 *모두를* 사용할 수
있게 합니다. 그러한 경우에는 보통 한쪽이 다른 쪽에 대해 우선 순위를 갖도록
결정합니다. 여러분 스스로의 연습 목적으로, 대소문자 구분 옵션을 커맨드
라인 혹은 환경 변수로 제어하는 시도를 해보세요. 한쪽은 대소문자를
구분하도록 하고 다른 쪽은 대소문자 구분을 무시하도록 설정되어 실행되었을
경우에는 커맨드 라인 인자쪽 혹은 환경 변수쪽이 우선권을 갖도록 하는
결정이 필요합니다.

`std:;env` 모듈에는 환경 변수를 다루기 위한 더 유용한 기능들을 많이 가지고
있습니다: 어떤 것들이 가능한지는 문서를 확인해 보세요.
