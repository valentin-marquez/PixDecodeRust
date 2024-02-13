const decode = require('./index.node');
const fs = require('fs');

const JPEG_PATH = "C:/Users/valen/OneDrive/Escritorio/JPG_Test.jpg";
const PNG_PATH = "C:/Users/valen/OneDrive/Escritorio/PNG_Test.png";

async function readImage(imagePath) {
    let image = await fs.promises.readFile(imagePath);
    return image;
}

async function main() {
    let stattime = Date.now();
    let image_png = await readImage(PNG_PATH);
    let image_jpg = await readImage(JPEG_PATH);


    console.log("Decoding with rust addon");
    let png_palette = decode.png(image_png);
    let jpg_palette = decode.jpg(image_jpg);

    console.log(png_palette);
    console.log(jpg_palette);

    let endtime = Date.now();
    console.log("Time: ", endtime - stattime, "ms");
}

main();