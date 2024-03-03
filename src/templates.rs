use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use chrono::*;
use handlebars::*;

use crate::{ThumbnailData, ThumbnailMap, SIZES};

fn format_date(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output
) -> HelperResult {
    let unformatted = h.param(0).unwrap().value().render();
    let format = h.param(1).unwrap().value().render();
    let datetime = NaiveDate::parse_from_str(&unformatted, "%F")
        .map_err(|err| RenderErrorReason::NestedError(Box::new(err)))?;
    let formatted = format!("{}", datetime.format(&format));
    out.write(&formatted)?;
    Ok(())
}

fn ratio(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output
) -> HelperResult {
    let width_str = h.param(0).unwrap().value().render();
    let height_str = h.param(1).unwrap().value().render();

    let width: f32 = width_str.parse().unwrap();
    let height: f32 = height_str.parse().unwrap();

    let ratio = height / width * 100.0;

    out.write(&format!("{:.1}%", ratio).to_string())?;

    Ok(())
}

struct TitleLookupHelper {
    titles: BTreeMap<String, String>,
}

impl HelperDef for TitleLookupHelper {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
        let slug = h.param(0).unwrap().value().render();
        
        if let Some(title) = self.titles.get(&slug) {
            out.write(title)?;
        } else {
            Err(RenderErrorReason::Other("Couldn't find title".to_string()))?;
        }

        Ok(())
    }
}

struct PicHelper<'a> {
    thumbnail_map: &'a ThumbnailMap,
}

impl<'a> HelperDef for PicHelper<'a> {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
        let slug = h.param(0).unwrap().value().render();
        let file = h.param(1).unwrap().value().render();
        let cmd = h.param(2).unwrap().value().render();
        let path = slug.clone() + "/" + &file;

        match cmd.as_str() {
            "placeholder" =>
                out.write(&self.thumbnail_map[&path].placeholder)?,
            "width" =>
                out.write(&self.thumbnail_map[&path].width.to_string())?,
            "height" =>
                out.write(&self.thumbnail_map[&path].height.to_string())?,
            "src" =>
                out.write(&self.src(&self.thumbnail_map[&path], &slug, &file)?)?,
            "srcset" => {
                let max_width = match h.param(3) {
                    Some(json) => json.value().render().parse().unwrap_or(u32::MAX),
                    None => u32::MAX
                };
                out.write(&self.srcset(
                        &self.thumbnail_map[&path],
                        &slug, &file, max_width
                    )?)?
            },
            _ =>
                Err(RenderErrorReason::Other(
                        format!("Bad cmd '{}' in img", cmd)))?
        }

        Ok(())
    }
}

impl PicHelper<'_> {
    fn src(&self, thumbnail: &ThumbnailData, slug: &String, file: &String) 
        -> Result<String, RenderError>
    {
        let out = PathBuf::from(file).file_stem().unwrap()
            .to_str().unwrap().to_string();
        let mut out = "/".to_string() + &slug + "/" + &out + "-";

        let width = thumbnail.width;

        out += &{
            if width < SIZES[SIZES.len()-1] { width }
            else { SIZES[SIZES.len()-1] }
        }.to_string();
        out += ".jpg";

        Ok(out)
    }

    fn srcset(&self, 
              thumbnail: &ThumbnailData, 
              slug: &String, 
              file: &String,
              max_width: u32) 
        -> Result<String, RenderError>
    {
        let basename = PathBuf::from(file).file_stem().unwrap()
            .to_str().unwrap().to_string();
        let basename = "/".to_string() + &slug + "/" + &basename + "-";
        let mut out = String::new();
        let mut comma = false;

        let mut output = |w: u32| -> HelperResult {
            if w > 20 && w <= max_width {
                if comma { out += ", "; }
                else { comma = true; }

                out += &basename;
                out += &w.to_string();
                out += ".jpg ";
                out += &w.to_string();
                out += "w";
            }

            Ok(())
        };

        let width = thumbnail.width;
        for w in SIZES {
            if w < width {
                output(w)?;
            }
        }

        if width < SIZES[SIZES.len()-1] {
            output(width)?;
        }

        Ok(out)
    }
}



#[derive(Clone, Copy)]
struct HeightHelper<'a> {
    thumbnail_map: &'a ThumbnailMap,
}

impl<'a> HelperDef for HeightHelper<'a> {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
        let slug = h.param(0).unwrap().value().render();
        let file = h.param(1).unwrap().value().render();
        let path = slug.clone() + "/" + &file;

        out.write(&self.thumbnail_map[&path].height.to_string())?;

        Ok(())
    }
}

pub fn setup_handlebars(
    post_titles: BTreeMap<String, String>, 
    thumbnail_map: &ThumbnailMap) 
    -> Result<Handlebars>
{
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("format-date", Box::new(format_date));
    handlebars.register_helper("ratio", Box::new(ratio));
    handlebars.register_helper("titlelookup", Box::new(
            TitleLookupHelper { titles: post_titles }
            ));
    handlebars.register_helper("pic", Box::new(
            PicHelper { thumbnail_map: &thumbnail_map }
            ));

    for entry in fs::read_dir("templates")? {
        let entry = entry?;
        let path = entry.path();
        let basename = &path.file_stem().unwrap().to_str().unwrap();

        handlebars.register_template_string(basename,
            fs::read_to_string(&path)?)?;
    }

    Ok(handlebars)
}
