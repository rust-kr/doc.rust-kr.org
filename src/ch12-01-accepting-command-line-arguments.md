## 커맨드 라인 인자 받기

언제나처럼 `cargo new`로 새 프로젝트를 만들어봅시다. 여러분의 시스템에 이미
설치되어 있을지도 모를 `grep` 도구와 구분하기 위하여 우리 프로젝트 이름은
`minigrep`으로 하겠습니다.

```console
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

`minigrep`을 만들기 위한 첫 과제는 두 개의 커맨드 라인 인자를 받는 것입니다:
바로 검색할 파일 경로와 문자열이지요. 그 말인 즉슨, 다음과 같이 프로그램을 실행하기
위해 `cargo run`와, `cargo` 대신 우리 프로그램을 위한 인자가 나올 것임을 알려주는
두 개의 하이픈, 검색을 위한 문자열, 그리고 검색하길 원하는 파일을 사용할 수 있도록
하고 싶다는 것입니다:

```console
$ cargo run -- searchstring example-filename.txt
```

지금 시점에서 `cargo new`로 생성된 프로그램은 전달하는 인자를 처리할
수 없습니다. [crates.io](https://crates.io/)에 있는 몇 가지 라이브러리가
커맨드 라인 인자를 받는 프로그램 작성에 도움되겠지만, 지금은 이 개념을 막
배우고 있는 중이므로 직접 이 기능을 구현해봅시다.

### 인자값 읽기

`minigrep`이 커맨드 라인 인자로 넘겨진 값들을 읽을 수 있도록 하기 위해서는
러스트의 표준 라이브러리가 제공하는 `std::env::args` 함수를 사용할 필요가
있겠습니다. 이 함수는 `minigrep`으로 넘겨진 커맨드 라인 인자의 반복자(iterator)를
반환합니다. 반복자에 대해서는 [13장][ch13]<!-- ignore -->에서 모든 내용을 다룰
예정입니다. 지금은 반복자에 대한 두 가지 세부사항만 알면 됩니다: 반복자는
일련의 값들을 생성하고, 반복자의 `collect` 메소드를 호출하여 반복자가
생성하는 모든 원소들을 담고 있는 벡터 같은 콜렉션으로 바꿀 수 있다는
것입니다.

Listing 12-1의 코드는 `minigrep` 프로그램이 넘겨진 어떤 커맨드 라인 인자들을
읽은 후, 그 값들을 벡터로 모아주도록 해줍니다.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-01/src/main.rs}}
```

<span class="caption">Listing 12-1: 커맨드 라인 인자들을 벡터로 모으고
출력하기</span>

먼저 `use`를 사용하여 `std::env` 모듈을 스코프로 가져와서 `args`
함수를 사용할 수 있게 합니다. `std::env::args` 함수는 모듈의 2단계로
중첩되어 있는 점을 주목하세요. [7장][ch7-idiomatic-use]<!-- ignore -->에서
논의한 것처럼, 하나 이상의 모듈로 중첩된 곳에 원하는 함수가 있는 경우에는,
함수 보다는 차라리 그 부모 모듈을 스코프로 가져오는 선택을 했습니다.
그렇게 함으로써 `std::env`의 다른 함수들도 쉽게 사용할 수 있습니다.
또한 `use std::env::args`를 추가하고 `args` 만으로 함수를 호출하는
것보다 이쪽이 덜 모호한데, 이는 `args`가 현재의 모듈 내에 정의된 다른
함수로 쉽게 오해받을 수 있을지도 모르기 때문입니다.

> ### `args` 함수와 유효하지 않은 유니코드
>
> 어떤 인자라도 유효하지 않은 유니코드를 포함한다면 `std::env::args`가 패닉을 일으킨다는
> 점을 주의하세요. 만일 여러분의 프로그램이 유효하지 않은 유니코드를 포함하는 인자들을
> 받을 필요가 있다면, 대신 `std::env::args_os`를 사용하세요. 이 함수는 `String` 대신
> `OsString` 값을 생성하는 반복자를 반환합니다. 여기서는 단순함을 위해 `std::env::args`을
> 사용했는데, 이는 `OsString` 값이 플랩폼 별로 다르고 `String` 값을 가지고 작업하는
> 것 보다 더 복잡하기 때문입니다.

`main`의 첫째 줄에서는 `env::args`를 호출한 즉시 `collect`를
사용하여 반복자에 의해 만들어지는 모든 값을 담고 있는 벡터로 바꿉니다.
`colloect` 함수를 사용하여 다양한 종류의 콜렉션을 만들 수 있으므로,
`args`의 타입을 명시적으로 표기하여 문자열의 벡터가 필요하다는 것을
명시하도록 합니다. 러스트에서는 타입을 표기할 필요가 거의 없지만,
러스트가 여러분이 원하는 종류의 콜렉션을 추론할 수는 없으므로
`collect`는 타입 표기가 자주 필요한 함수 중 하나입니다.

마지막으로 디버그 매크로를 사용하여 벡터를 출력합니다. 먼저 인자 없이 코드를
실행해보고, 그 다음 인자 두 개를 넣어 실행해봅시다:

```console
{{#include ../listings/ch12-an-io-project/listing-12-01/output.txt}}
```

```console
{{#include ../listings/ch12-an-io-project/output-only-01-with-args/output.txt}}
```

벡터의 첫번째 값이 `"target/debug/minigrep"`, 즉 이 바이너리 파일의
이름인 점을 주목하세요. 이는 C에서의 인자 리스트의 동작과 일치하며,
프로그램 실행 중 호출된 이름을 프로그램이 사용할 수 있게 해줍니다.
프로그램의 이름에 접근할 수 있는 것은 메세지에 이름을 출력하고 싶을 때라던가
프로그램을 호출할 때 사용된 커맨드 라인 별명이 무엇이었는지에 기반하여
프로그램의 동작을 바꾸고 싶을때 종종 편리하게 이용됩니다. 하지만 이 장의 목적을
위해서 지금은 이걸 무시하고 우리가 필요로 하는 두 인자만 저장하겠습니다.

### 인자값들을 변수에 저장하기

이 프로그램은 현재 커맨드 라인 인자로 지정된 값들에 접근할 수 있습니다.
이제는 두 인자의 값들을 변수에 저장할 필요가 있는데, 그렇게 하면 프로그램의
나머지 부분을 통틀어 이 값들을 사용할 수 있겠습니다. Listing 12-2에서
이 동작을 수행합니다.

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-02/src/main.rs}}
```

<span class="caption">Listing 12-2: 질의 인자와 파일 경로 인자를 담은
변수 생성하기</span>

벡터를 출력할 때 봤듯이, 프로그램의 이름이 벡터의 첫번째 값 `args[0]`을
잡아먹으므로, 인덱스 `1`에 있는 인자부터 시작하는 중입니다.
`minigrep`이 취하는 첫번째 인자는 검색하고자 하는 문자열이므로,
첫번째 인자의 참조자를 `query` 변수에 집어넣습니다. 두번째 인자는
파일 경로가 될 것이므로, 두번째 인자의 참조자를 `file_path`에
집어넣습니다.

우리 의도대로 코드가 동작하는지 검증하기 위해 이 변수의 값들을 임시로
출력하겠습니다. `test`와 `sample.txt`를 인자로 하여 이 프로그램을
다시 실행해봅시다:

```console
{{#include ../listings/ch12-an-io-project/listing-12-02/output.txt}}
```

프로그램이 훌륭하게 동작하네요! 우리가 필요로 하는 인자의 값들이 올바른 변수들에
저장되고 있습니다. 나중에는 사용자가 아무런 인자를 제공했을 경우 같이 에러가
발생할 수 있는 특정한 경우를 다루기 위한 에러 처리 기능을 몇가지 추가할
것입니다; 지금은 그런 경우를 무시하고 파일 읽기 기능을 추가하는 작업으로
넘어가겠습니다.

[ch13]: ch13-00-functional-features.html
[ch7-idiomatic-use]: ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths
