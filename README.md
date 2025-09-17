# Hexa Paint

16進数の入力から白黒のピクセルアート画像を生成するシンプルなRustプログラムです。各16進数の桁は4ピクセルを表し、各ビットがピクセルの色を黒(1)または白(0)で決定します。

## Usage
```bash
cargo run <output-filename> <width> <height>
```

プロンプトが表示されたら16進数の値を入力してください。16進数の桁数は(幅 × 高さ) ÷ 4と等しくなければなりません。

## Example

```bash
vscode ➜ /workspaces/RustPractice/hexa-paint (master) $ cargo run cannonball.png 8 8
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/hexa-paint cannonball.png 8 8`
Enter Hex values: 187E5EFFFF7E7E18
```

<img src="cannonball.png" alt="cannonball" width="512" height="512" />

inspired by https://youtu.be/eCSZNINKOKk?feature=shared
