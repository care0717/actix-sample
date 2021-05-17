# actix-sample
todoリストを管理するサーバーとフロント

## How to use
```
cargo build
wasm-pack build --target web --out-name wasm --out-dir ./static/wasm ./front
cargo run
open http://localhost:8080/index.html   
```

## 参考
https://speakerdeck.com/helloyuk13/rusthanzuondi-4hui-webbatukuendobian  
https://speakerdeck.com/helloyuk13/rusthanzuondi-5hui-webassemblybian
