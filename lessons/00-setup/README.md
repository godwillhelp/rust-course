# L0. Установка (без оценки)

Всё ставится на ваш рабочий ПК с Windows 10/11. Команды выполняются в PowerShell.

## 1. Компилятор Rust

Rust на Windows собирает программы линкером Microsoft, поэтому нужны MSVC Build Tools и Windows SDK.

1. Скачайте `rustup-init.exe` с https://rustup.rs и запустите.
2. Если Visual Studio не установлена, установщик предложит поставить её автоматически («Quick install via the Visual Studio Community installer», пункт 1). Соглашайтесь.
3. Затем выберите стандартную установку Rust (тоже пункт 1).
4. Закройте и заново откройте PowerShell, чтобы обновился PATH.

Альтернатива через winget (команда из официальной документации rustup):

```
winget install --id Microsoft.VisualStudio.2022.Community --source winget --force --override "--add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 --add Microsoft.VisualStudio.Component.Windows11SDK.22621 --addProductLang En-us"
winget install --id Rustlang.Rustup
```

Английский языковой пакет Visual Studio нужен, чтобы сообщения линкера были на английском: их проще искать.

Проверка:

```
rustc --version
cargo --version
rustup show
```

Ожидается `rustc 1.100.0` или новее. `clippy` и `rustfmt` ставятся вместе с профилем по умолчанию.

## 2. Редактор Zed

```
winget install --id ZedIndustries.Zed
```

Поддержка Rust в Zed встроена. Если `rust-analyzer` нет в PATH, Zed скачает свою стабильную версию сам.

Выключите ИИ-функции на время курса: решения пишутся руками. Откройте настройки (`Ctrl+,`) и добавьте:

```json
{
  "disable_ai": true
}
```

## 3. Git и репозиторий курса

```
winget install --id Git.Git
winget install --id GitHub.cli
gh auth login
gh repo clone godwillhelp/rust-course
cd rust-course
git config user.name "godwillhelp"
git config user.email "337953335+godwillhelp@users.noreply.github.com"
```

## 4. Первая сборка

```
cargo build
cargo test -p l01-task1
```

Сборка должна пройти без ошибок. Тесты `l01-task1` упадут с `not yet implemented`: это заглушки, их вы реализуете в задании 1. Зафиксируйте созданный `Cargo.lock`:

```
git add Cargo.lock
git commit -m "Add Cargo.lock"
git push
```

## 5. Rustlings (практика)

Ставится отдельно от репозитория курса, в любую другую папку:

```
cargo install rustlings
cd D:\study
rustlings init
cd rustlings
rustlings
```

## Готово, если

- [ ] `cargo --version` печатает версию;
- [ ] Zed открывает папку курса, при наведении на переменную видно её тип;
- [ ] `cargo build` в корне курса проходит;
- [ ] `rustlings` запускается и показывает `intro1`.
