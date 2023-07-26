# Cargo & Crates.io

Cargo 와 Crates.io 의 사용법에 대해 더 알아보자.

# 프로필을 이용한 빌드 커스터마이징

```shell
$ cargo build --release
```

`Cargo.toml` 을 이용해서 프로필을 설정 가능하다

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

# 문서화 주석 

```shell
$ cargo doc --open
```

# Crates.io 배포

* 계정 설정: 
https://crates.io/settings/tokens 에 접속하여 토큰 획득

```shell
$ cargo login <YOUR_TOKEN>
```

* 배포: `Cargo.toml` 에 

```shell
$ cargo publish
```