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

絵文字を作成(デフォルトでは文字の色が pink)

```
egc 完全に理解した
```

文字の色を指定して絵文字を作成

```
egc 完全に理解した -c yellow
```

指定できる色

- pink
- yellow
- black
- red
- green
- blue
