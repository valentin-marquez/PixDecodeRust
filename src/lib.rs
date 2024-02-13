use neon::{prelude::*, types::buffer::TypedArray};
use palette_extract::{get_palette_rgb, Color};

struct Image {
    pixels: Vec<u8>,
}

impl Image {
    fn to_object<'a, C: Context<'a>>(cx: &mut C, img: Image) -> Handle<'a, JsObject> {
        let obj = JsObject::new(cx);

        let r: Vec<Color> = get_palette_rgb(&img.pixels);
        let palette = JsArray::new(cx, r.len() as u32);
        r.iter().enumerate().for_each(|(_i, color)| {
            let obj = JsObject::new(cx);
            let r = cx.number(color.r as f64);
            let g = cx.number(color.g as f64);
            let b = cx.number(color.b as f64);
            obj.set(cx, "r", r).unwrap();
            obj.set(cx, "g", g).unwrap();
            obj.set(cx, "b", b).unwrap();
            palette.set(cx, _i as u32, obj).unwrap();
        });

        obj.set(cx, "palette", palette).unwrap();

        obj
    }
}

fn decode(mut cx: FunctionContext) -> JsResult<JsObject> {
    let buffer = cx.argument::<JsBuffer>(0)?;
    let data = buffer.as_slice(&mut cx);

    let pixels = image::load_from_memory(data)
        .expect("Failed to load image")
        .to_rgba8()
        .into_raw();

    let obj = Image::to_object(&mut cx, Image { pixels });

    Ok(obj)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("decode", decode)?;
    Ok(())
}
