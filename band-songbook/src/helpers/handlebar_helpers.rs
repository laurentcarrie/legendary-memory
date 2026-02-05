use crate::chords::glyph::glyph_of_baritem;
use crate::chords::model::BarItem;
use crate::chords::parse::parse;
use handlebars::{
    Context, Handlebars, Helper, Output, RenderContext, RenderError, RenderErrorReason,
};

/// Returns the length of an array
pub fn len_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).and_then(|v| v.value().as_array());
    let len = param.map(|arr| arr.len()).unwrap_or(0);
    out.write(&len.to_string())?;
    Ok(())
}

/// Returns the number of bars in a chord string
pub fn number_of_bars_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let input = h
        .param(0)
        .and_then(|v| v.value().as_str())
        .ok_or(RenderErrorReason::Other(
            "missing input parameter".to_string(),
        ))?;

    let count = parse(input)
        .map_err(|e| RenderErrorReason::Other(e.to_string()))?
        .bars
        .len();
    out.write(&count.to_string())?;
    Ok(())
}

/// Returns the content of a bar at a given index
pub fn bar_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let input = h
        .param(0)
        .and_then(|v| v.value().as_str())
        .ok_or(RenderErrorReason::Other(
            "missing input parameter".to_string(),
        ))?;

    let index = h
        .param(1)
        .and_then(|v| v.value().as_u64())
        .ok_or(RenderErrorReason::Other(
            "missing index parameter".to_string(),
        ))? as usize;

    let result = parse(input)
        .ok()
        .and_then(|parsed| parsed.bars.get(index).cloned())
        .map(|bar| {
            bar.items
                .iter()
                .map(|item| match item {
                    BarItem::Chord(chord) => chord.name.clone(),
                    BarItem::Rest(_) => "HRest".to_string(),
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_default();

    out.write(&result)?;
    Ok(())
}

/// Returns the glyph of a bar at a given index
pub fn bar_glyph_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let input = h
        .param(0)
        .and_then(|v| v.value().as_str())
        .ok_or(RenderErrorReason::Other(
            "missing input parameter".to_string(),
        ))?;

    let index = h
        .param(1)
        .and_then(|v| v.value().as_u64())
        .ok_or(RenderErrorReason::Other(
            "missing index parameter".to_string(),
        ))? as usize;

    let result = parse(input)
        .ok()
        .and_then(|parsed| parsed.bars.get(index).cloned())
        .map(|bar| {
            bar.items
                .iter()
                .map(glyph_of_baritem)
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_default();

    out.write(&result)?;
    Ok(())
}

/// Draws rectangles for each bar in a row with glyphs
/// Usage: {{bar_rects row color}}
pub fn bar_rects_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let input = h
        .param(0)
        .and_then(|v| v.value().as_str())
        .ok_or(RenderErrorReason::Other(
            "missing input parameter".to_string(),
        ))?;

    let color = h
        .param(1)
        .and_then(|v| v.value().as_str())
        .unwrap_or("gray");

    let parsed = parse(input).map_err(|e| RenderErrorReason::Other(e.to_string()))?;

    for (i, bar) in parsed.bars.iter().enumerate() {
        let glyphs: Vec<_> = bar.items.iter().map(glyph_of_baritem).collect();

        if glyphs.len() >= 2 {
            // Two or more chords: draw rectangle, then position first chord up 10%, second down 10%
            let draw_cmd = format!(
                "    \\draw[draw=black, fill={color}] (\\columnleft + {i}*\\xr, \\currentline) rectangle ++(\\xr, -\\yr);\n"
            );
            out.write(&draw_cmd)?;

            // First chord shifted towards upper left corner (10% left, 10% up)
            let first_chord_cmd = format!(
                "    \\node at (\\columnleft + {}*\\xr + 0.4*\\xr, \\currentline - 0.3*\\yr) {{ {} }};\n",
                i, glyphs[0]
            );
            out.write(&first_chord_cmd)?;

            // Second chord shifted towards lower right corner (10% right, 10% down)
            let second_chord_cmd = format!(
                "    \\node at (\\columnleft + {}*\\xr + 0.8*\\xr, \\currentline - 0.7*\\yr) {{ {} }};\n",
                i, glyphs[1]
            );
            out.write(&second_chord_cmd)?;

            // Draw oblique bar
            let oblique_cmd = format!(
                "    \\draw[draw=black] (\\columnleft + {i}*\\xr + \\xr, \\currentline) -- ++(-\\xr, -\\yr);\n"
            );
            out.write(&oblique_cmd)?;
        } else {
            // Single chord: original behavior
            let glyph = glyphs.join(" ");
            let draw_cmd = format!(
                "    \\draw[draw=black, fill={color}] (\\columnleft + {i}*\\xr, \\currentline) rectangle ++(\\xr, -\\yr) node[midway] {{ {glyph} }};\n"
            );
            out.write(&draw_cmd)?;
        }
    }

    Ok(())
}

/// Returns the repeat multiplier from a chord row (e.g., "Em|G|x2" returns 2)
/// Usage: {{row_multiplier row}}
pub fn row_multiplier_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let input = h
        .param(0)
        .and_then(|v| v.value().as_str())
        .ok_or(RenderErrorReason::Other(
            "missing input parameter".to_string(),
        ))?;

    let multiplier = parse(input).map(|parsed| parsed.repeat.n).unwrap_or(1);
    out.write(&multiplier.to_string())?;
    Ok(())
}

/// Returns the total bar count for a referenced section (by link/id)
/// Calculates sum of (number_of_bars * repeat) for each row in the referenced Chords section
/// Usage: {{ref_bar_count link song.structure}}
pub fn ref_bar_count_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let link = h
        .param(0)
        .and_then(|v| v.value().as_str())
        .ok_or(RenderErrorReason::Other(
            "missing link parameter".to_string(),
        ))?;

    let structure =
        h.param(1)
            .and_then(|v| v.value().as_array())
            .ok_or(RenderErrorReason::Other(
                "missing structure parameter".to_string(),
            ))?;

    // Find the referenced section by id
    let mut total_bars: u32 = 0;
    for item in structure {
        let id = item.get("id").and_then(|v| v.as_str());
        if id == Some(link) {
            // Found the referenced section, check if it's a Chords section
            if let Some(chords) = item.get("item").and_then(|v| v.get("Chords")) {
                if let Some(rows) = chords.get("rows").and_then(|v| v.as_array()) {
                    for row in rows {
                        if let Some(row_str) = row.as_str() {
                            if let Ok(parsed) = parse(row_str) {
                                let bars = parsed.bars.len() as u32;
                                let repeat = parsed.repeat.n;
                                total_bars += bars * repeat;
                            }
                        }
                    }
                }
            }
            break;
        }
    }

    out.write(&total_bars.to_string())?;
    Ok(())
}

/// Registers all custom helpers with the handlebars instance
pub fn register_helpers(handlebars: &mut Handlebars) {
    handlebars.register_helper("len-helper", Box::new(len_helper));
    handlebars.register_helper("number_of_bars", Box::new(number_of_bars_helper));
    handlebars.register_helper("bar", Box::new(bar_helper));
    handlebars.register_helper("bar_glyph", Box::new(bar_glyph_helper));
    handlebars.register_helper("bar_rects", Box::new(bar_rects_helper));
    handlebars.register_helper("row_multiplier", Box::new(row_multiplier_helper));
    handlebars.register_helper("ref_bar_count", Box::new(ref_bar_count_helper));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_bars_helper() {
        let mut handlebars = Handlebars::new();
        register_helpers(&mut handlebars);

        let template = "{{number_of_bars input}}";
        // "A|B|" has 2 bars (empty bars are filtered out)
        let data = serde_json::json!({"input": "A|B|"});

        let result = handlebars.render_template(template, &data).unwrap();
        assert_eq!(result, "2");
    }
}
