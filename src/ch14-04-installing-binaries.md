<!-- Old link, do not remove -->
<a id="installing-binaries-from-cratesio-with-cargo-install"></a>

## `cargo install`로 Crates.io에 있는 바이너리 설치하기

`cargo install` 명령어는 로컬 환경에 바이너리 크레이트를 설치하고 사용할
수 있도록 해줍니다. 이는 시스템 패키지를 대체할 의도는 아닙니다; 러스트
개발자들이 [crates.io](https://crates.io)<!-- ignore -->에서 공유하고 있는 도구를
편리하게 설치할 수 있도록 하기 위함입니다. *바이너리 타겟 (binary target)*을
가진 패키지만 설치할 수 있음을 주의하세요. *바이너리 타겟*이란 *src/main.rs*
파일 혹은 따로 바이너리로 지정된 파일을 가진 크레이트가 생성해 낸 실행 가능한
프로그램을 말하는 것으로, 혼자서 실행될 수 없지만 다른 프로그램에 포함되기에
적합한 라이브러리 타겟과는 반대되는 의미입니다. 보통은 크레이트의 *README*
파일에 해당 크레이트가 라이브러리인지, 바이너리 타겟을 갖는지, 혹은 둘 다인지에
대한 정보가 담겨있습니다.

`cargo install`을 이용해 설치된 모든 바이너리는 설치 루트의 *bin*
디렉토리에 저장됩니다. 만약 여러분이 *rustup.rs*를 이용해 러스트를 설치했고
따로 설정을 건들지 않았다면, 이 디렉토리는 *$HOME/.cargo/bin*일 것입니다.
`cargo install`로 설치한 프로그램을 실행하려면 `$PATH` 환경변수에 해당
디렉토리가 등록되어 있는지 확인하세요.

예를 들면, 12장에서 파일 검색용 `grep` 도구의 러스트 구현체인
`ripgrep`이라는 게 있다고 언급했었지요. `ripgrep`을 설치하려면
다음과 같이 하면 됩니다:

<!-- manual-regeneration
cargo install something you don't have, copy relevant output below
-->

```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v13.0.0
  Downloaded 1 crate (243.3 KB) in 0.88s
  Installing ripgrep v13.0.0
--snip--
   Compiling ripgrep v13.0.0
    Finished release [optimized + debuginfo] target(s) in 3m 10s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v13.0.0` (executable `rg`)
```

출력의 마지막 두 줄은 설치된 바이너리의 경로와 이름을 보여주는데,
`ripgrep`의 경우에는 `rg`로군요. 방금 전에 언급했듯 여러분의 `$PATH`
환경변수에 설치된 디렉토리가 등록되어 있다면 명령창에서 `rg --help`를 실행할
수 있고, 파일을 찾을 때 더 빠르고 러스트다운 도구를 사용할 수 있습니다!
