# I/O 프로젝트: 커맨드 라인 프로그램 만들기

이번 장에서는 여러분이 지금까지 배운 여러 기술들을 요약하고 표준 라이브러리의
기능을 몇 가지 더 탐색해 보겠습니다. 파일 및 커맨드 입출력을 통해 상호작용하는
커맨드 라인 도구를 만들면서 이제는 여러분이 이해하고 있을 러스트 개념 몇 가지를
연습해 볼 것입니다.

러스트의 속도, 안정성, 단일 바이너리 출력, 그리고 크로스 플랫폼 지원은
커맨드 라인 도구를 만들기 위한 이상적인 언어가 되게끔 하므로, 프로젝트를 위해
고전적인 커맨드 라인 검색 도구인 `grep`(**g**lobally search a **r**egular
**e**xpression and **p**rint)의 직접 구현한 버전을 만들어 보려고 합니다.
가장 단순한 사용례에서 `grep`은 어떤 특정한 파일에서 특정한 문자열을 검색합니다.
이를 위해 `grep`은 파일 경로와 문자열을 인수로 받습니다. 그다음
파일을 읽고, 그 파일에서 중 문자열 인수를 포함하고 있는 라인을 찾고,
그 라인들을 출력합니다.

그러는 와중에 수많은 다른 커맨드 라인 도구들이 사용하는 터미널의 기능을 우리의
커맨드 라인 도구도 사용할 수 있게 하는 방법을 알아보겠습니다. 먼저 환경 변수의
값을 읽어서 사용자가 커맨드 라인 도구의 동작을 설정하도록 할 것입니다. 또한 표준
출력 콘솔 스트림 (`stdout`) 대신 표준 에러 콘솔 스트림 (`stderr`) 에 에러 메시지를
출력하여, 예를 들자면 사용자가 화면을 통해 에러 메시지를 보는 동안에도
성공적인 출력을 파일로 리디렉션할 수 있게끔 할 것입니다.

러스트 커뮤니티 멤버 일원인 앤드루 갈란트 (Andrew Gallant) 가 이미
`ripgrep`이라는 이름의, 모든 기능을 가진 `grep`의 매우 빠른 버전을
만들었습니다. 그에 비해서 지금 만들어 볼 버전은 꽤 단순할 예정이지만, 이 장은
여러분에게 `ripgrep`과 같은 실제 프로젝트를 이해하는 데 필요한 배경
지식을 제공할 것입니다.

이 `grep` 프로젝트는 지금까지 배운 여러 개념을 조합할 것입니다:

* 코드 조직화하기 ([7장][ch7]<!-- ignore -->에서 모듈에 대해 배운
  것들을 사용)
* 벡터와 문자열 사용하기 ([8장][ch8]<!-- ignore -->의 컬렉션)
* 에러 처리하기 ([9장][ch9]<!-- ignore -->)
* 적절한 곳에 트레이트와 라이프타임 사용하기 ([10장][ch10]<!-- ignore
  -->)
* 테스트 작성하기 ([11장][ch11]<!-- ignore -->)

아울러 [13장][ch13]<!-- ignore -->과 [17장][ch17]<!-- ignore -->에서
자세히 다루게 될 클로저 (closure), 반복자 (iterator), 그리고
트레이트 객체 (trait object) 에 대해서도 간략히 소개하겠습니다.

[ch7]: ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[ch8]: ch08-00-common-collections.html
[ch9]: ch09-00-error-handling.html
[ch10]: ch10-00-generics.html
[ch11]: ch11-00-testing.html
[ch13]: ch13-00-functional-features.html
[ch17]: ch17-00-oop.html