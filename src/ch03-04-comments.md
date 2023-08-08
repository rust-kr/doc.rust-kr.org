## 주석

모든 프로그래머들은 쉽게 이해되는 코드를 작성하기 위해 노력하지만,
종종 부연 설명이 필요할 때도 있습니다. 그런 경우 프로그래머들은
*주석 (comment)* 을 코드에 남겨서, 컴파일러는 이를 무시하지만
코드를 읽는 사람들은 유용한 정보를 얻을 수 있게끔 합니다.

간단한 주석의 예를 봅시다:

```rust
// hello, world
```

러스트에서 주석은 두개의 슬래시로 시작하며,
이 주석은 해당 줄의 끝까지 계속됩니다. 한 줄을 넘기는
주석의 경우에는 아래처럼 각 줄마다 `//`를 추가하면 됩니다:

```rust
// 그래서 여기서는 여러 줄의 주석을 달 필요가 있을 정도로
// 복잡한 작업을 하고 있습니다! 휴우! 이 주석으로 무슨 일이
// 일어나고 있는지 설명할 수 있기를 바랍니다.
```

또한 주석은 코드의 뒷 부분에 위치할 수도 있습니다:

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

하지만 아래와 같이 코드 앞줄에 따로 주석을 작성한 형태를
더 자주 보게 될 겁니다.

<span class="filename">파일명: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

러스트는 문서화 주석 (documentation comment) 라고 불리는 또다른 주석 형태를
가지고 있는데, 14장의 [‘Crates.io에 크레이트 배포하기’][publishing]<!-- ignore -->에서
다루도록 하겠습니다.

[publishing]: ch14-02-publishing-to-crates-io.html
