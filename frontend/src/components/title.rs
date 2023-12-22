use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
pub struct TitleData {
    pub title: String,
    pub image: Option<String>,
}

impl TitleData {
    fn new(title: &str, image: Option<&str>) -> TitleData {
        TitleData {
            title: String::from(title),
            image: match image {
                Some(image) => Some(String::from(image)),
                None => None,
            },
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    pub title: TitleData,
    pub second: Option<TitleData>,
}

#[function_component(Title)]
pub fn title(TitleProps { title, second }: &TitleProps) -> Html {
    if let Some(second) = second {
        html! {
            <div class="title_wrapper">
                { title_html(title, None) }
                { title_html(second, Some("second")) }
            </div>
        }
    } else {
        html! {
            { title_html(title, None) }
        }
    }
}

fn bg_style(image: &Option<String>) -> String {
    match image {
        Some(image) => {
            format!("background-image: url(\"{}\")", image)
        }
        None => "".to_string(),
    }
}

fn title_html(title: &TitleData, extra_style: Option<&str>) -> Html {
    let classes = if let Some(extra_style) = extra_style {
        classes! {"title_box", String::from(extra_style)}
    } else {
        classes! {"title_box"}
    };
    let classes = match title.image {
        None => classes!(classes, "title_no_bg"),
        Some(_) => classes,
    };

    html! {
        <div class={ classes } style={ bg_style(&title.image) }>
            <div class="title" >
                { title.title.clone() }
            </div>
        </div>
    }
}

pub fn single_title(title: &str, image: Option<&str>) -> TitleProps {
    let title = TitleData::new(title, image);

    TitleProps {
        title: title,
        second: None,
    }
}

pub fn dual_title(
    title1: &str,
    image1: Option<&str>,
    title2: &str,
    image2: Option<&str>,
) -> TitleProps {
    let title1 = TitleData::new(title1, image1);
    let title2 = TitleData::new(title2, image2);

    TitleProps {
        title: title1,
        second: Some(title2),
    }
}
