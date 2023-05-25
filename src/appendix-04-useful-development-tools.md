## 부록 D - 유용한 개발 도구

이 부록에서는 러스트 프로젝트가 제공하는 유용한 개발 도구에 대해
알아보겠습니다. 자동 포맷팅, 경고 수정을 적용하는 빠른 방법, 린터,
IDE와의 통합 등을 살펴보겠습니다.

### `rustfmt`로 자동 포맷팅하기

`rustfmt` 도구는 커뮤니티 코드 스타일에 따라 여러분의 코드를 다시 포맷합니다.
많은 협업 프로젝트는 `rustfmt`를 사용하여 러스트를 작성할 때 사용할 스타일에
대한 논쟁을 방지합니다: 모든 사람들이 이 도구를 사용하여 코드를 포맷합니다.

`rustfmt` 도구를 설치하려면 다음을 입력하세요:

```console
$ rustup component add rustfmt
```

이 명령은 `rustc`와 `cargo`처럼 `rustfmt`와 `cargo-fmt`를 제공합니다.
어떤 카고 프로젝트를 포맷하려면, 다음을 입력하세요:

```console
$ cargo fmt
```

이 명령을 실행하면 현재 크레이트의 모든 러스트 코드를 다시 포맷합니다.
이 명령은 코드의 의미를 변경하지 않고 코드 스타일만 변경합니다.
`rustfmt`에 대한 자세한 내용은 [문서][rustfmt]를 참고하세요.

[rustfmt]: https://github.com/rust-lang/rustfmt

### `rustfix`로 코드 수정하기

`rustfix` 도구는 러스트 설치에 포함되어 있으며 원하는 문제를 해결할
수 있는 명확한 방법이 있는 컴파일러 경고를 자동으로 수정할 수
있습니다. 컴파일러 경고를 이미 본 적이 있을 것입니다. 예를 들어,
다음 코드를 살펴보겠습니다:

<span class="filename">파일명: src/main.rs</span>

```rust
fn do_something() {}

fn main() {
    for i in 0..100 {
        do_something();
    }
}
```

여기서는 `do_something` 함수를 100번 호출하지만, `for` 루프의 본문에서
`i` 변수를 사용하지 않습니다. 러스트는 이것에 대해 경고합니다:

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: unused variable: `i`
 --> src/main.rs:4:9
  |
4 |     for i in 0..100 {
  |         ^ help: consider using `_i` instead
  |
  = note: #[warn(unused_variables)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

이 경고는 대신에 `_i`라는 이름을 사용하라고 제안합니다: 밑줄은 이
변수를 사용하지 않을 것이라는 의도를 나타냅니다. `cargo fix` 명령을
실행하면 `rustfix` 도구를 사용하여 이 제안을 자동으로 적용할 수
있습니다:

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

*src/main.rs*를 다시 살펴보면, `cargo fix`가 코드를 변경했음을 알 수
있습니다:

<span class="filename">파일명: src/main.rs</span>

```rust
fn do_something() {}

fn main() {
    for _i in 0..100 {
        do_something();
    }
}
```

`for` 루프 변수가 이제 `_i`라는 이름이 되었고, 경고는 더 이상 나타나지 않습니다.

또한 `cargo fix` 명령을 사용하여 코드에 대해 서로 다른 러스트 에디션 간
변경을 적용할 수도 있습니다. 에디션은 부록 E에서 다룹니다.

### Clippy로 더 많은 린트 사용하기

Clippy 도구는 코드를 분석하여 일반적인 실수를 잡고 러스트 코드를
개선할 수 있도록 하는 린트 모음입니다.

Clippy를 설치하려면 다음을 입력하세요:

```console
$ rustup component add clippy
```

Clippy의 린트를 어떤 카고 프로젝트에 실행하려면 다음을 입력하세요:

```console
$ cargo clippy
```

예를 들어 다음과 같이 수학적 상수(예: pi)의 근사치를 사용하는 프로그램을
작성했다고 가정해 보겠습니다:

<span class="filename">파일명: src/main.rs</span>

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

`cargo clippy`를 이 프로젝트에 실행하면 다음과 같은 오류가 발생합니다:

```text
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: `#[deny(clippy::approx_constant)]` on by default
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
```

이 에러는 러스트에 이미 더 정확한 `PI` 상수가 정의되어 있으며, 프로그램이
이 상수를 대신 사용하도록 수정하면 더 정확해진다는 것을 알려줍니다.
그러면 여러분이 `PI` 상수를 사용하도록 코드를 변경할 수 있습니다.
다음 코드는 Clippy에서 어떠한 오류나 경고도 발생하지 않습니다:

<span class="filename">파일명: src/main.rs</span>

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

Clippy에 대한 더 많은 정보를 보려면 [Clippy 문서][clippy]를 참조하세요.

[clippy]: https://github.com/rust-lang/rust-clippy

### `rust-analyzer`를 사용한 IDE 통합

러스트 커뮤니티는 IDE 통합을 돕기 위해 [`rust-analyzer`][rust-analyzer]<!-- ignore -->를
추천합니다. 이 도구는 [언어 서버 프로토콜 (Language Server Protocol)][lsp]<!-- ignore -->을
사용하는 컴파일러 중심의 유틸리티 세트인데, 이 프로토콜은 IDE와
프로그래밍 언어가 서로 통신할 수 있도록 하는 사양입니다.
[Visual Studio Code의 Rust analyzer 플러그인][vscode]<!-- ignore -->과 같은
다른 클라이언트에서도 `rust-analyzer`를 사용할 수 있습니다.

[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

설치 방법을 보려면 `rust-analyzer` 프로젝트의 [홈페이지][rust-analyzer]<!-- ignore -->를
방문하고, 여러분의 IDE에 특정한 언어 서버 지원을 설치하세요.
여러분의 IDE는 자동 완성, 정의로 이동, 인라인 오류 등과 같은
기능을 얻게 될 것입니다.

[rust-analyzer]: https://rust-analyzer.github.io
