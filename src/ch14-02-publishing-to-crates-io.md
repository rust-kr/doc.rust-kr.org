## Crates.io에 크레이트 배포하기

여지까지 프로젝트의 의존성으로서 [crates.io](https://crates.io/)<!-- ignore -->의
패키지를 이용해왔지만, 여러분도 자신만의 패키지를 배포(publish)하여 다른
사람들과 코드를 공유할 수 있습니다. [crates.io](https://crates.io/)<!-- ignore -->에
있는 크레이트 등기소(registry)는 여러분 패키지의 소스 코드를 공개하므로,
이는 주로 오픈 소스인 코드를 호스팅합니다.

러스트와 카고는 여러분이 배포한 패키지를 사람들이 더 쉽게 찾고 사용할 수
있도록 도와주는 기능이 있습니다. 이 기능들 몇가지에 대해 바로 다음에 이야기한 후
패키지를 배포하는 방법을 설명하겠습니다.

### 유용한 문서화 주석 만들기

여러분의 패키지에 대한 상세한 문서화는 다른 사용자들이 패키지를 어떻게, 언제
사용해야 하는지 알게 해주므로, 문서 작성에 시간을 투자하는 것은 가치있는 일입니다.
3장에서 러스트 코드에 두 개의 슬래시 `//`를 이용하여 주석을 다는 법을
이야기했습니다. 러스트에는 또한 *문서화 주석(documentation comment)* 이라고
불리는 문서화를 위한 특별한 종류의 주석이 있는데, 이 주석이 HTML 문서를 생성할
겁니다. 이 HTML에는 여러분의 크레이트가 어떻게 *구현되었는지*가 아닌 어떻게
*사용하는지*에 관심있는 프로그래머들을 위하여 공개 API 아이템들에 대한 문서화
주석 내용을 보여줍니다.

문서화 주석은 슬래시 두 개가 아니라 세 개 `///`를 이용하며 텍스트 서식을
위한 마크다운 표기법을 지원합니다. 문서화할 아이템 바로 앞에 문서화 주석을
배치하세요. Listing 14-1 은 `my_crate`라는 이름의 크레이트에 있는
`add_one` 함수에 대한 문서화 주석을 보여줍니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-01/src/lib.rs}}
```

<span class="caption">Listing 14-1: 함수에 대한 문서화
주석</span>

여기서 `add_one` 함수가 무슨 일을 하는지에 대한 설명을 적었고,
제목 `Example`으로 절을 시작한 다음, `add_one` 함수의 사용법을
보여주는 코드를 제공했습니다. `cargo doc`을 실행하면 이 문서화
주석으로부터 HTML 문서를 생성할 수 있습니다. 이 명령어는 러스트와
함께 배포되는 `rustdoc` 도구를 실행하여 생성된 HTML 문서를
*target/doc* 디렉토리에 넣습니다.

편의성의 위하여 `cargo doc --open`을 실행시키면 여러분의 현재 크레이트의
문서에 대해 (심지어 여러분의 크레이트가 가진 모든 의존성들의 문서까지)
HTML을 생성하고 그 결과를 웹 브라우저에 띄워줄 겁니다. 이제 `add_one`
함수를 찾아보면 Figure 14-1에 보시는 것처럼 문서화 주석의 텍스트가 어떤
식으로 렌더링되는지 알 수 있을 겁니다:

<img alt="Rendered HTML documentation for the `add_one` function of `my_crate`" src="img/trpl14-01.png" class="center" />

<span class="caption">Figure 14-1: `add_one` 함수에 대한 HTML
문서</span>

#### 자주 사용되는 절

Listing 14-1에서는 HTML에 "Examples" 제목을 가진 절을 만들기 위해
`# Examples` 마크다운 제목을 사용했습니다. 이외에 크레이트 저자가
문서에서 자주 사용하는 구절은 다음과 같습니다:

* **Panics**: 문서화된 함수가 패닉을 일으킬 수 있는 시나리오입니다.
  함수를 호출하는 쪽에서 자신의 프로그램이 패닉을 일으키는 것을 원치 않는다면
  이러한 상황에서 함수를 호출하지 않음을 확실해 해야 합니다.
* **Errors**: 해당 함수가 `Result`를 반환하는 경우에는 발생할 수 있는
  에러의 종류와 해당 에러들이 발생하는 조건을 설명해 준다면 호출하는
  사람이 다양한 종류의 에러를 여러 방법으로 처리할 수 있도록 코드를
  작성하는데 도움을 줄 수 있습니다.
* **Safety**: 함수가 호출하기에 `unsafe`한 경우라면 (불안전성에 대해서는
  19장에서 다룹니다), 왜 이 함수가 안전하지 않은지와 호출자가 이 함수를
  호출할 때 지켜야 할 불변성에 대해 설명하는 절이 있어야 합니다.

대부분의 문서화 주석은 이 절들이 모두 필요하진 않습니다만, 여러분의
코드를 사용하는 사람들이 알고 싶어하는 것에 대한 측면을 상기하는데
좋은 체크리스트입니다.

#### 테스트로서의 문서화 주석

문서화 주석에 예시 코드를 추가하는 건 라이브러리의 사용 방법을 보여주는데
도움이 될 뿐더러 추가적인 보너스도 가질 수 있습니다: 무려 `cargo test`를
실행하면 여러분의 문서에 들어있던 예시 코드들이 테스트로서 실행됩니다!
예시를 포함한 문서보다 좋은 문서는 없습니다. 하지만 문서가 작성된 이후
코드가 변경되어 작동하지 않게 되버린 예제보다 나쁜 것도 없습니다.
Listing 14-1의 `add_one` 함수에 대한 문서를 가지고 `cargo test`를
실행하면 다음과 같이 테스트 결과 절을 볼 수 있습니다:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-01/
cargo test
copy just the doc-tests section below
-->

```text
   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
```

이제 함수나 예제를 변경하여 예시 코드의 `assert_eq!`가 패닉을 발생시키는
상태로 `cargo test`를 다시 실행하면, 문서 테스트 기능이 해당 예제를 찾아내어
이 코드가 더 이상 기능하지 못함을 알려줄 것입니다!

#### 주석이 포함된 아이템

문서화 주석 스타일 `//!`은 주석 뒤에 오는 아이템을 문서화하는 것이 아닌
주석을 포함하는 아이템을 문서화합니다. 이러한 문서화 주석은 일반적으로
크레이트 루트 파일 (관례상 *src/lib.rs*) 혹은 모듈 내에 사용하여
크레이트 혹은 모듈 전체에 대한 문서를 작성하는데 씁니다.

예를 들어 `add_one` 함수를 담고 있는 `my_crate` 크레이트의
목적을 설명하는 문서를 추가하려면 Listing 14-2와 같이
*src/lib.rs* 파일의 시작 지점에 `//!`로 시작하는 문서화 주석을
추가합니다:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-02/src/lib.rs:here}}
```

<span class="caption">Listing 14-2: `my_crate` 크레이트 전체에 대한
문서</span>

`//!`로 시작하는 라인 중 마지막 라인 이후에 아무 코드도 없음을 주목하세요.
`///` 대신 `//!`로 주석을 시작하였기 때문에, 이 주석 뒤에 나오는 아이템이 아닌
이 주석을 포함하고 있는 아이템에 대한 문서화를 하는 중입니다. 위의 경우,
그 아이템은 *src/lib.rs* 파일이고, 이는 크레이트 루트입니다. 이 주석은
크레이트 전체를 설명합니다

`cargo doc --open`을 실행하면 Figure 14-2와 같이 문서
첫 페이지 내용 중 크레이트의 공개 아이템 리스트 상단에
이 주석의 내용이 나타날 것입니다:

<img alt="전체 크레이트를 위한 주석을 포함하는 렌더링된 HTML 문서" src="img/trpl14-02.png" class="center" />

<span class="caption">Figure 14-2: 전체 크레이트를 설명하는 주석이 포함된
`my_crate`의 렌더링된 문서</span>

아이템 내 문서화 주석은 특히 크레이트와 모듈에 대해 기술할 때
유용합니다. 이를 이용해 주석이 담긴 것의 전체 목적을 설명해서
사용자들이 크레이트 구조를 이해할 수 있도록 해보세요.

## `pub use`로 편리한 공개 API 내보내기

크레이트를 배포할 때는 공개 API의 구조가 주요 고려사항입니다.
여러분의 크레이트를 사용하는 사람들은 여러분보다 그 구조에 덜 익숙하고,
크레이트가 커다란 모듈 계층 구조를 이루고 있다면 사용하고자 하는 조각들을
찾아내는데 어려움이 있을 수도 있습니다.

7장에서는 `pub` 키워드를 사용하여 아이템을 공개하는 법, 그리고 `use` 키워드를
가지고 스코프 내로  아이템을 가져오는 법을 다루었습니다. 하지만 크레이트를 개발하는
동안 여러분에게 익숙해진 구조가 사용자들에게는 마냥 편리하지 않을런지도 모릅니다.
여러분은 여러 단계로 구성된 계층 구조에 구조체들을 조직화하고 싶을지도
모르겠지만, 그러면 계층 구조 내에 깊숙히 정의된 타입을 이용하고 싶어하는
사람들은 해당 타입이 존재하는지 발견하는데 어려움을 겪을 수도 있습니다.
또한 사용자들은 `use` `my_crate::UsefulType;`이 아니라 `use`
`my_crate::some_module::another_module::UsefulType;`라고 입력해야
하는데 짜증을 낼지도 모릅니다.

좋은 소식은 지금의 구조가 다른 사람들이 다른 라이브러리에서 사용하는데 편리하지
*않더라도* 굳이 내부 구조를 뒤엎을 필요는 없다는 겁니다. 대신에 `pub use`를
이용하여 내부 아이템을 다시 내보내서 (*re-export*) 기존의 비공개 구조와
다른 공개 구조를 만들 수 있습니다. 다시 내보내기는 어떤 위치에서 공개
아이템(public item)을 가져와서 이를 마치 다른 위치에 정의된 것처럼 해당
위치의 공개 아이템으로 만듭니다.

예를 들어, 예술적인 개념을 모델링하기 위해 `art`라는 라이브러리를 만들었다고
가정해 봅시다. 이 라이브러리에는 두 모듈이 들어 있습니다: Listing 14-3과 같이,
`kinds` 모듈은 `PrimaryColor`와 `SecondaryColor` 열거형을 가지고 있고,
`utils` 모듈은 `mix`라는 이름의 함수를 가지고 있습니다:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-03/src/lib.rs:here}}
```

<span class="caption">Listing 14-3: `kinds`와 `utils` 모듈에 아이템을 구성한
`art` 라이브러리</span>

Figure 14-3은 이 크레이트에 대하여 `cargo doc`으로 생성시킨 문서의
첫 화면입니다:

<img alt="`kinds`와 `utils` 모듈이 리스트에 올라와 있는 `art` 크레이트에 대한 렌더링된 문서" src="img/trpl14-03.png" class="center" />

<span class="caption">Figure 14-3: `kinds`와 `utils` 모듈이 리스트에 나타난
`art`의 문서 첫 화면</span>

`PrimaryColor`와 `SecondaryColor` 타입도, `mix` 함수도 리스트에
나타나지 않았음을 주목하세요. 이들을 보려면 각각 `kinds` 와 `utils`를
클릭해야 합니다.

이 라이브러리를 의존성으로 가지고 있는 다른 크레이트에서는 `art`의 아이템을
스코프 내로 가져오는 `use`를 사용할 필요가 있는데, 현재 정의된 모듈의 구조대로
명시해야 합니다. Listing 14-4는 어떤 크레이트에서 `art` 크레이트의
`PrimaryColor`와 `mix`를 이용하는 예시를 보여줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-04/src/main.rs}}
```

<span class="caption">Listing 14-4: `art` 크레이트의 내부 구조에서 내보내진 
아이템을 이용하는 크레이트</span>

Listing 14-4 코드의 저자, 즉 `art` 크레이트를 사용하는 사람은
`PrimaryColor`가 `kinds` 모듈에 들어있고 `mix`가 `utils` 모듈에
들어있다는 사실을 알아내야 합니다. `art` 크레이트의 구조는 크레이트를
사용하는 사람보다 크레이트를 개발하는 사람에게 더 적합합니다. 내부 구조는
`art` 크레이트를 사용하고자 하는 사람에게는 전혀 필요 없는 정보이며
오히려 혼란만 가져올 수 있는데, 이는 크레이트를 사용하는 개발자가
어디를 찾아봐야 하는지알아내야 하고 `use` 구문에 모듈 이름들을
명시해야하기 때문입니다.

공개 API로부터 내부 구조를 제거하기 위해서는 Listing 14-5와 같이
Listing 14-3의 `art` 크레이트 코드에 `pub use` 구문을 추가하여
아이템들을 최상위 단계로 다시 내보내야 합니다:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-05/src/lib.rs:here}}
```

<span class="caption">Listing 14-5: `pub use` 구문을 추가하여 아이템을 다시
내보내기</span>

`cargo doc`이 생성한 이 크레이트의 API 문서는 이제 Figure 14-4와 같이 다시 내보내진
아이템을 첫 화면의 리스트에 보여주고 링크를 걸어줄 것이며, 이로써 `PrimaryColor`와
`SecondaryColor` 타입과 `mix` 함수를 더 쉽게 찾도록 만들어 줍니다.

<img alt="첫 화면에 다시 내보내진 아이템이 있는 `art` 크레이트의 렌더링된 문서" src="img/trpl14-04.png" class="center" />

<span class="caption">Figure 14-4: 다시 내보내진 아이템이 리스트에 있는 `art`
문서 첫 화면</span>

`art` 크레이트 사용자는 Listing 14-4에서 봤던 것처럼 Listing 14-3의 내부
구조를 여전히 보고 이용할 수 있고, 혹은 Listing 14-6과 같이 Listing 14-5의
더 편리해진 구조를 사용할 수도 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-06/src/main.rs:here}}
```

<span class="caption">Listing 14-6: `art` 크레이트의 다시 내보내진 아이템을 사용하는
프로그램</span>

중첩된 모듈이 많이 있는 경우, `pub use`를 사용하여 최상위 단계로
타입들을 다시 내보내는 것은 크레이트를 사용하는 사람들의 경험을 크게
바꿀 수 있습니다. `pub use`의 또다른 일반적 사용법은 현재 크레이트가
의존하고 있는 크레이트 내에 정의된 것들을 다시 내보내서 그 크레이트의
정의들을 여러분의 크레이트의 공개 API의 일부분으로 만드는 것입니다.

유용한 공개 API를 만드는 것은 기술보단 예술에 가깝고, 여러분은
반복적으로 사용자들에게 가장 잘 맞는 API를 찾아갈 수 있습니다.
`pub use`를 사용하는 것은 크레이트를 내부적으로 구조화하는데 유연성을
제공하면서 이 내부 구조와 사용자에게 제공하는 것을 분리해줍니다.
여러분이 설치한 크레이트 코드 몇 개를 열어서 내부 구조와 공개 API가
얼마나 다른지 살펴보세요.

### Cartes.io 계정 설정하기

여러분이 어떤 크레이트를 배포하기에 앞서서, [crates.io](https://crates.io/)<!-- ignore -->에서
계정을 만들고 API 토큰을 얻을 필요가 있습니다. 그러려면
[crates.io](https://crates.io/)<!-- ignore --> 홈페이지에 방문해서
GitHub 계정으로 로그인하세요. (현재는 GitHub 계정이 필수지만, 나중에는 다른
계정 생성 방법을 지원할 수도 있습니다.) 일단 로그인되었다면
[https://crates.io/me/](https://crates.io/me/)<!-- ignore -->에 있는
계정 설정으로 가서 API 키를 얻으세요. 그런 다음 아래와 같이 여러분의 API 키로
`cargo login` 명령어를 실행하세요:

```console
$ cargo login abcdefghijklmnopqrstuvwxyz012345
```

이 명령어는 카고에게 여러분의 API 토큰을 알려주고 로컬의 *~/.cargo/credentials*에
저장하도록 합니다. 이 토큰은 *비밀키 (secret)* 임을 주의하세요: 아무와도
공유하지 마세요. 어떤 이유에서든 누군가와 공유했다면, 이 토큰을 무효화시키고
[crates.io](https://crates.io)<!-- ignore-->에서 새 토큰을 생성해야
합니다.

### 새 크레이트에 메타데이터 추가하기

이제 배포하고자 하는 크레이트가 있다고 칩시다. 배포하기 전, 크레이트의
*Cargo.toml* 파일의 `[package]` 절 안에 메타데이터 몇가지를 추가할 필요가
있을 것입니다.

여러분의 크레이트는 고유한 이름이 필요할 것입니다. 로컬에서 어떤 크레이로 작업하는
중이라면 이 크레이트의 이름을 뭐라고 짓든 상관없습니다. 하지만
[crates.io](https://crates.io)<!-- ignore --> 에 올라오는 크레이트의
이름은 선착순으로 배정됩니다. 일단 크레이트 이름이 써졌다면, 그 이름으로는 다른
누구도 크레이트를 배포할 수 없습니다. 크레이트를 배포하기 전에 사용하려는 이름을
검색해보세요. 해당 크레이트명이 사용되었다면, 다른 이름을 찾아서 *Cargo.toml*
파일 안의 `[package]` 절 아래에 다음과 같이 `name` 필드를 수정하여 배포를
위한 새로운 이름을 사용할 필요가 있을 것입니다:

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
```

고유한 이름을 선택했더라도 이 시점에서 `cargo publish`를 실행시켜 크레이트를
배포해보면 다음과 같은 경고 후 에러를 보게 될 것입니다:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-01/
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
--snip--
error: failed to publish to registry at https://crates.io

Caused by:
  the remote server responded with an error: missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for how to upload metadata
```

이 에러는 몇가지 중요한 정보가 없기 때문에 발생된 것입니다: 설명과 라이센스는
필수로서 여러분의 크레이트가 무엇을 하는지와 어떤 조건으로 사용할 수 있는지
사람들이 알게끔 할 것입니다. *Cargo.toml* 안에 한두 문장 정도만 설명을 추가해
주세요. 이 설명은 검색 결과에 여러분의 크레이트와 함께 나타나게 될 것입니다.
`license` 필드에는 *라이센스 식별자 값 (license identifier value)*이 필요합니다.
[리눅스 재단의 Software Package Data Exchange (SPDX)][spdx]에 이 값으로
사용할 수 있는 식별자 리스트가 있습니다. 예를 들어 여러분의 크레이트에 MIT
라이센스를 적용하고 싶다면, `MIT` 식별자를 추가합니다:

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
license = "MIT"
```

SPDX에 없는 라이센스를 사용하고 싶다면, 그 라이센스에 대한
텍스트를 파일에 넣어서 프로젝트 내에 포함시킨 다음, `license` 키
대신 `license-file`을 사용하여 해당 파일의 이름을 명시할 필요가
있습니다.

여러분의 프로젝트에 어떤 라이센스가 적합한지에 대한 안내는 이 책의 범위를
벗어납니다. 러스트 커뮤니티의 많은 이들은 자신의 프로젝트에 러스트가 쓰는
라이센스인 `MIT OR Apache-2.0` 듀얼 라이센스를 사용합니다. 이러한 실제 예는
여러분도 `OR`로 구분된 여러 라이센스 식별자를 지정하여 프로젝트에 여러 개의
라이센스를 적용할 수 있음을 보여줍니다.

고유한 이름, 버전, 설명, 그리고 라이센스가 추가된 상태에서 배포할
준비가 된 프로젝트의 *Cargo.toml* 파일은 아래처럼 생겼습니다:

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

[카고 공식 문서](https://doc.rust-lang.org/cargo/)에는 다른 사람들이
여러분의 크레이트를 더 쉽게 발견하고 사용하도록 해주기 위해 지정할 수 있는
다른 메타데이터에 대해 설명되어 있습니다.

### Crates.io에 배포하기

이제 계정을 만들었고, API 토큰을 저장했고, 크레이트의 이름도 정했고,
필요한 메타데이터도 지정되었다면, 배포할 준비가 된 것입니다!
크레이트 배포는 다른 사람들이 사용할 특정 버전을
[crates.io](https://crates.io/)<!-- ignore -->에 올리는 것입니다.

배포는 *영구적이므로* 주의하세요. 버전은 덮어씌워질 수 없고,
코드는 삭제될 수 없습니다. [crates.io](https://crates.io/)<!-- ignore -->의
주요 목표 한가지는 영구적인 코드 보관소로서 동작하여
[crates.io](https://crates.io/)<!-- ignore -->의 크레이트에
의존하는 모든 프로젝트의 빌드가 계속 동작하도록 하는 것입니다.
버전 삭제를 서용하면 이 목표의 이행을 불가능하게 할 것입니다.
하지만 배포할 수 있는 크레이트 버전의 숫자에 제한은 없습니다.

다시 한번 `cargo publish` 명령어를 수행해보세요. 이제 성공해야 합니다:

<!-- manual-regeneration
go to some valid crate, publish a new version
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
   Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
   Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
   Compiling guessing_game v0.1.0
(file:///projects/guessing_game/target/package/guessing_game-0.1.0)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
   Uploading guessing_game v0.1.0 (file:///projects/guessing_game)
```

축하합니다! 여러분은 이제 코드를 러스트 커뮤니티에 공유하였고, 다른 사람들이
자신의 프로젝트에 여러분의 크레이트를 의존성으로 쉽게 추가할 수 있습니다.

### 이미 존재하는 크레이트의 새 버전 배포하기

크레이트를 변경하여 새 버전을 배포할 준비가 되었다면, *Cargo.toml*
파일에 명시된 `version` 값을 바꿔 다시 배포하면 됩니다.
변경사항의 종류에 기반하여 적절한 버전 숫자를 결정하려면
[유의적 버전 규칙 (Semantic Versioning rules)][semver]을 사용하세요.
그 다음 `cargo publis`를 실행하여 새 버전을 올립니다.

<!-- Old link, do not remove -->
<a id="removing-versions-from-cratesio-with-cargo-yank"></a>

### `cargo yank`로 Crates.io에서 버전 사용하지 않게 하기

킄레이트의 이전 버전을 제거할 수는 없지만, 향후의 프로젝트들이 이를 새로운
의존성으로 추가하는 것을 방지할 수는 있습니다. 이는 어떤 크레이트 버전이
어떤 이유에서인가 깨졌을 때 유용합니다. 그런 상황에서 카고는
어떤 크레이트 버전의 *끌어내기 (yanking)* 를 지원합니다.

버전 끌어내기는 이 버전에 의존중인 존재하는 모든 프로젝트들이 계속 지원하면서
새 프로젝트가 이 버전에 의존하는 것을 방지합니다. 근본적으로 끌어내기란
*Cargo.lock*이 있는 모든 프로젝트가 깨지지 않으면서, 이후에 생성되는
*Cargo.lock* 파일에는 끌어내려진 버전을 사용하지 않을 것임을 의미합니다.

크레이트의 버전을 끌어내리려면 이전에 배포했던 크레이트 디렉토리에서
`cargo yank`를 실행하여 끌어내리고자 하는 버전을 지정하세요.
예를 들머 `guessing_game`이라는 이름의 크레이트 버전 1.0.1을
배포했었고 이를 끌어내리고자 한다면, `guessing_game`의 프로젝트
디렉토리에서 다음과 같이 실행합니다:

<!-- manual-regeneration:
cargo yank carol-test --version 2.1.0
cargo yank carol-test --version 2.1.0 --undo
-->

```console
$ cargo yank --vers 1.0.1
    Updating crates.io index
        Yank guessing_game@1.0.1
```

명령어에 `--undo`를 추가하면 끌어내기를 되돌려 다른 프로젝트들이
다시 이 버전에 대한 의존을 허용할 수 있습니다:

```console
$ cargo yank --vers 1.0.1 --undo
    Updating crates.io index
      Unyank guessing_game@1.0.1
```

끌어내기는 어떤 코드도 삭제하지 *않습니다*. 예를 들어 실수로 업로드된 비밀키 같은걸
삭제할 수는 없습니다. 그런 일이 벌어졌다면 즉시 해당 비밀키를 리셋해야 합니다.

[spdx]: http://spdx.org/licenses/
[semver]: http://semver.org/
