// Copyright (c) 2017 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

#[macro_use]
extern crate rouille;

use rouille::Request;
use rouille::Response;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::Mutex;

/// Runs the HTTP server forever on the given address.
pub fn start<A>(addr: A)
where
    A: ToSocketAddrs,
{
    rouille::start_server(addr, move |request| {
        rouille::content_encoding::apply(
            request,
            rouille::log(request, io::stdout(), || {
                {
                    let mut r = rouille::match_assets(request, "./static");
                    if r.is_success() {
                        r.headers.push((
                            "Cache-Control".into(),
                            format!("max-age={}", 2 * 60 * 60).into(),
                        ));
                        return r;
                    }
                }

                routes(request)
            }),
        )
    });
}

// Handles all the non-static routes.
fn routes(request: &Request) -> Response {
    router!(request,
        (GET) (/) => {
            main_template(include_str!("../content/home.html"))
        },
        (GET) (/donate) => {
            main_template(include_str!("../content/donate.html"))
        },

        (GET) (/guide/introduction) => {
            guide_template_markdown(include_str!("../content/guide/introduction/introduction.md"))
        },
        (GET) (/guide/initialization) => {
            guide_template_markdown(include_str!("../content/guide/initialization/initialization.md"))
        },
        (GET) (/guide/device-creation) => {
            guide_template_markdown(include_str!("../content/guide/initialization/device-creation.md"))
        },

        (GET) (/guide/buffer-creation) => {
            guide_template_markdown(include_str!("../content/guide/buffer_creation/buffer_creation.md"))
        },
        (GET) (/guide/example-operation) => {
            guide_template_markdown(include_str!("../content/guide/buffer_creation/example_operation.md"))
        },

        (GET) (/guide/compute-intro) => {
            guide_template_markdown(include_str!("../content/guide/compute_pipeline/compute_intro.md"))
        },
        (GET) (/guide/compute-pipeline) => {
            guide_template_markdown(include_str!("../content/guide/compute_pipeline/compute_pipeline.md"))
        },
        (GET) (/guide/descriptor-sets) => {
            guide_template_markdown(include_str!("../content/guide/compute_pipeline/descriptor_sets.md"))
        },
        (GET) (/guide/dispatch) => {
            guide_template_markdown(include_str!("../content/guide/compute_pipeline/dispatch.md"))
        },

        (GET) (/guide/image-creation) => {
            guide_template_markdown(include_str!("../content/guide/images/image_creation.md"))
        },
        (GET) (/guide/image-clear) => {
            guide_template_markdown(include_str!("../content/guide/images/image_clear.md"))
        },
        (GET) (/guide/image-export) => {
            guide_template_markdown(include_str!("../content/guide/images/image_export.md"))
        },
        (GET) (/guide/mandelbrot) => {
            guide_template_markdown(include_str!("../content/guide/images/mandelbrot.md"))
        },

        (GET) (/guide/what-graphics-pipeline) => {
            guide_template_markdown(include_str!("../content/guide/graphics_pipeline/introduction.md"))
        },
        (GET) (/guide/vertex-input) => {
            guide_template_markdown(include_str!("../content/guide/graphics_pipeline/vertex_shader.md"))
        },
        (GET) (/guide/fragment-shader) => {
            guide_template_markdown(include_str!("../content/guide/graphics_pipeline/fragment_shader.md"))
        },
        (GET) (/guide/render-pass-framebuffer) => {
            guide_template_markdown({
                include_str!("../content/guide/graphics_pipeline/render_pass_framebuffer.md")
            })
        },
        (GET) (/guide/graphics-pipeline-creation) => {
            guide_template_markdown({
                include_str!("../content/guide/graphics_pipeline/pipeline_creation.md")
            })
        },

        // todo: redirect to the other url
        (GET) (/guide/windowing) => {
            guide_template_markdown({
                include_str!("../content/guide/windowing/introduction.md")
            })
        },
        (GET) (/guide/windowing/introduction) => {
            guide_template_markdown({
                include_str!("../content/guide/windowing/introduction.md")
            })
        },
        (GET) (/guide/windowing/swapchain-creation) => {
            guide_template_markdown({
                include_str!("../content/guide/windowing/swapchain_creation.md")
            })
        },
        (GET) (/guide/windowing/other-initialization) => {
            guide_template_markdown({
                include_str!("../content/guide/windowing/other_initialization.md")
            })
        },
        (GET) (/guide/windowing/event-handling) => {
            guide_template_markdown(include_str!("../content/guide/windowing/event_handling.md"))
        },

        (GET) (/guide/memory) => {
            guide_template_markdown(include_str!("../content/guide/wip/memory.md"))
        },
        _ => {
            main_template(include_str!("../content/404.html"))
                .with_status_code(404)
        }
    )
}

// `body` is expected to be HTML code. Puts `body` inside of the main template and builds a
// `Response` that contains the whole.
fn main_template<S>(body: S) -> Response
where
    S: Into<String>,
{
    lazy_static::lazy_static! {
        static ref MAIN_TEMPLATE: mustache::Template = {
            mustache::compile_str(&include_str!("../content/template_main.html")).unwrap()
        };

        static ref CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    }

    let body = body.into();

    let mut compil_cache = CACHE.lock().unwrap();
    let html = match compil_cache.entry(body) {
        Entry::Occupied(e) => e.into_mut(),
        Entry::Vacant(e) => {
            let data = mustache::MapBuilder::new()
                .insert_str("body", e.key().as_str())
                .build();

            let mut out = Vec::new();
            MAIN_TEMPLATE.render_data(&mut out, &data).unwrap();
            e.insert(String::from_utf8(out).unwrap())
        }
    };

    Response::html(html.clone())
}

// `body` is expected to be HTML code. Puts `body` inside of the guide template and builds a
// `Response` that contains the whole.
fn guide_template<S>(body: S) -> Response
where
    S: Into<String>,
{
    lazy_static::lazy_static! {
        static ref GUIDE_TEMPLATE: mustache::Template = {
            mustache::compile_str(&include_str!("../content/guide/template.html")).unwrap()
        };

        static ref CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    }

    let body = body.into();

    let mut compil_cache = CACHE.lock().unwrap();
    let html = match compil_cache.entry(body) {
        Entry::Occupied(e) => e.into_mut(),
        Entry::Vacant(e) => {
            let data = mustache::MapBuilder::new()
                .insert_str("body", e.key().as_str())
                .build();

            let mut out = Vec::new();
            GUIDE_TEMPLATE.render_data(&mut out, &data).unwrap();
            e.insert(String::from_utf8(out).unwrap())
        }
    };

    main_template(html.clone())
}

// `body` is expected to be markdown. Turns it into HTML and calls `guide_template`.
fn guide_template_markdown<S>(body: S) -> Response
where
    S: Into<String>,
{
    lazy_static::lazy_static! {
        static ref CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    }

    let body = body.into();

    let mut compil_cache = CACHE.lock().unwrap();
    let html = match compil_cache.entry(body) {
        Entry::Occupied(e) => e.into_mut(),
        Entry::Vacant(e) => {
            let mut html = String::new();
            pulldown_cmark::html::push_html(&mut html, pulldown_cmark::Parser::new(e.key()));
            e.insert(html)
        }
    };

    guide_template(html.clone())
}
