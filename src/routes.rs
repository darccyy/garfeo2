use ibex::prelude::*;

use crate::posts::Special;
use crate::views::post_transcript;

use super::posts::{PostList, PostRef};
use super::views::{list_item, post_title, use_base, STAR};
use super::URL_ROOT;

pub fn at_index(posts: &PostList) -> Document {
    view! { @use_base [
        "",
        view! {},
        None,
        posts,
    ] {
        ol ."post-list" [
            reversed=true,
            start=posts.first().index,
        ] {
            [:for post in posts {
                @list_item [&post]
            }]
        }
    }}
    .into()
}

pub fn at_favorites(posts: &PostList) -> Document {
    view! { @use_base [
        "Plej bonaj",
        view! { "Plej bonaj bildstrioj" },
        None,
        posts,
    ] {
        ol ."post-list" [
            reversed=true,
            start=posts.first().index,
        ] {
            [:for post in posts {
                [:if post.get().props.good {
                    @list_item [&post]
                }]
            }]
        }
    }}
    .into()
}

pub fn at_about(posts: &PostList) -> Document {
    view! { @use_base [
        "Informejo",
        view! { "Informejo" },
        None,
        posts,
    ] {

        h3 { "Kio estas Garfield-EO?" }
        p {
            "Mi tradukas bildstriojn de Garfildo en Esperanton."
            br/
            "Parto de" ~ i{"Mondo da Komiksoj"} "."
        }

        p {
            "Vidu kiel mi kreas kaj alŝutas tradukojn,"
            ~ a [href=url!("instrukcio")] { i{"ĉi tie"} } "."
        }

        h3 { "Ligiloj" }
        ul ."links" {
            li { a [href="https://github.com/darccyy/garfeo"]
                { strong { "Fonta Kodo kaj ĉiu bildstrio" }
                    ~ "- por ĉi tiu retejo (en la angla)" }
            }
            li { a [href="https://github.com/darccyy/garfeo/issues/new"]
                { strong { "Mi havas concernon!" }
                    ~ "- Informu min per GitHub" }
            }
            li { a [href="https://github.com/darccyy/everygarf"]
                { strong { "EveryGarf" }
                    ~ "- Elŝuti ĉiujn Garfildajn bildstriojn ĝisnune" }
            }
            li { a [href="https://mastodon.world/@garfieldeo"]
                { strong { "Mastodon @garfieldeo@mastodon.world" }
                    ~ "- Esperantaj Garfildaj bildstrioj" }
            }
            li { a [href="https://instagram.com/garfield.eo"]
                { strong { "Instagram @garfield.eo" }
                    ~ "- Esperantaj Garfildaj bildstrioj" }
            }
            li { a [href="https://instagram.com/mondodakomiksoj"]
                { strong { "Mondo da Komiksoj" }
                    ~ "- Grupo de tradukistoj" }
            }
            li { a [href=url!("rss.xml")]
                { strong { "RSS-fluo" }
                    ~ "- Aboni la RSS-fluon por novajn bildstriojn" }
            }
        }

        hr/
        br/
        img ."icon-image" [
            src=url!("static/icon.png"),
            alt="La vizaĝo de Garfildo",
            height=400,
        ]/
    }}
    .into()
}

pub fn at_instructions(posts: &PostList) -> Document {
    view! { @use_base [
        "Instrukcio",
        view! {"Instrukcio"},
        None,
        posts,
    ] {
        p { "Nenio estas ĉi tie..." }
        p { "Revenu baldaŭ..." }
    }}
    .into()
}

pub fn at_list(posts: &PostList) -> Document {
    view! { @use_base [
        "Alia listo",
        view!{},
        None,
        posts,
    ] {
        table ."graph" {
            [:for post in posts { [:where let post = post.get(); {
                    tr {
                        td { [:if post.props.good { [STAR] }] }
                        td { a [href=url!(post.index()), title=post.title] {
                            [:if post.is_sunday
                                { b  { [&post.index()] } }
                                else { [&post.index()] }
                            ]
                        }}
                        td { [:for _ in 0..post.version { span { "🟥" } }] }
                    }
                }]
            }]
        }

    }}
    .into()
}

pub fn at_post(post_ref: PostRef) -> Document {
    let post = post_ref.get();

    view! { @use_base [
        &format!("{} [{}]", post.title, post.index),
        post_title(&post_ref, false),
        Some(&format!("static/posts/{}/esperanto.png", post.index)),
        post_ref.list(),
    ] {
        p ."details" {
            span ."navigate prev" {
                [:if let Some(prev) = &post_ref.prev() {
                    a [href=url!(&prev.index())] { &laquo }
                }]
            }
            ~
            span ."text" {
                "[" span #"index" { [&post.index()] } "]"
                ~
                a [
                    href=format!("https://gocomics.com/garfield/{}", post.date.replace('-', "/")),
                    title="Spekti je GoComics.com",
                ] {
                    b #"date" { [&post.date] }
                }
            }
            ~
            span ."navigate next" {
                [:if let Some(next) = &post_ref.next() {
                    a [href=url!(&next.index())] { &raquo }
                }]
            }
        }

        div {
            img #"image-eo" ."comic" [
                alt="Esperanto bildstrio",
                src=url!(format!("static/posts/{}/esperanto.png", &post.index)),
                height=400,
            ]/
            img #"image-en" ."comic" [
                alt="Angla bildstrio",
                src=url!(format!("static/posts/{}/english.png", &post.index)),
                height=400,
            ]/
        }

        p ."small gray" {
            [:if post.version > 0 {
                b { "Revizio:" }
                ~ [post.version]
            }]
            ~
            [:if post.is_old {
                "(olda)"
            }]
        }

        [:if !post.errata.items.is_empty() { div ."errata" {
            h2 { "Eraroj:" }
            ol {
                [:for (old, new) in &post.errata.items { li {
                    b ."old" { [old] }
                    ~ &rarr ~
                    b ."new" { [new] }
                } }]
            }
        } }]

        div ."navigate" {
            [:if let Some(prev) = &post_ref.next_ref() {
                div ."prev" {
                    a [href=url!(&prev.get().index())] {
                        strong { "Antaŭa:" } ~
                        @post_title [prev, true]
                    }
                }
            }]
            [:if let Some(next) = &post_ref.prev_ref() {
                div ."next" {
                    a [href=url!(&next.get().index())] {
                        strong { "Sekva:" } ~
                        @post_title [&next, true]
                    }
                }
            }]
        }

        div ."special" {
            [match &post.special {
                Some(Special::Christmas) => view! { "Feliĉan Kristnaskon!" },
                Some(Special::Halloween) => view! { "Feliĉan Halovenon!" },
                _ => view! {},
            }]
        }

        [:if let Some(transcript) = &post.transcript {
            @post_transcript [transcript]
        }]

        hr/

        div ."caption" {
            HEAD { script { [include_str!("js/copy.js")] } }
            pre #"caption" [onclick="copy(this)"] {
                [&post.title] ~ "💚" "&#10;&#10;"
                 "#esperanto #garfield #mondodakomiksoj"
                ~ "[" [&post.index()] "]"
            }
        }

        a ."source" [
            href=format!("https://github.com/darccyy/garfeo/tree/master/static/posts/{}", post.index),
        ] {
            "Vidu fonton"
        }
    }}
    .into()
}

pub fn at_404(posts: &PostList) -> Document {
    view! { @use_base [
        "404",
        view! { "Paĝo ne trovita!" },
        None,
        &posts,
    ] {
        center {
            "404 - Not found"
        }
    }}
    .into()
}
