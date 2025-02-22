/*
 * render/html/element/mod.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2021 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

//! Module that implements HTML rendering for `Element` and its children.

mod collapsible;
mod container;
mod date;
mod definition_list;
mod embed;
mod footnotes;
mod iframe;
mod image;
mod include;
mod input;
mod link;
mod list;
mod math;
mod table;
mod tabs;
mod text;
mod toc;
mod user;

mod prelude {
    pub use super::super::attributes::AddedAttributes;
    pub use super::super::context::HtmlContext;
    pub use super::super::random::Random;
    pub use super::{render_element, render_elements};
    pub use crate::log::prelude::*;
    pub use crate::tree::{Element, SyntaxTree};
}

use self::collapsible::{render_collapsible, Collapsible};
use self::container::{render_color, render_container};
use self::date::render_date;
use self::definition_list::render_definition_list;
use self::embed::render_embed;
use self::footnotes::{render_footnote, render_footnote_block};
use self::iframe::{render_html, render_iframe};
use self::image::render_image;
use self::include::{render_include, render_variable};
use self::input::{render_checkbox, render_radio_button};
use self::link::{render_anchor, render_link};
use self::list::render_list;
use self::math::{render_equation_reference, render_math_block, render_math_inline};
use self::table::render_table;
use self::tabs::render_tabview;
use self::text::{render_code, render_email, render_wikitext_raw};
use self::toc::render_table_of_contents;
use self::user::render_user;
use super::attributes::AddedAttributes;
use super::HtmlContext;
use crate::log::prelude::*;
use crate::render::ModuleRenderMode;
use crate::tree::Element;
use ref_map::*;

pub fn render_elements(log: &Logger, ctx: &mut HtmlContext, elements: &[Element]) {
    info!(log, "Rendering elements"; "elements-len" => elements.len());

    for element in elements {
        render_element(log, ctx, element);
    }
}

pub fn render_element(log: &Logger, ctx: &mut HtmlContext, element: &Element) {
    macro_rules! ref_cow {
        ($input:expr) => {
            $input.ref_map(|s| s.as_ref())
        };
    }

    info!(log, "Rendering element"; "element" => element.name());

    match element {
        Element::Container(container) => render_container(log, ctx, container),
        Element::Module(module) => {
            ctx.handle()
                .render_module(log, ctx.buffer(), module, ModuleRenderMode::Html);
        }
        Element::Text(text) => ctx.push_escaped(text),
        Element::Raw(text) => render_wikitext_raw(log, ctx, text),
        Element::Variable(name) => render_variable(log, ctx, name),
        Element::Email(email) => render_email(log, ctx, email),
        Element::Table(table) => render_table(log, ctx, table),
        Element::TabView(tabs) => render_tabview(log, ctx, tabs),
        Element::Anchor {
            elements,
            attributes,
            target,
        } => render_anchor(log, ctx, elements, attributes, *target),
        Element::Link {
            link,
            label,
            target,
        } => render_link(log, ctx, link, label, *target),
        Element::Image {
            source,
            link,
            alignment,
            attributes,
        } => render_image(log, ctx, source, link, *alignment, attributes),
        Element::List {
            ltype,
            items,
            attributes,
        } => render_list(log, ctx, *ltype, items, attributes),
        Element::DefinitionList(items) => render_definition_list(log, ctx, items),
        Element::RadioButton {
            name,
            checked,
            attributes,
        } => render_radio_button(log, ctx, name, *checked, attributes),
        Element::CheckBox {
            checked,
            attributes,
        } => render_checkbox(log, ctx, *checked, attributes),
        Element::Collapsible {
            elements,
            attributes,
            start_open,
            show_text,
            hide_text,
            show_top,
            show_bottom,
        } => render_collapsible(
            log,
            ctx,
            Collapsible::new(
                elements,
                attributes,
                *start_open,
                ref_cow!(show_text),
                ref_cow!(hide_text),
                *show_top,
                *show_bottom,
            ),
        ),
        Element::TableOfContents { align, attributes } => {
            render_table_of_contents(log, ctx, *align, attributes)
        }
        Element::Footnote => render_footnote(log, ctx),
        Element::FootnoteBlock { title, hide } => {
            if !(*hide || ctx.footnotes().is_empty()) {
                render_footnote_block(log, ctx, ref_cow!(title));
            }
        }
        Element::User { name, show_avatar } => render_user(log, ctx, name, *show_avatar),
        Element::Date {
            value,
            format,
            hover,
        } => render_date(log, ctx, *value, ref_cow!(format), *hover),
        Element::Color { color, elements } => render_color(log, ctx, color, elements),
        Element::Code { contents, language } => {
            render_code(log, ctx, ref_cow!(language), contents)
        }
        Element::Math { name, latex_source } => {
            render_math_block(log, ctx, ref_cow!(name), latex_source)
        }
        Element::MathInline { latex_source } => {
            render_math_inline(log, ctx, latex_source)
        }
        Element::EquationReference(name) => render_equation_reference(log, ctx, name),
        Element::Embed(embed) => render_embed(log, ctx, embed),
        Element::Html { contents } => render_html(log, ctx, contents),
        Element::Iframe { url, attributes } => render_iframe(log, ctx, url, attributes),
        Element::Include {
            variables,
            location,
            elements,
            ..
        } => render_include(log, ctx, location, variables, elements),
        Element::LineBreak => {
            ctx.html().br();
        }
        Element::LineBreaks(amount) => {
            let amount = amount.get();

            for _ in 0..amount {
                ctx.html().br();
            }
        }
        Element::ClearFloat(clear_float) => {
            ctx.html().div().attr(attr!(
                "class" => "wj-clear-float " clear_float.html_class(),
            ));
        }
        Element::HorizontalRule => {
            ctx.html().hr();
        }
        Element::Partial(_) => panic!("Encountered partial element during parsing"),
    }
}
