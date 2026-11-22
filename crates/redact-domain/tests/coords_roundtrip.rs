use redact_domain::coords::{
    render_css_to_user, user_to_render_css, PageSizePts, RenderSizeCss,
};

#[test]
fn css_user_roundtrip_within_two_points() {
    let page = PageSizePts {
        width: 612.0,
        height: 792.0,
    };
    let render = RenderSizeCss {
        width: 612.0,
        height: 792.0,
    };
    let rect = render_css_to_user(0, 100.0, 200.0, 300.0, 400.0, page, render).unwrap();
    let (x0, y0, x1, y1) = user_to_render_css(&rect, page, render);
    assert!((x0 - 100.0).abs() <= 2.0);
    assert!((y0 - 200.0).abs() <= 2.0);
    assert!((x1 - 300.0).abs() <= 2.0);
    assert!((y1 - 400.0).abs() <= 2.0);
}

#[test]
fn scaled_render_roundtrip() {
    let page = PageSizePts {
        width: 612.0,
        height: 792.0,
    };
    let render = RenderSizeCss {
        width: 1224.0,
        height: 1584.0,
    };
    let rect = render_css_to_user(1, 50.0, 80.0, 250.0, 180.0, page, render).unwrap();
    let (x0, y0, x1, y1) = user_to_render_css(&rect, page, render);
    assert!((x0 - 50.0).abs() <= 2.0);
    assert!((y0 - 80.0).abs() <= 2.0);
    assert!((x1 - 250.0).abs() <= 2.0);
    assert!((y1 - 180.0).abs() <= 2.0);
}
