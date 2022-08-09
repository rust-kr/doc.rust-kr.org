# The Rust Programming Language

[The Rust Programming Language](title-page.md)
[들어가기에 앞서](foreword.md)
[소개](ch00-00-introduction.md)

## Getting started

- [시작해봅시다](ch01-00-getting-started.md)
    - [러스트 설치](ch01-01-installation.md)
    - [Hello, World!](ch01-02-hello-world.md)
    - [Cargo를 사용해봅시다](ch01-03-hello-cargo.md)

- [추리 게임](ch02-00-guessing-game-tutorial.md)

- [일반적인 프로그래밍 개념](ch03-00-common-programming-concepts.md)
    - [변수와 가변성](ch03-01-variables-and-mutability.md)
    - [데이터 타입](ch03-02-data-types.md)
    - [함수](ch03-03-how-functions-work.md)
    - [주석](ch03-04-comments.md)
    - [흐름 제어문](ch03-05-control-flow.md)

- [소유권 이해하기](ch04-00-understanding-ownership.md)
    - [소유권이 뭔가요?](ch04-01-what-is-ownership.md)
    - [참조자와 Borrow](ch04-02-references-and-borrowing.md)
    - [슬라이스(Slice)](ch04-03-slices.md)

- [연관된 데이터를 구조체로 구조화하기](ch05-00-structs.md)
    - [구조체 정의 및 인스턴트화](ch05-01-defining-structs.md)
    - [구조체를 사용한 예제 프로그램](ch05-02-example-structs.md)
    - [메소드 문법](ch05-03-method-syntax.md)

- [열거형과 패턴 매칭](ch06-00-enums.md)
    - [열거형 정의하기](ch06-01-defining-an-enum.md)
    - [`match` 흐름 제어 연산자](ch06-02-match.md)
    - [`if let`을 사용한 간결한 흐름 제어](ch06-03-if-let.md)

## Basic Rust Literacy

- [커져가는 프로젝트를 패키지, 크레이트, 모듈로 관리하기](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
    - [패키지, 크레이트](ch07-01-packages-and-crates.md)
    - [모듈을 정의하여 스코프 및 공개 여부 제어하기](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [경로를 사용해 모듈 트리에서 항목 가리키기](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [`use` 키워드로 경로를 스코프 내로 가져오기](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [별개의 파일로 모듈 분리하기](ch07-05-separating-modules-into-different-files.md)

- [일반적인 컬렉션](ch08-00-common-collections.md)
    - [벡터에 여러 값을 목록으로 저장하기](ch08-01-vectors.md)
    - [문자열에 UTF-8 텍스트를 저장하기](ch08-02-strings.md)
    - [해쉬맵(hash map)에 서로 연관된 키와 값을 저장하기](ch08-03-hash-maps.md)

- [에러 처리](ch09-00-error-handling.md)
    - [복구 불가능한 에러에는 `panic!`!](ch09-01-unrecoverable-errors-with-panic.md)
    - [`Result`와 함께하는 복구 가능한 에러](ch09-02-recoverable-errors-with-result.md)
    - [`panic!`이냐, `panic!`이 아니냐, 그것이 문제로다](ch09-03-to-panic-or-not-to-panic.md)

- [제네릭 타입, 트레잇, 라이프타임](ch10-00-generics.md)
    - [제네릭 데이터 타입](ch10-01-syntax.md)
    - [트레잇으로 공통된 동작을 정의하기](ch10-02-traits.md)
    - [라이프타임으로 참조자의 유효성 검증하기](ch10-03-lifetime-syntax.md)

- [자동화 테스트 작성하기](ch11-00-testing.md)
    - [테스트 작성 방법](ch11-01-writing-tests.md)
    - [테스트 실행 제어하기](ch11-02-running-tests.md)
    - [테스트 조직화](ch11-03-test-organization.md)

- [An I/O Project: Building a Command Line Program](ch12-00-an-io-project.md)
    - [Accepting Command Line Arguments](ch12-01-accepting-command-line-arguments.md)
    - [Reading a File](ch12-02-reading-a-file.md)
    - [Refactoring to Improve Modularity and Error Handling](ch12-03-improving-error-handling-and-modularity.md)
    - [Developing the Library’s Functionality with Test Driven Development](ch12-04-testing-the-librarys-functionality.md)
    - [Working with Environment Variables](ch12-05-working-with-environment-variables.md)
    - [Writing Error Messages to Standard Error Instead of Standard Output](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Thinking in Rust

- [함수형 언어의 특성들: 반복자와 클로저](ch13-00-functional-features.md)
    - [클로저: 환경을 캡처할 수 있는 익명 함수](ch13-01-closures.md)
    - [Processing a Series of Items with Iterators](ch13-02-iterators.md)
    - [Improving Our I/O Project](ch13-03-improving-our-io-project.md)
    - [Comparing Performance: Loops vs. Iterators](ch13-04-performance.md)

- [More about Cargo and Crates.io](ch14-00-more-about-cargo.md)
    - [Customizing Builds with Release Profiles](ch14-01-release-profiles.md)
    - [Publishing a Crate to Crates.io](ch14-02-publishing-to-crates-io.md)
    - [Cargo Workspaces](ch14-03-cargo-workspaces.md)
    - [Installing Binaries from Crates.io with `cargo install`](ch14-04-installing-binaries.md)
    - [Extending Cargo with Custom Commands](ch14-05-extending-cargo.md)

- [Smart Pointers](ch15-00-smart-pointers.md)
    - [Using `Box<T>` to Point to Data on the Heap](ch15-01-box.md)
    - [Treating Smart Pointers Like Regular References with the `Deref` Trait](ch15-02-deref.md)
    - [Running Code on Cleanup with the `Drop` Trait](ch15-03-drop.md)
    - [`Rc<T>`, the Reference Counted Smart Pointer](ch15-04-rc.md)
    - [`RefCell<T>` and the Interior Mutability Pattern](ch15-05-interior-mutability.md)
    - [Reference Cycles Can Leak Memory](ch15-06-reference-cycles.md)

- [Fearless Concurrency](ch16-00-concurrency.md)
    - [Using Threads to Run Code Simultaneously](ch16-01-threads.md)
    - [Using Message Passing to Transfer Data Between Threads](ch16-02-message-passing.md)
    - [Shared-State Concurrency](ch16-03-shared-state.md)
    - [Extensible Concurrency with the `Sync` and `Send` Traits](ch16-04-extensible-concurrency-sync-and-send.md)

- [Object Oriented Programming Features of Rust](ch17-00-oop.md)
    - [Characteristics of Object-Oriented Languages](ch17-01-what-is-oo.md)
    - [Using Trait Objects That Allow for Values of Different Types](ch17-02-trait-objects.md)
    - [Implementing an Object-Oriented Design Pattern](ch17-03-oo-design-patterns.md)

## Advanced Topics

- [Patterns and Matching](ch18-00-patterns.md)
    - [All the Places Patterns Can Be Used](ch18-01-all-the-places-for-patterns.md)
    - [Refutability: Whether a Pattern Might Fail to Match](ch18-02-refutability.md)
    - [Pattern Syntax](ch18-03-pattern-syntax.md)

- [Advanced Features](ch19-00-advanced-features.md)
    - [Unsafe Rust](ch19-01-unsafe-rust.md)
    - [Advanced Traits](ch19-03-advanced-traits.md)
    - [Advanced Types](ch19-04-advanced-types.md)
    - [Advanced Functions and Closures](ch19-05-advanced-functions-and-closures.md)
    - [Macros](ch19-06-macros.md)

- [Final Project: Building a Multithreaded Web Server](ch20-00-final-project-a-web-server.md)
    - [Building a Single-Threaded Web Server](ch20-01-single-threaded.md)
    - [Turning Our Single-Threaded Server into a Multithreaded Server](ch20-02-multithreaded.md)
    - [Graceful Shutdown and Cleanup](ch20-03-graceful-shutdown-and-cleanup.md)

- [Appendix](appendix-00.md)
    - [A - Keywords](appendix-01-keywords.md)
    - [B - Operators and Symbols](appendix-02-operators.md)
    - [C - Derivable Traits](appendix-03-derivable-traits.md)
    - [D - Useful Development Tools](appendix-04-useful-development-tools.md)
    - [E - Editions](appendix-05-editions.md)
    - [F - Translations of the Book](appendix-06-translation.md)
    - [G - How Rust is Made and “Nightly Rust”](appendix-07-nightly-rust.md)
