use dioxus::core::{Attribute, AttributeValue, DynamicValues, Element, Template, VNode};
use dioxus_core_template::{TemplateRawTree, TemplateStorage};

use crate::IconProps;

type AttributeDescription = (&'static str, Option<&'static str>, bool);
const SVG_NAMESPACE: Option<&'static str> = Some("http://www.w3.org/2000/svg");

const XMLNS: AttributeDescription = ("xmlns", None, false);
const WIDTH: AttributeDescription = ("width", None, false);
const HEIGHT: AttributeDescription = ("height", None, false);
const VIEW_BOX: AttributeDescription = ("viewBox", None, false);
const FILL: AttributeDescription = ("fill", None, false);
const STROKE: AttributeDescription = ("stroke", None, false);
const STROKE_WIDTH: AttributeDescription = ("stroke-width", None, false);
const STROKE_LINECAP: AttributeDescription = ("stroke-linecap", None, false);
const STROKE_LINEJOIN: AttributeDescription = ("stroke-linejoin", None, false);

/// Storage capacity for a single icon's template. Some icons (e.g. those with many dashed
/// path segments) need more than the default 128 ops/strings, so every icon gets a generous
/// fixed allowance.
pub(crate) type IconTemplateStorage = TemplateStorage<256, 256, 32>;

/// Build a static [`Template`] from a [`TemplateRawTree`].
///
/// The tree must already be fully assembled (with all nested `&TemplateRawTree` references
/// resolved) by the caller, since a `TemplateStorage` needs a `'static` place to live in and
/// can't be produced from inside a helper function.
#[inline]
pub(crate) const fn icon_template(storage: &'static IconTemplateStorage) -> Template {
    storage.as_template()
}

#[inline]
pub(crate) const fn icon_storage(tree: &'static TemplateRawTree) -> IconTemplateStorage {
    IconTemplateStorage::build_from_tree(tree)
}

#[inline]
pub(crate) const fn svg(children: &'static TemplateRawTree) -> TemplateRawTree {
    TemplateRawTree::Element {
        tag: "svg",
        namespace: SVG_NAMESPACE,
        attrs: &TemplateRawTree::DynamicAttr,
        children,
    }
}

#[inline]
pub(crate) const fn path(attrs: &'static TemplateRawTree) -> TemplateRawTree {
    child("path", attrs)
}

#[inline]
pub(crate) const fn circle(attrs: &'static TemplateRawTree) -> TemplateRawTree {
    child("circle", attrs)
}

#[inline]
pub(crate) const fn rect(attrs: &'static TemplateRawTree) -> TemplateRawTree {
    child("rect", attrs)
}

#[inline]
pub(crate) const fn line(attrs: &'static TemplateRawTree) -> TemplateRawTree {
    child("line", attrs)
}

#[inline]
pub(crate) const fn polyline(attrs: &'static TemplateRawTree) -> TemplateRawTree {
    child("polyline", attrs)
}

#[inline]
pub(crate) const fn polygon(attrs: &'static TemplateRawTree) -> TemplateRawTree {
    child("polygon", attrs)
}

#[inline]
pub(crate) const fn ellipse(attrs: &'static TemplateRawTree) -> TemplateRawTree {
    child("ellipse", attrs)
}

#[inline]
const fn child(tag: &'static str, attrs: &'static TemplateRawTree) -> TemplateRawTree {
    TemplateRawTree::Element {
        tag,
        namespace: SVG_NAMESPACE,
        attrs,
        children: &TemplateRawTree::Empty,
    }
}

#[inline]
pub(crate) const fn attr(name: &'static str, value: &'static str) -> TemplateRawTree {
    TemplateRawTree::StaticAttr {
        name,
        value,
        namespace: None,
    }
}

#[inline]
fn icon_attr(
    (name, namespace, volatile): AttributeDescription,
    value: AttributeValue,
) -> Attribute {
    Attribute {
        name,
        value,
        namespace,
        volatile,
    }
}

#[inline]
pub(crate) fn icon_element(
    template: Template,
    view_box: &'static str,
    props: IconProps,
) -> Element {
    let IconProps { size, attributes } = props;
    let size = size.into_value();

    let mut root_attributes = Vec::with_capacity(attributes.len() + 9);
    push_default_attr(
        &mut root_attributes,
        &attributes,
        XMLNS,
        "http://www.w3.org/2000/svg",
    );
    push_default_attr_value(&mut root_attributes, &attributes, WIDTH, size.clone());
    push_default_attr_value(&mut root_attributes, &attributes, HEIGHT, size);
    push_default_attr(&mut root_attributes, &attributes, VIEW_BOX, view_box);
    push_default_attr(&mut root_attributes, &attributes, FILL, "none");
    push_default_attr(&mut root_attributes, &attributes, STROKE, "currentColor");
    push_default_attr(&mut root_attributes, &attributes, STROKE_WIDTH, "2");
    push_default_attr(&mut root_attributes, &attributes, STROKE_LINECAP, "round");
    push_default_attr(&mut root_attributes, &attributes, STROKE_LINEJOIN, "round");
    root_attributes.extend(attributes);

    Ok(VNode::new(
        template,
        DynamicValues::from_parts(
            None,
            Box::new([]),
            Box::new([root_attributes.into_boxed_slice()]),
        ),
    ))
}

#[inline]
fn push_default_attr(
    output: &mut Vec<Attribute>,
    attributes: &[Attribute],
    description: AttributeDescription,
    value: &str,
) {
    if !has_attr(attributes, description) {
        push_default_attr_value(
            output,
            attributes,
            description,
            AttributeValue::Text(value.to_owned()),
        );
    }
}

#[inline]
fn push_default_attr_value(
    output: &mut Vec<Attribute>,
    attributes: &[Attribute],
    description: AttributeDescription,
    value: AttributeValue,
) {
    if !has_attr(attributes, description) {
        output.push(icon_attr(description, value));
    }
}

#[inline]
fn has_attr(attributes: &[Attribute], (name, namespace, _): AttributeDescription) -> bool {
    attributes
        .iter()
        .any(|attribute| attribute.name == name && attribute.namespace == namespace)
}
