## 패키지, 크레이트

모듈 시스템에서 처음 다뤄볼 내용은 패키지와 크레이트입니다.
크레이트는 바이너리일 수도 있고, 라이브러리일 수도 있습니다.
러스트 컴파일러는 *크레이트 루트* 라는 소스 파일부터 컴파일을 시작해서
여러분이 작성한 크레이트의 루트 모듈을 구성합니다.
(모듈은 ["모듈을 정의하여 스코프 및 공개 여부 제어하기"][modules]에서 알아볼 예정입니다)
*패키지* 는 하나 이상의 크레이트로 기능을 구성해 제공합니다.
패키지 내 *Cargo.toml* 파일은 패키지의 크레이트를 빌드하는 법을 나타냅니다.

패키지에 무엇을 포함할 수 있는가에 대해서는 규칙이 몇 가지 있습니다.
라이브러리 크레이트는 *하나만* 넣을 수 있습니다.
바이너리 크레이트는 원하는 만큼 포함할 수 있습니다.
단, 패키지에는 적어도 하나 이상의 크레이트(라이브러리이건, 바이너리이건)가 포함되어야 합니다.

패키지를 생성할 때 어떤 일이 일어나는지 살펴보죠.
먼저 `cargo new` 명령어를 입력합니다.

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

명령어를 입력하면 Cargo는 *Cargo.toml* 파일을 생성하여, 새로운 패키지를 만들어 줍니다.
패키지명과 같은 이름의 바이너리 크레이트는 크레이트 루트가 *src/main.rs* 라는 규칙이 있기 때문에,
*Cargo.toml* 파일을 살펴보아도 *src/main.rs* 가 따로 언급되진 않습니다.
마찬가지로, 패키지 디렉토리에 *src/lib.rs* 파일이 존재할 경우,
Cargo는 해당 패키지가 패키지명과 같은 이름의 라이브러리 크레이트를 포함하고 있다고 판단합니다.
물론 그 라이브러리 크레이트의 크레이트 루트는 *src/lib.rs* 이고요.
Cargo는 크레이트를 빌드할 때(라이브러리이건, 바이너리이건)
크레이트 루트 파일을 `rustc` 로 전달합니다.

현재 패키지는 *src/main.rs* 만 포함하고 있으므로 `my-project` 바이너리 크레이트만 포함합니다.
만약 어떤 패키지가 *src/main.rs* 와 *src/lib.rs* 를 포함한다면 해당 패키지는
패키지와 이름이 같은 바이너리, 라이브러리 크레이트를 포함하게 됩니다.
*src/bin* 디렉토리 내에 파일을 배치하면 각각의 파일이 바이너리 크레이트가 되어,
여러 바이너리 크레이트를 패키지에 포함할 수 있습니다.

크레이트는 관련된 기능을 그룹화함으로써
특정 기능을 쉽게 여러 프로젝트 사이에서 공유합니다.
예를 들어, [2장][rand]<!-- ignore --> 에서 사용한 `rand` 크레이트는
랜덤한 숫자를 생성하는 기능을 제공합니다.
우린 프로젝트 스코프에 `rand` 크레이트를 가져오기만 하면
우리가 만든 프로젝트에서 랜덤 숫자 생성 기능을 이용할 수 있죠.
`rand` 크레이트가 제공하는 모든 기능은 크레이트의 이름인 `rand` 를 통해 접근 가능합니다.

크레이트의 기능이 각각의 스코프를 갖도록 하면 특정 기능이 우리 크레이트에 있는지,
`rand` 크레이트에 있는지를 명확하게 알 수 있으며,
잠재적인 충돌을 방지할 수도 있습니다.
예를 들어, 우리가 만든 크레이트에 `Rng` 라는 이름의 구조체를 정의한 상태로,
`Rng` 트레잇을 제공하는 `rand` 크레이트를 의존성에 추가하더라도
컴파일러는 `Rng` 라는 이름이 무엇을 가리키는지 정확히 알 수 있습니다.
`Rng` 는 우리가 만든 크레이트 내에서 정의한 `struct Rng` 를 가르키고,
`rand` 크레이트의 `Rng` 트레잇은 `rand::Rng` 로 접근해야 하죠.

이어서 계속 모듈 시스템을 다뤄보죠!

[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#임의의-숫자를-생성하기
