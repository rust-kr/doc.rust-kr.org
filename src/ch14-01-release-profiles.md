# 릴리즈 프로필을 통한 빌드 커스터마이징하기

러스트에서의 *릴리즈 프로필 (release profiles)* 이란 설정값을 가지고 있는
미리 정의된, 커스터마이징 가능한 프로필인데, 이 설정값으로 프로그래머는 코드
컴파일을 위한 다양한 옵션을 제어할 수 있습니다. 각 프로필은 다른 프로필과
독립적으로 설정됩니다.

카고는 두 개의 주요 프로필을 가지고 있습니다: `cargo build`를 실행할 때
쓰는 `dev` 프로필과 `cargo build --release`를 실행할 때 쓰는 `release`
프로필이 바로 이 둘입니다. `dev` 프로필은 개발에 적합한 기본값으로 정의되었고,
`release` 프로필은 릴리즈 빌드용 설정을 기본값으로 가집니다.

이 프로필 이름이 빌드 출력에 나와서 익숙할 수도 있겠습니다:

<!-- manual-regeneration
anywhere, run:
cargo build
cargo build --release
and ensure output below is accurate
-->

```console
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
```

여기에서의 `dev`와 `release`가 바로 컴파일러에 의해 사용된 이 두 개의 프로필입니다.

카고에는 프로젝트의 *Cargo.toml* 파일에 `[profile.*]`절을 명시적으로
추가하지 않았을 경우 적용되는 각 프로필의 기본 설정이 있습니다.
커스터마이징을 원하는 프로필에 대해 `[profile.*]`절을 추가하면 이
기본 설정을 덮어씌울 수 있습니다. 여기 예시로 `opt-level` 설정에
대한 `dev` 와 `release` 프로필의 기본 설정값을 보여드리겠습니다:

<span class="filename">파일명: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

`opt-level` 설정은 러스트가 여러분의 코드에 적용할 최적화 수치이며,
0에서 3 사이의 값을 가집니다. 높은 최적화 수치를 적용할수록 컴파일
시간이 늘어나므로, 개발 중 코드를 자주 컴파일하는 상황이라면 코드의 실행
속도가 조금 느려지더라도 컴파일이 빨리 되도록 덜 최적화하길 원할 것입니다.
따라서 `dev`의 `opt-level` 기본값은 `0` 으로 되어 있습니다.
코드를 출시할 준비가 됐을 때라면 더 많은 시간을 컴파일에 쓰는 게 최상책입니다.
릴리즈 모드에서의 컴파일은 한 번이지만, 실행 횟수는 여러 번이니까요.
따라서 릴리즈 모드에서는 긴 컴파일 시간과 빠른 코드 실행 속도를 맞바꿉니다.
`release` 프로필의 `opt-level` 기본값이 `3`으로 되어 있는 이유는 이 때문입니다.

*Cargo.toml*에 기본 설정과 다른 값을 넣어서 기본 설정을 덮어씌울
수 있습니다. 예를 들어 개발용 프로필에 최적화 단계 1을 사용하고
싶다면, 프로젝트의 *Cargo.toml*에 아래의 두 줄을 추가하면
됩니다:

<span class="filename">파일명: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

이 코드는 기본 설정인 `0`을 덮어씌웁니다. 이제부터 `cargo build`를 실행할
때는 카고가 `dev` 프로필의 기본값과 커스터마이징된 `opt-level`을 사용하게
될 것입니다. `opt-level`을 `1`로 설정했으므로 카고는 릴리즈 빌드만큼은
아니지만 기본값보다 많은 최적화를 적용할 것입니다.

각 프로필의 설정 옵션 및 기본값의 전체 목록을 보시려면
[카고 공식 문서](https://doc.rust-lang.org/cargo/reference/profiles.html)를 참고해 주시기 바랍니다.
