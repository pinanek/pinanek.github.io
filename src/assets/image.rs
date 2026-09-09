use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{BufWriter, Cursor, Write},
    path::{Path, PathBuf, absolute},
    sync::atomic::{AtomicU64, Ordering},
};

use anyhow::{Context, Error, Result, anyhow, bail};
use base64::prelude::*;
use image::{
    DynamicImage, ExtendedColorType, ImageEncoder, ImageFormat, RgbaImage,
    codecs::{
        avif::AvifEncoder,
        jpeg::JpegEncoder,
        png::{CompressionType, FilterType as PngFilterType, PngEncoder},
        webp::WebPEncoder,
    },
    imageops::{self, FilterType},
};
use image::{GenericImageView, ImageDecoder, ImageReader};
use rayon::prelude::*;
use strum::{Display, EnumString, IntoStaticStr};
use thumbhash::{rgba_to_thumb_hash, thumb_hash_to_rgba};

use crate::config::CONFIG;

const RESPONSIVE_WIDTHS: &[u32] = &[320, 480, 640, 768, 960, 1280, 1536, 1920, 2560];

const HASH_LENGTH: usize = 12;
const JPEG_QUALITY: u8 = 90;

static TEMP_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

const AVIF_SPEED: u8 = 8;
const AVIF_QUALITY: u8 = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum AssetImageFormat {
    Avif,
    Webp,
    Png,
    Jpeg,
}

impl AssetImageFormat {
    pub const fn to_image_format(self) -> ImageFormat {
        match self {
            Self::Avif => ImageFormat::Avif,
            Self::Webp => ImageFormat::WebP,
            Self::Png => ImageFormat::Png,
            Self::Jpeg => ImageFormat::Jpeg,
        }
    }

    pub fn mime_type(self) -> &'static str {
        self.to_image_format().to_mime_type()
    }
}

impl From<AssetImageFormat> for ImageFormat {
    fn from(format: AssetImageFormat) -> Self {
        match format {
            AssetImageFormat::Avif => Self::Avif,
            AssetImageFormat::Webp => Self::WebP,
            AssetImageFormat::Png => Self::Png,
            AssetImageFormat::Jpeg => Self::Jpeg,
        }
    }
}

impl TryFrom<ImageFormat> for AssetImageFormat {
    type Error = Error;

    fn try_from(format: ImageFormat) -> Result<Self, Self::Error> {
        match format {
            ImageFormat::Avif => Ok(Self::Avif),
            ImageFormat::WebP => Ok(Self::Webp),
            ImageFormat::Png => Ok(Self::Png),
            ImageFormat::Jpeg => Ok(Self::Jpeg),
            _ => bail!("unsupported image format: {format:?}"),
        }
    }
}

#[derive(Debug)]
struct LoadedImage {
    path: PathBuf,
    image: DynamicImage,
    format: AssetImageFormat,
    source_hash: blake3::Hash,
}

#[derive(Debug)]
pub struct ImageAsset {
    pub source_path: PathBuf,
    source_hash: blake3::Hash,
    pub output_path: PathBuf,
    pub public_url: String,
    pub format: AssetImageFormat,
    pub width: u32,
    pub height: u32,
    pub is_original: bool,
}

type PictureSources = HashMap<AssetImageFormat, Vec<ImageAsset>>;

#[derive(Debug)]
pub struct PictureAsset {
    original: ImageAsset,
    sources: PictureSources,
    placeholder: String,
}

impl PictureAsset {
    pub fn sources(&self) -> &PictureSources {
        &self.sources
    }

    pub fn original(&self) -> &ImageAsset {
        &self.original
    }

    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    pub fn srcsets(&self) -> Vec<(AssetImageFormat, String)> {
        let mut srcsets = self
            .sources
            .iter()
            .map(|(&format, images)| {
                let srcset = images
                    .iter()
                    .map(|image| format!("{} {}w", image.public_url, image.width))
                    .collect::<Vec<_>>()
                    .join(", ");

                (format, srcset)
            })
            .collect::<Vec<_>>();

        srcsets.sort_by_key(|(format, _)| image_format_priority(*format));

        srcsets
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ImageRequest {
    Image(ImageAsset),
    Picture(PictureAsset),
}

impl ImageRequest {
    // pub fn image(path: impl AsRef<Path>, format: AssetImageFormat) -> Result<Self> {
    //     let source = Self::load_image(path)?;
    //     let (width, height) = source.image.dimensions();

    //     Ok(Self::Image(Self::create_image_asset(
    //         &source, width, height, format, false,
    //     )?))
    // }

    // pub fn image_resize(
    //     path: impl AsRef<Path>,
    //     width: u32,
    //     height: u32,
    //     format: AssetImageFormat,
    // ) -> Result<Self> {
    //     if width == 0 || height == 0 {
    //         return Err(anyhow!("image dimensions must be greater than zero"));
    //     }

    //     let source = Self::load_image(path)?;

    //     Ok(Self::Image(Self::create_image_asset(
    //         &source, width, height, format, false,
    //     )?))
    // }

    pub fn picture(path: impl AsRef<Path>, formats: &[AssetImageFormat]) -> Result<Self> {
        let source = Self::load_image(path)?;
        let (original_width, original_height) = source.image.dimensions();

        if original_width == 0 || original_height == 0 {
            return Err(anyhow!(
                "invalid image dimensions: {}",
                source.path.display()
            ));
        }

        let original = Self::create_image_asset(
            &source,
            original_width,
            original_height,
            source.format,
            true,
        )?;

        let widths = responsive_widths(original_width);

        let mut sources = PictureSources::with_capacity(formats.len());
        let mut seen_formats = HashSet::with_capacity(formats.len());

        for &format in formats {
            if !seen_formats.insert(format) {
                continue;
            }

            let images = widths
                .iter()
                .map(|&width| {
                    let height = scaled_height(width, original_width, original_height);

                    Self::create_image_asset(&source, width, height, format, false)
                })
                .collect::<Result<Vec<_>>>()?;

            sources.insert(format, images);
        }

        let placeholder = Self::compute_placeholder(&source.image)?;

        Ok(Self::Picture(PictureAsset {
            original,
            sources,
            placeholder,
        }))
    }

    fn load_image(path: impl AsRef<Path>) -> Result<LoadedImage> {
        let path = absolute(path.as_ref()).with_context(|| {
            format!(
                "failed to resolve absolute image path: {}",
                path.as_ref().display()
            )
        })?;

        let raw =
            fs::read(&path).with_context(|| format!("failed to read image: {}", path.display()))?;

        let format = image::guess_format(&raw)
            .with_context(|| format!("failed to detect image format: {}", path.display()))?;

        let source_hash = blake3::hash(&raw);

        let image = decode_image(&raw, &path)?;

        Ok(LoadedImage {
            path,
            image,
            format: AssetImageFormat::try_from(format)?,
            source_hash,
        })
    }

    fn create_image_asset(
        source: &LoadedImage,
        width: u32,
        height: u32,
        format: AssetImageFormat,
        is_original: bool,
    ) -> Result<ImageAsset> {
        let hash = asset_hash(&source.source_hash, width, height, format, is_original).to_hex();

        let short_hash = &hash.as_str()[..HASH_LENGTH];

        let extension = format.to_string();

        let filename = format!("image.{short_hash}.{extension}");
        let output_path = CONFIG.dist_assets_dir.join(filename);

        let public_path = output_path
            .strip_prefix(&CONFIG.dist_dir)
            .with_context(|| {
                format!(
                    "image output path is outside dist directory: {}",
                    output_path.display()
                )
            })?;

        let public_url = format!("/{}", public_path.to_string_lossy().replace('\\', "/"));

        Ok(ImageAsset {
            source_path: source.path.clone(),
            source_hash: source.source_hash,
            output_path,
            public_url,
            format,
            width,
            height,
            is_original,
        })
    }

    fn compute_placeholder(source: &DynamicImage) -> Result<String> {
        let thumbnail = source.thumbnail(100, 100).to_rgba8();

        let hash = rgba_to_thumb_hash(
            thumbnail.width() as usize,
            thumbnail.height() as usize,
            thumbnail.as_raw(),
        );

        let (width, height, rgba) =
            thumb_hash_to_rgba(&hash).map_err(|_| anyhow!("failed to decode ThumbHash"))?;

        let placeholder = RgbaImage::from_raw(width as u32, height as u32, rgba)
            .context("failed to create placeholder image")?;

        let mut output = Cursor::new(Vec::new());

        DynamicImage::ImageRgba8(placeholder)
            .write_to(&mut output, ImageFormat::Png)
            .context("failed to encode placeholder image")?;

        Ok(BASE64_STANDARD.encode(output.into_inner()))
    }
}

pub fn thumbhash_placeholder(path: impl AsRef<Path>) -> Result<String> {
    let source = ImageRequest::load_image(path)?;

    ImageRequest::compute_placeholder(&source.image)
}

pub type ImageRequests = HashMap<String, ImageRequest>;

pub fn write_images(requests: &[&ImageRequest]) -> Result<()> {
    fs::create_dir_all(&CONFIG.dist_assets_dir).with_context(|| {
        format!(
            "failed to create image output directory: {}",
            CONFIG.dist_assets_dir.display(),
        )
    })?;

    fs::create_dir_all(&CONFIG.cache_images_dir).with_context(|| {
        format!(
            "failed to create image cache directory: {}",
            CONFIG.cache_images_dir.display(),
        )
    })?;

    let mut images_by_source = HashMap::<&Path, Vec<&ImageAsset>>::new();
    let mut seen_outputs = HashSet::<&Path>::new();

    for request in requests {
        match request {
            ImageRequest::Image(image) => {
                insert_image(image, &mut seen_outputs, &mut images_by_source);
            }

            ImageRequest::Picture(picture) => {
                let images =
                    std::iter::once(picture.original()).chain(picture.sources().values().flatten());

                for image in images {
                    insert_image(image, &mut seen_outputs, &mut images_by_source);
                }
            }
        }
    }

    images_by_source
        .par_iter()
        .try_for_each(|(source_path, images)| write_images_for_source(source_path, images))?;

    Ok(())
}

fn insert_image<'a>(
    image: &'a ImageAsset,
    seen_outputs: &mut HashSet<&'a Path>,
    images_by_source: &mut HashMap<&'a Path, Vec<&'a ImageAsset>>,
) {
    if !seen_outputs.insert(&image.output_path) {
        return;
    }

    images_by_source
        .entry(&image.source_path)
        .or_default()
        .push(image);
}

fn write_images_for_source(source_path: &Path, images: &[&ImageAsset]) -> Result<()> {
    let raw = fs::read(source_path)
        .with_context(|| format!("failed to read source image: {}", source_path.display()))?;
    let source_hash = blake3::hash(&raw);

    for image in images {
        if image.source_hash != source_hash {
            bail!(
                "source image changed while preparing assets: {}",
                source_path.display()
            );
        }
    }

    if images.iter().all(|image| image.output_path.exists()) {
        return Ok(());
    }

    let source = decode_image(&raw, source_path)?.into_rgba8();

    for image in images {
        if image.output_path.exists() {
            continue;
        }

        if image.is_original {
            encode_image_atomically(&image.output_path, &source, image.format)?;
            continue;
        }

        let cache_path = cached_image_path(image);

        if !cache_path.exists() {
            let resized;
            let variant = if source.width() == image.width && source.height() == image.height {
                &source
            } else {
                resized =
                    imageops::resize(&source, image.width, image.height, FilterType::Lanczos3);
                &resized
            };

            encode_image_atomically(&cache_path, variant, image.format)?;
        }

        copy_file_atomically(&cache_path, &image.output_path)?;
    }

    Ok(())
}

fn decode_image(raw: &[u8], path: &Path) -> Result<DynamicImage> {
    let reader = ImageReader::new(Cursor::new(raw))
        .with_guessed_format()
        .with_context(|| format!("failed to detect source image format: {}", path.display()))?;
    let mut decoder = reader
        .into_decoder()
        .with_context(|| format!("failed to create image decoder: {}", path.display()))?;
    let orientation = decoder
        .orientation()
        .with_context(|| format!("failed to read image orientation: {}", path.display()))?;
    let mut image = DynamicImage::from_decoder(decoder)
        .with_context(|| format!("failed to decode source image: {}", path.display()))?;

    image.apply_orientation(orientation);

    Ok(image)
}

fn encode_image_atomically(path: &Path, image: &RgbaImage, format: AssetImageFormat) -> Result<()> {
    if path.exists() {
        return Ok(());
    }

    let (temporary_path, file) = create_temporary_file(path)?;
    let result = (|| -> Result<()> {
        let mut writer = BufWriter::new(file);

        encode_image(&mut writer, image, format)
            .with_context(|| format!("failed to encode image: {}", path.display()))?;
        writer
            .flush()
            .with_context(|| format!("failed to flush image: {}", path.display()))?;
        writer
            .get_ref()
            .sync_all()
            .with_context(|| format!("failed to sync image: {}", path.display()))?;

        Ok(())
    })();

    if let Err(error) = result {
        let _ = fs::remove_file(&temporary_path);
        return Err(error);
    }

    publish_temporary_file(&temporary_path, path)
}

fn copy_file_atomically(source: &Path, destination: &Path) -> Result<()> {
    if destination.exists() {
        return Ok(());
    }

    let (temporary_path, file) = create_temporary_file(destination)?;
    drop(file);

    let result = (|| -> Result<()> {
        fs::copy(source, &temporary_path).with_context(|| {
            format!(
                "failed to copy image from {} to {}",
                source.display(),
                temporary_path.display(),
            )
        })?;
        fs::File::open(&temporary_path)
            .with_context(|| format!("failed to open copied image: {}", temporary_path.display()))?
            .sync_all()
            .with_context(|| {
                format!("failed to sync copied image: {}", temporary_path.display())
            })?;

        Ok(())
    })();

    if let Err(error) = result {
        let _ = fs::remove_file(&temporary_path);
        return Err(error);
    }

    publish_temporary_file(&temporary_path, destination)
}

fn create_temporary_file(path: &Path) -> Result<(PathBuf, fs::File)> {
    let parent = path
        .parent()
        .with_context(|| format!("image output path has no parent: {}", path.display()))?;
    let filename = path
        .file_name()
        .with_context(|| format!("image output path has no filename: {}", path.display()))?
        .to_string_lossy();
    let counter = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let temporary_path = parent.join(format!(".{filename}.{}.{}", std::process::id(), counter));
    let file = fs::File::options()
        .write(true)
        .create_new(true)
        .open(&temporary_path)
        .with_context(|| {
            format!(
                "failed to create temporary image: {}",
                temporary_path.display()
            )
        })?;

    Ok((temporary_path, file))
}

fn publish_temporary_file(temporary_path: &Path, destination: &Path) -> Result<()> {
    fs::rename(temporary_path, destination).with_context(|| {
        format!(
            "failed to publish image from {} to {}",
            temporary_path.display(),
            destination.display(),
        )
    })
}

fn encode_image(
    writer: &mut BufWriter<fs::File>,
    image: &RgbaImage,
    format: AssetImageFormat,
) -> Result<()> {
    let raw = image.as_raw();
    let width = image.width();
    let height = image.height();

    match format {
        AssetImageFormat::Avif => {
            AvifEncoder::new_with_speed_quality(writer, AVIF_SPEED, AVIF_QUALITY).write_image(
                raw,
                width,
                height,
                ExtendedColorType::Rgba8,
            )?;
        }
        AssetImageFormat::Webp => {
            WebPEncoder::new_lossless(writer).write_image(
                raw,
                width,
                height,
                ExtendedColorType::Rgba8,
            )?;
        }
        AssetImageFormat::Png => {
            PngEncoder::new_with_quality(writer, CompressionType::Default, PngFilterType::Adaptive)
                .write_image(raw, width, height, ExtendedColorType::Rgba8)?;
        }
        AssetImageFormat::Jpeg => {
            JpegEncoder::new_with_quality(writer, JPEG_QUALITY).encode_image(image)?;
        }
    }

    Ok(())
}

fn cached_image_path(image: &ImageAsset) -> PathBuf {
    let filename = image
        .output_path
        .file_name()
        .expect("image output path has no filename");

    CONFIG.cache_images_dir.join(filename)
}

fn responsive_widths(original_width: u32) -> Vec<u32> {
    let mut widths = RESPONSIVE_WIDTHS
        .iter()
        .copied()
        .take_while(|&width| width < original_width)
        .collect::<Vec<_>>();

    widths.push(original_width);

    widths
}

fn scaled_height(width: u32, original_width: u32, original_height: u32) -> u32 {
    let width = u64::from(width);
    let original_width = u64::from(original_width);
    let original_height = u64::from(original_height);

    let height = (width * original_height + original_width / 2) / original_width;

    height.max(1) as u32
}

fn asset_hash(
    source_hash: &blake3::Hash,
    width: u32,
    height: u32,
    format: AssetImageFormat,
    is_original: bool,
) -> blake3::Hash {
    let mut hasher = blake3::Hasher::new();

    hasher.update(source_hash.as_bytes());
    hasher.update(&width.to_le_bytes());
    hasher.update(&height.to_le_bytes());
    hasher.update(format.to_string().as_bytes());
    hasher.update(&[u8::from(is_original)]);

    hasher.finalize()
}

fn image_format_priority(format: AssetImageFormat) -> u8 {
    match format {
        AssetImageFormat::Avif => 0,
        AssetImageFormat::Webp => 1,
        AssetImageFormat::Jpeg => 2,
        AssetImageFormat::Png => 3,
    }
}
