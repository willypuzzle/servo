/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use std::fmt;

use dom_struct::dom_struct;

use crate::dom::bindings::codegen::Bindings::TrustedScriptBinding::TrustedScriptMethods;
use crate::dom::bindings::codegen::UnionTypes::TrustedScriptOrString;
use crate::dom::bindings::error::Fallible;
use crate::dom::bindings::reflector::{Reflector, reflect_dom_object};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::globalscope::GlobalScope;
use crate::dom::trustedtypepolicy::TrustedType;
use crate::dom::trustedtypepolicyfactory::TrustedTypePolicyFactory;
use crate::script_runtime::CanGc;

#[dom_struct]
pub struct TrustedScript {
    reflector_: Reflector,

    data: DOMString,
}

impl TrustedScript {
    fn new_inherited(data: DOMString) -> Self {
        Self {
            reflector_: Reflector::new(),
            data,
        }
    }

    #[cfg_attr(crown, allow(crown::unrooted_must_root))]
    pub(crate) fn new(data: DOMString, global: &GlobalScope, can_gc: CanGc) -> DomRoot<Self> {
        reflect_dom_object(Box::new(Self::new_inherited(data)), global, can_gc)
    }

    pub(crate) fn get_trusted_script_compliant_string(
        global: &GlobalScope,
        value: TrustedScriptOrString,
        containing_class: &str,
        field: &str,
        can_gc: CanGc,
    ) -> Fallible<DOMString> {
        match value {
            TrustedScriptOrString::String(value) => {
                let sink = format!("{} {}", containing_class, field);
                TrustedTypePolicyFactory::get_trusted_type_compliant_string(
                    TrustedType::TrustedScript,
                    global,
                    value,
                    &sink,
                    "'script'",
                    can_gc,
                )
            },

            TrustedScriptOrString::TrustedScript(trusted_script) => Ok(trusted_script.data.clone()),
        }
    }
}

impl fmt::Display for TrustedScript {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.data)
    }
}

impl TrustedScriptMethods<crate::DomTypeHolder> for TrustedScript {
    /// <https://www.w3.org/TR/trusted-types/#trustedscript-stringification-behavior>
    fn Stringifier(&self) -> DOMString {
        self.data.clone()
    }

    /// <https://www.w3.org/TR/trusted-types/#dom-trustedscript-tojson>
    fn ToJSON(&self) -> DOMString {
        self.data.clone()
    }
}
