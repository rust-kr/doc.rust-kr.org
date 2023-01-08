# 러스트 프로그래밍 언어

### 알립니다

반갑습니다! 저는 이 저장소를 운영하고 있는 rinthel입니다.

저는 2017년 7월경 러스트 공부를 하던 중 당시 최신의 러스트 문서의 (영어에서조차
묻어나는) 친절한 어조와 자세한 설명에 감명을 받았었고, 이 정도면 저수준 프로그래밍
언어에 능숙하지 않은 분들도 읽어보면 러스트를 쉽게 접할 수 있겠다는 생각으로 인해
생전 해본 적 없는 번역 작업을 통해 커뮤니티 기여에 도전하게 되었습니다. 그리고
수 많은 분들의 contribution에 의해 2판 한국어 번역본을 온라인 상에서 제공할 수
있었습니다.

직장 생활에 치여 거의 2년간 업데이트를 하지 못하고 있던 중, 지난 2022년말
제이펍 출판사가 2021년 버전의 문서를 번역해보지 않겠냐는 제안을 해 주셨고,
고민 끝에 제안을 받아들여 현재의 러스트 문서 2021년 판을 제가 번역하기로
하였습니다. **따라서 초벌 번역이 완성될 때까지는 별도의 번역 contribution은
받지 않도록 할 예정입니다.** 양해 부탁드리겠습니다. 🙇‍♂️

하지만 출판사의 협조 덕분에, github을 통한 번역 작업 및 초벌 번역본의
공개가 가능해졌습니다. 앞으로의 번역 작업은 여전히 이 저장소에서 진행될 예정이며,
매 commit 마다 자동적으로 아래 링크에 있는 빌드된 문서가 갱신될 것입니다.
이 자리를 빌어 제이펍 출판사 관계자 여러분께 감사의 말씀을 전합니다.

### 환영합니다!

[![빌드 상태](https://github.com/rust-kr/doc.rust-kr.org/actions/workflows/build.yml/badge.svg)](https://github.com/rust-kr/doc.rust-kr.org/actions/workflows/build.yml)

[2021판 번역본의 빌드된 문서 바로가기](https://rust-kr.github.io/doc.rust-kr.org)

안녕하세요? 한국어를 쓰시는 러스트 유저 여러분들을 환영합니다.
이 저장소는 러스트 프로그래밍 언어 문서(소위 the book이라고 일컫는)의 
2021년 판에 대한 번역 작업을 위해 만들어졌습니다.

2번째 판의 번역을 위한 저장소는 아래 링크를 통해 읽으실 수 있습니다.

- [2판 번역본](https://rinthel.github.io/rust-lang-book-ko)
- [2판 번역본 저장소](https://github.com/rinthel/rust-lang-book-ko): 현재는 오타 수정만 하고 있습니다

혹시 함께 번역에 참여하고 싶으신 분들, 환영합니다! :)
저에게 메세지를 주시거나 discussions에 요청을 남겨주시면,
collaborator로 등록해 드리겠습니다!

### 번역 기조

#### 친절한 구어체

1번째 판도 그렇지만 2번째 판을 보면서 느낀 점은, 기초 C 프로그래밍 정도의 수준을
익힌 분들 혹은 스크립트 언어만 공부해본 분들 또한 읽기 쉽게끔 기초 개념에 충실한
설명을 하고 있다는 점입니다. 우리가 러스트의 저변을 더 넓히고자 한다면 보다
친절한 어투가 좋겠다고 생각하고 있습니다.

#### 번역하기 애매한 용어는 가급적 원어로

번역하기 애매한 용어를 억지로 한글화 하는 것 보다는 원어를 그대로 사용하는 편이
오해를 줄이는데 더 도움이 될거라고 생각하고 있습니다.

### 2018-edition 번역 과정 제안

- chapter 별로 issue를 만들어주세요
  * 혹시 같은 chapter에 대한 issue가 이미 만들어져있는지,
    assignee 가 할당되어 있는지 확인해주세요
- 해당 chapter에 대한 second edition 번역본을 비교하여 동일한 부분을 우선 붙여넣기 합시다
  * 붙여넣기 하면서 혹시 예제 코드 번호 등이 변경되진 않았는지 살펴봅시다
- 붙여넣은 부분에서 기존 번역 중 어색한 부분이 있다면 수정한 다음, 1차 pull-request를 날려줍시다.
- 새로 추가된 부분에 대한 번역을 진행한 뒤, 2차 pull-request를 날려줍시다.

### 오타 및 오역 수정 관련

repo fork -> 수정 후 pull-request 하셔도 좋고, 오타 및 오역에 대한
issue를 만들어 주셔도 좋습니다.

### 현재까지 번역 용어 정리 

번역 용어 정리는 Appendix - H에서 관리할 예정입니다.
더 좋은 번역 용어가 있으신 분들은 discussions에 의견 주시면 반영하겠습니다!

### 기타 사항

기타 논의할 사항 혹은 건의할 내용이 있다면 discussions에 남겨서 함께
이야기하면 좋겠습니다.

<!-- 1번째 판에 대한 번역 작업은 [sarojaba님께서 운영하시는 penflip 페이지](
https://www.penflip.com/sarojaba/rust-doc-korean/blob/master/About.txt)에
거의 완성되어 있으므로, 혹시 교본을 보러 오신 분이라면 이쪽을 추천하고 싶습니다. :)

이 저장소는 앞서 말씀드렸듯이 2번째 판에 대한 번역 작업을 위해 만들어졌습니다. 아래에 있는 설명에서
보시는 것과 같이, 러스트 문서는 현재 [mdbook](https://github.com/azerupi/mdBook)이라고
부르는 마크다운 기반의 웹문서 작성 툴을 이용해 빌드되는데, 이 툴에 다국어 지원 기능이 완성되면
번역본들도 함께 합쳐질 예정이라고 합니다. 한편, 2번째 판의 내용은 1번째 판과 많은 부분이 달라졌기에
새로 번역해볼 필요 겸 공부할 차원에서 만들어진 저장소라고 보시면 되겠습니다.

현재 2번째 판의 문서는 수시로 고쳐지고 있는 상태지만, [원본 저장소](https://github.com/rust-lang/book)에
프로젝트란을 보시면 frozen column이라고 되어 있는 부분들은 거의 완성되어 많은 수정이 이루어지지 않을 것이라고
언급되고 있기에, 이런 부분들을 위주로 먼저 번역해보고자 합니다. -->

---

# The Rust Programming Language
This repository contains the source of "The Rust Programming Language" book.

[The book is available in dead-tree form from No Starch Press][nostarch].

[nostarch]: https://nostarch.com/rust

You can also read the book for free online. Please see the book as shipped with
the latest [stable], [beta], or [nightly] Rust releases. Be aware that issues
in those versions may have been fixed in this repository already, as those
releases are updated less frequently.

[stable]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[nightly]: https://doc.rust-lang.org/nightly/book/

See the [releases] to download just the code of all the code listings that appear in the book.

[releases]: https://github.com/rust-lang/book/releases

## Requirements

Building the book requires [mdBook], ideally the same version that
rust-lang/rust uses in [this file][rust-mdbook]. To get it:

[mdBook]: https://github.com/rust-lang-nursery/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --version <version_num>
```

## Building

To build the book, type:

```bash
$ mdbook build
```

The output will be in the `book` subdirectory. To check it out, open it in
your web browser.

_Firefox:_
```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_
```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

To run the tests:

```bash
$ mdbook test
```

## Contributing

We'd love your help! Please see [CONTRIBUTING.md][contrib] to learn about the
kinds of contributions we're looking for.

[contrib]: https://github.com/rust-lang/book/blob/main/CONTRIBUTING.md

Because the book is [printed](https://nostarch.com/rust), and because we want
to keep the online version of the book close to the print version when
possible, it may take longer than you're used to for us to address your issue
or pull request.

So far, we've been doing a larger revision to coincide with [Rust
Editions](https://doc.rust-lang.org/edition-guide/). Between those larger
revisions, we will only be correcting errors. If your issue or pull request
isn't strictly fixing an error, it might sit until the next time that we're
working on a large revision: expect on the order of months or years. Thank you
for your patience!

### Translations

We'd love help translating the book! See the [Translations] label to join in
efforts that are currently in progress. Open a new issue to start working on
a new language! We're waiting on [mdbook support] for multiple languages
before we merge any in, but feel free to start!

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/rust-lang-nursery/mdBook/issues/5

## Spellchecking

To scan source files for spelling errors, you can use the `spellcheck.sh`
script available in the `ci` directory. It needs a dictionary of valid words,
which is provided in `ci/dictionary.txt`. If the script produces a false
positive (say, you used word `BTreeMap` which the script considers invalid),
you need to add this word to `ci/dictionary.txt` (keep the sorted order for
consistency).
