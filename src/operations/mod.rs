mod crop;
mod resize;

use image::DynamicImage;
use serde::{Deserialize, Serialize};

use crate::{OperationError, Process};

pub use crop::Crop;
pub use resize::Resize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Grayscale {}

impl Process for Grayscale {
	fn process(&self, image: DynamicImage) -> Result<DynamicImage, OperationError> {
		Ok(image.grayscale())
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Blur {
	pub sigma: f32,
}

impl Process for Blur {
	fn process(&self, image: DynamicImage) -> Result<DynamicImage, OperationError> {
		Ok(image.blur(self.sigma))
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdjustBrightness {
	Darken(u16),
	Brighten(u16),
}

impl Process for AdjustBrightness {
	fn process(&self, image: DynamicImage) -> Result<DynamicImage, OperationError> {
		let value = match self {
			Self::Darken(value) => -(*value as i32),
			Self::Brighten(value) => *value as i32,
		};

		Ok(image.brighten(value))
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Invert;

impl Process for Invert {
	fn process(&self, mut image: DynamicImage) -> Result<DynamicImage, OperationError> {
		image.invert();
		Ok(image)
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Unsharpen {
	pub sigma: f32,
	pub threshold: i32,
}

impl Process for Unsharpen {
	fn process(&self, image: DynamicImage) -> Result<DynamicImage, OperationError> {
		Ok(image.unsharpen(self.sigma, self.threshold))
	}
}

// TODO next filter3x3
// TODO - include predefined kernels for sharpening and shit?
// See: https://programmathically.com/understanding-convolutional-filters-and-convolutional-kernels/
// Or use guassian and box kernels for blur, and maybe the sharpen filter for `Sharpen`
