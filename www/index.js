import {AverageColor} from "orta_tus";

var avco = new AverageColor()
var image = document.getElementById("image")

// let canvas = document.createElement('canvas')
// let ctx = canvas.getContext("2d")

// canvas.width =300;
// canvas.height =300;
// ctx.drawImage(image, 0,0, 300,300,0,0,300,300)

// let data = ctx.getImageData(0, 0, 300,300).data;

// console.time("oldFunc");
// let color = avco.get_color_from_clamp(data);
// console.timeEnd("oldFunc");

// console.log(color.rgb())

console.time("oldFunc");
let col = avco.get_color_from_image(image,"simple");
console.timeEnd("oldFunc");
console.log(col[0])

var canvas = document.getElementById("simpleCanvas");
var ctx = canvas.getContext("2d");
ctx.fillStyle = col[0];
ctx.fillRect(0, 0, canvas.width, canvas.height);

//oldfunc();
let newCol = avco.get_color_from_image(image,"sqrt");

console.log(newCol[0])

var canvas = document.getElementById("sqrtCanvas");
var ctx = canvas.getContext("2d");
ctx.fillStyle = newCol[0];
ctx.fillRect(0, 0, canvas.width, canvas.height);
