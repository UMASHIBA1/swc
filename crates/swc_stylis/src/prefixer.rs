use std::mem::take;

use swc_common::DUMMY_SP;
use swc_css_ast::*;
use swc_css_utils::replace_ident;
use swc_css_visit::{VisitMut, VisitMutWith};

pub fn prefixer() -> impl VisitMut {
    Prefixer::default()
}

#[derive(Default)]
struct Prefixer {
    in_block: bool,
    added: Vec<Declaration>,
}

impl VisitMut for Prefixer {
    fn visit_mut_declaration(&mut self, n: &mut Declaration) {
        n.visit_mut_children_with(self);

        if !self.in_block {
            return;
        }

        macro_rules! simple {
            ($name:expr,$val:expr) => {{
                let val = Value::Ident(Ident {
                    span: DUMMY_SP,
                    value: $val.into(),
                    raw: $val.into(),
                });
                let name = DeclarationName::Ident(Ident {
                    span: DUMMY_SP,
                    value: $name.into(),
                    raw: $name.into(),
                });
                self.added.push(Declaration {
                    span: n.span,
                    name,
                    value: vec![val],
                    important: n.important.clone(),
                });
            }};
        }

        macro_rules! same_content {
            ($name:expr) => {{
                let name = DeclarationName::Ident(Ident {
                    span: DUMMY_SP,
                    value: $name.into(),
                    raw: $name.into(),
                });
                self.added.push(Declaration {
                    span: n.span,
                    name,
                    value: n.value.clone(),
                    important: n.important.clone(),
                });
            }};
        }

        macro_rules! same_name {
            ($name:expr) => {{
                let val = Ident {
                    span: DUMMY_SP,
                    value: $name.into(),
                    raw: $name.into(),
                };
                self.added.push(Declaration {
                    span: n.span,
                    name: n.name.clone(),
                    value: vec![Value::Ident(val)],
                    important: n.important.clone(),
                });
            }};
        }

        let is_dashed_ident = match n.name {
            DeclarationName::Ident(_) => false,
            DeclarationName::DashedIdent(_) => true,
        };

        if is_dashed_ident {
            return;
        }

        let name = match &n.name {
            DeclarationName::Ident(i) => &*i.value,
            _ => {
                unreachable!();
            }
        };

        match name {
            "appearance" => {
                same_content!("-webkit-appearance");
                same_content!("-moz-appearance");
                same_content!("-ms-appearance");
            }

            "animation" => {
                same_content!("-webkit-animation");
            }

            "animation-duration" => {
                same_content!("-webkit-animation-duration");
            }

            "animation-name" => {
                same_content!("-webkit-animation-name");
            }

            "animation-iteration-count" => {
                same_content!("-webkit-animation-iteration-count");
            }

            "animation-timing-function" => {
                same_content!("-webkit-animation-timing-function");
            }

            "background-clip" => {
                same_content!("-webkit-background-clip");
            }

            "box-decoration-break" => {
                same_content!("-webkit-box-decoration-break");
            }

            "color-adjust" => {
                same_content!("-webkit-print-color-adjust");
            }

            "columns" => {
                same_content!("-webkit-columns");
            }

            "column-count" => {
                same_content!("-webkit-column-count");
            }

            "column-fill" => {
                same_content!("-webkit-column-fill");
            }

            "column-gap" => {
                same_content!("-webkit-column-gap");
            }

            "column-rule" => {
                same_content!("-webkit-column-rule");
            }

            "column-rule-color" => {
                same_content!("-webkit-column-rule-color");
            }

            "column-rule-style" => {
                same_content!("-webkit-column-rule-style");
            }

            "column-span" => {
                same_content!("-webkit-column-span");
            }

            "column-rule-width" => {
                same_content!("-webkit-column-rule-width");
            }

            "column-width" => {
                same_content!("-webkit-column-width");
            }

            "background" => {
                if !n.value.is_empty() {
                    if let Value::Function(f) = &n.value[0] {
                        if &*f.name.value == "image-set" {
                            let val = Value::Function(Function {
                                span: DUMMY_SP,
                                name: Ident {
                                    span: DUMMY_SP,
                                    value: "-webkit-image-set".into(),
                                    raw: "-webkit-image-set".into(),
                                },
                                value: f.value.clone(),
                            });
                            self.added.push(Declaration {
                                span: n.span,
                                name: n.name.clone(),
                                value: vec![val],
                                important: n.important.clone(),
                            });
                        }
                    }
                }
            }

            "background-image" => {
                if !n.value.is_empty() {
                    if let Value::Function(f) = &n.value[0] {
                        if let "image-set" = &*f.name.value {
                            let val = Value::Function(Function {
                                span: DUMMY_SP,
                                name: Ident {
                                    span: DUMMY_SP,
                                    value: "-webkit-image-set".into(),
                                    raw: "-webkit-image-set".into(),
                                },
                                value: f.value.clone(),
                            });
                            self.added.push(Declaration {
                                span: n.span,
                                name: n.name.clone(),
                                value: vec![val],
                                important: n.important.clone(),
                            });
                        }
                    }
                }
            }

            "cursor" => {
                if !n.value.is_empty() {
                    let new_value = n
                        .value
                        .iter()
                        .map(|node| match node {
                            Value::Ident(Ident { value, .. }) => {
                                if &**value == "grab" {
                                    Value::Ident(Ident {
                                        span: DUMMY_SP,
                                        value: "-webkit-grab".into(),
                                        raw: "-webkit-grab".into(),
                                    })
                                } else {
                                    node.clone()
                                }
                            }
                            Value::Function(Function { name, value, .. }) => {
                                if &*name.value == "image-set" {
                                    Value::Function(Function {
                                        span: DUMMY_SP,
                                        name: Ident {
                                            span: DUMMY_SP,
                                            value: "-webkit-image-set".into(),
                                            raw: "-webkit-image-set".into(),
                                        },
                                        value: value.clone(),
                                    })
                                } else {
                                    node.clone()
                                }
                            }
                            _ => node.clone(),
                        })
                        .collect();

                    if n.value != new_value {
                        self.added.push(Declaration {
                            span: n.span,
                            name: n.name.clone(),
                            value: new_value,
                            important: n.important.clone(),
                        });
                    }
                }
            }

            "display" => {
                if n.value.len() == 1 {
                    if let Value::Ident(Ident { value, .. }) = &n.value[0] {
                        match &**value {
                            "flex" => {
                                same_name!("-webkit-box");
                                same_name!("-webkit-flex");
                                same_name!("-ms-flexbox");
                            }

                            "inline-flex" => {
                                same_name!("-webkit-inline-box");
                                same_name!("-webkit-inline-flex");
                                same_name!("-ms-inline-flexbox");
                            }

                            _ => {}
                        }
                    }
                }
            }

            "flex" => {
                same_content!("-webkit-flex");
                same_content!("-ms-flex");
            }

            "flex-grow" => {
                same_content!("-webkit-box-flex");
                same_content!("-webkit-flex-grow");
                same_content!("-ms-flex-positive");
            }

            "flex-shrink" => {
                same_content!("-webkit-flex-shrink");
                same_content!("-ms-flex-negative");
            }

            "flex-basis" => {
                same_content!("-webkit-flex-basis");
                same_content!("-ms-flex-preferred-size");
            }

            "align-self" => {
                same_content!("-webkit-align-self");
                same_content!("-ms-flex-item-align");
            }

            "align-content" => {
                same_content!("-webkit-align-content");
                same_content!("-ms-flex-line-pack");
            }

            "align-items" => {
                same_content!("-webkit-align-items");
                same_content!("-webkit-box-align");
                same_content!("-ms-flex-align");
            }

            "justify-content" => {
                if n.value.len() == 1 {
                    if let Value::Ident(Ident { value, .. }) = &n.value[0] {
                        match &**value {
                            "flex-end" => {
                                simple!("-webkit-box-pack", "end");
                                simple!("-ms-flex-pack", "end");
                            }

                            "flex-start" => {
                                simple!("-webkit-box-pack", "start");
                                simple!("-ms-flex-pack", "start");
                            }

                            "justify" => {
                                same_content!("-webkit-box-pack");
                                same_content!("-ms-flex-pack");
                            }

                            "space-between" => {
                                simple!("-webkit-box-pack", "justify");
                            }

                            _ => {}
                        }
                    }
                }

                same_content!("-webkit-justify-content");
            }

            "order" => {
                same_content!("-webkit-order");
                same_content!("-ms-flex-order");
            }

            "flex-direction" => {
                same_content!("-webkit-flex-direction");
                same_content!("-ms-flex-direction");
            }

            "filter" => {
                same_content!("-webkit-filter");
            }

            "mask" => {
                same_content!("-webkit-mask");
            }

            "mask-image" => {
                same_content!("-webkit-mask-image");
            }

            "mask-mode" => {
                same_content!("-webkit-mask-mode");
            }

            "mask-clip" => {
                same_content!("-webkit-mask-clip");
            }

            "mask-size" => {
                same_content!("-webkit-mask-size");
            }

            "mask-repeat" => {
                same_content!("-webkit-mask-repeat");
            }

            "mask-origin" => {
                same_content!("-webkit-mask-origin");
            }

            "mask-position" => {
                same_content!("-webkit-mask-position");
            }

            "mask-composite" => {
                same_content!("-webkit-mask-composite");
            }

            "margin-inline-start" => {
                same_content!("-webkit-margin-start");
            }

            "margin-inline-end" => {
                same_content!("-webkit-margin-end");
            }

            "backface-visibility" => {
                same_content!("-webkit-backface-visibility");
            }

            "clip-path" => {
                same_content!("-webkit-clip-path");
            }

            "position" => {
                if n.value.len() == 1 {
                    if let Value::Ident(Ident { value, .. }) = &n.value[0] {
                        if &**value == "sticky" {
                            same_name!("-webkit-sticky");
                        }
                    }
                }
            }

            "user-select" => {
                same_content!("-webkit-user-select");
                same_content!("-moz-user-select");
                same_content!("-ms-user-select");
            }

            "transform" => {
                same_content!("-webkit-transform");
                same_content!("-moz-transform");
                same_content!("-ms-transform");
            }

            "text-decoration" => {
                if n.value.len() == 1 {
                    if let Value::Ident(Ident { value, .. }) = &n.value[0] {
                        if &**value == "none" {
                            same_content!("-webkit-text-decoration");
                        }
                    }
                }
            }

            "text-size-adjust" => {
                if n.value.len() == 1 {
                    if let Value::Ident(Ident { value, .. }) = &n.value[0] {
                        if &**value == "none" {
                            same_content!("-webkit-text-size-adjust");
                            same_content!("-moz-text-size-adjust");
                            same_content!("-ms-text-size-adjust");
                        }
                    }
                }
            }

            "transition" => {
                let mut value = n.value.clone();

                replace_ident(&mut value, "transform", "-webkit-transform");

                let name = DeclarationName::Ident(Ident {
                    span: DUMMY_SP,
                    value: "-webkit-transition".into(),
                    raw: "-webkit-transition".into(),
                });
                self.added.push(Declaration {
                    span: n.span,
                    name,
                    value,
                    important: n.important.clone(),
                });
            }

            "writing-mode" => {
                if n.value.len() == 1 {
                    if let Value::Ident(Ident { value, .. }) = &n.value[0] {
                        match &**value {
                            "none" => {
                                same_content!("-webkit-writing-mode");
                                same_content!("-ms-writing-mode");
                            }

                            "vertical-lr" | "sideways-lr" => {
                                same_content!("-webkit-writing-mode");
                                simple!("-ms-writing-mode", "tb");
                            }

                            "vertical-rl" | "sideways-rl" => {
                                same_content!("-webkit-writing-mode");
                                simple!("-ms-writing-mode", "tb-rl");
                            }

                            "horizontal-tb" => {
                                same_content!("-webkit-writing-mode");
                                simple!("-ms-writing-mode", "lr");
                            }

                            _ => {}
                        }
                    }
                }
            }

            "min-width" | "width" | "max-width" | "min-height" | "height" | "max-height"
            | "min-block-size" | "min-inline-size" => {
                if n.value.len() == 1 {
                    if let Value::Ident(Ident { value, .. }) = &n.value[0] {
                        match &**value {
                            "fit-content" => {
                                same_name!("-webkit-fit-content");
                                same_name!("-moz-fit-content");
                            }

                            "max-content" => {
                                same_name!("-webkit-max-content");
                                same_name!("-moz-max-content");
                            }

                            "min-content" => {
                                same_name!("-webkit-min-content");
                                same_name!("-moz-min-content");
                            }

                            "fill-available" => {
                                same_name!("-webkit-fill-available");
                                same_name!("-moz-available");
                            }

                            "stretch" => {
                                same_name!("-webkit-fill-available");
                                same_name!("-moz-available");
                                same_name!("fill-available");
                            }

                            _ => {}
                        }
                    }
                }
            }

            _ => {}
        }
    }

    fn visit_mut_declaration_block_items(&mut self, props: &mut Vec<DeclarationBlockItem>) {
        let mut new = vec![];
        for mut n in take(props) {
            n.visit_mut_with(self);
            new.extend(self.added.drain(..).map(DeclarationBlockItem::Declaration));
            new.push(n);
        }

        *props = new;
    }

    fn visit_mut_qualified_rule(&mut self, n: &mut QualifiedRule) {
        let old_in_block = self.in_block;
        self.in_block = true;

        n.visit_mut_children_with(self);

        self.in_block = old_in_block;
    }
}
