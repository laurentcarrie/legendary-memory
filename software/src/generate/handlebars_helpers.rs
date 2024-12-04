use handlebars::Handlebars;
use handlebars::*;
use std::io::Error;

// implement by a structure impls HelperDef
#[derive(Clone, Copy)]
struct SimpleHelper;

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
        for _i in 0..n - 1 {
            out.write(motif.value().render().as_ref())?;
            out.write(glue.value().render().as_ref())?;
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
        let result = values.iter().fold(0, |acc, v| acc + v);
        out.write(format!("{}", result).as_str())?;
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
        out.write(format!("{}", result).as_str())?;
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
        out.write(format!("% padhelper : {} ; {} ; {}", p, n, pad).as_str())?;
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
struct LenHelper;
impl HelperDef for crate::generate::handlebars_helpers::LenHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let p = h.param(0).unwrap().value().as_array().unwrap().len();
        out.write(format!("{}", p).as_str())?;
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

    Ok(reg)
}
