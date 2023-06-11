## 별개의 파일로 모듈 분리하기

이번 장에서 여태 나온 모든 예제는 하나의 파일에 여러 모듈을 정의했습니다.
큰 모듈이라면, 정의를 여러 파일로 나누어 코드를 쉽게 찾아갈 수 있도록
만들어야겠죠.

예를 들어 여러 개의 레스토랑 관련 모듈을 가지고 있는 예제 7-17 코드로
시작해 봅시다. 크레이트 루트 파일에 모든 모듈이 정의되는 형태 대신
이 모듈들을 파일로 추출해 보겠습니다. 이 경우 크레이트 루트 파일은
*src/lib.rs*지만, 이러한 절차는 크레이트 루트 파일이 *src/main.rs*인
바이너리 크레이트에서도 작동합니다.

먼저 `front_of_house` 모듈을 파일로 추출하겠습니다. `front_of_house`
모듈에 대한 중괄호 내부의 코드를 지우고 `mod front_of_house;` 선언
부분만 남겨서, *src/lib.rs*가 예제 7-21의 코드만 있도록 해봅시다.
예제 7-22의 *src/front_of_house.rs* 파일을 만들기 전까지는
컴파일되지 않음을 유의하세요.

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

<span class="caption">예제 7-21: 본문이 *src/front_of_house.rs*에 위치할
`front_of_house` 모듈 선언하기</span>

다음으로 예제 7-22처럼 *src/front_of_house.rs*이라는 새 파일을
만들어 중괄호 안에 있던 코드를 위치시킵니다. 크레이트 루트에
`front_of_house`라는 이름의 모듈 선언이 나왔으므로 컴파일러는
이 파일을 살펴봐야 한다는 것을 알게 됩니다.

<span class="filename">파일명: src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

<span class="caption">예제 7-22: *src/front_of_house.rs* 파일에
`front_of_house` 모듈 본문 정의하기</span>

모듈 트리에서 `mod` 선언을 사일을 로드하는 것은 *한 번만* 하면 됩니다.
일단 그 파일이 프로젝트의 일부란 것을 컴파일러가 파악하면
(그래서 모듈 트리 내 `mod` 구문을 집어넣은 곳 옆에 코드가 있음을 알게 되면),
[‘경로를 사용하여 모듈 트리의 아이템 참조하기’][paths]<!-- ignore -->절에서
다루었던 것처럼 프로젝트의 다른 파일들은 선언된 위치의 경로를 사용하여
로드된 파일의 코드를 참조해야 합니다. 즉, `mod`는 다른
프로그래밍 언어에서 볼 수 있는 ‘포함하기 (include)’ 연산이
*아닙니다*.

다음으로 `hosting` 모듈을 파일로 추출하겠습니다. `hosting`이
루트 모듈이 아니라 `front_of_house`의 자식 모듈이기 때문에 과정이
약간 다릅니다. `hosting`의 파일을 모듈 트리 내 부모의 이름이 될
새 디렉터리, 즉 이 경우에는 *src/front_of_house/*에 위치시키겠습니다.

`hosting`을 옮기는 작업을 시작하기 위하여, *src/front_of_house.rs*에는
`hosting` 모듈의 선언만 있도록 수정합니다:

<span class="filename">파일명: src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

그다음 *src/front_of_house* 디렉터리를 만들고 그 안에 *hosting.rs*
파일을 생성한 다음 `hosting` 모듈 내용을 작성합니다:

<span class="filename">파일명: src/front_of_house/hosting.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

*hosting.rs*를 *src/front_of_house* 대신 *src* 디렉터리에 넣으면 컴파일러는
*hosting.rs* 코드가 `front_of_house` 모듈의 하위에 선언되지 않고 크레이트 루트에
선언된 `hosting` 모듈에 있을 것으로 예상합니다. 어떤 파일에서
어떤 모듈의 코드를 확인할지에 대한 컴파일러의 규칙은 디렉터리와 파일이
모듈 트리와 더 밀접하게 일치한다는 것을 의미합니다.

> ### 대체 파일 경로
>
> 지금까지는 러스트 컴파일러가 사용하는 가장 관용적인 파일 경로를 다루었지만,
> 러스트는 예전 스타일의 파일 경로 또한 지원합니다. 크레이트 루트 내에
> 선언된 `front_of_house` 모듈에 대하여, 컴파일러는 다음의 위치에서 모듈의
> 코드를 찾아볼 것입니다:
>
> * *src/front_of_house.rs* (우리가 지금 다룬 형태)
> * *src/front_of_house/mod.rs* (예전 스타일, 여전히 지원되는 경로)
>
> `front_of_house`의 서브모듈인 `hosting`이라는 모듈에 대해서는 다음의
> 위치에서 모듈의 코드를 찾아볼 것입니다:
>
> * *src/front_of_house/hosting.rs* (우리가 지금 다룬 형태)
> * *src/front_of_house/hosting/mod.rs* (예전 스타일, 여전히 지원되는 경로)
>
> 만약 같은 모듈에 대해 두 스타일 모두를 사용하면 컴파일 에러가 납니다. 같은
> 프로젝트에서 서로 다른 모듈에 대해 양쪽 스타일을 섞어 사용하는 것은 허용되지만,
> 프로젝트를 살펴보는 사람들에게 혼란을 줄 가능성이 있습니다.
>
> *mod.rs*라는 이름의 파일을 사용하는 스타일의 주요 단점은 프로젝트에 여러 파일의
> 이름이 *mod.rs*로 끝나게 되어, 에디터에서 이 파일들을 동시에 열어두었을 때
> 헷갈릴 수 있다는 점입니다.

각 모듈의 코드를 별도의 파일로 옮겼고, 모듈 트리는 동일한 상태로 남아있습니다.
`eat_at_restaurant` 내의 함수 호출은 그 정의가 다른 파일들에 있다 하더라도
아무 수정 없이 동작할 것입니다. 이러한 기술은 모듈의 크기가 증가했을 때
이를 새로운 파일로 옮기도록 해줍니다.

*src/lib.rs* 파일의 `pub use crate::front_of_house::hosting` 구문을 변경하지 않았으며,
`use` 구문이 크레이트의 일부로 컴파일되는 파일에 영향을 주지 않는다는 점도 주목해 주세요.
`mod` 키워드는 모듈을 선언하고,
러스트는 모듈과 같은 이름의 파일에서
해당 모듈에 들어가는 코드를 찾습니다.

## 정리

러스트에서는 패키지를 여러 크레이트로 나누고, 크레이트를 여러 모듈로 나누어
한 모듈에 정의된 아이템을 다른 모듈에서 참조할 수 있게 해줍니다. 절대 경로나
상대 경로를 지정하면 이를 수행할 수 있습니다. 이러한 경로는 `use` 구문을
사용해 스코프 안으로 가져올 수 있으므로 해당 스코프에 있는 아이템을 여러 번
사용해야 할 때 더 짧은 경로를 사용할 수 있습니다. 모듈 코드는 기본적으로
비공개지만, `pub` 키워드를 추가해 정의를 공개할 수 있습니다.

다음 장에서는 이렇게 깔끔하게 구성된 여러분의 코드에서 사용할 수 있는
표준 라이브러리의 컬렉션 자료구조를 몇 가지 살펴보겠습니다.

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
