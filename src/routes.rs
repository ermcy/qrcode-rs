use base64::Engine;
use qrcode::{QrCode, QrResult};
use qrcode::render::svg;
use tide::Request;
use image::{DynamicImage, ImageResult, Luma};
pub async fn get_world(mut _request: Request<()>) -> tide::Result {
    let string = "Hello, world!";
    Ok(string.into())
}


pub async fn get_some_word<'a>(mut request: Request<()>) -> tide::Result {
    let mut buffer = vec![];
    let input = match request.param("input") {
        Ok(i) => i,
        Err(err) => {
            return Ok(err.to_string().into());
        }
    };
    let data =format!("{}", input);
    let code = QrCode::new(data);
    match code {
        Ok(res) => {
            let original_image = res.render::<Luma<u8>>()
                .min_dimensions(128, 128)
                .max_dimensions(256, 256)
                .quiet_zone(false)
                .build();
            let dyn_image = DynamicImage::ImageLuma8(original_image);
            let final_image = dyn_image.write_to(&mut buffer, image::ImageOutputFormat::Png);
            match final_image {
                Ok(_) => {
                    let base64 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(buffer);
                    Ok(base64.into())
                },
                Err(err) => {
                    let error_message = format!("Error: {:#?}", err);
                    Ok(error_message.into())
                }
            }
        }
        Err(err) => {
            let error_message = format!("Error: {:#?}", err);
            Ok(error_message.into())
        }
    }

}