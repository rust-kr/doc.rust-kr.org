## 파일 읽기

이제는 `file_path` 인수에 명시된 파일을 읽는 기능을 추가해 보겠습니다.
우선 테스트에 사용할 샘플 파일이 필요합니다: 여러 줄의 몇 개의 반복된
단어들로 구성된 작은 양의 텍스트로 된 파일을 사용하겠습니다. 예제 12-3은
딱 맞게 사용될 에밀리 딕킨슨 (Emily Dickinson) 의 시가 있습니다! 프로젝트의
루트 레벨에 *poem.txt*이라는 이름의 파일을 만들고, 시 “I’m Nobody! Who are
you?”를 입력하세요.

<span class="filename">파일명: poem.txt</span>

```text
{{#include ../listings/ch12-an-io-project/listing-12-03/poem.txt}}
```

<span class="caption">예제 12-3: 에밀리 딕킨슨의 시는 좋은 테스트 케이스를
만들어 줍니다</span>

텍스트를 채워 넣었다면 예제 12-4에서 보는 것처럼 *src/main.rs*에 파일을
읽는 코드를 추가하세요.

<span class="filename">파일명: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/src/main.rs:here}}
```

<span class="caption">예제 12-4: 두 번째 인수로 명시된 파일의
내용물 읽기</span>

먼저 `use` 구문을 사용하여 표준 라이브러리의 연관된 부분을 가져옵니다:
파일을 다루기 위해서는 `std::fs`가 필요하죠.

`main`에서, 새로운 구문 `fs::read_to_string`이 `file_path`를 받아서 그
파일을 열고, 파일 내용물의 `std::io::Result<String>`을 반환합니다.

그런 뒤, 다시 한번 임시로 `println!` 구문을 추가하여 파일을 읽은 후
`contents`의 값을 출력함으로써, 현재까지의 프로그램이 잘 작동하는지
확인합니다.

첫 번째 커맨드 라인 인수에는 아무 문자열이나 넣고 (아직 검색 부분은 구현하지
않았으므로) 두 번째 인수에는 *poem.txt* 파일을 넣어서 이 코드를
실행해 봅시다:

```console
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/output.txt}}
```

훌륭해요! 코드가 파일의 내용물을 읽은 뒤 출력했습니다. 하지만 이 코드는
몇 가지 결점이 있습니다. 현재 `main` 함수는 여러 가지 기능을 하고 있습니다:
일반적으로는 함수 하나당 오직 하나의 아이디어에 대한 기능을 구현할 때
함수가 더 명료해지고 관리하기 쉬워집니다. 또 한 가지 문제는 우리가 할 수
있는 만큼의 에러 처리를 안 하고 있다는 점입니다. 프로그램은 여전히 작고,
따라서 이러한 결점이 큰 문제는 아니지만, 프로그램이 커짐에 따라 이 문제들은
깔끔하게 고치기 어려워질 것입니다. 프로그램을 개발할 때 일찍 리팩터링하는 것은
좋은 관행인데, 이는 작은 양의 코드를 리팩터링하는 것이 훨씬 쉽기 때문입니다.
이걸 바로 다음에 하겠습니다.
