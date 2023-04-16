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
    - [`match` 흐름 제어 구성](ch06-02-match.md)
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

- [I/O 프로젝트: 커맨드 라인 프로그램 만들기](ch12-00-an-io-project.md)
    - [커맨드 라인 인자 받기](ch12-01-accepting-command-line-arguments.md)
    - [파일 읽기](ch12-02-reading-a-file.md)
    - [모듈성과 에러 처리 향상을 위한 리팩토링](ch12-03-improving-error-handling-and-modularity.md)
    - [테스트 주도 개발로 라이브러리 기능 개발하기](ch12-04-testing-the-librarys-functionality.md)
    - [환경 변수 사용하기](ch12-05-working-with-environment-variables.md)
    - [표준 출력 대신 표준 에러로 에러 메세지 작성하기](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Thinking in Rust

- [함수형 언어의 특성들: 반복자와 클로저](ch13-00-functional-features.md)
    - [클로저: 자신의 환경을 캡처하는 익명 함수](ch13-01-closures.md)
    - [반복자로 일련의 아이템들 처리하기](ch13-02-iterators.md)
    - [I/O 프로젝트 개선하기](ch13-03-improving-our-io-project.md)
    - [성능 비교하기: 루프 vs. 반복자](ch13-04-performance.md)

- [Cargo와 Crates.io 더 알아보기](ch14-00-more-about-cargo.md)
    - [릴리즈 프로필을 통한 빌드 커스터마이징하기](ch14-01-release-profiles.md)
    - [Crates.io에 크레이트 배포하기](ch14-02-publishing-to-crates-io.md)
    - [Cargo 작업공간](ch14-03-cargo-workspaces.md)
    - [`cargo install`로 Crates.io에 있는 바이너리 설치하기](ch14-04-installing-binaries.md)
    - [커스텀 명령어로 Cargo 확장하기](ch14-05-extending-cargo.md)

- [스마트 포인터](ch15-00-smart-pointers.md)
    - [`Box<T>`를 사용하여 힙에 있는 데이터 가리키기](ch15-01-box.md)
    - [`Deref` 트레잇으로 스마트 포인터를 보통의 참조자처럼 취급하기](ch15-02-deref.md)
    - [`Drop` 트레잇으로 메모리 정리 코드 실행하기](ch15-03-drop.md)
    - [`Rc<T>`, 참조 카운트 스마트 포인터](ch15-04-rc.md)
    - [`RefCell<T>`와 내부 가변성 패턴](ch15-05-interior-mutability.md)
    - [순환 참조는 메모리 릭을 발생시킬 수 있습니다](ch15-06-reference-cycles.md)

- [겁없는 동시성](ch16-00-concurrency.md)
    - [스레드를 이용하여 코드를 동시에 실행하기](ch16-01-threads.md)
    - [메세지 패싱을 사용하여 스레드 간 데이터 전송하기](ch16-02-message-passing.md)
    - [공유 상태 동시성](ch16-03-shared-state.md)
    - [`Sync`와 `Send` 트레잇을 이용한 확장 가능한 동시성](ch16-04-extensible-concurrency-sync-and-send.md)

- [러스트의 객체 지향 프로그래밍 기능들](ch17-00-oop.md)
    - [객체 지향 언어의 특성](ch17-01-what-is-oo.md)
    - [트레잇 객체를 사용하여 다른 타입의 값 허용하기](ch17-02-trait-objects.md)
    - [객체 지향 디자인 패턴 구현하기](ch17-03-oo-design-patterns.md)

## Advanced Topics

- [패턴과 매칭](ch18-00-patterns.md)
    - [패턴이 사용될 수 있는 모든 곳](ch18-01-all-the-places-for-patterns.md)
    - [반박 가능성 (Refutability): 패턴이 매칭에 실패할지의 여부](ch18-02-refutability.md)
    - [패턴 문법](ch18-03-pattern-syntax.md)

- [고급 기능들](ch19-00-advanced-features.md)
    - [안전하지 않은 러스트](ch19-01-unsafe-rust.md)
    - [고급 트레잇](ch19-03-advanced-traits.md)
    - [고급 타입](ch19-04-advanced-types.md)
    - [고급 함수와 클로저](ch19-05-advanced-functions-and-closures.md)
    - [매크로](ch19-06-macros.md)

- [최종 프로젝트: 멀티스레드 웹 서버 구축하기](ch20-00-final-project-a-web-server.md)
    - [싱글스레드 웹 서버 구축하기](ch20-01-single-threaded.md)
    - [싱글스레드 서버를 멀티스레드 서버로 바꾸기](ch20-02-multithreaded.md)
    - [우아한 종료와 정리](ch20-03-graceful-shutdown-and-cleanup.md)

- [Appendix](appendix-00.md)
    - [A - Keywords](appendix-01-keywords.md)
    - [B - Operators and Symbols](appendix-02-operators.md)
    - [C - Derivable Traits](appendix-03-derivable-traits.md)
    - [D - Useful Development Tools](appendix-04-useful-development-tools.md)
    - [E - Editions](appendix-05-editions.md)
    - [F - Translations of the Book](appendix-06-translation.md)
    - [G - How Rust is Made and “Nightly Rust”](appendix-07-nightly-rust.md)
