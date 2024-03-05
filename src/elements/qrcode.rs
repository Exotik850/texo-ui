use base64::Engine;
use dioxus::prelude::*;

use image::Luma;
use qrcode::QrCode as QrCodeInner;

#[component]
pub fn QrCode<P: AsRef<[u8]> + Clone + PartialEq + 'static>(
    data: P,
    #[props(extends=img)] rest_attributes: Vec<Attribute>,
) -> Element {
    let code = QrCodeInner::new(data).ok()?;
    let img = code.render::<Luma<u8>>().build();
    let mut bytes = vec![];
    let mut cursor = std::io::Cursor::new(&mut bytes);
    img.write_to(
        &mut std::io::BufWriter::new(&mut cursor),
        image::ImageOutputFormat::Png,
    )
    .ok()?;
    let encoded = base64::engine::general_purpose::STANDARD_NO_PAD.encode(&bytes);
    rsx!( img { ..rest_attributes, src: "data:image/png;base64,{encoded}" } )
}
