# Testing

# Commands

* 병렬 테스트

```shell
$ cargo test -- --test-threads=1
```

* 출력 값을 보기 위해서는 출력 캡쳐를 비활성화한다.

```shell
$ cargo test -- --nocapture
```

* 테스트 일부만 실행

```shell
$ cargo test <TEST-NAME>
```

* `ignored` 어노테이션으로 제외된 테스트 실행

```shell
$ cargo test -- --ignored
```