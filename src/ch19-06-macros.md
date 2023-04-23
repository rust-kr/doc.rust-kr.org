## 매크로

이 책 전체에서 `println!`과 같은 매크로를 사용해 왔지만, 매크로가 무엇이고 어떻게
동작하는지는 충분히 설명하지 않았습니다. *매크로 (macro)* 라는 용어는 러스트의
기능군을 의미합니다: `macro_rules!`를 쓰는 *선언적 (declarative)* 매크로와
세 가지 종류의 *절차적 (procedural)* 매크로가 있습니다:

* 구조체와 열거형에 사용되는 `derive` 속성이 추가된 코드를 지정하는
  커스텀 `#[derive]` 매크로
* 모든 아이템에 사용 가능한 커스텀 속성을 정의하는 속성형 (attribute-like) 매크로
* 함수 호출처럼 보이지만 지정된 토큰을 인자로써 조작하는 
  함수형 (function-like) 매크로

순차적으로 각각에 대해 이야기할 것이지만, 먼저 함수가 이미 있음에도 매크로가
필요한 이유부터 살펴보겠습니다.

### 매크로와 함수의 차이

기본적으로 매크로는 다른 코드를 작성하는 코드를 작성하는 방법이며, 이를
*메타프로그래밍 (metaprogramming)* 이라고 합니다. 부록 C에서는 다양한
트레잇의 구현을 생성하는 `derive` 속성에 대해 설명합니다. 또한 책 전체에서
`println!`과 `vec!` 매크로를 사용했습니다. 이 모든 매크로는 수동으로
작성한 코드보다 더 많은 코드를 생성하기 위해 *확장*됩니다.

메타프로그래밍은 작성 및 유지 관리해야 하는 코드의 양을 줄이는 데
유용하며, 이는 함수의 역할 중 하나이기도 합니다. 하지만 매크로에는
함수에는 없는 몇 가지 추가 기능이 있습니다.

함수 시그니처는 함수에 있는 매개변수의 개수와 타입을 선언해야
합니다. 반면 매크로는 가변적인 수의 매개변수를 사용할 수 있습니다:
하나의 인수로 `println!("hello")`를 호출하거나 두 개의 인수로
`println!("hello {}", name)`을 호출할 수 있는 것처럼요. 또한 매크로는
컴파일러가 코드의 의미를 해석하기 전에 확장되기 때문에, 매크로는 이를테면
주어진 타입에 대한 트레잇을 구현할 수 있습니다. 이는 함수로는 불가능한데,
함수는 런타임에 호출되고 트레잇은 컴파일 타임에 구현되어야 하기 때문입니다.

함수 대신 매크로를 구현할 때의 단점은 매크로 정의가 러스트
코드를 작성하는 러스트 코드를 작성하는 것이기 때문에 함수
정의보다 더 복잡하다는 점입니다. 이러한 간접성으로 인해 매크로
정의는 일반적으로 함수 정의보다 읽고, 이해하고, 유지 관리하기가
더 어렵습니다.

매크로와 함수의 또 다른 중요한 차이점은, 어디서나 정의하고 어디서나
호출할 수 있는 함수와 달리 매크로의 경우 정의하거나 파일에서 호출하기
*전에* 매크로를 스코프로 가져와야 한다는 점입니다.

### 일반적인 메타프로그래밍을 위한 `macro_rules!`를 사용한 선언적 매크로

러스트에서 가장 널리 사용되는 매크로 형태는 *선언적 매크로*입니다.
선언적 매크로는 “예제 매크로 (macros by example)”,
“`macro_rules!` 매크로”, 또는 그냥 “매크로”라고도 불립니다.
선언적 매크로의 핵심은 러스트 `match` 표현식과 비슷한 무언가를
작성할 수 있다는 것입니다. 6장에서 설명한 것처럼, `match`
표현식은 표현식을 받아서 표현식의 결과 값을 패턴과 비교한
다음, 일치하는 패턴과 연관된 코드를 실행하는 제어 구조입니다.
매크로도 특정 코드와 연관된 패턴과 값을 비교합니다: 이 경우
값은 매크로에 전달된 리터럴 러스트 소스 코드이고, 패턴은 해당
소스 코드의 구조와 비교되고, 매칭이 되면, 매크로에 전달된 코드는
해당 패턴과 연관된 코드로 대체됩니다. 이 모든 과정은 컴파일
중에 이루어집니다.

매크로를 정의하려면 `macro_rules!` 구문을 사용합니다. `vec!` 매크로가 어떻게
정의되는지 살펴보는 것으로 `macro_rules!`를 사용하는 방법을 알아보도록 합시다.
8장에서는 `vec!` 매크로를 사용해 특정 값들을 가진 새로운 벡터를 생성하는
방법을 다뤘습니다. 예를 들어, 다음 매크로는 세 개의 정수를 포함하는 새로운
벡터를 생성합니다:

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

또한 `vec!` 매크로를 사용하여 두 개의 정수로 이루어진 벡터나 다섯 개의 문자열
슬라이스로 이루어진 벡터를 만들 수도 있습니다. 값의 개수나 타입을 미리 알 수
없으므로 함수를 사용하는 것으로는 동일한 작업을 수행할 수 없습니다.

Listing 19-28은 `vec!` 매크로의 약간 간소화된 정의를 보여줍니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-28/src/lib.rs}}
```

<span class="caption">Listing 19-28: 간소화된 버전의 `vec!` 매크로
정의</span>

> Note: 표준 라이브러리의 `vec!` 매크로의 실제 정의에는 정확한 양의
> 메모리를 미리 할당하는 코드가 포함되어 있습니다. 이 코드는 예제를
> 더 간단하게 만들기 위해 여기에는 포함하지 않은 최적화 코드입니다.

`#[macro_export]` 어노테이션은 이 매크로가 정의된 크레이트를 스코프로
가져올 때마다 이 매크로를 사용할 수 있어야 함을 나타냅니다. 이 어노테이션이
없으면 매크로를 스코프로 가져올 수 없습니다.

그런 다음 `macro_rules!`와 정의하려는 매크로의 이름을 느낌표 *없이* 사용하는
것으로 매크로 정의를 시작합니다. 이 이름의 (지금의 경우 `vec`) 뒤에는 매크로
정의의 본문을 나타내는 중괄호가 따라옵니다.

`vec!` 본문의 구조는 `match` 표현식의 구조와 유사합니다.
여기서는 `( $( $x:expr ),* )` 패턴에 `=>`이 붙고, 이 패턴과
연관된 코드 블록으로 되어 있는 갈래 하나가 있습니다. 패턴이
매칭되면 연관된 코드 블록이 튀어나옵니다. 이것이 이 매크로의
유일한 패턴이므로, 일치하는 유효한 방법은 하나뿐입니다; 즉 다른
패턴은 에러가 발생합니다. 더 복잡한 매크로에는 두 개 이상의 갈래가
있겠지요.

매크로 패턴은 값이 아닌 러스트 코드 구조에 대한 매칭을 하기
때문에 매크로 정의에서 유효한 패턴 문법은 18장에서 다루었던
패턴 구문과 다릅니다. Listing 19-28의 패턴 조각이 무엇을
의미하는지 살펴봅시다; 전체 매크로 패턴 문법은
[러스트 레퍼런스 문서][ref]를 참조하세요.

먼저 괄호 한 쌍을 사용하여 전체 패턴을 둘러쌉니다. 달러 기호(`$`)를
사용하여 패턴에 매칭될 러스트 코드를 담는 매크로 시스템 내의 변수를
선언합니다. 달러 기호는 이 변수가 일반적인 러스트 변수가 아닌 매크로
변수임을 명백하게 해줍니다. 다음에는 대체되는 코드에서 사용할 목적으로
괄호 안에 패턴과 매칭되는 값을 캡처하는 괄호 한 쌍이 있습니다. `$()`
안에는 `$x:expr`이 있는데, 이는 모든 러스트 표현식과 매칭되며 그 표현식에
`$x`라는 이름을 부여합니다.

`$()` 뒤에 오는 쉼표는 `$()` 안의 코드와 매칭되는 코드 뒤에 선택적으로
쉼표 구분자 문자가 나올 수 있음을 나타냅니다. `*`는 `*` 앞에 오는 모든
것과 매칭되는 것이 0개 이상이라는 것을 명시합니다.

이 매크로를 `vec![1, 2, 3];`으로 호출하면, `$x` 패턴은 세 표현식
`1`, `2`, `3`으로 세 번 매칭됩니다.

이제 이 갈래와 연관된 코드 본문의 패턴을 살펴봅시다: 패턴에서
`$()`와 매칭되는 각 부분에 대하여 패턴이 매칭되는 횟수에 따라
`$()*` 내의 `temp_vec.push()`가 0회 이상 생성됩니다. `$x`는
각각의 매칭된 표현식으로 대체됩니다. 이 매크로를
`vec![1, 2, 3];`으로 호출하면, 이 매크로 호출을 대체하는 코드가
다음과 같이 생성됩니다:

```rust,ignore
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

이렇게 정의된 매크로는 아무 타입의 인자를 얼마든지 사용할 수 있고 특정한
원소들을 담은 벡터를 생성하는 코드를 생성할 수 있습니다.

매크로를 작성하는 방법에 대해 더 알아보려면 온라인 문서 혹은 다른 자료,
이를테면 Daniel Keep이 시작하고 Lukas Wirth가 이어오고 있는
[“The Little Book of Rust Macros”][tlborm] 같은 것을 참조하세요.

### 속성에서 코드를 생성하기 위한 절차적 매크로

두 번째 형태의 매크로는 *절차적 매크로*로, 더 함수처럼 작동합니다. (그리고
프로시저의 일종입니다.) 절차적 매크로는 선언적 매크로처럼 패턴에 매칭해보고
코드를 다른 코드로 대체하는 것이 아니라, 어떤 코드를 입력으로 받아서 해당
코드에 대해 작업을 수행한 다음, 어떤 코드를 출력으로 생성합니다. 절차적
매크로는 커스텀 파생 (custom derive), 속성형 (attribute-like),
함수형 (function-like) 세 종류가 있으며, 모두 비슷한 방식으로 작동합니다.

절차적 매크로를 만들 때, 그 정의는 특별한 크레이트 타입을 가진 자신만의
크레이트에 있어야 합니다. 이는 복잡한 기술적인 이유 때문이고 앞으로는 이러한
문제를 없애고자 합니다. Listing 19-29에서는 절차적 매크로를 정의하는 방법을
보여주는데, 여기서 `some_attribute`는 특정 매크로 종류를 사용하기 위한
자리 표시자입니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

<span class="caption">Listing 19-29: 절차적 매크로 정의
예제</span>

절차적 매크로를 정의하는 함수는 `TokenStream`을 입력으로 받아서
`TokenStream`을 출력으로 생성합니다. `TokenStream` 타입은 러스트에
포함되어 있는 `proc_macro` 크레이트에 정의되어 있으며 토큰의 시퀀스를
나타냅니다. 이것이 이 매크로의 핵심입니다: 매크로가 작동하는 소스 코드가
입력 `TokenStream`을 구성하고, 매크로가 생성하는 코드가 출력
`TokenStream`입니다. 또한 이 함수에는 어떤 종류의 절차적 매크로를
만들고 있는지를 지정하는 속성이 붙어있습니다. 같은 크레이트에는
여러 종류의 절차적 매크로를 넣을 수 있습니다.

각각의 절차적 매크로 종류를 살펴보겠습니다. 우선 커스텀 파생
매크로부터 시작하고 그 다음에는 다른 형태의 작은 차이점들을
설명하겠습니다.

### 커스텀 `derive` 매크로 작성 방법

`HelloMacro`라는 이름의 트레잇과 `hello_macro`라는 하나의 연관 함수를
정의하는 `hello_macro`라는 이름의 크레이트를 만들어 봅시다. 사용자가
자신의 타입에 대해 `HelloMacro` 트레잇을 구현하도록 하는 대신,
절차적 매크로를 제공하여 사용자가 `#[derive(HelloMacro)]`라고
타입에 명시하여 `hello_macro` 함수의 기본 구현을 가져올 수 있도록
하겠습니다. 기본 구현은 `Hello, Macro! My name is TypeName!`라고
출력하는데, 여기서 `TypeName`은 이 트레잇이 정의된 타입의 이름입니다.
바꿔 말하면, 다른 프로그래머가 우리의 크레이트를 사용하여 Listing
19-30과 같은 코드를 작성할 수 있도록 하는 크레이트를 작성할 것입니다.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-30/src/main.rs}}
```

<span class="caption">Listing 19-30: 우리의 절차적 매크로를 사용할 때
크레이트 사용자가 작성할 수 있게될 코드</span>

작업을 마치면 이 코드는 `Hello, Macro! My name is Pancakes!`를 출력할 것입니다.
첫 번째 단계는 다음과 같이 새로운 라이브러리 크레이트를 만드는 것입니다:

```console
$ cargo new hello_macro --lib
```

다음은 `HelloMacro` 트레잇과 그 연관 함수를 정의하는 것입니다:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-20-impl-hellomacro-for-pancakes/hello_macro/src/lib.rs}}
```

트레잇과 그 함수를 정의했습니다. 이 시점에서 크레이트 사용자는 다음과 같이
트레잇을 구현하여 원하는 기능을 구현할 수 있습니다:

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-20-impl-hellomacro-for-pancakes/pancakes/src/main.rs}}
```

하지만 이렇게 하면 `hello_macro`와 함께 사용하고자 하는 각 타입에 대해
구현 블록을 작성해야 합니다; 사용자가 이러한 작업을 생략할 수 있도록
하려고 합니다.

또한, 트레잇이 구현된 타입의 이름을 출력하는 `hello_macro`
함수를 기본 구현으로 제공할 수도 없습니다: 러스트에는 리플렉션
기능이 없기 때문에 런타임에 타입의 이름을 조회할 수 없습니다.
컴파일 타임에 코드를 생성하려면 매크로가 필요합니다.

다음 단계는 절차적 매크로를 정의하는 것입니다. 이 글을 쓰는 시점에서, 절차적
매크로는 자체 크레이트에 있어야 합니다. 궁극적으로는 이 제한이 해제될 수도
있습니다. 크레이트 및 매크로 크레이트를 구조화하는 관례는 다음과 같습니다:
`foo`라는 이름의 크레이트에 대해 커스텀 파생 절차적 매크로 크레이트는
`foo_derive`라고 부릅니다. `hello_macro` 프로젝트 안에 `hello_macro_derive`라는
새 크레이트를 만들어 보겠습니다:

```console
$ cargo new hello_macro_derive --lib
```

두 크레이트는 서로 밀접하게 연관되어 있으므로, `hello_macro`
크레이트의 디렉토리 내에 절차적 매크로 크레이트를 생성합니다.
`hello_macro`에서 트레잇 정의를 변경하면, `hello_macro_derive`의
절차적 매크로 구현도 변경해야 합니다. 두 크레이트는 별도로 배포되어야
하며, 이 크레이트를 사용하는 프로그래머는 두 크레이트를 종속성으로
추가하고 두 크레이트를 모두 스코프 안으로 가져와야 합니다. 대신
`hello_macro` 크레이트가 `hello_macro_derive`를 종속성으로 사용하고
절차적 매크로 코드를 다시 내보내도록 할 수도 있습니다. 그렇지만 우리가
프로젝트를 구조화한 방식은 프로그래머가 `derive` 기능을 원하지 않더라도
`hello_macro`를 사용할 수 있게 해줍니다.

`hello_macro_derive` 크레이트는 절차적 매크로 크레이트로 선언될 필요가
있습니다. 잠시 후에 보게 되겠지만, `syn` 및 `quote` 크레이트의 기능도
필요하므로, 이들을 종속성으로 추가해야 합니다. `hello_macro_derive`의
*Cargo.toml* 파일에 다음을 추가하세요:

<span class="filename">Filename: hello_macro_derive/Cargo.toml</span>

```toml
{{#include ../listings/ch19-advanced-features/listing-19-31/hello_macro/hello_macro_derive/Cargo.toml:6:12}}
```

절차적 매크로 정의를 시작하려면 Listing 19-31의 코드를 `hello_macro_derive`
크레이트의 *src/lib.rs* 파일에 넣으세요. 이 코드는 `impl_hello_macro` 함수에
대한 정의를 추가할 때까지 컴파일되지 않는다는 점에 유의하세요.

<span class="filename">Filename: hello_macro_derive/src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-31/hello_macro/hello_macro_derive/src/lib.rs}}
```

<span class="caption">Listing 19-31: 대부분의 절차적 매크로 크레이트가
러스트 코드를 처리하기 위해 필요로 하는 코드</span>

`TokenStream`의 파싱을 담당하는 `hello_macro_derive` 함수와
구문 트리의 변환을 담당하는 `impl_hello_macro` 함수로 코드를
분할한 것을 주목하세요: 이렇게 하면 절차적 매크로를 더 편리하게
작성할 수 있습니다. 외부 함수 (위의 경우 `hello_macro_derive`) 의
코드는 여러분이 보거나 만드는 거의 모든 절차적 매크로 크레이트에서
동일합니다. 내부 함수 본문에 지정하는 코드 (위의 경우
`impl_hello_macro`) 는 절차적 매크로의 목적에 따라
달라질 수 있습니다.

세 가지 새로운 크레이트를 도입했습니다: `proc_macro`, [`syn`], 그리고 [`quote`]
입니다. `proc_macro` 크레이트는 러스트와 함께 제공되므로, *Cargo.toml*의 종속성에
추가할 필요는 없습니다. `proc_macro` 크레이트는 우리 코드에서 러스트 코드를
읽고 조작할 수 있게 해주는 컴파일러의 API입니다.

`syn` 크레이트는 러스트 코드를 문자열에서 연산을 수행할 수 있는 데이터
구조로 파싱합니다. `quote` 크레이트는 `syn` 데이터 구조를 다시 러스트
코드로 변환합니다. 이러한 크레이트를 사용하면 처리하려는 모든 종류의 러스트
코드를 훨씬 간단하게 파싱할 수 있습니다: 러스트 코드에 대한 전체 파서를
작성하는 것은 간단한 작업이 아닙니다.

`hello_macro_derive` 함수는 라이브러리 사용자가 타입에
`#[derive(HelloMacro)]`를 지정할 때 호출됩니다. 이는
`hello_macro_derive` 함수에 `proc_macro_derive`를 명시하고
트레잇 이름과 일치하는 `HelloMacro`라는 이름을 지정했기 때문에
가능합니다; 이는 대부분의 절차적 매크로가 따르는 관례입니다.

`hello_macro_derive` 함수는 먼저 `TokenStream`의 `input`을
해석 및 연산 수행이 가능한 데이터 구조로 변환합니다. 여기서
`syn`이 등장합니다. `syn`의 `parse` 함수는 `TokenStream`을
받아 파싱된 러스트 코드를 나타내는 `DeriveInput` 구조체를
반환합니다. Listing 19-32는 `struct Pancakes;` 문자열을 파싱하여
얻은 `DeriveInput` 구조체의 관련 부분을 보여줍니다:

```rust,ignore
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

<span class="caption">Listing 19-32: Listing 19-30의 매크로 속성이
있는 코드를 파싱했을 때 얻게 되는 `DeriveInput` 인스턴스</span>

이 구조체의 필드는 파싱한 러스트 코드가 `Pancakes`의 `ident` (식별자
(identifier), 이름을 의미) 를 가진 유닛 구조체라는 것을 보여줍니다.
이 구조체에는 모든 종류의 러스트 코드를 기술하는 더 많은 필드가 있습니다;
자세한 내용은 [`DeriveInput`에 대한 `syn` 문서][syn-docs]를 참조하세요.

곧 `impl_hello_macro` 함수를 정의할 텐데, 이 함수가 포함하고자 하는
새로운 러스트 코드를 만들어낼 곳입니다. 하지만 그 전에, 파생 매크로의
출력도 `TokenStream`이라는 점에 유의하세요. 반환된 `TokenStream`은
크레이트 사용자가 작성하는 코드에 추가되므로, 크레이트 사용자가 자신의
크레이트를 컴파일하면, 수정된 `TokenStream`에서 제공하는 추가 기능을
사용할 수 있습니다.

여기서 `syn::parse` 함수에 대한 호출이 실패하면 `unwrap`을 호출하여
`hello_macro_derive` 함수가 패닉을 일으키도록 하고 있다는 것을 눈치채셨을
것입니다. 절차적 매크로가 에러 발생 시 패닉을 일으키는 이유는 절차적
매크로 API를 준수하기 위해 `proc_macro_derive` 함수가 `Result`가 아닌
`TokenStream`을 반환해야 하기 때문입니다. 이 예제에서는 `unwrap`을 사용하여
단순화했습니다; 프로덕션 코드에서는 `panic!` 또는 `expect`를 사용하여 무엇이
잘못되었는지에 대한 보다 구체적인 에러 메시지를 제공해야 합니다.

이제 `TokenStream`으로부터 어노테이션된 러스트 코드를 `DeriveInput`
인스턴스로 변환하는 코드가 있으니, Listing 19-33에 나온 것처럼 어노테이션된
타입에 `HelloMacro` 트레잇을 구현하는 코드를 생성해 봅시다.

<span class="filename">Filename: hello_macro_derive/src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-33/hello_macro/hello_macro_derive/src/lib.rs:here}}
```

<span class="caption">Listing 19-33: 파싱된 러스트 코드를 사용하여
`HelloMacro` 트레잇 구현하기</span>

`ast.ident`을 사용하여 어노테이션된 타입의 이름(식별자)을 담고 있는
`Ident` 구조체 인스턴스를 얻습니다. Listing 19-32의 구조체는 Listing
19-30의 코드에서 `impl_hello_macro` 함수를 실행할 때, 얻게 되는
`ident`에 `"Pancakes"` 값을 가진 `ident` 필드가 있음을 보여줍니다.
따라서, Listing 19-33의 `name` 변수에는 `Ident` 구조체 인스턴스가
포함되며, 이 인스턴스를 출력하면 Listing 19-30의 구조체 이름인
`"Pancakes"` 문자열이 됩니다.

`quote!` 매크로는 반환하고자 하는 러스트 코드를 정의하도록 해줍니다.
컴파일러는 `quote!` 매크로 실행의 직접적인 결과와는 다른 것을 기대하므로,
이를 `TokenStream`으로 변환할 필요가 있습니다. 이 중간 표현을 소비하고
필요한 `TokenStream` 타입의 값을 반환하는 `into` 메소드를 호출하여
이 작업을 수행합니다.

`quote!` 매크로는 또한 매우 멋진 템플릿 매커니즘도 제공합니다: `#name`을
입력하면 `quote!`가 이를 변수 `name`안 들어있는 값으로 대체합니다.
일반적인 매크로가 작동하는 방식과 유사하게 반복을 실행할 수도 있습니다.
자세한 소개는 [`quote` 크레이트 문서][quote-docs]를 참조하세요.

우리의 절차적 매크로는 사용자가 어노테이션한 타입에 대해 `HelloMacro`
트레잇의 구현을 생성하도록 하고 싶고, 이는 `#name`을 사용하여 얻을 수
있습니다. 트레잇 구현에는 `hello_macro` 함수가 하나 있고, 그 본문에는
제공하고자 하는 기능이 담겨 있습니다: 바로 `Hello, Macro! My name is`을
출력한 다음 어노테이션된 타입의 이름을 출력하는 것이죠.

여기에 사용된 `stringify!` 매크로는 러스트에 내장되어 있습니다. 이
매크로는 `1 + 2`와 같은 러스트 표현식을 받아서, 컴파일 타임에 이 표현식을
`"1 + 2"`와 같은 문자열 리터럴로 변환합니다. 이는 표현식을 평가한 다음
결과를 `String`으로 변환하는 매크로인 `format!` 또는 `println!`과는
다릅니다. `#name` 입력이 문자 그대로 인쇄할 표현식일 가능성이 있으므로
`stringify!`를 사용합니다. `stringify!`를 사용하면 컴파일 타임에
`#name`을 문자열 리터럴로 변환하여 할당량을 절약할 수도 있습니다.

이 시점에서는 `cargo build`가 `hello_macro`와 `hello_macro_derive` 둘
모두에서 성공적으로 완료되어야 합니다. 이 크레이트를 Listing 19-30의 코드에
연결하여 절차적 매크로가 작동하는 것을 확인해 봅시다! `cargo new pancakes`를
사용하여 *projects* 디렉토리에 새 바이너리 프로젝트를 생성하세요. `pancake`
크레이트의 *Cargo.toml*에 종속성으로 `hello_macro`와 `hello_macro_derive`를
추가해야 합니다. [crates.io](https://crates.io/)에 `hello_macro`와
`hello_macro_derive` 버전을 배포하는 중이라면, 일반적인 종속성이 됩니다;
그렇지 않은 경우 다음과 같이 `path` 종속성으로 지정할 수 있습니다:

```toml
{{#include ../listings/ch19-advanced-features/no-listing-21-pancakes/pancakes/Cargo.toml:7:9}}
```

Listing 19-30의 코드를 *src/main.rs*에 넣고 `cargo run`을 실행하세요:
`Hello, Macro! My name is Pancakes!"라고 출력되어야 합니다. 절차적
매크로의 `HelloMacro` 트레잇 구현은 `pancakes` 크레이트가 구현할 필요
없이 포함되었습니다; `#[derive(HelloMacro)]`이 트레잇 구현을
추가한 것이지요.

다음으로 다른 종류의 절차적 매크로는 커스텀 파생 매크로와 어떻게 다른지
알아보겠습니다.

### 속성형 매크로

속성형 매크로는 커스텀 파생 매크로와 비슷하지만, `derive` 속성에
대한 코드를 생성하는 대신 새 속성을 생성할 수 있습니다. 속성형
매크로는 더 유연합니다: `derive`는 구조체와 열거형에만 작동합니다;
속성은 함수와 같은 다른 아이템에도 적용이 가능합니다. 다음은 속성형
매크로를 사용하는 예제입니다: 웹 애플리케이션 프레임워크를 사용할
때 함수에 어노테이션하는 `route`라는 속성이 있다고 가정해 보겠습니다:

```rust,ignore
#[route(GET, "/")]
fn index() {
```

이 `#[route]` 속성은 절차적 매크로로써 프레임워크에 의해 정의된
것입니다. 매크로 정의 함수의 시그니처는 다음과 같습니다:

```rust,ignore
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

여기에는 `TokenStream` 타입의 매개변수 두 개가 있습니다. 첫 번째는
속성의 내용에 대한 것입니다: 즉, `GET, "/"` 부분입니다. 두 번째는
속성이 연결된 아이템의 본문입니다: 이 경우에는 `fn index() {}`와
나머지 함수 본문입니다.

그 외에, 속성형 매크로는 커스텀 파생 매크로와 동일한 방식으로
작동합니다: `proc-macro` 크레이트 타입으로 크레이트를 생성하고
원하는 코드를 생성하는 함수를 구현하면 됩니다!

### 함수형 매크로

함수형 매크로는 함수 호출처럼 보이는 매크로를 정의합니다. `macro_rules!`
매크로와 유사하게, 함수형 매크로는 함수보다 더 유연합니다; 이를테면
임의 개수의 인수를 사용할 수 있습니다. 그러나, `macro_rules!` 매크로는
앞서 [“일반적인 메타프로그래밍을 위한 `macro_rules!`를 사용한 선언적 매크로”][decl]<!-- ignore -->절에서
설명한 매칭과 유사한 구문을 사용해야만 정의할 수 있습니다.
함수형 매크로는 `TokenStream` 매개변수를 취하고 그 정의는
다른 두 가지 종류의 절차적 매크로와 마찬가지로 러스트 코드를
사용하여 해당 `TokenStream`을 조작합니다. 함수형 매크로의 예로는
다음과 같이 호출할 수도 있는 `sql!` 매크로가 있습니다:

```rust,ignore
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

이 매크로는 내부에 있는 SQL 문을 파싱하고 문법적으로 올바른지
확인하는데, 이는 `macro_rules!` 매크로가 할 수 있는 것보다 훨씬
더 복잡한 처리입니다. `sql!` 매크로는 다음과 같이 정의됩니다:

```rust,ignore
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

이 정의는 커스텀 파생 매크로의 시그니처와 유사합니다:
괄호 안에 있는 토큰을 받아서 생성하고자 하는 코드를
반환합니다.

## 정리

휴! 자주 사용하지는 않겠지만, 이제 여러분의 도구 상자에는
매우 특정한 상황에서 사용할 수 있는 러스트 기능이 몇 가지
생겼습니다. 몇몇 복잡한 주제를 소개해서, 에러 메시지의
제안에서나 다른 사람의 코드에서 이 주제를 접했을 때는 이
개념과 문법을 인식할 수 있을 것입니다. 이 장을 참고하여
해결 방법을 찾아보세요.

다음으로는 이 책에서 논의한 모든 내용을 실제로 적용하여 프로젝트를
하나 더 해보겠습니다!

[ref]: ../reference/macros-by-example.html
[tlborm]: https://veykril.github.io/tlborm/
[`syn`]: https://crates.io/crates/syn
[`quote`]: https://crates.io/crates/quote
[syn-docs]: https://docs.rs/syn/1.0/syn/struct.DeriveInput.html
[quote-docs]: https://docs.rs/quote
[decl]: #declarative-macros-with-macro_rules-for-general-metaprogramming
