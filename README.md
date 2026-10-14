# PC Status to OSC VRChat Chatbox - PCOSCVRC

VRChat のチャットボックスに、この PC の状態（CPU・メモリ・GPU・ディスク）を OSC で表示します。

```
CPU: 27% | RAM: 80% (12.4 GB / 15.3 GB) | GPU: 38% | VRAM: 49% (2.0 GB / 4.0 GB) | Disk: R 1.1 MB/s W 0 B/s | [テスト中]
```

値は [pcsc-rs](https://github.com/Zel9278/pcsc-rs)（PC Status のクライアント）と同じ方法で、この PC から直接読みます。PC Status のサーバーや pcsc-rs を動かしておく必要はなく、ネットにつながっていなくても動きます。GPU は NVIDIA・AMD・Intel に対応しています。

v1（Node.js）からの変更: Node.js と pnpm は不要になり、exe 1つで動きます。v1 は PC Status の v1 サーバー（socket.io）から値をもらっていましたが、v2 のサーバーとはつながらないので、その場で読むようにしました。

## 使い方

1. [Releases](https://github.com/Zel9278/VRChat-PC-Status-to-OSC-Chat/releases/latest) から `vrc-pc-status-<バージョン>-x86_64-pc-windows-msvc.exe` をダウンロードします。
2. VRChat で OSC を有効にします（アクションメニュー → オプション → OSC → 有効）。
3. ダウンロードした exe を起動します。3 秒ごとにチャットボックスへ送ります。

起動したウィンドウに文字を打って Enter を押すと、その文字がチャットボックスの最後に `[…]` で付きます。何も打たずに Enter を押すと消えます。

## オプション

| オプション | 内容 |
|---|---|
| `--interval <秒>` | 送る間隔（既定 3、最短 2。速すぎると VRChat が受け付けないことがあります） |
| `--text <文字>` | 起動したときに付ける文字 |
| `--format <書式>` | チャットボックスの文字を自分で決める。`\n` で改行 |
| `--sound` | 送るたびにチャットボックスの通知音を鳴らす |
| `--host <アドレス>` / `--port <ポート>` | VRChat の OSC の受け口（既定 `127.0.0.1` / `9000`） |

`--format` で使える置き換え: `{cpu}` `{ram}` `{ram_used}` `{ram_total}` `{gpu}` `{vram}` `{vram_used}` `{vram_total}` `{disk_read}` `{disk_write}` `{text}`。値が無いもの（GPU の無い PC など）は `--` になります。

```bat
vrc-pc-status.exe --format "CPU {cpu} / GPU {gpu}\nRAM {ram_used} / {ram_total}\n{text}"
```

チャットボックスは 144 文字までなので、それを超えた分は切れます。

## ビルド

```sh
cargo build --release
```

Rust 1.98 以降が必要です（`rust-toolchain.toml` で 1.99 を使います）。
