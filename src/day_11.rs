use axum::extract::Multipart;
use image::GenericImageView;

pub async fn task_2(mut multipart: Multipart) -> String {
    let Some(field) = multipart.next_field().await.unwrap() else {
        return 0.to_string();
    };

    let img = image::load_from_memory(&field.bytes().await.unwrap()).unwrap();
    img.pixels()
        .filter(|(_, _, color)| {
            let [red, green, blue, _] = color.0;
            red as u16 > green as u16 + blue as u16
        })
        .count()
        .to_string()
}
