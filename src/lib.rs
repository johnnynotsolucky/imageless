use crate::{
	operations::{AdjustBrightness, Blur, Crop, Grayscale, Resize},
	Unit::{Percentage, Pixel},
};
use image::{io::Reader as ImageReader, DynamicImage};
use serde::{Deserialize, Serialize};
use std::{
	io,
	ops::{Add, Sub},
	path::Path,
};
use thiserror::Error;

pub mod operations;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PixelUnit {
	pixels: u32,
}

impl From<PixelUnit> for u32 {
	fn from(value: PixelUnit) -> Self {
		value.pixels
	}
}

impl From<u32> for PixelUnit {
	fn from(pixels: u32) -> Self {
		Self { pixels }
	}
}

impl Sub for PixelUnit {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		assert!(self.pixels > rhs.pixels);
		Self::from(self.pixels - rhs.pixels)
	}
}

impl Add for PixelUnit {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self::from(self.pixels + rhs.pixels)
	}
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PercentageUnit {
	percentage: f32,
}

#[derive(Error, Clone, Copy, Debug, PartialOrd, PartialEq)]
#[error("Percentage out of range: {percentage}")]
pub struct PercentageOutOfRangeError {
	pub percentage: f32,
}

impl PercentageOutOfRangeError {
	fn new(percentage: f32) -> Self {
		Self { percentage }
	}
}

impl TryFrom<f32> for PercentageUnit {
	type Error = PercentageOutOfRangeError;

	fn try_from(value: f32) -> std::result::Result<Self, Self::Error> {
		if !(0.0..=1.0).contains(&value) {
			return Err(PercentageOutOfRangeError::new(value));
		}

		Ok(Self { percentage: value })
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Unit {
	Pixel(PixelUnit),
	Percentage(PercentageUnit),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Coordinate {
	x: Unit,
	y: Unit,
}

impl Unit {
	#[inline]
	fn as_pixel(&self, dimension: PixelUnit) -> PixelUnit {
		match self {
			Pixel(pixels) => *pixels,
			Percentage(percentage) => {
				let dimension = dimension.pixels as f32;
				let pixels = dimension * percentage.percentage;
				PixelUnit::from(pixels as u32)
			}
		}
	}
}

#[derive(Error, Debug)]
#[error("Error processing image: {message}")]
pub struct OperationError {
	pub message: String,
}

impl OperationError {
	fn new(message: String) -> Self {
		Self { message }
	}
}

pub trait Process {
	fn process(&self, image: DynamicImage) -> Result<DynamicImage, OperationError>;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Operation {
	AdjustBrightness(AdjustBrightness),
	Blur(Blur),
	Crop(Crop),
	Grayscale(Grayscale),
	Resize(Resize),
}

impl Operation {
	pub fn get_process(&self) -> &dyn Process {
		match self {
			Self::AdjustBrightness(adjust) => adjust,
			Self::Blur(blur) => blur,
			Self::Crop(crop) => crop,
			Self::Grayscale(grayscale) => grayscale,
			Self::Resize(resize) => resize,
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ImageOutputFormat {
	/// An Image in PNG Format
	Png,
	/// An Image in JPEG Format with specified quality, up to 100
	Jpeg { quality: u8 },
	// /// An Image in one of the PNM Formats
	// Pnm(PnmSubtype),
	/// An Image in GIF Format
	Gif,
	/// An Image in ICO Format
	Ico,
	/// An Image in BMP Format
	Bmp,
	/// An Image in farbfeld Format
	Farbfeld,
	/// An Image in TGA Format
	Tga,
	/// An Image in OpenEXR Format
	OpenExr,
	/// An Image in TIFF Format
	Tiff,
	/// An image in AVIF Format
	Avif,
	/// An image in QOI Format
	Qoi,
	/// An image in WebP Format.
	WebP,
}

impl From<ImageOutputFormat> for image::ImageOutputFormat {
	fn from(value: ImageOutputFormat) -> Self {
		match value {
			ImageOutputFormat::Png => Self::Png,
			ImageOutputFormat::Jpeg { quality } => Self::Jpeg(quality),
			ImageOutputFormat::Gif => Self::Gif,
			ImageOutputFormat::Ico => Self::Ico,
			ImageOutputFormat::Bmp => Self::Bmp,
			ImageOutputFormat::Farbfeld => Self::Farbfeld,
			ImageOutputFormat::Tga => Self::Tga,
			ImageOutputFormat::OpenExr => Self::OpenExr,
			ImageOutputFormat::Tiff => Self::Tiff,
			ImageOutputFormat::Avif => Self::Avif,
			ImageOutputFormat::Qoi => Self::Qoi,
			ImageOutputFormat::WebP => Self::WebP,
		}
	}
}

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	OperationError(#[from] OperationError),

	#[error(transparent)]
	PercentageOutOfRangeError(#[from] PercentageOutOfRangeError),

	#[error("IO error")]
	IoError(#[from] io::Error),

	#[error("Image error")]
	ImageError(#[from] image::ImageError),
}

pub fn process_file<P: AsRef<Path>>(
	in_path: P,
	operations: Vec<Operation>,
) -> Result<DynamicImage, Error> {
	let mut image = ImageReader::open(in_path)?.decode()?;

	for operation in operations.into_iter() {
		image = operation.get_process().process(image)?;
	}

	Ok(image)
}
