use crate::{Coordinate, OperationError, PixelUnit, Process};

use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Crop {
	pub from: Coordinate,
	pub to: CropOrigin,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CropOrigin {
	Minimum(Coordinate),
	Maximum(Coordinate),
	CropStart(Coordinate),
}

impl CropOrigin {
	fn as_pixel_coordinate(
		&self,
		x: PixelUnit,
		y: PixelUnit,
		width: PixelUnit,
		height: PixelUnit,
	) -> (PixelUnit, PixelUnit) {
		match self {
			Self::Minimum(coordinate) => {
				(coordinate.x.as_pixel(width), coordinate.y.as_pixel(height))
			}
			Self::Maximum(coordinate) => {
				let x = width - coordinate.x.as_pixel(width);
				let y = height - coordinate.y.as_pixel(height);
				(x, y)
			}
			Self::CropStart(coordinate) => {
				let x = x + coordinate.x.as_pixel(width);
				let y = y + coordinate.y.as_pixel(height);
				(x, y)
			}
		}
	}
}

impl Process for Crop {
	fn process(&self, image: DynamicImage) -> Result<DynamicImage, OperationError> {
		let (width, height) = image.dimensions();
		let width = PixelUnit::from(width);
		let height = PixelUnit::from(height);

		let left = self.from.x.as_pixel(height);
		let top = self.from.y.as_pixel(width);

		let (right, bottom) = self.to.as_pixel_coordinate(left, top, width, height);

		if bottom < top {
			return Err(OperationError::new(format!(
				"Bottom cannot be less than top for crop operation {self:?}"
			)));
		}

		if right < left {
			return Err(OperationError::new(format!(
				"Right cannot be less than left for crop operation {self:?}"
			)));
		}

		Ok(image.crop_imm(
			left.into(),
			top.into(),
			(right - left).into(),
			(bottom - top).into(),
		))
	}
}

#[cfg(test)]
mod tests {
	use crate::operations::crop::CropOrigin;
	use crate::{Coordinate, Unit};

	const CANVAS_WIDTH: u32 = 100;
	const CANVAS_HEIGHT: u32 = 100;

	#[test]
	fn crop_origin_as_pixel_coordinate_minimum_pixel() {
		let crop_origin = CropOrigin::Minimum(Coordinate {
			x: Unit::Pixel(10.into()),
			y: Unit::Pixel(10.into()),
		});

		assert_eq!(
			(10.into(), 10.into()),
			crop_origin.as_pixel_coordinate(
				5.into(),
				5.into(),
				CANVAS_WIDTH.into(),
				CANVAS_HEIGHT.into(),
			)
		);
	}

	#[test]
	fn crop_origin_as_pixel_coordinate_minimum_percent() {
		let crop_origin = CropOrigin::Minimum(Coordinate {
			x: Unit::Percentage(0.8.try_into().unwrap()),
			y: Unit::Percentage(0.8.try_into().unwrap()),
		});

		assert_eq!(
			(80.into(), 80.into()),
			crop_origin.as_pixel_coordinate(
				5.into(),
				5.into(),
				CANVAS_WIDTH.into(),
				CANVAS_HEIGHT.into(),
			)
		);
	}

	#[test]
	fn crop_origin_as_pixel_coordinate_minimum_mixed() {
		let crop_origin = CropOrigin::Minimum(Coordinate {
			x: Unit::Percentage(0.8.try_into().unwrap()),
			y: Unit::Pixel(50.into()),
		});

		assert_eq!(
			(80.into(), 50.into()),
			crop_origin.as_pixel_coordinate(
				5.into(),
				5.into(),
				CANVAS_WIDTH.into(),
				CANVAS_HEIGHT.into(),
			)
		);
	}

	#[test]
	fn crop_origin_as_pixel_coordinate_maximum_pixel() {
		let crop_origin = CropOrigin::Maximum(Coordinate {
			x: Unit::Pixel(10.into()),
			y: Unit::Pixel(10.into()),
		});

		assert_eq!(
			(90.into(), 90.into()),
			crop_origin.as_pixel_coordinate(
				5.into(),
				5.into(),
				CANVAS_WIDTH.into(),
				CANVAS_HEIGHT.into(),
			)
		);
	}

	#[test]
	fn crop_origin_as_pixel_coordinate_maximum_percent() {
		let crop_origin = CropOrigin::Maximum(Coordinate {
			x: Unit::Percentage(0.2.try_into().unwrap()),
			y: Unit::Percentage(0.2.try_into().unwrap()),
		});

		assert_eq!(
			(80.into(), 80.into()),
			crop_origin.as_pixel_coordinate(
				5.into(),
				5.into(),
				CANVAS_WIDTH.into(),
				CANVAS_HEIGHT.into(),
			)
		);
	}

	#[test]
	fn crop_origin_as_pixel_coordinate_maximum_mixed() {
		let crop_origin = CropOrigin::Maximum(Coordinate {
			x: Unit::Percentage(0.2.try_into().unwrap()),
			y: Unit::Pixel(50.into()),
		});

		assert_eq!(
			(80.into(), 50.into()),
			crop_origin.as_pixel_coordinate(
				5.into(),
				5.into(),
				CANVAS_WIDTH.into(),
				CANVAS_HEIGHT.into(),
			)
		);
	}

	#[test]
	fn crop_origin_as_pixel_coordinate_cropstart_pixel() {
		let crop_origin = CropOrigin::CropStart(Coordinate {
			x: Unit::Pixel(10.into()),
			y: Unit::Pixel(10.into()),
		});

		assert_eq!(
			(15.into(), 15.into()),
			crop_origin.as_pixel_coordinate(
				5.into(),
				5.into(),
				CANVAS_WIDTH.into(),
				CANVAS_HEIGHT.into(),
			)
		);
	}

	#[test]
	fn crop_origin_as_pixel_coordinate_cropstart_percent() {
		let crop_origin = CropOrigin::CropStart(Coordinate {
			x: Unit::Percentage(0.2.try_into().unwrap()),
			y: Unit::Percentage(0.2.try_into().unwrap()),
		});

		assert_eq!(
			(25.into(), 25.into()),
			crop_origin.as_pixel_coordinate(
				5.into(),
				5.into(),
				CANVAS_WIDTH.into(),
				CANVAS_HEIGHT.into(),
			)
		);
	}

	#[test]
	fn crop_origin_as_pixel_coordinate_cropstart_mixed() {
		let crop_origin = CropOrigin::CropStart(Coordinate {
			x: Unit::Percentage(0.2.try_into().unwrap()),
			y: Unit::Pixel(50.into()),
		});

		assert_eq!(
			(25.into(), 55.into()),
			crop_origin.as_pixel_coordinate(
				5.into(),
				5.into(),
				CANVAS_WIDTH.into(),
				CANVAS_HEIGHT.into(),
			)
		);
	}
}
