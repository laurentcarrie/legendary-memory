use handlebars::Handlebars;
use handlebars::*;
use serde_json::Value;
use std::io::Error;

// implement by a structure impls HelperDef
#[derive(Clone, Copy)]
struct SimpleHelper;

fn i64_of_value(value: &Value) -> i64 {
    if value.is_string() {
        value.as_str().unwrap().parse::<i64>().unwrap()
    } else if value.is_i64() {
        value.as_i64().unwrap()
    } else {
        dbg!(&value);
        panic!("{}:{} bad type", file!(), line!());
    }
}

impl HelperDef for SimpleHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).unwrap();

        out.write("1st helper: ")?;
        out.write(param.value().render().as_ref())?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct RepeatHelper;
impl HelperDef for RepeatHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).unwrap();
        let count = h.param(1).unwrap();

        let n = count.value().render().parse::<u32>().unwrap();
        for _i in 0..n {
            out.write(param.value().render().as_ref())?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct JoinHelper;
impl HelperDef for JoinHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let motif = h.param(0).unwrap();
        let glue = h.param(1).unwrap();
        let count = h.param(2).unwrap();

        let n = count.value().render().parse::<u32>().unwrap();
        if n > 0 {
            for _i in 0..n - 1 {
                out.write(motif.value().render().as_ref())?;
                out.write(glue.value().render().as_ref())?;
            }
        }
        out.write(motif.value().render().as_ref())?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct AddHelper;
impl HelperDef for AddHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let values: Vec<_> = h
            .params()
            .iter()
            .map(|p| p.value().render().parse::<i32>().unwrap())
            .collect();
        let result = values.iter().sum::<i32>();
        out.write(format!("{result}").as_str())?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct RemoveFileExtension;
impl HelperDef for RemoveFileExtension {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let filename = h.param(0).unwrap().value().render();
        let extension = h.param(1).unwrap().value().render();
        let result = filename.as_str().replace(extension.as_str(), "");
        out.write(result.as_str())?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct PadHelper;
impl HelperDef for PadHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let p = h.param(0).unwrap().value().as_array().unwrap().len();
        let n = h.param(1).unwrap().value().as_i64().unwrap() as usize;
        let pad = h.param(2).unwrap().value().as_str().unwrap();
        out.write(format!("% padhelper : {p} ; {n} ; {pad}").as_str())?;
        out.write(
            (0..(std::cmp::max(0, n - p)))
                .map(|_| pad)
                .collect::<Vec<_>>()
                .join("")
                .as_str(),
        )?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct MulticolsHelper;
impl HelperDef for MulticolsHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let p = h.param(0).unwrap().value().as_array().unwrap().len();
        let n = h.param(1).unwrap().value().as_i64().unwrap() as usize;
        out.write(format!("% multicol helper : {p} ; {n} \n").as_str())?;
        if n - p > 0 {
            out.write(format!("& \\multicolumn{{{}}}{{c}}{{}}\n", n - p).as_str())?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct GreaterThanHelper;
impl HelperDef for GreaterThanHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let a = i64_of_value(h.param(0).unwrap().value());
        let b = i64_of_value(h.param(1).unwrap().value());
        let j = {
            let b = a > b;
            JsonValue::from(b)
        };
        let x = ScopedJson::Constant(&j);
        // dbg!(&x);
        write!(out, "{}", x.render()).unwrap();

        Ok(())
    }
}

#[derive(Clone, Copy)]
struct LenHelper;
impl HelperDef for LenHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let p = h.param(0).unwrap().value().as_array().unwrap().len();
        out.write(format!("{p}").as_str())?;
        // let j = JsonValue::from(999 as i64);
        // let x = ScopedJson::Constant(&j);
        // dbg!(&x);
        // write!(out, "{}", x.render());
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct RowStartBarTimeHelper;
impl HelperDef for RowStartBarTimeHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let nbar = i64_of_value(h.param(0).unwrap().value());
        let tempo = i64_of_value(h.param(1).unwrap().value());
        //@ todo : we assume that a bar is always 4 times
        let total_seconds = (nbar - 1) as f64 * 4_f64 * 60_f64 / tempo as f64;
        let minutes = (total_seconds / 60_f64).floor() as u64;
        let seconds = total_seconds as u64 - minutes * 60;
        let _ = write!(out, "{minutes}'{seconds:0>2}\"");

        Ok(())
    }
}

#[derive(Clone, Copy)]
struct TexSanitizeHelper;
impl HelperDef for TexSanitizeHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let s = h.param(0).unwrap().value().as_str().unwrap();
        // let s = s.replace("_", "\\_");
        let s = s.replace("_", " ");
        out.write(&s)?;
        Ok(())
    }
}

pub fn get_handlebar() -> Result<Handlebars<'static>, Error> {
    let mut reg = Handlebars::new();
    reg.register_helper("simple-helper", Box::new(SimpleHelper));
    reg.register_helper("repeat-helper", Box::new(RepeatHelper));
    reg.register_helper("join-helper", Box::new(JoinHelper));
    reg.register_helper("add-helper", Box::new(AddHelper));
    reg.register_helper(
        "remove-file-extension-helper",
        Box::new(RemoveFileExtension),
    );
    reg.register_helper("pad-helper", Box::new(PadHelper));
    reg.register_helper("len-helper", Box::new(LenHelper));
    reg.register_helper("helper_tex_sanitize", Box::new(TexSanitizeHelper));
    reg.register_helper("multicols_helper", Box::new(MulticolsHelper));
    reg.register_helper("greater-than-helper", Box::new(GreaterThanHelper));
    reg.register_helper("row_start_bar_time", Box::new(RowStartBarTimeHelper));

    Ok(reg)
}
