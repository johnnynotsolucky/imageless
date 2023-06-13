use crate::{OperationError, PixelUnit, Process, Unit};
use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Resize {
	pub width: Unit,
	pub height: Unit,
	pub filter: FilterType,
	pub crop_mode: CropMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FilterType {
	/// Nearest Neighbor
	Nearest,
	/// Linear Filter
	Triangle,
	/// Cubic Filter
	CatmullRom,
	/// Gaussian Filter
	Gaussian,
	/// Lanczos with window 3
	Lanczos3,
}

impl Default for FilterType {
	fn default() -> Self {
		Self::Nearest
	}
}

impl From<FilterType> for image::imageops::FilterType {
	fn from(filter: FilterType) -> Self {
		match filter {
			FilterType::Nearest => Self::Nearest,
			FilterType::Triangle => Self::Triangle,
			FilterType::CatmullRom => Self::CatmullRom,
			FilterType::Gaussian => Self::Gaussian,
			FilterType::Lanczos3 => Self::Lanczos3,
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CropMode {
	Preserve,
	Fill,
	Exact,
}

impl Process for Resize {
	fn process(&self, image: DynamicImage) -> Result<DynamicImage, OperationError> {
		let (width, height) = image.dimensions();
		let width = PixelUnit::from(width);
		let height = PixelUnit::from(height);

		let image = match self.crop_mode {
			CropMode::Preserve => image.resize(
				self.width.as_pixel(width).pixels,
				self.height.as_pixel(height).pixels,
				self.filter.into(),
			),
			CropMode::Exact => image.resize_exact(
				self.width.as_pixel(width).pixels,
				self.height.as_pixel(height).pixels,
				self.filter.into(),
			),
			CropMode::Fill => image.resize_to_fill(
				self.width.as_pixel(width).pixels,
				self.height.as_pixel(height).pixels,
				self.filter.into(),
			),
		};

		Ok(image)
	}
}
