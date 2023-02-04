## 구조체 정의 및 인스턴트화

구조체는 여러 개의 연관된 값을 가질 수 있다는 측면에서 볼 때
[“튜플 타입”][tuples]<!-- ignore -->절에서 살펴본 튜플과 비슷합니다.
튜플처럼 구조체의 구성 요소들은 각각 다른 타입이 될 수 있습니다.
그리고 여기에 더해서, 구조체는 각각의 구성 요소에 이름을 붙일 수 있습니다.
따라서 각 요소가 더 명확한 의미를 갖게 되고, 특정 요소에 접근할 때
순서에 의존할 필요도 사라집니다. 결론적으로, 튜플보다 유연하게 사용할 수 있습니다.

구조체를 정의할 땐 `struct` 키워드와 해당 구조체에 지어줄 이름을 입력하면 됩니다.
이때 구조체 이름은 함께 묶을 데이터의 의미에 맞도록 지어주세요.
이후 중괄호 안에서는 필드(*field*)라 하는
각 구성 요소의 이름 및 타입을 정의합니다.
다음 Listing 5-1은 사용자 계정 정보를 저장하는 구조체입니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

<span class="caption">Listing 5-1: 사용자 계정 정보를 저장하는 `User` 구조체 정의</span>

정의한 구조체를 사용하려면 해당 구조체 내
각 필드의 값을 정해 인스턴스(*instance*)를 생성해야 합니다.
인스턴스를 생성하려면 먼저 구조체의 이름을 적고, 중괄호를 열고,
그 안에 필드의 이름(key)와 해당 필드에 저장할 값을 *key: value* 형태로 추가 해야합니다.
이때 필드의 순서는 구조체를 정의했을 때와 동일하지 않아도 됩니다.
요약하자면, 구조체 정의는 대충 해당 구조체에 무엇이 들어갈지를 정해둔 양식이며,
인스턴스는 거기에 실제 값을 넣은 것이라고 생각하시면 됩니다.
예시로 확인해 보죠. Listing 5-2 에서 앞서 정의한 `User` 구조체로
사용자를 선언해보겠습니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

<span class="caption">Listing 5-2: `User` 구조체의
인스턴스 생성</span>

구조체 내 특정 값은 점(.) 표기법으로 얻어올 수 있습니다. 예를 들어
사용자의 이메일 주소를 얻어야 한다치면 `user1.email` 처럼 사용할 수 있죠.
변경 가능한 인스턴스라면, 같은 방식으로 특정 필드의 값을 변경할 수도 있습니다.
다음 Listing 5-3 이 변경 가능한 인스턴스의
`email` 필드 값을 변경하는 예시입니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

<span class="caption">Listing 5-3: `User` 인스턴스의
`email` 필드 값 변경</span>

가변성은 해당 인스턴스 전체가 지니게 됩니다.
일부 필드만 변경 가능하도록 만들 수는 없으니, 기억해두시기 바랍니다.
또한, 구조체도 다른 표현식과 마찬가지로 함수 마지막 표현식에서
암묵적으로 새 인스턴스를 생성하고 반환할 수 있습니다.

Listing 5-4 에선 `build_user` 함수가 사용자 이메일과 이름을 전달 받고,
`acitve`, `sign_in_count` 를 각각 `true`, `1` 로 설정한
`User` 인스턴스를 반환하는 모습을 보여줍니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

<span class="caption">Listing 5-4: 사용자의 이메일과 이름을 전달 받고
`User` 인스턴스를 반환하는 `build_user` 함수</span>

특별히 나쁜 부분은 없지만, 매개변수명과 구조체 필드명이
`email`, `username` 으로 동일한데 굳이 반복해서 작성하는 건 귀찮은 감이 있군요.
구조체의 필드 개수가 많아지면 많아질수록 이런 귀찮음은 커질 겁니다.
한번 축약법을 사용해볼까요?

<!-- Old heading. Do not remove or links may break. -->
<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>

### 필드 초기화 축약법 사용하기

Listing 5-4 처럼 변수명과 구조체 필드명이 같을 땐,
필드 초기화 축약법(*field init shorthand*)을 사용해서 더 적은 타이핑으로
같은 기능을 구현할 수 있습니다. 다음 Listing 5-5 는 `email`, `username` 을
반복 작성하는 대신 필드 초기화 축약법을 사용한 예제입니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

<span class="caption">Listing 5-5: 변수명과 필드명이 같던
`username`, `email` 에 필드 초기화 축약법을 적용한
`build_user` 함수</span>

이번에는 `build_user` 함수에서
`User` 구조체의 인스턴스를 생성할 때
`email: email` 처럼 작성하는 대신,
변수명과 필드명이 같다는 점을 이용해 `email` 로만 작성한 모습입니다.
물론, 함수는 이전과 같이 잘 작동합니다.

### 기존 인스턴스를 이용해 새 인스턴스를 만들 때 구조체 갱신 문법 사용하기

다른 인스턴스에서 대부분의 값을 유지한 채로 몇 개의 값만 바꿔
새로운 인스턴스를 생성하게 되는 경우가 간혹 있습니다.
그럴 때 유용한 게 바로 구조체 갱신 문법(*struct update syntax*)입니다.

먼저 Listing 5-6에서는 구조체 갱신 문법을 사용하지 않고 새로운 `User`
인스턴스 `user2`를 만드는 방법을 보여줍니다. `email`에는 새로운 값을
설정했지만 나머지 값들에는 Listing 5-2에서 만들었던 `user1`의 값과 동일합니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

<span class="caption">Listing 5-6: `user1`의 값 중 하나를 달리한
새로운 `User` 인스턴스를 생성</span>

구조체 갱신 문법을 사용하면 다음 Listing 5-7처럼 더 적은 코드로
같은 효과를 낼 수 있습니다. `..` 은 따로 명시된 필드를 제외한
나머지 필드를, 주어진 인스턴스의 필드 값으로 설정하는 구문입니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

<span class="caption">Listing 5-7: 새로운 `email` 값으로 `User` 구조체의
인스턴스를 생성하되, 나머지 필드는 구조체 갱신 문법을 이용해 `user1`의 필드
값을 사용하기</span>

Listing 5-7의 코드 또한 `email` 값이 다른 `user2` 인스턴스를
생성하지만 `username`, `active`, `sign_in_count`는 `user1`의
필드와 같은 값을 갖게 합니다. `user1`의 값들과 동일한 값들로
나머지를 채우려면 `..user1`를 제일 끝에 적어야 하지만,
다른 필드들은 구조체의 정의 내에 있는 필드들의 순서와는
상관없이 우리 마음대로 몇 개든 임의 순서로 적을 수
있습니다.

구조체 갱신 문법이 대입처럼 `=`을 이용한다는 점을 주목하세요;
[“이동을 통하여 변수와 데이터 상호작용하기”][move]<!-- ignore -->절에서
본 것처럼, 이 구문은 데이터를 이동시킵니다. 이 예제에서 `user2`를 생성한
이후에는 `user1`를 더 이상 사용할 수 없는데, 이는 `user1`의 `username`
필드의 `String`이 `user2`로 이동되기 때문입니다. `user2`에 `email`과
`username`의 `String` 모두를 제공하여 `user1`의 `active`과
`sign_in_count`값만 사용한다면, `user2`를 만든 이후에도 `user1`는
유효합니다. `active`와 `sign_in_count` 모두 `Copy` 트레잇을 구현한
타입이므로, [“스택만 사용하는 데이터: 카피”][copy]<!-- ignore -->절에서
살펴본 동작이 적용됩니다.

### 필드명이 없는, 타입 구분용 튜플 구조체

러스트는 튜플과 유사한 형태의 튜플 구조체(*tuple structs*)도
지원합니다. 튜플 구조체는 구조체 자체에는 이름을 지어 의미를 주지만
이를 구성하는 필드에는 이름을 붙이지 않고 타입만 적어넣은 형태입니다.
튜플 구조체는 튜플 전체에 이름을 지어주거나 특정 튜플을 다른 튜플과 구분을
짓고는 싶은데, 그렇다고 각 필드명을 일일이 정해 일반적인 구조체 형태로
만들면 너무 수다스럽거나 불필요할 경우 유용합니다.

튜플 구조체의 정의는 일반적인 구조체처럼 `struct` 키워드와 구조체 명으로 시작되나,
그 뒤에는 타입들로 이루어진 튜플이 따라옵니다. 예시로 살펴볼까요?
다음은 각각 `Color`, `Point` 라는 두 개의 튜플 구조체 정의 및 사용 예시입니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

`black`, `origin` 이 서로 다른 튜플 구조체의 인스턴스이므로,
타입이 서로 달라진다는 점이 중요합니다.
구조체 내 필드 구성이 같더라도 각각의 구조체는 별도의 타입이기 때문이죠.
즉, `Color` 타입과 `Point` 타입은 둘 다 `i32` 값 3 개로 이루어진 타입이지만,
`Color` 타입을 매개변수로 받는 함수에
`Point` 타입을 인자로 넘겨주는 건 불가능합니다.
앞서 말한 점을 제외하면 튜플처럼 사용할 수 있습니다.
여러 부분으로 해체할 수도 있고, `.` 과 색인으로 요소에 접근할 수도 있죠.

### 필드가 없는 유사 유닛 구조체

필드가 아예 없는 구조체를 정의할 수도 있습니다. 이는
[“튜플 타입”][tuples]<!-- ignore -->에서 다룬 유닛 타입 `()`과
비슷하게 동작하므로 유사 유닛 구조체(*unit-like structs*)라 지칭합니다.
유사 유닛 구조체는 어떤 타입에 대해 트레잇을 구현하고 싶지만 타입 내부에
어떤 데이터를 저장할 필요는 없을 경우 유용합니다. 트레잇에 대해서는 10장에서
자세히 다루겠습니다. 아래는 `AlwaysEqual`이라는 이름의 유닛 구조체를
선언하고 생성하는 예제입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

`AlwaysEqual`을 정의하기 위해서 `struct` 키워드 뒤에 이름을 쓰고
바로 세미콜론을 붙였습니다. 중괄호나 괄호도 필요없습니다! 그런 다음
`subject` 변수에 `AlwaysEqual` 인스턴스를 만들어 넣을 때도 비슷한
방식을 사용합니다: 정의한 이름을 적어넣고, 중괄호나 괄호는 안썼습니다.
나중에 `AlwaysEqual`의 모든 인스턴스는 언제나 다른 모든 타입의
인스턴스와 같도록 하는 동작을 구현하여, 이미 알고 있는 결과값의
테스트 용도로 사용한다고 가정해봅시다. 이런 동작에는 데이터가 필요없을
것입니다! 유사 유닛 구조체를 포함한 임의 타임에 대한 트레잇을 정의하고
구현하는 방법은 10장에서 다룰 것입니다.

> ### 구조체 데이터의 소유권
>
> Listing 5-1 의 `User` 구조체 정의에서는 의도적으로
> `&str` 문자열 슬라이스 대신 구조체가 소유권을 갖는 `String` 타입을 사용했습니다.
> 구조체 인스턴스가 유효한 동안 각 인스턴스 내의
> 모든 데이터가 유효하도록 만들기 위해서죠.
>
> 참조자를 이용해 구조체가 소유권을 갖지 않는 데이터도 저장할 수는 있지만,
> 이는 10장에서 배울 라이프타임(*lifetime*)을 활용해야 합니다.
> 라이프타임을 사용하면 구조체가 존재하는 동안에
> 구조체 내 참조자가 가리키는 데이터의 유효함을 보장받을 수 있기 때문이죠.
> 만약 라이프타임을 명시하지 않고 참조자를 저장하고자 하면 다음처럼 문제가 발생합니다.
>
> <span class="filename">Filename: src/main.rs</span>
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore,does_not_compile
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
>
> fn main() {
>     let user1 = User {
>         active: true,
>         username: "someusername123",
>         email: "someone@example.com",
>         sign_in_count: 1,
>     };
> }
> ```
>
> 라이프타임이 명시돼야 한다며 컴파일러가 에러를 일으킬 겁니다.
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` due to 2 previous errors
> ```
>
> 위 에러를 해결하여 구조체에 참조자를 저장하는 방법은 10장에서 알아볼 겁니다.
> 지금 당장은 `&str` 대신 `String` 을 사용함으로써
> 넘어가도록 하죠.

<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->

[tuples]: ch03-02-data-types.html#the-tuple-type
[move]: ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
[copy]: ch04-01-what-is-ownership.html#stack-only-data-copy
