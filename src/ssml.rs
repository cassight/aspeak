use crate::cli::TextOptions;
use crate::error::Result;

use xml::{
    writer::{events::StartElementBuilder, XmlEvent},
    EventWriter,
};

trait StartElementBuilderExt<'a> {
    fn optional_attrs(self, attrs: &'a [(&str, Option<&str>)]) -> Self;
}

impl<'a> StartElementBuilderExt<'a> for StartElementBuilder<'a> {
    fn optional_attrs(self, attrs: &'a [(&str, Option<&str>)]) -> Self {
        attrs.into_iter().fold(self, |acc, (name, value)| {
            if let Some(ref v) = value {
                acc.attr(*name, v)
            } else {
                acc
            }
        })
    }
}

pub(crate) fn interpolate_ssml(options: &TextOptions) -> Result<String> {
    let mut buf = Vec::new();
    let mut writer = EventWriter::new(&mut buf);
    writer.write(
        XmlEvent::start_element("speak")
            .default_ns("http://www.w3.org/2001/10/synthesis")
            .ns("mstts", "http://www.w3.org/2001/mstts")
            .ns("emo", "http://www.w3.org/2009/10/emotionml")
            .attr("version", "1.0")
            .attr("xml:lang", "en-US"),
    )?;

    writer.write({
        let builder = XmlEvent::start_element("voice");
        if let Some(ref voice) = options.common_args.voice {
            builder.attr("name", voice)
        } else {
            builder
        }
    })?;

    // Make the borrow checker happy
    let style_degree = options.style_degree.map(|x| x.to_string());
    writer.write(
        XmlEvent::start_element("mstts:express-as")
            .optional_attrs(&[
                ("role", options.role.map(|role| role.into())),
                ("styledegree", style_degree.as_deref()),
            ])
            .attr("style", options.style.as_deref().unwrap_or("general")),
    )?;
    writer.write(XmlEvent::start_element("prosody").optional_attrs(&[
        ("pitch", options.pitch.as_deref()),
        ("rate", options.rate.as_deref()),
    ]))?;
    writer.write(XmlEvent::characters(options.text.as_deref().unwrap()))?;
    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::end_element())?;
    return Ok(String::from_utf8(buf).unwrap());
}
