use dioxus::prelude::*;
use dioxus::ssr::render_element;
use dioxus_icons::lucide::{Calendar1, Trash, Type};
use dioxus_vdom::AttributeValue;

fn render_icon(element: Element) -> String {
    render_element(element)
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(
        haystack.contains(needle),
        "expected rendered icon to contain {needle:?}\nrendered: {haystack}"
    );
}

fn assert_not_contains(haystack: &str, needle: &str) {
    assert!(
        !haystack.contains(needle),
        "expected rendered icon not to contain {needle:?}\nrendered: {haystack}"
    );
}

#[test]
fn default_svg_shape() {
    let html = render_icon(rsx! { Trash {} });

    assert!(html.starts_with("<svg "));
    assert_contains(&html, r#"xmlns="http://www.w3.org/2000/svg""#);
    assert_contains(&html, "width=24");
    assert_contains(&html, "height=24");
    assert_contains(&html, r#"viewBox="0 0 24 24""#);
    assert_contains(&html, r#"fill="none""#);
    assert_contains(&html, r#"stroke="currentColor""#);
    assert_contains(&html, r#"stroke-width="2""#);
    assert_contains(&html, r#"stroke-linecap="round""#);
    assert_contains(&html, r#"stroke-linejoin="round""#);
    assert_not_contains(&html, "class=");
    assert_contains(&html, r#"<path d="M19 6v14"#);
}

#[test]
fn props_map_to_svg_attributes() {
    let html = render_icon(rsx! {
        Trash {
            size: 32,
            stroke: "red",
            stroke_width: 3,
            stroke_linecap: "butt",
            stroke_linejoin: "bevel",
            class: "text-red-500",
        }
    });

    assert_contains(&html, "width=32");
    assert_contains(&html, "height=32");
    assert_contains(&html, r#"fill="none""#);
    assert_contains(&html, r#"stroke="red""#);
    assert_not_contains(&html, r#"fill="red""#);
    assert_contains(&html, "stroke-width=3");
    assert_contains(&html, r#"stroke-linecap="butt""#);
    assert_contains(&html, r#"stroke-linejoin="bevel""#);
    assert_contains(&html, r#"class="text-red-500""#);
}

#[test]
fn size_accepts_css_lengths() {
    let html = render_icon(rsx! {
        Trash {
            size: "2em",
        }
    });

    assert_contains(&html, r#"width="2em""#);
    assert_contains(&html, r#"height="2em""#);
}

#[test]
fn size_accepts_attribute_values() {
    let html = render_icon(rsx! {
        Trash {
            size: AttributeValue::Text("1.5rem".to_owned()),
        }
    });

    assert_contains(&html, r#"width="1.5rem""#);
    assert_contains(&html, r#"height="1.5rem""#);
}

#[test]
fn explicit_svg_attributes_override_defaults() {
    let html = render_icon(rsx! {
        Trash {
            width: "1em",
            height: "2em",
            view_box: "0 0 16 16",
            fill: "currentColor",
            stroke: "blue",
            stroke_width: 1.5,
            role: "img",
            "aria-label": "Delete",
            "data-testid": "trash-icon",
        }
    });

    assert_contains(&html, r#"width="1em""#);
    assert_contains(&html, r#"height="2em""#);
    assert_contains(&html, r#"viewBox="0 0 16 16""#);
    assert_contains(&html, r#"fill="currentColor""#);
    assert_contains(&html, r#"stroke="blue""#);
    assert_contains(&html, "stroke-width=1.5");
    assert_contains(&html, r#"role="img""#);
    assert_contains(&html, r#"aria-label="Delete""#);
    assert_contains(&html, r#"data-testid="trash-icon""#);
    assert_not_contains(&html, r#"width="24""#);
    assert_not_contains(&html, r#"height="24""#);
    assert_not_contains(&html, r#"viewBox="0 0 24 24""#);
    assert_not_contains(&html, r#"stroke-width="2""#);
}

#[test]
fn public_imports_cover_keyword_and_numeric_names() {
    assert_contains(&render_icon(rsx! { Type {} }), r#"viewBox="0 0 24 24""#);
    assert_contains(
        &render_icon(rsx! { Calendar1 {} }),
        r#"viewBox="0 0 24 24""#,
    );
}
