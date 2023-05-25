## 카고 작업공간

12장에서 바이너리 크레이트와 라이브러리 크레이트를 포함하는 패키지를 만들어
봤습니다. 하지만 프로젝트를 개발하다 보면, 라이브러리 크레이트가 점점
거대해져서 패키지를 여러개의 라이브러리 크레이트로 분리하고 싶을 겁니다.
카고는 *작업공간 (workspace)*이라는 기능을 제공하여 나란히 개발되는
여러 관련 패키지를 관리하는데 도움을 줄 수 있습니다.

### 작업공간 생성하기

*작업공간 (workspace)*은 동일한 *Cargo.lock*과 출력 디렉토리를 공유하는
패키지들의 집합입니다. 작업공간을 이용하여 프로젝트를 만들어 봅시다-
여기서는 간단한 코드만 사용하여 작업공간의 구조에 집중하겠습니다.
작업공간을 구성하는 방법은 여러가지가 있으므로, 그 중 일반적인 방법 하나를
보겠습니다. 우리의 작업공간은 하나의 바이너리와 두개의 라이브러리를 담을
것입니다. 주요 기능을 제공할 바이너리는 두 라이브러리를 의존성으로 가지게
될 것입니다. 첫번째 라이브러리는 `add_one` 함수를 제공하고, 두번째 라이브러리는
`add_two` 함수를 제공할 것입니다. 이 세 크레이트는 같은 작업공간의 일부가 될
겁니다. 작업공간을 위한 새 디렉토리를 만드는 것부터 시작하겠습니다:

```console
$ mkdir add
$ cd add
```

다음으로 *add* 디렉토리 내에 *Cargo.toml*을 생성하여 전체 작업공간에
대한 설정을 합니다. 이 파일은 `[package]` 절이 없습니다. 대신
`[workspace]` 절로 시작하여 바이너리 크레이트 패키지에 대한 경로를
명시하는 방식으로 이 작업공간에 멤버를 추가할 것입니다; 지금의 경우
해당 경로는 *adder*입니다:

<span class="filename">파일명: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace-with-adder-crate/add/Cargo.toml}}
```

다음엔 *add* 디렉토리 내에서 `cargo new`를 실행하여 `adder` 바이너리
크레이트를 생성하겠습니다:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-01-adder-crate/add
rm -rf adder
cargo new adder
copy output below
-->

```console
$ cargo new adder
     Created binary (application) `adder` package
```

이 시점에서 작업 공간을 `cargo build`로 빌드할 수 있습니다. *add* 디렉토리
내의 파일들은 아래와 같은 형태여야 합니다:

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

작업공간은 컴파일된 결과가 위치할 하나의 *target* 디렉토리를 최상위
디렉토리에 가집니다; `adder` 크레이트는 자신의 *target* 디렉토리를
갖지 않습니다. *adder* 디렉토리 내에서 `cargo build` 명령어를
실행하더라도 컴파일 결과는 *add/adder/target*이 아닌 *add/target*에
위치하게 될 겁니다. 카고가 이와 같이 *target* 디렉토리를 작업공간 내에 구성하는
이유는, 작업공간 내의 크레이트들이 서로 의존하기로 되어있기 때문입니다.
만약 각 크레이트가 각자의 *target* 디렉토리를 갖는다면, 각 크레이트는
작업공간 내의 다른 크레이트들을 다시 컴파일하여 그 결과물을 자신의 *target*
디렉토리에 넣어야 합니다. 하나의 *target* 디렉토리를 공유하면 크레이트들의
불필요한 재빌드를 피할 수 있습니다.

### 작업공간에 두번째 패키지 생성하기

다음으로 다른 멤버 패키지를 작업공간에 생성하여 `add_one`라고 이름을
붙입시다. 최상위 *Cargo.toml*을 수정하여 `members` 리스트에
*add_one* 경로를 지정하세요:

<span class="filename">파일명: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```

그런 다음 `add_one`이라는 이름의 새 라이브러리 크레이트를 생성하세요:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-02-add-one/add
rm -rf add_one
cargo new add_one --lib
copy output below
-->

```console
$ cargo new add_one --lib
     Created library `add_one` package
```

*add* 디렉토리는 이제 다음과 같은 디렉토리와 파일을 갖추어야 합니다:

```text
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

*add_one/src/lib.rs* 파일에 `add_one` 함수를 추가합시다:

<span class="filename">파일명: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/add_one/src/lib.rs}}
```

이제 바이너리를 가지고 있는 `adder` 패키지와 이것이 의존하는 라이브러리를 갖고 있는
`add_one` 패키지를 갖추었습니다. 먼저 *adder/Cargo.toml*에 `add_one`의 경로
의존성을 추가할 필요가 있겠습니다.

<span class="filename">파일명: adder/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:6:7}}
```

카고는 작업 공간 내의 크레이트들이 서로 의존할 것이라고 가정하지 않으므로,
의존성 관계에 대해 명시할 필요가 있습니다.

다음으로 `adder` 크레이트에서 (`add_one` 크레이트에 있는) `add_one` 함수를
사용해봅시다. *adder/src/main.rs* 파일을 열어서 제일 윗 줄에 `use`을 추가하여
스코프로 새로운 `add_one` 라이브러리를 가져옵시다. 그런 다음 예제 14-7과
같이 `main` 함수를 수정하여 `add_one` 함수를 호출하세요.

<span class="filename">파일명: adder/src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/add/adder/src/main.rs}}
```

<span class="caption">예제 14-7: `adder` 크레이트에서 `add_one` 라이브러리
크레이트 사용하기</span>

최상위 *add* 디렉토리에서 `cargo build`를 실행하여 작업공간을
빌드해 봅시다!

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s
```

*add* 디렉토리에서 바이너리 크레이트를 실행하기 위해서는 `cargo run`에
`-p` 인자와 패키지명을 써서 작업공간 내의 어떤 패키지를 실행하고 싶은지
지정해야 합니다:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo run -p adder
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo run -p adder
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

이 명령은 *adder/src/main.rs*의 코드를 실행시키고, 이는 `add_one` 크레이트에 의존하고 있습니다.

#### 작업공간에서 외부 패키지 의존하기

작업공간에는 각 크레이트 디렉토리마다 *Cargo.lock*이 생기지 않고, 최상위에
하나의 *Cargo.lock*이 생긴다는 점을 주목하세요. 이는 모든 크레이트가 모든
의존성에 대해 같은 버전을 사용함을 보증합니다. *adder/Cargo.toml*과
*add_one/Cargo.toml*에 `rand` 패키지를 추가하면, 카고는 이 둘을
하나의 `rand` 버전으로 결정하여 하나의 *Cargo.lock*에 기록합니다.
작업공간 내 모든 크레이트가 동일한 의존성을 사용하도록 만드는 것은
이 크레이트들이 항상 서로 호환될 것임을 뜻합니다. **add_one/Cargo.toml*
파일의 `[dependencies]` 절에 `rand` 크레이트를 추가하여 `add_one`
크레이트에서 `rand` 크레이트를 사용해봅시다:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">파일명: add_one/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```

이제 *add_one/src/lib.rs* 파일에 `use rand;`를 추가할 수 있으며,
*add* 디렉토리에서 `cargo build`를 실행하여 전체 작업공간을 빌드하면
`rand` 크레이트를 가져와 컴파일할 것입니다. 아직 스코프로 가져온
`rand`를 참조하지 않았으므로 경고 하나를 받을 겁니다:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --snip--
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `add_one` (lib) generated 1 warning
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 10.18s
```

최상위의 *Cargo.lock*에는 이제 `add_one`의 `rand` 의존성에 대한 정보가
포함됩니다. 하지만 작업공간의 어딘가에서 `rand`가 사용되더라도 작업공간의 다른
크레이트의 *Cargo.toml* 파일에 마찬가지로 `rand`를 추가하지 않으면 이를
사용할 수 없습니다. 예를 들어 `use rand;`를 `adder` 패키지의
*adder/src/main.rs* 파일에 추가하면 다음과 같은 에러가 납니다:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-03-use-rand/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

이를 수정하려면 `adder` 패키지의 *Cargo.toml*을 고쳐서 이 패키지도
`rand`에 의존함을 알려주세요. `adder` 패키지를 빌드하면 *Cargo.lock*에
있는 `adder`에 대한 의존성 리스트에 `rand`를 추가하지만, `rand`의 추가
복제본을 내려받지는 않을 것입니다. 카고는 작업공간 내에서 `rand` 패키지를
사용하는 모든 패키지의 모든 크레이트가 동일한 버전을 사용할 것임을
보증하여 저장공간을 아끼고 작업공간 내의 크레이트들이 확실히 서로
호환되도록 합니다.

#### 작업공간에 테스트 추가하기

또다른 발전을 위해 `add_one::add_one` 함수의 테스트를 `add_one`
크레이트 내에 추가해봅시다:

<span class="filename">파일명: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```

이제 최상위 *add* 디렉토리에서 `cargo test`을 실행해보세요. 이런 구조의
작업공간에서 `cargo test`를 실행하면 작업공간의 모든 크레이트에 대한
테스트를 실행할 것입니다:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test
copy output below; the output updating script doesn't handle subdirectories in
paths properly
-->

```console
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.27s
     Running unittests src/lib.rs (target/debug/deps/add_one-f0253159197f7841)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-49979ff40686fa8e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

출력의 첫번째 절은 `add_one` 크레이트의 `it_works` 테스트가 통과되었음을
보여줍니다. 다음 절은 `adder` 크레이트에서 아무 테스트도 발견하지 못했음을
보여주고, 마지막 절에서는 `add_one` 크레이트 내에서 아무런 문서 테스트도
발견하지 못했음을 보여줍니다.

`-p` 플래그와 테스트하고자 하는 크레이트의 이름을 명시하면 최상위
디렉토리에서 어떤 작업공간 내 특정한 크레이트에 대한 테스트를 실행할
수도 있습니다:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p add_one
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo test -p add_one
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-b3235fea9a156f74)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

이 출력은 `cargo test`가 `add_one` 크레이트에 대한 테스트만 실행했으며
`adder` 크레이트의 테스트는 실행하지 않았음을 보여줍니다.

작업공간의 크레이트를 [crates.io](https://crates.io/)에 배포한다면,
작업공간 내 각 크레이트를 별도로 배포할 필요가 있습니다. `cargo test`처럼
`-p` 플래그와 배포하고자 하는 크레이트의 이름을 지정하여 작업공간 내의
특정 크레이트를 배포할 수 있습니다.

추가 연습으로 `add_one` 크레이트와 비슷한 방식으로 이 작업공간에 `add_two`
크레이트를 추가하세요!

여러분의 프로젝트가 커지면 작업공간 사용을 고려해보세요: 하나의 커다란 코드
덩어리 보다는 작고 개별적인 구성 요소들을 이해하는 것이 쉽습니다. 게다가 작업공간에
크레이트들를 유지하는 것은 이 크레이트들이 자주 동시에 변경될 경우 이들간의
조정을 더 쉽게 해줄 수 있습니다.
