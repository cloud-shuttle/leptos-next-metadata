//! Image encoding for OG image generation
//!
//! This module handles converting SVG content to images and encoding them
//! in various formats (PNG, JPEG, WebP).

use crate::{Error, Result};

#[cfg(feature = "og-images")]
use {
    image::{DynamicImage, Rgba, RgbaImage},
    resvg::usvg::{self, TreeParsing},
    tiny_skia,
};

impl super::OgImageGenerator {
    #[cfg(feature = "og-images")]
    /// Convert SVG content to image
    pub async fn svg_to_image(
        &self,
        svg_content: &str,
        params: &super::OgImageParams,
    ) -> Result<DynamicImage> {
        let size = params.size.unwrap_or(self.config.default_size);

        // Configure usvg options
        let options = usvg::Options::default();

        // Parse SVG with usvg
        let tree = usvg::Tree::from_str(svg_content, &options)
            .map_err(|e| Error::ImageError(format!("Failed to parse SVG: {}", e)))?;

        // Create pixmap for rendering
        let mut pixmap = tiny_skia::Pixmap::new(size.0, size.1)
            .ok_or_else(|| Error::ImageError("Failed to create pixmap".to_string()))?;

        // Render SVG to pixmap
        let transform = usvg::Transform::from_scale(
            size.0 as f32 / tree.size.width(),
            size.1 as f32 / tree.size.height(),
        );
        resvg::Tree::from_usvg(&tree).render(transform, &mut pixmap.as_mut());

        // Convert pixmap to image
        let pixels = pixmap.data();
        let mut image = RgbaImage::new(size.0, size.1);

        for (i, chunk) in pixels.chunks_exact(4).enumerate() {
            let x = (i as u32) % size.0;
            let y = (i as u32) / size.0;

            // tiny_skia uses premultiplied RGBA, convert to normal RGBA
            let alpha = chunk[3];
            let (r, g, b) = if alpha > 0 {
                (
                    ((chunk[2] as u16 * 255) / alpha as u16) as u8,
                    ((chunk[1] as u16 * 255) / alpha as u16) as u8,
                    ((chunk[0] as u16 * 255) / alpha as u16) as u8,
                )
            } else {
                (0, 0, 0)
            };

            image.put_pixel(x, y, Rgba([r, g, b, alpha]));
        }

        Ok(DynamicImage::ImageRgba8(image))
    }

    #[cfg(feature = "og-images")]
    /// Encode image to output format
    pub async fn encode_image(
        &self,
        image: &DynamicImage,
        _params: &super::OgImageParams,
    ) -> Result<Vec<u8>> {
        let mut output = Vec::new();

        match self.config.format {
            crate::ImageFormat::PNG => {
                image
                    .write_to(
                        &mut std::io::Cursor::new(&mut output),
                        image::ImageFormat::Png,
                    )
                    .map_err(|e| Error::ImageError(format!("PNG encoding error: {}", e)))?;
            }
            crate::ImageFormat::JPEG => {
                let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(
                    &mut output,
                    self.config.quality,
                );
                image
                    .write_with_encoder(encoder)
                    .map_err(|e| Error::ImageError(format!("JPEG encoding error: {}", e)))?;
            }
            crate::ImageFormat::WebP => {
                #[cfg(feature = "webp-support")]
                {
                    use webp::Encoder;

                    let rgba = image.to_rgba8();
                    let encoder = Encoder::from_rgba(rgba.as_raw(), rgba.width(), rgba.height());
                    let webp_data = encoder.encode(self.config.quality as f32);
                    output = webp_data.to_vec();
                }

                #[cfg(not(feature = "webp-support"))]
                {
                    return Err(Error::ImageError(
                        "WebP encoding requires the 'webp-support' feature flag. Add 'webp-support' to your Cargo.toml features.".to_string(),
                    ));
                }
            }
        }

        Ok(output)
    }
}
