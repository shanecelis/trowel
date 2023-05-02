use bitflags::bitflags;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Dimensions,
    prelude::{Point, Size},
    primitives::{PointsIter, Rectangle},
    Pixel,
};

bitflags! {
    pub struct Axes: u8 {
        const X = 0b00000001;
        const Y = 0b00000010;
    }
}

impl Axes {
    pub fn flip(&self, q: Point, size: Size) -> Point {
        let mut p = q;
        if self.contains(Axes::X) {
            p.x = size.width as i32 - p.x - 1;
        }

        if self.contains(Axes::Y) {
            p.y = size.height as i32 - p.y - 1;
        }
        p
    }
}

pub trait DrawTargetExt2: DrawTarget + Sized {
    /// Creates a translated draw target based on this draw target.
    ///
    /// All drawing operations are translated by `offset` pixels, before being passed to the parent
    /// draw target.
    ///
    /// # Examples
    ///
    /// ```
    /// use embedded_graphics::{
    ///     mock_display::MockDisplay,
    ///     mono_font::{ascii::FONT_6X9, MonoTextStyle},
    ///     pixelcolor::BinaryColor,
    ///     prelude::*,
    ///     text::Text,
    /// };
    ///
    /// let mut display = MockDisplay::new();
    /// let mut translated_display = display.translated(Point::new(5, 10));
    ///
    /// let style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
    ///
    /// // Draws text at position (5, 10) in the display coordinate system
    /// Text::new("Text", Point::zero(), style).draw(&mut translated_display)?;
    /// #
    /// # let mut expected = MockDisplay::new();
    /// #
    /// # Text::new("Text", Point::new(5, 10), style).draw(&mut expected)?;
    /// #
    /// # display.assert_eq(&expected);
    /// #
    /// # Ok::<(), core::convert::Infallible>(())
    /// ```
    fn flipped(&mut self, axes: Axes) -> Flipped<'_, Self>;
}

impl<T> DrawTargetExt2 for T
where
    T: DrawTarget,
{
    fn flipped(&mut self, axes: Axes) -> Flipped<'_, Self> {
        Flipped::new(self, axes)
    }
}

/// Flipped draw target.
///
/// Created by calling [`translated`] on any [`DrawTarget`].
/// See the [`translated`] method documentation for more.
///
/// [`translated`]: crate::draw_target::DrawTargetExt::translated
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct Flipped<'a, T>
where
    T: DrawTarget,
{
    parent: &'a mut T,

    axes: Axes,
}

impl<'a, T> Flipped<'a, T>
where
    T: DrawTarget,
{
    pub(super) fn new(parent: &'a mut T, axes: Axes) -> Self {
        Self { parent, axes }
    }
}

impl<T> DrawTarget for Flipped<'_, T>
where
    T: DrawTarget,
{
    type Color = T::Color;
    type Error = T::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let rect = self.bounding_box();
        self.parent.draw_iter(pixels.into_iter().map(|Pixel(q, c)| {
            let p = self.axes.flip(q, rect.size);
            Pixel(p, c)
        }))
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        self.draw_iter(
            area.points()
                .zip(colors.into_iter())
                .map(|(p, c)| Pixel(p, c)),
        )
        // area.size.width * area.size.height
        // let area = area.translate(self.offset);
        // self.parent.fill_contiguous(&area, colors)
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        // let area = area.translate(self.offset);
        self.parent.fill_solid(&area, color)
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.parent.clear(color)
    }
}

impl<T> Dimensions for Flipped<'_, T>
where
    T: DrawTarget,
{
    fn bounding_box(&self) -> Rectangle {
        self.parent.bounding_box()
    }
}
