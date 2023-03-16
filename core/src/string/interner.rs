use std::borrow::Cow;

use fnv::FnvHashSet;
use gc_arena::{Collect, MutationContext};
use ruffle_wstr::WStr;

use super::avm_string::AvmString;

#[derive(Collect, Default)]
#[collect(no_drop)]
pub struct AvmStringInterner<'gc> {
    // TODO(moulins): use some kind of weak map
    interned: FnvHashSet<AvmString<'gc>>,
}

impl<'gc> AvmStringInterner<'gc> {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn intern_wstr<'a, S>(
        &mut self,
        gc_context: MutationContext<'gc, '_>,
        s: S,
    ) -> AvmString<'gc>
    where
        S: AsRef<WStr> + Into<Cow<'a, WStr>>,
    {
        if let Some(s) = self.interned.get(s.as_ref()) {
            *s
        } else {
            let s = AvmString::new(gc_context, s.into().into_owned());
            self.interned.insert(s);
            s
        }
    }

    #[must_use]
    pub fn intern(&mut self, s: AvmString<'gc>) -> AvmString<'gc> {
        if let Some(s) = self.interned.get(&s) {
            *s
        } else {
            self.interned.insert(s);
            s
        }
    }
}