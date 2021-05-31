use super::{Value, ValueTrait};

impl ValueTrait for bool {
    fn type_name(&self) -> &'static str {
        "bool"
    }

    fn box_clone(&self) -> Box<dyn ValueTrait> {
        Box::new(*self)
    }

    fn format(&self, _indent: usize, spans: &mut Vec<crate::Span>) {
        spans.push(crate::Span {
            string: self.to_string(),
            kind: crate::SpanKind::Boolean,
        });
    }

    fn not(&self) -> Result<Value<'static>, String> {
        Ok((!self).into())
    }
}