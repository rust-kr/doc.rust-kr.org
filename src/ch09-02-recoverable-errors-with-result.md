## `Result`와 함께하는 복구 가능한 에러

대부분 에러는 프로그램을 전부 중단해야 할 정도로 심각하진 않습니다.
때때로 어떤 함수가 실패할 때는, 여러분이 쉽게 해석하고 대응할 수 있는 원인 때문입니다.
예를 들어, 어떤 파일을 열려고 했는데 해당 파일이 존재하지 않아서 실패했다면,
프로세스를 종료해버리는 대신 파일을 생성하는 것을
원할지도 모르죠.

2장의 [“`Result` 타입으로 잠재된 실패 다루기”][handle_failure]<!-- ignore --> 절에서
`Result` 열거형은 다음과 같이 `Ok`와 `Err`라는 두 개의 variant를 갖도록 정의되어 있음을
상기하세요:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T`와 `E`는 제네릭 타입 매개변수입니다.
제네릭은 10장에서 자세히 다룰 예정입니다.
지금 당장은, `T`는 성공한 경우에 `Ok` variant 내에 반환될 값의 타입을
나타내고 `E`는 실패한 경우에 `Err` variant 내에 반환될 에러의 타입을
나타낸다는 점만 알아둡시다. `Result`가 이러한 제네릭 타입 매개변수를
갖기 때문에, 반환하고자 하는 성공적인 값과 에러값이 달라질 수 있는
다양한 상황에서 `Result` 타입과 그에 정의된 함수들을 사용할 수
있습니다.

실패할 가능성이 있어서 `Result` 값을 반환하는 함수를 한번 호출해 봅시다.
Listing 9-3는 파일을 열어보는 코드입니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

<span class="caption">Listing 9-3: 파일 열기</span>

`File::open`의 반환 타입은 `Result<T, E>` 입니다. 제네릭 매개변수
`T`는 `File::open`의 구현부에 성공값인 `std::fs::File`로 채워져
있는데, 이는 파일 핸들입니다. 에러값에 사용된 `E`의 타입은
`std::io::Error`입니다. 이 반환 타입은 `File::open`의 호출이
성공하여 읽거나 쓸 수 있는 파일 핸들을 반환할 수도 있음을 뜻합니다.
이 함수 호출은 실패할 수도 있습니다: 예를 들면 해당 파일이 존재하지
않거나, 파일 접근을 위한 권한이 없을지도 모릅니다. `File::open`
함수는 우리에게 함수가 성공하거나 실패할 수 있음을 알려주고
동시에 파일 핸들 혹은 에러 정보를 제공할 방법이 필요합니다.
이러한 정보가 정확하게 `Result` 열거형이 전달하는 것입니다.

`File::open`이 성공한 경우에는 `greeting_file_result`
변수의 값이 파일 핸들을 가지고 있는 `Ok` 인스턴스가 될
것입니다. 실패한 경우 `greeting_file_result`는 발생한
에러의 종류에 관한 더 자세한 정보가 담긴 `Err` 인스턴스가
될 것입니다.

Listing 9-3 코드에 `File::open` 반환 값에 따라
다르게 작동하는 코드를 추가해봅시다.
Listing 9-4은 6장에서 다뤘던 `match` 표현식을 이용하여
`Result`를 처리하는 한 가지 방법을 보여줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

<span class="caption">Listing 9-4: 반환될 수 있는 `Result` variant들을 `match`
표현식으로 처리하기</span>

`Option` 열거형과 같이 `Result` 열거형과 variant들은 프렐루드(prelude)로부터
가져와 진다는 점을 기억하세요. 따라서 `Ok`와 `Err` 앞에 `Result::`를 특정하지 않아도
됩니다.

결과가 `Ok`일 때 이 코드는 `Ok` variant 내부의 `file` 값을 반환하고,
그후 이 파일 핸들 값을 변수 `greeting_file`에 대입합니다.
`match` 이후에는 이 파일 핸들을 읽거나 쓰는 데에 사용할 수
있습니다.

`match`의 다른 경우는 `File::open`으로부터 `Err`를 얻은 경우를 처리합니다.
이 예제에서는 `panic!` 매크로를 호출하는 방법을 택했습니다.
디렉토리 내에 *hello.txt*라는 이름의 파일이 없는 경우에 이 코드를 실행하면,
`panic!` 매크로로부터 다음과 같은 출력을 보게 될 것입니다:

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

여태 그래왔듯, 이 출력은 어떤 것이 잘못되었는지 정확하게 알려줍니다.

### 서로 다른 에러에 대해 매칭하기

Listing 9-4의 코드는 `File::open`이 실패한 원인이 무엇이든 간에 `panic!`을
일으킵니다. 하지만 우리는 어떠한 이유로 실패했느냐에 따라 다른 동작을
취하도록 하기를 원합니다: 파일이 없어서 `File::open`이 실패했다면 새로운
파일을 만들어서 핸들을 반환하겠습니다. 그 밖의 이유로 (예를 들어
파일을 열 권한이 없다거나 하는 이유로) 실패했다면 Listing 9-4처럼
`panic!`을 일으키도록 말이죠. `match`에 내용을 추가한 Listing 9-5를
살펴봅시다.

<span class="filename">Filename: src/main.rs</span>

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

<span class="caption">Listing 9-5: 다른 종류의 에러를
다른 방식으로 처리하기</span>

`Err` variant 내에 있는 `File::open`이 반환하는 값의 타입은 `io::Error`인데,
이는 표준 라이브러리에서 제공하는 구조체입니다.
이 구조체는 `kind` 메서드를 제공하는데 이를 호출하여 `io::ErrorKind`값을 얻을 수 있습니다.
`io::ErrorKind`는 `io` 연산으로부터 발생할 수 있는 여러 종류의 에러를 표현하는 variant를 가진,
표준 라이브러리에서 제공하는 열거형입니다. 우리가 사용하고자 하는
variant는 `ErrorKind::NotFound`인데, 이는 열고자 하는 파일이 아직 존재하지
않음을 나타냅니다. 따라서 이를 `greeting_file_result`와 매치시켰습니다.
그런데 `error.kind()` 내부에 매치가 하나 더 있군요.

내부 매치에서는 `error.kind()`가 반환한 값이 `ErrorKind` 열거형의
`NotFound` variant가 맞는지 확인하고, 맞다면 `File::create`로 파일을 생성합니다.
하지만 `File::create`도 실패할 수 있으니, 내부 `match` 표현식의
두 번째 갈래 또한 작성해야 합니다.
파일을 생성하지 못한 경우에는 별도의 에러 메세지가 출력됩니다.
외부 `match`의 두 번째 갈래 또한 동일 하므로,
파일을 찾을 수 없는 에러인 경우 외에는 모두 패닉이 발생합니다.

> ### Alternatives to Using `match` with `Result<T, E>`
>
> `match`가 정말 많군요! `match` 표현식은 매우 유용하지만 굉장히 원시적이기도
> 합니다. 13장에서는 클로저에 대해서 배워볼 텐데, `Result<T, E>` 타입에는
> 클로저를 사용하는 여러 메서드가 있습니다. 여러분의 코드에서 `Result<T, E>`
> 값들을 처리할 때 이 메서드들로 `match` 보다 더 간결하게 만들 수 있습니다.
>
> 예를 들면, Listing 9-5와 동일한 로직을 작성한 다른 방법이 아래 있는데,
> 이번에는 `unwrap_or_else` 메서드와 클로저를 사용했습니다:
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic!("Problem creating the file: {:?}", error);
>             })
>         } else {
>             panic!("Problem opening the file: {:?}", error);
>         }
>     });
> }
> ```
>
> 이 코드는 Listing 9-5와 완벽히 동일하게 작동하지만, 
> `match` 표현식을 전혀 사용하지 않았으며 읽기도 더 깔끔합니다.
> 13장을 읽고 이 예제로 돌아와서, 표준 라이브러리 문서에서
> `unwrap_or_else` 메서드를 찾아보세요. 에러를 다룰 때 이런 메서드들을
> 사용하면 거대하게 중첩된 `match` 표현식 덩어리를 제거할 수 있습니다.

### 에러가 났을 때 패닉을 위한 숏컷: `unwrap`과 `expect`

`match`의 사용은 충분히 잘 동작하지만, 살짝 장황하기도 하고 의도를 항상
잘 전달하는 것도 아닙니다. `Result<T, E>` 타입은 다채롭고 더 특정한 작업을
하기 위해 정의된 수많은 헬퍼 메서드를 가지고 있습니다. `unwrap` 메서드는
Listing 9-4에서 작성한 `match` 구문과 비슷한 구현을 한 숏컷 메서드입니다.
만일 `Result` 값이 `Ok` variant라면, `unwrap`은 `Ok` 내의 값을 반환할
것입니다. 만일 `Result`가 `Err` variant라면, `unwrap`은 우리를 위해
`panic!` 매크로를 호출할 것입니다. 아래에 `unwrap`이 작동하는 예가 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

*hello.txt* 파일이 없는 상태에서 이 코드를 실행시키면, `unwrap` 메서드에 의한 `panic!`
호출로부터의 에러 메세지를 보게 될 것입니다:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-04-unwrap
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:4:49
```

비슷하게 `expect`는 우리가 `panic!` 에러 메세지 또한 선택할 수 있게 해줍니다.
`unwrap` 대신 `expect`를 이용하고 좋은 에러 메세지를 제공하는 것은 여러분의
의도를 전달해주고 패닉의 근원을 추적하는 걸 쉽게 해 줄 수 있습니다. `expect`의
문법은 아래와 같이 생겼습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

`unwrap`과 똑같이 파일 핸들을 반환하거나 `panic!` 매크로를 호출하도록 하는 데에
`expect`를 사용했습니다. `unwrap`은 `panic!`의 기본 메세지가 출력되지만,
`expect`는 매개변수로 넘긴 메세지를 에러 메세지로 출력합니다.
나타나는 모습은 다음과 같습니다.

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-05-expect
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at 'hello.txt should be included in this project: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:5:10
```

출시 가능한 품질의 코드에서 대부분의 러스트인들은 `unwrap` 보다 `expect`를
선택하고 왜 해당 연산이 언제나 성공하기를 기대하는지에 대한 더 많은 내용을
제공합니다. 이렇게 함으로써 여러분의 가정이 틀린것으로 판명났다면, 여러분은
디버깅에 사용할 정보를 더 얻게 됩니다.

### 에러 전파하기

함수의 구현체 내부에서 실패할지도 모르는 무언가를 호출할 때, 이 함수
내에서 에러를 처리하는 대신, 에러를 호출하는 코드 쪽으로 반환하여
그쪽에서 어떻게 할지 결정하도록 할 수 있습니다. 이는 에러 *전파하기 (propagating)* 로
알려져 있으며, 에러가 어떻게 처리해야 좋을지 좌우해야 할 상황에서, 여러분의 코드
내용 내에서 이용 가능한 것들보다 더 많은 정보와 로직을 가지고 있을 수도 있는
호출하는 코드 쪽에 더 많은 제어권을 줍니다.

예를 들면, Listing 9-6는 파일로부터 사용자 이름을 읽는 함수를 작성한 것입니다.
만일 파일이 존재하지 않거나 읽을 수 없다면, 이 함수는 호출하는 코드 쪽으로
해당 에러를 반환할 것입니다:

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

<span class="caption">Listing 9-6: `match`를 이용하여 호출 코드 쪽으로
에러를 반환하는 함수</span>

이 함수는 더 간결하게 작성할 수 있지만, 에러 처리를 배우기 위해
과정을 하나씩 직접 작성해보고, 간결한 버전은 마지막에 살펴보도록 하겠습니다.
함수의 반환 타입인 `Result<String, io::Error>` 부터 먼저 살펴봅시다.
이는 함수가 `Result<T, E>` 타입의 값을 반환하는데 제네릭 매개변수 `T`는
구체 타입(concrete type)인 `String`로 채워져 있고, 제네릭 타입 `E`는
구체 타입인 `io::Error`로 채워져 있다는 의미입니다.

만일 이 함수가 어떠한 문제 없이 성공하면, 함수를 호출한 코드는
`String`을 담은 값 (이 함수가 파일로부터 읽어 들인 사용자
이름이겠지요) 을 받을 것입니다. 만일 어떤 문제가 발생한다면,
이 함수를 호출한 코드는 문제가 무엇이었는지에 대한 더 많은
정보를 담고 있는 `io::Error`의 인스턴스를 담은 `Err` 값을
받을 것입니다. 이 함수의 반환 타입으로서 `io::Error`를 선택했는데,
그 이유는 우리가 이 함수 내부에서 호출하고 있는 실패 가능한
연산 두 가지가 모두 이 타입의 에러 값을 반환하기
때문입니다: `File::open` 함수와 `read_to_string`
메서드 말이죠.

함수의 본체는 `File::open` 함수를 호출하면서 시작합니다. 그다음에는
Listing 9-4에서 본 `match`와 유사한 식으로 `match`을 이용해서 `Result`
값을 처리합니다. 만약 `File::open`이 성공하면 패턴 변수 `file` 내의
파일 핸들은 가변 변수 `username_file`의 값이 되고 함수는 계속됩니다.
`Err` 경우에는 `panic!`을 호출하는 대신 `return` 키워드를 사용하여
함수 전체를 일찍 끝내고 호출한 코드 쪽에 이 함수의 에러 값으로서
`File::open`로부터 얻어진 (지금의 경우 패턴 변수 `e`에 있는)
에러값을 넘깁니다.

그래서 `username_file`에 파일 핸들을 얻게 되면, 함수는 `username` 변수에
새로운 `String`을 생성하고 `username_file`의 파일 핸들에 대해
`read_to_string` 메서드를 호출하여 파일의 내용물을 `username`으로
읽어들입니다. `File::open`이 성공했을지라도 `read_to_string` 메서드
또한 실패할 수 있으므로 `Result`를 반환합니다. 따라서 이 `Result`를 처리하기
위한 또다른 `match`가 필요합니다: `read_to_string`이 성공하면, 이 함수는
성공한 것이고, 이제는 `username`에 있는 파일로부터 읽은 사용자명을
`Ok`로 감싸서 반환합니다. 만일 `read_to_string`이 실패하면, `File::open`의
반환값을 처리했던 `match`에서의 에러값 반환과 똑같은 방식으로 에러값을
반환합니다. 하지만 이 함수의 마지막 표현식이기 때문에 명시적으로 `return`이라고
적을 필요는 없습니다.

그러면 이 코드를 호출하는 코드는 사용자 이름을 담은 `Ok` 값 혹은 `io::Error`를 담은
`Err` 값을 얻는 처리를 하게 될 것입니다. 이 값을 가지고 어떤 일을 할지에 대한 결정은
호출하는 코드 쪽에 달려 있습니다. 만일 그쪽에서 `Err` 값을 얻었다면, 예를 들면
`panic!`을 호출하여 프로그램을 종료시키는 선택을 할 수도 있고, 기본 사용자 이름을
사용할 수도 있으며, 혹은 파일이 아닌 다른 어딘가에서 사용자 이름을 찾을 수도 있습니다.
호출하는 코드가 정확히 어떤 것을 시도하려 하는지에 대한 충분한 정보가 없기 때문에,
우리는 모든 성공 혹은 에러 정보를 위로 전파하여 호출하는 코드가
적절하게 처리를 하도록 합니다.

러스트에서 에러를 전파하는 패턴은 너무 흔하여 러스트에서는 이를 더 쉽게 해주는
물음표 연산자 `?`를 제공합니다.

### 에러를 전파하기 위한 숏컷: `?`

Listing 9-7은 Listing 9-6과 같은 기능을 가진
`read_username_from_file`의 구현을 보여주는데,
다만 이 구현은 `?` 연산자를 이용하고 있습니다:

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

<span class="caption">Listing 9-7: `?` 연산자를 이용하여 에러를 호출 코드쪽으로
반환하는 함수</span>

`Result` 값 뒤의 `?`는 Listing 9-6에서 `Result` 값을 다루기 위해 정의했던
`match` 표현식과 거의 같은 방식으로 동작하게끔 정의되어 있습니다.
만일 `Result`의 값이 `Ok`라면, `Ok` 내의 값이 이 표현식으로부터
얻어지고 프로그램이 계속됩니다.
만일 값이 `Err`라면, 우리가 `return` 키워드를 사용하여 에러 값을 호출하는
코드에게 전파하는 것과 같이 전체 함수로부터 `Err` 내의 값이
반환될 것입니다.

Listing 9-6의 `match` 표현식과 `?` 연산자의 차이점은,
`?` 연산자를 사용할 때의 에러 값들은 `from` 함수를 거친다는 것입니다.
`from` 함수는 표준 라이브러리 내의 `From` 트레잇에 정의되어 있으며
어떤 값의 타입을 다른 타입으로 변환하는 데에 사용합니다.
`?` 연산자가 `from` 함수를 호출하면, `?` 연산자가 얻게 되는 에러를 `?`
연산자가 사용된 현재 함수의 반환 타입에 정의된 에러 타입으로
변환합니다. 이는 어떤 함수가 다양한 종류의 에러로 인해 실패할
수 있지만, 모든 에러를 하나의 에러 타입으로 반환할 때
유용합니다.

예를 들면 Listing 9-7의 `read_username_from_file` 함수가 직접 정의한
`OurError`라는 이름의 커스텀 에러 타입을 반환하도록 고칠 수 있겠습니다.
또한 `impl From<io::Error> for OurError`를 정의하여 `io::Error`로부터
`OurError`를 생성하도록 한다면, `read_username_from_file` 본체에 있는
`?` 연산자 호출은 다른 코드를 추가할 필요없이 `from`을 호출하여 에러 타입을
변환할 것입니다.

Listing 9-7의 내용에서, `File::open` 호출 부분의 끝에 있는 `?`는
`Ok`내의 값을 변수 `username_file`에게 반환해줄 것입니다. 만일 에러가
발생하면 `?`는 전체 함수로부터 일찍 빠져나와 호출하는 코드에게 어떤
`Err` 값을 줄 것입니다. `read_to_string` 호출의 끝부분에 있는 `?`도
같은 것이 적용되어 있습니다.

`?`는 많은 양의 보일러플레이트를 제거해주고 이 함수의 구현을 더 단순하게 만들어 줍니다.
심지어는 Listing 9-8과 같이 `?` 뒤에 바로 메서드 호출을 연결하는 식으로 이 코드를 더
줄일 수도 있습니다:

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

<span class="caption">Listing 9-8: `?` 연산자 뒤에 메서드 호출을
연결하기</span>

새로운 `String`을 만들어 `username`에 넣는 부분을 함수의 시작 부분으로
옮겼습니다. 이 부분은 달라진 것이 없습니다. `username_file` 변수를 만드는
대신, `File::open("hello.txt")?`의 결과 바로 뒤에 `read_to_string`의
호출을 연결했습니다. `read_to_string` 호출의 끝에는 여전히 `?`가 남아있고,
`File::open`과 `read_to_string`이 모두 에러를 반환하지 않고 성공할 때
`username` 안의 사용자 이름을 담은 `Ok`를 여전히 반환합니다.
함수의 기능 또한 Listing 9-6과 Listing 9-7의 것과 동일하고,
다만 작성하기에 더 인체공학적인 방법이라는 차이만 있을 뿐입니다.

Listing 9-9에는 `fs::read_to_string`을 사용하여 더 짧게 만든 예시가 있습니다.

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

<span class="caption">Listing 9-9: 파일을 열고, 읽는 대신 `fs::read_to_string`를
사용하기</span>

파일에서 문자열을 읽는 코드는 굉장히 흔하게 사용되기 때문에,
표준 라이브러리에서는 파일을 열고, 새 `String`을 생성하고,
파일 내용을 읽고, 내용을 `String`에 집어넣고 반환하는
편리한 `fs::read_to_string` 함수를 제공합니다.
다만 `fs::read_to_string`를 사용해버리면 여러분에게 에러를 다루는 법을
자세히 설명할 수 없으니 긴 코드로 먼저 설명했습니다.

### `?` 연산자가 사용될 수 있는 곳

`?`는 `?`이 사용된 값과 호환 가능한 반환 타입을 가진 함수에서만
사용될 수 있습니다. 이는 `?` 연산자가 Listing 9-6에서 정의한 `match`
표현식과 동일한 방식으로 함수를 일찍 끝내면서 값을 반환하는 동작을
수행하도록 정의되어 있기 때문입니다. Listing 9-6에서 `match`는
`Result` 값을 사용하고 있었고, 빠른 반환 갈래는 `Err(e)` 값을
반환했습니다. 이 함수의 반환 타입이 `Result`여야 이 `return`과
호환 가능합니다.

만일 반환 타입이 `()`인 `main` 함수 내에서 `?`를 사용한 값의 타입이 호환되지
않는 `?` 연산자를 사용하면 어떤 에러를 얻게 되는지 Listing 9-10에서 살펴보도록
합시다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

<span class="caption">Listing 9-10: `()`를 반환하는 `main`에서의
`?` 사용 시도는 컴파일되지 않습니다</span>

이 코드는 파일을 여는데, 이는 실패할 수도 있습니다. `?` 연산자는
`File::open`에 의해 반환되는 `Result`값을 따르지만, `main` 함수는 반환
타입이 `Result`가 아니라 `()`입니다. 이 코드를 컴파일하면 다음과 같은
에러 메세지를 얻게 됩니다:

```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

이 에러는 `?` 연산자가 `Result`, `Option` 혹은 `FromResidual`을
구현한 타입을 반환하는 함수에서만 사용될 수 있음을 지적하고
있습니다.

이 에러를 고치기 위해서는 두 가지 선택지가 있습니다. 첫번째는 `?` 연산자가
사용되는 곳의 값과 호환되는 반환 타입으로 함수를 고치는 것인데, 이렇게
고치는 걸 방지하는 어떤 제약이 없는 한도 내에서 가능하겠습니다. 다른 방법은
`Result<T, E>`를 적절한 식으로 처리하기 위해 `match` 혹은 `Result<T, E>`의
메서드 중 하나를 사용하는 것입니다.

에러 메세지는 또한 `?`가 `Option<T>` 값에 대해서도 사용될 수 있음을
알려주었습니다. `Result`에 대하여 `?`를 사용할 때와 마찬가지로, 함수가
`Option`를 반환하는 경우에 `Option`에 대해서만 `?`를 사용할 수 있습니다.
`Option<T>` 상에서 호출되었을 때의 `?` 연산자 동작은 `Result<T, E>` 상에서
호출되었을 때의 동작과 유사합니다: `None` 값인 경우 그 함수의 해당
지점으로부터 `None`값을 일찍 반환할 것입니다. `Some` 값이라면 `Some`
안에 있는 값이 이 표현식의 결과값이 되면서 함수가 계속됩니다.
Listing 9-11은 주어진 텍스트에서 첫 번째 줄의 마지막 문자를 찾는
함수의 예제를 보여줍니다:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

<span class="caption">Listing 9-11: `Option<T>` 값에 대한 `?` 연산자의
사용</span>

이 함수는 `Option<char>`를 반환하는데 이는 어떤 문자가 있을 수도,
없을 수도 있기 때문입니다. 이 코드는 `text` 문자열 슬라이스 인자를
가져와서 `lines` 메서드를 호출하는데, 이는 해당 문자열의 라인에 대한
반복자를 반환합니다. 이 함수가 첫번쨰 줄을 시험하길 원하므로,
반복자의 `next`를 호출하여 첫번째 값을 얻어옵니다.
만일 `text`가 빈 문자열이라면 `next` 호출은 `None`을 반환하는데,
여기서 `?`를 사용하여 `last_char_of_first_line`의 실행을 멈추고
`None`을 반환합니다. 만약 `text`가 빈 문자열이 아니라면 `next`는
`text`의 첫번째 줄의 문자열 슬라이스를 담고 있는 `Some`의 값을 반환합니다.

`?`이 문자열 슬라이스를 추출하고, 이 문자열 슬라이스의 `chars`를
호출하여 문자들에 대한 반복자를 얻어올 수 있습니다. 이 첫번째 라인의
마지막 문자에 관심이 있으므로, `last`를 호출하여 이 반복자의 마지막
아이템을 얻어옵니다. `"\nhi"`처럼 빈 줄로 시작하지만 다른 줄에는 문자가
담겨있는 경우처럼, 첫번째 라인이 빈 문자열일 수 있으므로 반복자의 결과는
`Option`입니다. 만약 첫번째 라인에 어떤 마지막 문자가 있다면 `Some`
variant를 반환할 것입니다. 가운데 `?` 연산자가 이러한 로직을 표현할
간단한 방식을 제공하여 이 함수를 한줄로 작성할 수 있도록 해줍니다.
만일 `Option`에 대하여 `?`  연산자를 이용할 수 없었다면 더 많은
메서드 호출 혹은 `match` 표현식을 사용하여 이 로직을 구현했어야 할 것입니다.

`Result`를 반환하는 함수에서는 `Result`에서 `?` 연산자를 사용할 수 있고,
`Option`을 반환하는 함수에서는 `Option`에 대해 `?` 연산자를 사용할 수 있지만,
이를 섞어서 사용할 수 는 없음을 주목하세요. `?` 연산자는 `Result`를 `Option`으로
자동으로 변환하거나 혹은 그 반대를 할 수 없습니다; 그러한 경우에는
`Result`의 `ok` 메서드 혹은 `Option`의 `ok_or` 메서드 같은 것을 통해
명시적으로 변환을 할 수 있습니다.

여기까지 우리가 다뤄본 `main` 함수는 `()`를 반환했습니다. `main` 함수는
실행 프로그램의 시작점과 종료점이므로 특별한 경우로서, 프로그램이 기대한
대로 동작하기 위해서는 어떠한 반환 타입을 가질 수 있는지에 대한 제약사항이
있습니다.

다행히도 `main`은 `Result<(), E>`도 반환할 수 있습니다. Listing 9-12는
Listing 9-10의 코드에서 `main`의 반환 타입을 `Result<(), Box<dyn Error>>`로
변경하고 함수 마지막에 반환값 `Ok(())`를 추가한 것입니다. 이 코드는 이제
컴파일될 것입니다:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

<span class="caption">Listing 9-12: `main`이 `Result<(), E>`을 반환하도록
하여 `Result` 값에 대한 `?` 사용 가능하게 하기</span>

`Box<dyn Error>` 타입은 *트레잇 오브젝트 (trait object)*인데, 17장의
[“트레잇 오브젝트를 사용하여 서로 다른 타입에 대한 값 허용하기”][trait-objects]<!-- ignore -->절에서
다룰 예정입니다. 지금은 `Box<dyn Error>`가 “어떠한 종류의 에러”를
의미한다고 읽으면 됩니다. `main` 내에서 에러 타입 `Box<dyn Error>`를
갖는 `Result`에 대해 `?`의 사용이 허용되는데, 이는 어떠한 `Err` 값을
일찍 반환하도록 해주기 때문입니다. `main` 함수의 구현 내용이
`std::io::Error` 타입의 에러만 반환할지라도, `Box<dyn Error>`라고
명시함으로써 이 함수 시그니처는 이후 `main`의 구현체에 다른 에러들을
반환하는 코드가 추가될지라도 계속 올바르게 될 것입니다.

`main` 함수가 `Result<(), E>`를 반환하게 되면, 실행 파일은 `main`이
`Ok(())`를 반환할 경우 `0` 값으로 종료되고, `main`이 `Err` 값을 반환할
경우 0이 아닌 값으로 종료됩니다. C로 작성된 실행파일은 종료될 때 정수값을
반환합니다: 성공적으로 종료된 프로그램은 정수 `0`을 반환하고, 에러가
발생한 프로그램은 `0`이 아닌 어떤 정수값을 반환합니다. 러스트 또한
이러한 규칙과 호환될 목적으로 실행파일이 정수값을 반환합니다.

`main` 함수가 [`std::process::Termination` 트레잇][termination]<!-- ignore -->을
구현한 어떤 타입을 반환할 수도 있는데, 이는 `ExitCode`를 반환하는
`report`라는 함수를 가지고 있습니다. 여러분이 만든 타입에 대해
`Termination` 트레잇을 구현하려면 표준 라이브러리 문서에서 더 많은
정보를 찾아보세요.

`panic!`을 호출하거나 `Result`를 반환하는 것의 자세한 부분을 논의했으니,
어떤 경우에 어떤 방법을 사용하는 것이 적합할지를 어떻게 결정하는가에 대한
주제로 돌아갑시다.

[handle_failure]: ch02-00-guessing-game-tutorial.html#result-%ED%83%80%EC%9E%85%EC%9C%BC%EB%A1%9C-%EC%9E%A0%EC%9E%AC%EB%90%9C-%EC%8B%A4%ED%8C%A8-%EB%8B%A4%EB%A3%A8%EA%B8%B0
[trait-objects]: ch17-02-trait-objects.html#%ED%8A%B8%EB%A0%88%EC%9E%87-%EA%B0%9D%EC%B2%B4%EB%A5%BC-%EC%82%AC%EC%9A%A9%ED%95%98%EC%97%AC-%EB%8B%A4%EB%A5%B8-%ED%83%80%EC%9E%85-%EA%B0%84%EC%9D%98-%EA%B0%92-%ED%97%88%EC%9A%A9%ED%95%98%EA%B8%B0
[termination]: ../std/process/trait.Termination.html
