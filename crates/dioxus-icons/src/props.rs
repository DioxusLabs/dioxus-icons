use dioxus::prelude::*;
use dioxus_vdom::{AttributeValue, IntoAttributeValue};

/// Convenience size for SVG width and height.
#[derive(Clone, PartialEq)]
pub struct IconSize(AttributeValue);

impl IconSize {
    pub(crate) fn into_value(self) -> AttributeValue {
        self.0
    }
}

impl Default for IconSize {
    fn default() -> Self {
        Self(AttributeValue::Int(24))
    }
}

impl<T> From<T> for IconSize
where
    T: IntoAttributeValue,
{
    fn from(value: T) -> Self {
        Self(value.into_value())
    }
}

/// Properties shared by every Lucide icon component.
#[derive(Clone, PartialEq, Props)]
pub struct IconProps {
    /// Convenience size for SVG width and height when those attributes are not set directly.
    #[props(into, default = IconSize::default())]
    pub size: IconSize,
    /// Attributes passed to the root SVG.
    #[props(extends = SvgAttributes)]
    pub attributes: Vec<Attribute>,
}
