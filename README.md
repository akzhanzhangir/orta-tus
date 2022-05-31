<div align="center">
  <h1><code>average color in wasm</code></h1>
</div>

## About

Average color of image with rust wasm-bindgen.


## 🚴 Try

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```


### 🐑 Add it in your package.json and then import in your project

```javascript
import {AverageColor} from "orta_tus";

var avco = new AverageColor()
var image = document.getElementById("image")
let col = avco.get_color_from_image(image,"simple");
console.log(col[0]) // RGB
console.log(col[1]) // RGBA
console.log(col[2]) // HEX
```