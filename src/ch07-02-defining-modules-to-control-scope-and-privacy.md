## 모듈을 정의하여 스코프 및 공개 여부 제어하기

이번에는 모듈, 아이템의 이름을 지정하는 *경로 (path)*,
스코프에 경로를 가져오는 `use` 키워드,
아이템을 공개하는 데 사용하는 `pub` 키워드를 알아보겠습니다.
`as` 키워드, 외부 패키지, 글롭 (glob) 연산자 등도 다룰 예정입니다.

우선은 여러분이 미래에 코드를 구조화할 때 쉽게 참조할 수 있는 규칙을
나열하는 것으로 시작해 보겠습니다. 그 다음 각각의 규칙에 대한 세부 사항을
설명해보겠습니다.

### 모듈 치트 시트

아래에 모듈, 경로, `use`, `pub` 키워드가 컴파일러에서 동작하는 방법과
대부분의 개발자들이 코드를 구성하는 방법에 대한 퀵 레퍼런스가 있습니다.
이 장을 거치면서 각각의 규칙에 대한 예제를 살펴볼 것이지만,
이곳이 모듈의 작동 방법을 기억하는 데에 참조할 좋은 위치가
되겠습니다.

- **크레이트 루트부터 시작**: 크레이트를 컴파일할 때 컴파일러는 먼저 크레이트
  루트 파일을 봅니다 (보통은 라이브러리 크레이트의 경우 *src/lib.rs* 혹은
  바이너리 크레이트의 경우 *src/main.rs* 입니다).
- **모듈 선언**: 크레이트 루트 파일에는 새로운 모듈을 선언할 수 있습니다;
  `mod garden;` 이라는 코드로 “garden” 모듈을 선언할 수 있습니다. 컴파일러는
  아래의 장소에서 이 모듈의 코드가 있는지 살펴볼 것입니다: 
  - 인라인, `mod garden` 뒤에 세미콜론 대신 중괄호를 쓴 곳의
    안쪽
  - *src/garden.rs* 파일 안
  - *src/garden/mod.rs* 파일 안
- **서브모듈 선언**: 크레이트 루트가 아닌 다른 파일에서는 서브모듈을 선언할 수
  있습니다. 예를 들면 *src/garden.rs* 안에 `mod vegetables;`를 선언할 수도
  있습니다. 컴파일러는 부모 모듈 이름의 디렉토리 안쪽에 위치한 아래의 장소들에서 
  이 서브모듈의 코드가 있는지 살펴볼 것입니다:
  - 인라인, `mod vegetables` 뒤에 세미콜론 대신 중괄호를 바로 쓴 곳의
    안쪽
  - *src/garden/vegetables.rs* 파일 안
  - *src/garden/vegetables/mod.rs* 파일 안
- **모듈 내 코드로의 경로**: 일단 모듈이 크레이트의 일부로서 구성되면, 공개 규칙이
  허용하는 한도 내에서라면 해당 코드의 경로를 사용하여 동일한 크레이트의 어디에서든
  이 모듈의 코드를 참조할 수 있게 됩니다. 예를 들면, garden vegetables 모듈 안에
  있는 `Asparagus` 타입은 `crate::garden::vegetables::Asparagus`로 찾아
  쓸 수 있습니다.
- **비공개 vs 공개**: 모듈 내의 코드는 그 부모 모듈에게는 기본적으로 비공개 (private)
  입니다. 모듈을 공개 (public) 로 만들려면, `mod` 대신 `pub mod`를 써서
  선언하세요. 공개 모듈 내의 아이템들을 공개하려면 마찬가지로 그 선언 앞에
  `pub`을 붙이세요.
- **`use` 키워드**: 어떤 스코프 내에서 `use` 키워드는 긴 경로의 반복을 줄이기
  위한 어떤 아이템으로의 단축경로를 만들어줍니다. `crate::garden::vegetables::Asparagus`를
  참조할 수 있는 어떤 스코프내에서라도 `use crate::garden::vegetables::Asparagus;`를
  사용하여 단축경로를 만들고 나면 그 스코프 내에서 이 타입을 사용할 때는
  `Asparagus`만 착성해주면 됩니다.

위의 규칙들을 보여주는 `backyard`라는 이름의 바이너리 크레이트를 만들어보았습니다.
디렉토리명 또한 `backyard`로서, 아래의 파일들과 디렉토리들로 구성되어 있습니다.

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

지금의 경우 크레이트 루트 파일은 *src/main.rs*이고, 내용은 아래와 같습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```

`pub mod garden;` 라인이 컴파일러에게 *src/garden.rs*에 있는 코드를 포함할 것을
알려주고, *src/garden.rs*는 아래와 같습니다:

<span class="filename">Filename: src/garden.rs</span>

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden.rs}}
```

여기 `pub mod vegetables;`은 *src/garden/vegetables.rs*의 코드 또한 포함되어야
함을 의미합니다. 이 코드는 아래와 같습니다:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden/vegetables.rs}}
```

이제 이 규칙들의 세부 사항으로 넘어가서 실제로 해보면서 확인합시다!

### 모듈로 관련된 코드 묶기

*모듈*은 크레이트 내 코드를 읽기 쉽고 재사용하기도 쉽게끔 구조화를 할 수 있게 해줍니다.
모듈 내의 코드는 기본적으로 비공개이므로, 모듈은 아이템의 *공개 여부 (privacy)* 를
제어하도록 해주기도 합니다. 비공개 아이템은 바깥쪽에서 사용이 허용되지 않는 내부의
새부 구현입니다. 모듈과 모듈 내 아이템을 선택적으로 공개할 수 있는데, 이렇게
하여 외부의 코드가 모듈 및 아이템을 의존하고 사용할 수 있도록
노출해줍니다.

예시로, 레스토랑 기능을 제공하는
라이브러리 크레이트를 작성한다고 가정해보죠.
코드 구조에 집중할 수 있도록 레스토랑을 실제 코드로 구현하지는 않고,
본문은 비워둔 함수 시그니처만 정의하겠습니다.

레스토랑 업계에서는 레스토랑을 크게 *접객 부서(front of house)*와
*지원 부서(back of house)*로 나눕니다. 접객 부서는 호스트가 고객을
안내하고, 웨이터가 주문 접수 및 결제를 담당하고, 바텐더가 음료를
만들어 주는 곳입니다. 지원 부서는 셰프, 요리사, 주방보조가 일하는 주방과
매니저가 행정 업무를 하는 곳입니다.

중첩 (nested) 모듈 안에 함수를 집어넣어 구성하면 크레이트 구조를 실제 레스토랑이
일하는 방식과 동일하게 구성할 수 있습니다. `cargo new --lib restaurant` 명령어를
실행하여 `restaurant` 라는 새 라이브러리를 생성하고, Listing 7-1 코드를 *src/lib.rs*에
작성하여 모듈, 함수 시그니처를 정의합시다. 아래는 접객 부서 쪽 코드입니다:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

<span class="caption">Listing 7-1: 함수를 포함하는 별도의 모듈을 포함한
`front_of_house` 모듈</span>

`mod` 키워드와 모듈 이름 (위의 경우 `front_of_house`)을 명시하여
모듈을 정의합니다. 모듈의 본문은 중괄호로 감싸져 있습니다.
`hosting`, `serving` 모듈처럼, 모듈 내에는 다른 모듈을 넣을
수 있습니다. 모듈에는 구조체, 열거형, 상수, 트레잇,
함수(Listing 7-1처럼) 등의 아이템 정의 또한 가질 수
있습니다.

모듈을 사용함으로서 관련된 정의드를 하나로 묶고 어떤 연관성이 있는지 이름을
지어줄 수 있습니다. 모듈화된 코드를 사용하는 프로그래머가 자신에게 필요한
어떠한 정의를 찾을 때, 모든 정의를 읽어 내릴 필요 없이 그룹 기반으로 탐색할
수 있으므로 훨씬 쉽게 찾아낼 수 있죠. 코드에 새로운 기능을 추가하려는 프로그래머도
자신이 어디에 코드를 작성해야 프로그램 구조를 그대로 유지할지 파악할 수 있습니다.

앞서 *src/main.rs*와 *src/lib.rs*는 크레이트 루트라고 부른다고
언급했습니다. 이 두 파일이 그런 이름을 갖게 된 이유는 *모듈 트리 (module tree)*라고
불리우는 크레이트 모듈 구조의 최상위(root)에 위치한 `crate` 라는 이름을 갖는
일종의 모듈을 형성하기 때문입니다.

Listing 7-2는 Listing 7-1의 구조를 모듈 트리로 나타낸 모습입니다.

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

<span class="caption">Listing 7-2: Listing 7-1 코드를 모듈 트리로
나타낸 모습</span>

트리는 모듈이 서로 어떻게 중첩되어 있는지 보여줍니다; 에를 들어 `hosting`
모듈은 `front_of_house` 내에 위치합니다. 이 트리는 또한 어떤 모듈이 서로
*형제 (sibling)* 관계에 있는지 나타내기도 하는데, 이는 동일한 모듈 내에
정의되어 있음을 말합니다; `hosting`과 `serving`은 `front_of_house` 모듈
내에 정의된 형제입니다. 모듈 A가 모듈 B 내에 있을 경우, 모듈 A는 모듈 B의
*자식* 이며, 모듈 B는 모듈 A의 *부모* 라고 말합니다. 전체 모듈 트리
최상위에 `crate` 라는 모듈이 암묵적으로 위치한다는 점을 기억해두세요.

모듈 트리에서 컴퓨터 파일 시스템의 디렉토리 트리를 연상하셨다면, 적절한 비유입니다!
파일 시스템의 디렉토리처럼, 여러분은 모듈로 코드를 조직화합니다.
또한 디렉토리에서 파일을 찾는 것처럼,
우리는 모듈을 찾아낼 방법이 필요하죠.

