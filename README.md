# PixDecodeRust
 A neon-rust program that gets the buff from an image and then returns the palette (WIP), 


## How to use
```javascript
const PixDecode = require('../index.node');
const fs = require('fs');

const image = fs.readFileSync('image.png'); // image format png
const palette = PixDecode.png(image);
console.log(palette);
```


## Formats supported tabble
| Format | Supported |
| ------ | ------ |
| PNG | Yes |
| JPEG | Yes |
| GIF | Yes |
| BMP | Yes |
| WEBP | Yes |


## Credits
- [@DigitalZebra](https://github.com/DigitalZebra) for [palette_extract](https://crates.io/crates/palette_extract/0.1.0)
- [@Etemesi254](https://github.com/etemesi254) for [zune-jpeg](https://crates.io/crates/zune-jpeg) and [zune-bmp](https://crates.io/crates/zune-bmp)
- [@Image-rs]() for [png](https://crates.io/crates/png) and [gif](https://crates.io/crates/gif)
-[@qnighy] for [libwebp](https://crates.io/crates/libwebp)


## Technologies
- [Neon](https://neon-bindings.com/)
- [Rust](https://www.rust-lang.org/)
- [Node.js](https://nodejs.org/)