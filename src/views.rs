use ibex::prelude::*;
use ibex::ssg;

use super::posts::PostList;
use super::URL_ROOT;

pub fn use_base(
    title: &str,
    header: View,
    image: Option<&str>,
    posts: &PostList,
    children: View,
) -> View {
    let mut full_title = "Garfildo Esperanta".to_string();
    if !title.is_empty() {
        full_title += " - ";
        full_title += title
    };

    view! {
        HEAD {
            @use_meta [ibex::meta! {
                url: url!(),
                title: &full_title,
                desc: "Legu 500+ bildstrioj de Garfildo, tradukitaj en Esperanton!",
                image: url!(image.unwrap_or("static/icon.png")),
                author: "darcy",
                color: "#ffb24e",
                large_image: true,
            }]

            title { [full_title] }
            link [rel="shortcut icon", href=url!("static/icon.png")]/
            link [rel="stylesheet",    href=url!("css/base.css")]/
            @ssg::use_autoreload []
        }

        div ."header" {
            h1 ."title" {
                a [href=url!()] {
                    "Garfildo Esperanta"
                }
            }

            h2 ."actions" {
                // HEAD { script { [include_str!("js/random.js")] } }
                a #"random" [title="Klaku por iri al iun bildstrio"] {
                    i { "Arbitra" }
                    span ."icon" { "⚄" }
                }

                span ."fallback-divider" { ~ } // fallback for css
                a [href=url!("informejo")] {
                    i { "Informejo" }
                    span ."icon smaller" { "🛈 " }
                }

                span ."fallback-divider" { ~ }
                a [href=url!("plej-bonaj")] {
                    i { "Plej Bonaj" }
                    span ."icon" { "★" }
                }
            }

            [:where
                let first = &posts.first().index;
                let last = &posts.last().index;
             {
                script { [format!("set_random_url('{}', '{}', '{}')", url!(), first, last)] }
            }]
        }
        hr/

        article ."manual-width" {
            [:if !header.is_empty() {
                h2 { [header] }
            }]
            [children]
        }

        footer {
            a [href="https://darccyy.github.io"] {
                "kreita de darcio"
            }
        }
    }
}
