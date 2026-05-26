use dioxus::core::{
    Attribute, AttributeValue, DynamicNode, Element, Template, TemplateAttribute, TemplateNode,
    VNode,
};

use crate::IconProps;

type AttributeDescription = (&'static str, Option<&'static str>, bool);
const SVG_NAMESPACE: Option<&'static str> = Some("http://www.w3.org/2000/svg");
const EMPTY_CHILDREN: &[TemplateNode] = &[];
const ICON_ATTR_PATHS: &[&[u8]] = &[&[0u8]];
static DEFAULT_SVG_ATTRS: [TemplateAttribute; 1] = svg_attrs();

const XMLNS: AttributeDescription = ("xmlns", None, false);
const WIDTH: AttributeDescription = ("width", None, false);
const HEIGHT: AttributeDescription = ("height", None, false);
const VIEW_BOX: AttributeDescription = ("viewBox", None, false);
const FILL: AttributeDescription = ("fill", None, false);
const STROKE: AttributeDescription = ("stroke", None, false);
const STROKE_WIDTH: AttributeDescription = ("stroke-width", None, false);
const STROKE_LINECAP: AttributeDescription = ("stroke-linecap", None, false);
const STROKE_LINEJOIN: AttributeDescription = ("stroke-linejoin", None, false);

#[inline]
pub(crate) const fn icon_template(roots: &'static [TemplateNode]) -> Template {
    Template::new(roots, &[], ICON_ATTR_PATHS)
}

#[inline]
pub(crate) const fn svg_attrs() -> [TemplateAttribute; 1] {
    [dynamic_attr(0)]
}

#[inline]
pub(crate) const fn svg(children: &'static [TemplateNode]) -> TemplateNode {
    svg_with_attrs(&DEFAULT_SVG_ATTRS, children)
}

#[inline]
pub(crate) const fn svg_with_attrs(
    attrs: &'static [TemplateAttribute],
    children: &'static [TemplateNode],
) -> TemplateNode {
    TemplateNode::Element {
        tag: "svg",
        namespace: SVG_NAMESPACE,
        attrs,
        children,
    }
}

#[inline]
pub(crate) const fn path(attrs: &'static [TemplateAttribute]) -> TemplateNode {
    child("path", attrs)
}

#[inline]
pub(crate) const fn circle(attrs: &'static [TemplateAttribute]) -> TemplateNode {
    child("circle", attrs)
}

#[inline]
pub(crate) const fn rect(attrs: &'static [TemplateAttribute]) -> TemplateNode {
    child("rect", attrs)
}

#[inline]
pub(crate) const fn line(attrs: &'static [TemplateAttribute]) -> TemplateNode {
    child("line", attrs)
}

#[inline]
pub(crate) const fn polyline(attrs: &'static [TemplateAttribute]) -> TemplateNode {
    child("polyline", attrs)
}

#[inline]
pub(crate) const fn polygon(attrs: &'static [TemplateAttribute]) -> TemplateNode {
    child("polygon", attrs)
}

#[inline]
pub(crate) const fn ellipse(attrs: &'static [TemplateAttribute]) -> TemplateNode {
    child("ellipse", attrs)
}

#[inline]
const fn child(tag: &'static str, attrs: &'static [TemplateAttribute]) -> TemplateNode {
    TemplateNode::Element {
        tag,
        namespace: SVG_NAMESPACE,
        attrs,
        children: EMPTY_CHILDREN,
    }
}

#[inline]
pub(crate) const fn attr(name: &'static str, value: &'static str) -> TemplateAttribute {
    TemplateAttribute::Static {
        name,
        value,
        namespace: None,
    }
}

#[inline]
const fn dynamic_attr(id: usize) -> TemplateAttribute {
    TemplateAttribute::Dynamic { id }
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

    let dynamic_attributes = Box::new([root_attributes.into_boxed_slice()]);
    let dynamic_nodes: Box<[DynamicNode]> = Box::new([]);
    Ok(VNode::new(
        None,
        template,
        dynamic_nodes,
        dynamic_attributes,
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
