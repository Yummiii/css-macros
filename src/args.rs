use syn::{LitStr, meta::ParseNestedMeta, Result};

#[derive(Default)]
pub(super) struct CssAttributes {
    pub path: Option<LitStr>,
}

impl CssAttributes {
   pub(super) fn parse(&mut self, meta: ParseNestedMeta) -> Result<()> {
        if meta.path.is_ident("path") {
            self.path = Some(meta.value()?.parse()?);
            Ok(())        
        } else {
            Err(meta.error("unsupported property"))
        }
    }
}