# egc(emoji-generator-cli)

Slack などで使用する絵文字を生成する CLI ツール

# Install

Mac でのみ使用可能です。

## Homebrew

```
brew tap apple-yagi/tap
brew install egc
```

# Usage

## 絵文字を作成(デフォルトでは文字の色が pink)

```
egc 完全に理解した
```

生成物

![完全に理解した](https://github.com/apple-yagi/egc/assets/57742720/b5f676a1-2b49-470f-9e64-612357942034)

## 文字の色を指定して絵文字を作成

```
egc 完全に理解した -c yellow
```

生成物

![完全に理解した](https://github.com/apple-yagi/egc/assets/57742720/23256e8d-51ec-43f7-995d-aee0d793286a)

指定できる色

- pink
- yellow
- black
- red
- green
- blue
