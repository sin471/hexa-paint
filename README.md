# Hexa Paint

A simple Rust program that generates black and white pixel art images from hexadecimal input. Each hexadecimal digit represents 4 pixels, with each bit determining whether a pixel is black (1) or white (0).

## Usage
```bash
cargo run <output-filename> <width> <height>
```

Enter hexadecimal values when prompted. The number of hex digits must equal (width × height) ÷ 4.

## Example

```bash
vscode ➜ /workspaces/RustPractice/hexa-paint (master) $ cargo run cannonball.png 8 8
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/hexa-paint cannonball.png 8 8`
Enter Hex values: 187E5EFFFF7E7E18
```

<img src="cannonball.png" alt="cannonball" width="512" height="512" />
