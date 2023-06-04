## 카고를 사용해봅시다

카고 (Cargo) 는 러스타시안이라면 대부분 사용하는 러스트 빌드 시스템 및
패키지 매니저입니다. 이 도구는 코드 빌드나, 코드 작성에 필요한 외부 라이브러리를
다운로드할 때나, 라이브러리를 제작할 때 겪는 귀찮은 일들을 상당수 줄여주는 편리한
도구입니다. (앞으로 외부 라이브러리는 *의존성 (dependency)* 이라고
지칭하겠습니다.)

여태 우리가 작성해 본 간단한 러스트 프로그램에는 의존성을 추가하지 않았습니다.
카고를 가지고 “Hello, world!” 프로젝트를 만들었다면, 코드 빌드를 처리하는
카고의 기는 일부만을 사용했을 것입니다. 훗날 복잡한 프로그램을 작성하게
되면 의존성을 추가하게 될 것이고, 카고를 사용하여 프로젝트를 시작하면
의존성을 추가하는 일이 훨씬 더 쉬워질 것입니다.

러스트 프로젝트 대부분이 카고를 사용하고 있기 때문에, 이 책의 이후
내용도 여러분이 카고를 사용한다는 전제로 작성했습니다.
[“러스트 설치”][installation]<!-- ignore --> 절을 따라 하셨다면
이미 카고가 설치되어 있을 테니 따로 설치하실 필요는 없으나,
다른 방법을 이용하신 경우엔 다음 명령어로
카고가 설치되어 있는지 확인하시기 바랍니다:

```console
$ cargo --version
```

버전 숫자가 나타나면 정상입니다.
`command not found` 등 에러가 나타날 경우 여러분이 설치하면서
참고한 문서에서 카고를 따로 설치하는 방법을 찾아보세요.

### 카고로 프로젝트 생성하기

카고로 프로젝트를 생성해 보고
앞서 만들었던 “Hello, world!” 프로젝트와 비교해 봅시다.
*projects* 디렉터리로 (다른 곳에 코드를 만드신 분은 해당 위치로)
돌아가 다음 명령어를 실행해 보세요.

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

첫 번째 명령어는 *hello_cargo*라는 디렉터리를 생성합니다.
우리는 프로젝트의 이름을 *hello_cargo* 로 지정했고
카고는 동일한 이름의 디렉터리 안에 파일들을 생성합니다.

*hello_cargo* 디렉터리로 이동해 파일을 살펴보면
*Cargo.toml* 파일과 *src* 디렉터리를 확인할 수 있으며,
*src* 디렉터리 내에는 *main.rs* 파일이 있는 것도 볼 수 있습니다.

그 외에도 *.gitignore* 파일과 함께 새 Git 저장소가 초기화됩니다. 여러분이
이미 Git 저장소가 있는 디렉터리에서 `cargo new`를 실행시킨다면 Git 파일들은
생성되지 않을 것입니다. 이 동작은 `cargo new --vcs=git` 명령을 통해 덮어쓸 수 있습니다.

> Note: Git은 일반적으로 사용하는 버전 관리 시스템입니다. 따라서 기본 설정되어 있으며,
> 이 설정은 `cargo new` 명령어의 `--vcs` 플래그로 변경할 수 있습니다.
> 그 외의 다른 옵션들은 `cargo new --help`로 확인할 수 있습니다.

이제 텍스트 에디터로 *Cargo.toml* 을 열어보세요.
예제 1-2처럼 나오면 정상입니다.

<span class="filename">파일명: Cargo.toml</span>

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

<span class="caption">예제 1-2: `cargo new`로 생성한
*Cargo.toml* 파일의 내용</span>

이 파일은 [*TOML*](https://toml.io)<!-- ignore --> (*Tom’s Obvious, Minimal
Language*) 포맷으로 되어있고, 이 포맷은 카고 설정에서 사용하는 포맷입니다.

`[package]`라고 적힌 첫 번째 라인은 섹션 헤더로,
뒤에 패키지 설정 구문들이 따라오는 걸 보실 수 있습니다.
나중에 우리가 이 파일에 내용을 추가하며 새로운 섹션을 만들어 볼 겁니다.

다음 세 라인은 카고가 코드를 컴파일하는 데 필요한 설정 정보로,
각각 패키지명, 버전, 작성자, 사용하는 러스트 에디션을 나타냅니다.
`edition` 키에 대한 설명은 [부록 E][appendix-e]<!-- ignore -->에서 다룹니다.

마지막 라인의 `[dependencies]`는 프로젝트에서 사용하는 의존성
목록입니다. 러스트에서는 코드 패키지를 *크레이트 (crate)* 라고
부릅니다. 이 프로젝트에는 크레이트가 필요 없지만, 2장 첫 프로젝트에서는
필요하므로 그때 사용해 보겠습니다.

이제 *src/main.rs*를 열어 살펴봅시다:

<span class="filename">파일명: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

카고가 “Hello, world!” 프로그램을 만들어 놨네요.
예제 1-1에서 만든 프로젝트와 다른 점은
이번엔 코드 위치가 *src* 디렉터리라는 점과
최상위 디렉터리에 *Cargo.toml* 설정 파일이 존재한다는 점입니다.

카고는 소스 파일이 *src* 내에 있다고 예상합니다.
최상위 프로젝트 디렉터리를 README, 라이선스, 설정 파일 등
코드 자체와는 관련 없는 파일들을 저장하는 데 사용됩니다.
이처럼 카고는 각각의 파일을 알맞은 위치에 배치하여
여러분이 프로젝트를 조직화하는 걸 돕습니다.

“Hello, world!” 프로젝트에서처럼 프로젝트 생성 시 카고를
사용하지 않았어도, *Cargo.toml* 파일을 알맞게 작성하고 프로젝트
코드를 *src* 디렉터리로 옮기면, 카고를 사용하는 프로젝트로 변경이
가능합니다.

### 카고로 프로젝트를 빌드하고 실행하기

이제 카고로 생성한 “Hello, world!” 프로그램은 빌드하고 실행했을
때 어떤 점이 다른지 확인해 봅시다! *hello_cargo* 디렉터리에서
다음 명령어를 이용해 프로젝트를 빌드해주세요:

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

이 명령어는 현재 디렉터리가 아닌 *target/debug/hello_cargo*
(Windows에서는 *target\debug\hello_cargo.exe*)에 실행 파일을 생성합니다.
기본 빌드가 디버그 빌드기 때문에, 카고는 *debug*라는 디렉터리에 바이너리를
생성합니다. 실행 파일은 다음 명령어로 실행할 수 있습니다:

```console
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

터미널에 `Hello, world!`가 출력되면 제대로 진행된 겁니다.
처음 `cargo build` 명령어를 실행하면 최상위 디렉터리에
*Cargo.lock* 파일이 생성될 텐데, 이 파일은 프로젝트에서
사용하는 의존성의 정확한 버전을 자동으로 기록해 두는
파일이니 여러분이 직접 수정할 필요는 없습니다. 물론 이번
프로젝트는 의존성을 갖지 않으므로 현재는 파일에 특별한 내용이 없습니다.

방금은 `cargo build`로 빌드한 후 `./target/debug/hello_cargo`
명령어로 실행했지만, 컴파일과 실행을 한 번에 진행하는 `cargo run`
명령어도 있습니다:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

`cargo run`을 사용하면 `cargo build` 실행 후 바이너리 경로를
입력해서 실행하는 것보다 편리하므로, 대부분의 개발자들이 `cargo run`을
이용합니다.

출력 내용에 `hello_cargo`를 컴파일 중이라는 내용이 없는 걸 눈치채셨나요?
이는 카고가 파일 변경 사항이 없음을 알아채고 기존 바이너리를 그대로
실행했기 때문입니다. 소스 코드를 수정한 뒤 명령어를 다시 실행해 보면 다음과
같이 프로젝트를 다시 빌드한 후에 바이너리를 실행함을 알 수
있습니다.

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

카고에는 `cargo check`라는 명령어도 있는데, 이는 실행 파일은 생성하지 않고
작성한 소스가 문제없이 컴파일되는지만 빠르게 확인하는 명령어입니다.

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

실행 파일도 생성하지 않는 명령어가 왜 필요할까요? `cargo check`는 실행
파일을 생성하는 단계를 건너뛰기 때문에 `cargo build`보다 훨씬 빠릅니다.
코드를 작성하는 동안 여러분의 프로젝트가 컴파일되는지 지속적으로 검사하려면,
`cargo check` 사용이 코드가 계속 컴파일되는지 확인하는 과정을 빠르게
해줄 것입니다! 러스타시안은 대부분 주기적으로 이 명령어를 실행해
코드에서 컴파일 문제가 발생하지 않는지 확인하고, 실행 파일이 필요할 경우에만
`cargo build`를 사용합니다.

여태까지 카고에 대해 배운 내용을 복습해 봅시다:

* `cargo new`로 새 프로젝트를 생성할 수 있습니다.
* `cargo build` 명령으로 프로젝트를 빌드할 수 있습니다.
* `cargo run` 명령어는 한 번에 프로젝트를 빌드하고 실행할 수 있습니다.
* `cargo check` 명령으로 바이너리를 생성하지 않고 프로젝트의 에러를
  체크할 수 있습니다.
* 빌드로 만들어진 파일은 작성한 소스 코드와 뒤섞이지 않도록
  *target/debug* 디렉터리에 저장됩니다.

운영체제에 상관없이 같은 명령어를 사용한다는 것도
카고 사용으로 얻는 추가적인 장점입니다.
따라서 이 시점부터는 운영체제별로 명령어를 따로 알려드리지 않겠습니다.

### 릴리즈 빌드 생성하기

프로젝트를 완성해서 배포(릴리즈)할 준비가 끝났다면, `cargo build --release`
명령어를 사용해 릴리즈 빌드를 생성할 수 있습니다. 일반 빌드와 차이점은
*target/debug* 가 아닌 *target/release* 에 실행 파일이 생성된다는 점,
그리고 컴파일 시 최적화를 진행하여 컴파일이 오래 걸리는 대신 러스트 코드가 더 빠르게 작동하는 점입니다.
릴리즈 빌드가 더 빠르게 작동한다면, 왜 일반 빌드시에는 최적화를 진행하지 않을까요?
이에 대한 해답은 빌드가 두 종류로 나뉘게 된 이유이기도 한데, 개발 중에는 빌드가
잦으며 작업의 흐름을 끊지 않기 위해 빌드 속도 또한 빠를수록 좋지만, 배포용
프로그램은 잦은 빌드가 필요 없으며 빌드 속도보단 프로그램의 작동 속도가 더 중요하기
때문입니다. 이와 같은 이유로, 작성한 코드 작동 속도를 벤치마킹할 시에는
릴리즈 빌드를 기준으로 해야 한다는 것도 알아두시기 바랍니다.

### 관례로서의 카고

카고는 단순한 프로젝트에서는 그냥 `rustc`만 사용할 때와 비교하여
큰 값어치를 못하지만, 프로그램이 더욱 복잡해지면 그 가치를 증명할
것입니다. 여러 개의 파일 혹은 의존성을 필요로 하는 복잡한 프로젝트에서는
카고가 빌드를 조정하게 하는 것이 훨씬 쉽습니다.

`hello_cargo` 프로젝트는 단순하지만, 이미 여러분은 앞으로 러스트를 사용하며
쓰게 될 카고 명령어 중 대부분을 써본 것과 다름없습니다. 실제로 기존에 있던
러스트 프로젝트를 Git으로 가져와서, 해당 디렉터리로 이동하고, 빌드하는 과정은
다음과 같은 명령을 이용하면 됩니다.

```console
$ git clone someurl.com/someproject
$ cd someproject
$ cargo build
```

더 자세한 내용은 [카고 문서][cargo]를 확인하세요.

## 정리

여러분은 이미 러스트 여정의 위대한 시작을 한 발 내디뎠습니다! 이번 장에서 배운
내용은 다음과 같습니다:

* `rustup` 으로 최신 stable 버전 러스트를 설치하기
* 러스트를 새 버전으로 업데이트하기
* 로컬 설치된 문서 열어보기
* 직접 `rustc`를 사용해 “Hello, world!” 프로그램을 작성하고 실행해 보기
* 일반적인 카고의 사용법으로 프로젝트를 생성하고 실행하기

지금이 좀 더 실질적인 프로그램을 만들어 코드를 읽고 쓰는 데 익숙해지기
좋은 타이밍입니다. 그리하여 2장은 추리 게임 프로그램을 만들어 보겠습니다.
러스트에서 사용되는 보편적인 프로그래밍 개념부터 살펴보실 분들은
3장부터 읽고 2장을 읽는 것도 나쁘지 않습니다.

[installation]: ch01-01-installation.html#러스트-설치
[toml]: https://toml.io
[appendix-e]: appendix-05-editions.html
[cargo]: https://doc.rust-lang.org/cargo/
