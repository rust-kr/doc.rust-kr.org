## 경로를 사용하여 모듈 트리의 아이템 참조하기

러스트 모듈 트리에서 아이템을 찾는 방법은,
파일 시스템에서 경로를 사용하는 방법과 동일합니다.
함수를 호출하려면 그 함수의 경로를 알아야 합니다.

경로는 두 가지 형태가 존재합니다.

* *절대 경로 (absolute path)* 는 크레이트 루트로부터 시작되는 전체 경로입니다;
  외부 크레이트로부터의 코드에 대해서는 해당 크레이트 이름으로 절대 경로가 시작되고
  현재의 크레이트로부터의 코드에 대해서는 `crate` 리터럴로부터 시작됩니다.
* *상대 경로 (relative path)* 는 현재의 모듈을 시작점으로 하여
  `self`, `super` 혹은 현재 모듈 내의 식별자를 사용합니다.

절대 경로, 상대 경로 뒤에는 `::`으로 구분된 식별자가
하나 이상 따라옵니다.

예제 7-1로 돌아와서, `add_to_waitlist` 함수를 호출하고 싶다고 칩시다.
이는 다음 질문과 같습니다: `add_to_waitlist` 함수의 경로는 무엇일까요?
예제 7-3은 예제 7-1의 일부 모듈과 함수를 제거한 내용을 담고
있습니다.

예제는 크레이트 루트에 정의된 `eat_at_restaurant`라는 새로운 함수에서
`add_to_waitlist` 함수를 호출하는 두 가지 방법을 보여줍니다. 두 경로 모두
맞지만, 이 예제를 이대로 컴파일되지 못하게 하는 다른 문제가 남아있습니다.
무슨 이유인지는 곧 설명하겠습니다.

`eat_at_restaurant` 함수는 우리가 만든 라이브러리 크레이트의 공개 API 중
하나입니다. 따라서 `pub` 키워드로 지정되어 있습니다. `pub`에 대해서는
[‘`pub` 키워드로 경로 노출하기’][pub]<!-- ignore --> 절에서 자세히 알아볼 예정입니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

<span class="caption">예제 7-3: 절대 경로와 상대 경로로
`add_to_waitlist` 함수 호출하기</span>

`eat_at_restaurant` 함수에서 처음 `add_to_waitlist` 함수를 호출할 때는 절대
경로를 사용했습니다. `add_to_waitlist` 함수는 `eat_at_restaurant` 함수와
동일한 크레이트에 정의되어 있으므로, 절대 경로의 시작점에 `crate` 키워드를 사용할
수 있습니다. 그 뒤로는 `add_to_waitlist` 함수에 도달할 때까지의 이어지는 모듈을
포함시켰습니다. 같은 구조의 파일 시스템을 생각해 볼 수 있습니다:
`/front_of_house/hosting/add_to_waitlist` 경로를 써서 `add_to_waitlist`
프로그램을 실행했군요; `crate`를 작성해 크레이트 루트를 기준으로 사용하는 것은
셸 (shell) 에서 `/` 로 파일 시스템의 최상위 디렉터리를 기준으로 사용하는 것과 같습니다.

`eat_at_restaurant` 함수에서 두 번째로 `add_to_waitlist` 함수를 호출할
때는 상대 경로를 사용했습니다. 경로는 모듈 트리에서 `eat_at_restaurant`
함수와 동일한 위치에 정의되어 있는 `front_of_house` 모듈로 시작합니다.
파일 시스템으로 비유하자면 `front_of_house/hosting/add_to_waitlist`가
되겠네요. 모듈 이름으로 시작한다는 것은 즉 상대
경로를 의미합니다.

상대 경로, 절대 경로 중 무엇을 사용할지는 프로젝트에 따라, 그리고
아이템을 정의하는 코드와 아이템을 사용하는 코드를 분리하고 싶은지,
혹은 같이 두고 싶은지에 따라 여러분이 결정해야 할 사항입니다.
예를 들어, `front_of_house` 모듈과 `eat_at_restaurant` 함수를
`customer_experience`라는 모듈 내부로 이동시켰다고 가정해 보죠.
`add_to_waitlist` 함수를 절대 경로로 작성했다면 코드를 수정해야 하지만,
상대 경로는 수정할 필요가 없습니다. 반면, `eat_at_restaurant` 함수를 분리하여
`dining`이라는 모듈 내부로 이동시켰다면, `add_to_waitlist` 함수를 가리키는
절대 경로는 수정할 필요가 없지만, 상대 경로는 수정해야 합니다. 일반적으로 선호하는
경로는 절대 경로입니다. 아이템을 정의하는 코드와 호출하는 코드는 분리되어 있을
가능성이 높기 때문입니다.

이제 예제 7-3이 컴파일되지 않는 이유를 알아봅시다!
컴파일 시 나타나는 에러는 예제 7-4와 같습니다.

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

<span class="caption">예제 7-4: 예제 7-3 코드 컴파일 시
발생하는 에러</span>

에러 메시지는 `hosting` 모듈이 비공개 (private) 라는 내용입니다.
`hosting` 모듈과 `add_to_waitlist` 함수의 경로를 정확히 명시했지만,
해당 영역은 비공개 영역이기 때문에 러스트가 접근을 허용하지 않습니다.
러스트에서는 (함수, 메서드, 구조체, 열거형, 모듈, 그리고 상수 등) 모든
아이템이 기본적으로 부모 모듈에 대해 비공개입니다. 함수나 구조체 같은
아이템을 비공개로 하고 싶다면 모듈에 넣으면 됩니다.

부모 모듈 내 아이템은 자식 모듈 내 비공개 아이템을 사용할 수 없지만,
자식 모듈 내 아이템은 부모 모듈 내 아이템을 사용할 수 있습니다.
이유는, 자식 모듈의 세부 구현은 감싸져서 숨겨져 있지만,
자식 모듈 내에서는 자신이 정의된 컨텍스트를 볼 수 있기 때문입니다.
레스토랑 비유로 돌아와, 비공개 규칙을 레스토랑의 지원 부서로 생각해 보죠.
레스토랑 고객들은 내부에서 진행되는 일을 알 수 없지만, 사무실 관리자는
자신이 운영하는 레스토랑의 모든 것을 보고, 행동할 수 있습니다.

러스트 모듈 시스템은 내부의 세부 구현을 기본적으로 숨기도록 되어 있습니다.
이로써, 여러분은 외부 코드의 동작을 망가뜨릴 걱정 없이
수정할 수 있는 코드가 어느 부분인지 알 수 있죠. 그렇지만
러스트에서는 `pub` 키워드를 사용하여 자식 모듈의 내부 구성 요소를
공개 (public) 함으로써 외부의 상위 모듈로 노출할 방법을 제공합니다.

### `pub` 키워드로 경로 노출하기

`hosting` 모듈이 비공개라고 했던 예제 7-4 에러로 돌아와 보죠.
부모 모듈 내 `eat_at_restaurant` 함수가 자식 모듈 내 `add_to_waitlist`
함수에 접근해야 하니, `hosting` 모듈에 `pub` 키워드를 추가했습니다.
작성한 모습은 예제 7-5와 같습니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs}}
```

<span class="caption">예제 7-5: `eat_at_restaurant` 함수에서 `hosting` 모듈을
사용할 수 있도록 `pub`으로 선언</span>

안타깝게도, 예제 7-5 코드 또한
예제 7-6과 같은 에러가 발생합니다.

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

<span class="caption">예제 7-6: 예제 7-5 코드 컴파일 시 발생하는
에러</span>

어떻게 된 걸까요? `mod hosting` 앞에 `pub` 키워드를 추가하여 모듈이 공개되었습니다.
따라서, `front_of_house`에 접근할 수 있다면 `hosting` 모듈에도 접근할 수 있죠.
하지만, `hosting` 모듈의 *내용*은 여전히 비공개입니다. 모듈을 공개했다고 해서
내용까지 공개되지는 않습니다. 모듈의 `pub` 키워드는 상위 모듈이 해당 모듈을 가리킬
수 있도록 할 뿐, 그 내부 코드에 접근하도록 하는 것은 아닙니다.
모듈은 단순한 컨테이너이기 때문에 모듈을 공개하는 것 만으로 할 수 있는
것은 별로 없으며, 여기에 더해서 모듈이 가지고 있는 아이템도
마찬가지로 공개해야 합니다.

예제 7-6의 에러는 `add_to_waitlist` 함수가 비공개라는 내용입니다.
비공개 규칙은 구조체, 열거형, 함수, 메서드, 모듈 모두에게
적용됩니다.

예제 7-7처럼 `add_to_waitlist` 함수도
정의에 `pub` 키워드를 추가하여 공개해 봅시다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs}}
```

<span class="caption">예제 7-7: `mod hosting`, `fn add_to_waitlist` 에
`pub` 키워드를 추가해 `eat_at_restaurant` 함수에서
호출 가능하도록 만들기</span>

드디어 코드를 컴파일할 수 있습니다! `pub` 키워드를 추가하는 것이 어째서 비공개
규칙과 관련하여 `add_to_waitlist`에서 이러한 경로를 사용할 수 있게 하는지
알아보기 위해서, 절대 경로와 상대 경로를 살펴봅시다.

절대 경로는 크레이트 모듈 트리의 최상위인 `crate`로 시작합니다.
`front_of_house` 모듈은 크레이트 루트 내에 정의되어 있습니다.
`front_of_house` 모듈은 공개가 아니지만, `eat_at_restaurant` 함수와
`front_of_house` 모듈은 같은 모듈 내에 정의되어 있으므로 (즉, 서로 형제 관계이므로)
`eat_at_restaurant` 함수에서 `front_of_house` 모듈을 참조할 수 있습니다.
다음은 `pub` 키워드가 지정된 `hosting` 모듈입니다.
`hosting`의 부모 모듈에 접근할 수 있으니, `hosting`에도 접근할 수 있습니다.
마지막 `add_to_waitlist` 함수 또한 `pub` 키워드가 지정되어 있고, 부모 모듈에 접근할 수 있으니,
호출 가능합니다!

상대 경로는 첫 번째 과정을 제외하면 절대 경로와 동일합니다.
상대 경로는 크레이트 루트에서 시작하지 않고, `front_of_house`로 시작합니다.
`front_of_house` 모듈은 `eat_at_restaurant` 함수와 동일한 모듈 내에 정의되어 있으므로,
`eat_at_restaurant` 함수가 정의되어 있는 모듈에서 시작하는 상대 경로를 사용할 수 있습니다.
이후 `hosting`, `add_to_waitlist`은  `pub`으로 지정되어 있으므로
나머지 경로도 문제없습니다.
따라서 이 함수 호출도 유효합니다!

다른 프로젝트에서 여러분의 코드를 사용할 수 있도록 라이브러리 크레이트를 공유할
계획이라면, 여러분의 공개 API는 크레이트의 사용자가 코드와 상호 작용하는 방법을
결정하는 계약입니다. 사람들이 여러분의 크레이트에 더 쉽게 의존할 수 있도록 하기
위해서는 공개 API의 변경사항을 관리할 때 고려해야 할 사항이 많습니다.
이러한 고려사항은 이 책의 범위를 벗어납니다; 이 주제에 관심이 있다면
[러스트 API 가이드라인][api-guidelines]을 참조하세요.

> #### 바이너리와 라이브러리가 함께 있는 패키지를 위한 최고의 예제
>
> 패키지에는 *src/main.rs* 바이너리 크레이트 루트뿐만 아니라 *src/lib.rs* 라이브러리
> 크레이트 루트도 같이 집어넣을 수 있음을 언급했었고, 두 크레이트 모두 기본적으로
> 같은 이름을 갖게 됩니다. 통상적으로 이렇게 라이브러리와 바이너리 크레이트 모두를
> 가지는 패턴의 패키지들은 라이브러리 크레이트에 있는 코드를 호출하여 실행파일을
> 시작하기 위한 양만큼의 코드가 바이너리 크레이트에 담긴 형태가 됩니다.
> 라이브러리 크레이트의 코드가 공유될 수 있으므로, 이렇게 하는 것으로 패키지가
> 제공하는 대부분의 기능을 다른 프로젝트에서 사용할 수 있도록 해줍니다.
>
> 모듈 트리는 *src/lib.rs* 내에 정의되어야 합니다. 그러면 바이너리 크레이트 내에서는
> 패키지 이름으로 시작하는 경로를 사용함으로써 모든 공개 아이템을 사용할 수 있습니다.
> 바이너리 크레이트는 완전히 외부에 있는 다른 크레이트가 이 라이브러리 크레이트를
> 사용하는 식과 동일하게 이 라이브러리 크레이트의 사용자가 됩니다: 즉 공개 API만
> 사용할 수 있습니다. 이는 여러분이 좋은 API를 설계하는 데 도움을 줍니다; 여러분이
> 저자일 뿐만 아니라, 고객도 겸하게 되니까요!
>
> [12장][ch12]<!-- ignore -->에서는 바이너리 크레이트와 라이브러리 크레이트를
> 모두 가지고 있는 커맨드 라인 프로그램을 작성해 보면서 이와 같은 구조에 대한
> 예제를 보여 드리겠습니다.

### `super`로 시작하는 상대 경로

`super`로 시작하면 현재 모듈 혹은 크레이트 루트 대신 자기 부모
모듈부터 시작되는 상대 경로를 만들 수 있습니다. 이는 파일시스템
경로에서 `..`로 시작하는 것과 동일합니다. `super`를 사용하면
부모 모듈에 위치하고 있음을 알고 있는 아이템을 참조하도록 해주고,
이는 모듈이 부모 모듈과 밀접한 관련이 있지만 부모 모듈은 나중에
모듈 트리의 다른 어딘가로 옮겨질지도 모르는 경우 모듈 트리의
재조정을 편하게 만들어 줍니다.

예제 7-8은 셰프가 잘못된 주문을 수정하여
고객에게 직접 전달하는 상황을 묘사한 코드입니다.
`back_of_house` 모듈에 정의된 `fix_incorrect_order` 함수는
`super`로 시작하는 `deliver_order`로의 경로를 특정하는 것으로
부모 모듈에 정의된 `deliver_order` 함수를 호출합니다:

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

<span class="caption">예제 7-8: `super`로 시작하는 상대 경로를 사용해
함수 호출하기</span>

`fix_incorrect_order` 함수는 `back_of_house` 모듈 내에 위치하므로,
`super`는  `back_of_house`의 부모 모듈, 즉 루트를 의미합니다.
그리고 해당 위치에 `deliver_order`가 존재하니 호출은 성공합니다.
`back_of_house` 모듈과 `deliver_order` 함수는 크레이트 모듈 구조 변경 시
서로의 관계를 유지한 채 함께 이동될 가능성이 높습니다.
그러므로 `super`를 사용하면,
차후에 다른 모듈에 이동시키더라도
수정해야 할 코드를 줄일 수 있습니다.

### 구조체, 열거형을 공개하기

`pub` 키워드로 구조체와 열거형을 공개할 수도 있지만,
이를 활용하기 전에 알아두어야 할 추가사항이 몇 가지 있습니다.
구조체 정의에 `pub`를 쓰면 구조체는 공개되지만, 구조체의
필드는 비공개로 유지됩니다. 공개 여부는 각 필드마다 정할 수 있습니다.
예제 7-9는 공개 구조체 `back_of_house::Breakfast`를 정의하고
`toast` 필드는 공개하지만 `seasonal_fruit` 필드는 비공개로 둔 예제입니다.
이는 레스토랑에서 고객이 식사와 같이 나올 빵 종류를 선택하고,
셰프가 계절과 재고 상황에 맞춰서 식사에 포함할 과일을 정하는 상황을 묘사한 예제입니다.
과일은 빈번히 변경되므로, 고객은 직접 과일을 선택할 수 없으며
어떤 과일을 받을지도 미리 알 수 없습니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

<span class="caption">예제 7-9: 일부 필드는 공개하고,
일부 필드는 비공개인 구조체</span>

`back_of_house::Breakfast` 구조체의 `toast` 필드는
공개 필드이기 때문에 `eat_at_restaurant` 함수에서 점
표기법으로 `toast` 필드를 읽고 쓸 수 있습니다.
반면, `seasonal_fruit` 필드는 비공개 필드이기 때문에 `eat_at_restaurant` 함수에서 사용할 수 없습니다.
`seasonal_fruit` 필드를 수정하는 코드의 주석을 한번 해제하여 어떤 에러가 발생하는지 확인해 보세요!

또한, `back_of_house::Breakfast` 구조체는 비공개 필드를 갖고 있기 때문에,
`Breakfast` 인스턴스를 생성할 공개 연관 함수(예제에서는 `summer` 함수입니다)를 반드시 제공해야 합니다.
만약 `Breakfast` 구조체에 그런 함수가 존재하지 않을 경우,
`eat_at_restaurant` 함수에서 `Breakfast` 인스턴스를 생성할 수 없습니다.
`eat_at_restaurant` 함수에서는 비공개 필드인 `seasonal_fruit` 필드의 값을
지정할 방법이 없기 때문입니다.

반대로, 열거형은 공개로 지정할 경우 모든 배리언트가 공개됩니다. 열거형을 공개하는 방법은
`enum` 키워드 앞에 `pub` 키워드만 작성하면 됩니다. 작성한 모습은 예제 7-10과 같습니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

<span class="caption">예제 7-10: 열거형과 열거형의 모든 배리언트를
공개로 지정하기</span>

`Appetizer` 열거형을 공개하였으니, `eat_at_restaurant` 함수에서
`Soup`, `Salad` 배리언트를 사용할 수 있습니다.

열거형은 그 배리언트가 공개되지 않는다면 큰 쓸모가 없습니다;
열거형의 모든 배리언트에 대해 전부 `pub`을 붙이는 것은 귀찮은
일이 될 것이므로, 열거형의 배리언트는 기본적으로 공개입니다. 구조체의
경우 필드를 공개로 하지 않는 것이 종종 유용하므로, 구조체 필드는 `pub`을
명시하지 않는 한 기본적으로 모든 것이 비공개라는 일반적인 규칙을 따릅니다.

남은 `pub` 키워드 관련 내용은 모듈 시스템의 마지막 기능인 `use` 키워드입니다.
먼저 `use` 키워드 단독 사용법을 다루고, 그다음 `use`와 `pub`을 연계하여
사용하는 방법을 다루겠습니다.

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#pub-키워드로-경로-노출하기
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-an-io-project.html
