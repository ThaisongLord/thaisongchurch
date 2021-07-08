use crate::style::{px, DynamicCSSProps, StaticCSSProps};
use crate::*;

#[derive(Default)]
pub struct Padding<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Padding<'a> {
    pub fn all(self, padding: u32) -> Self {
        self.x(padding).y(padding)
    }

    pub fn x(self, x: u32) -> Self {
        self.left(x).right(x)
    }

    pub fn y(self, y: u32) -> Self {
        self.top(y).bottom(y)
    }

    pub fn top(mut self, top: u32) -> Self {
        self.static_css_props.insert("padding-top", px(top));
        self
    }

    pub fn right(mut self, right: u32) -> Self {
        self.static_css_props.insert("padding-right", px(right));
        self
    }

    pub fn bottom(mut self, bottom: u32) -> Self {
        self.static_css_props.insert("padding-bottom", px(bottom));
        self
    }

    pub fn left(mut self, left: u32) -> Self {
        self.static_css_props.insert("padding-left", px(left));
        self
    }
}

impl<'a> Style<'a> for Padding<'a> {
    fn into_css_props(self) -> (StaticCSSProps<'a>, DynamicCSSProps) {
        (self.static_css_props, self.dynamic_css_props)
    }
}