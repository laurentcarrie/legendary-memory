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

pub fn get_handlebar() -> Result<Handlebars<'static>, Error> {
    let mut reg = Handlebars::new();
    reg.register_helper("simple-helper", Box::new(SimpleHelper));
    reg.register_helper("repeat-helper", Box::new(RepeatHelper));
    reg.register_helper("join-helper", Box::new(JoinHelper));
    reg.register_helper("add-helper", Box::new(AddHelper));

    // let template =
    //     String::from_utf8(include_bytes!("../../others/texfiles/struct.tex").to_vec())
    //         .unwrap();

    // let output_data = reg.render("t1", song).unwrap();
    // let _ = output.write(output_data.as_bytes()).unwrap();

    Ok(reg)
}
