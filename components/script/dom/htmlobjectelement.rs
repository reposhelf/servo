/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::dom::attr::Attr;
use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::HTMLObjectElementBinding;
use crate::dom::bindings::codegen::Bindings::HTMLObjectElementBinding::HTMLObjectElementMethods;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::root::{DomRoot, MutNullableDom};
use crate::dom::bindings::str::DOMString;
use crate::dom::document::Document;
use crate::dom::element::{AttributeMutation, Element};
use crate::dom::htmlelement::HTMLElement;
use crate::dom::htmlformelement::{FormControl, HTMLFormElement};
use crate::dom::node::{window_from_node, Node};
use crate::dom::validation::Validatable;
use crate::dom::validitystate::{ValidationFlags, ValidityState};
use crate::dom::virtualmethods::VirtualMethods;
use dom_struct::dom_struct;
use html5ever::{LocalName, Prefix};
use net_traits::image::base::Image;
use servo_arc::Arc;
use std::default::Default;

#[dom_struct]
pub struct HTMLObjectElement {
    htmlelement: HTMLElement,
    #[ignore_malloc_size_of = "Arc"]
    image: DomRefCell<Option<Arc<Image>>>,
    form_owner: MutNullableDom<HTMLFormElement>,
}

impl HTMLObjectElement {
    fn new_inherited(
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
    ) -> HTMLObjectElement {
        HTMLObjectElement {
            htmlelement: HTMLElement::new_inherited(local_name, prefix, document),
            image: DomRefCell::new(None),
            form_owner: Default::default(),
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
    ) -> DomRoot<HTMLObjectElement> {
        Node::reflect_node(
            Box::new(HTMLObjectElement::new_inherited(
                local_name, prefix, document,
            )),
            document,
            HTMLObjectElementBinding::Wrap,
        )
    }
}

trait ProcessDataURL {
    fn process_data_url(&self);
}

impl<'a> ProcessDataURL for &'a HTMLObjectElement {
    // Makes the local `data` member match the status of the `data` attribute and starts
    /// prefetching the image. This method must be called after `data` is changed.
    fn process_data_url(&self) {
        let elem = self.upcast::<Element>();

        // TODO: support other values
        match (
            elem.get_attribute(&ns!(), &local_name!("type")),
            elem.get_attribute(&ns!(), &local_name!("data")),
        ) {
            (None, Some(_uri)) => {
                // TODO(gw): Prefetch the image here.
            },
            _ => {},
        }
    }
}

impl HTMLObjectElementMethods for HTMLObjectElement {
    // https://html.spec.whatwg.org/multipage/#dom-cva-validity
    fn Validity(&self) -> DomRoot<ValidityState> {
        let window = window_from_node(self);
        ValidityState::new(&window, self.upcast())
    }

    // https://html.spec.whatwg.org/multipage/#dom-object-type
    make_getter!(Type, "type");

    // https://html.spec.whatwg.org/multipage/#dom-object-type
    make_setter!(SetType, "type");

    // https://html.spec.whatwg.org/multipage/#dom-fae-form
    fn GetForm(&self) -> Option<DomRoot<HTMLFormElement>> {
        self.form_owner()
    }
}

impl Validatable for HTMLObjectElement {
    fn is_instance_validatable(&self) -> bool {
        true
    }
    fn validate(&self, validate_flags: ValidationFlags) -> bool {
        if validate_flags.is_empty() {}
        // Need more flag check for different validation types later
        true
    }
}

impl VirtualMethods for HTMLObjectElement {
    fn super_type(&self) -> Option<&dyn VirtualMethods> {
        Some(self.upcast::<HTMLElement>() as &dyn VirtualMethods)
    }

    fn attribute_mutated(&self, attr: &Attr, mutation: AttributeMutation) {
        self.super_type().unwrap().attribute_mutated(attr, mutation);
        match attr.local_name() {
            &local_name!("data") => {
                if let AttributeMutation::Set(_) = mutation {
                    self.process_data_url();
                }
            },
            &local_name!("form") => {
                self.form_attribute_mutated(mutation);
            },
            _ => {},
        }
    }
}

impl FormControl for HTMLObjectElement {
    fn form_owner(&self) -> Option<DomRoot<HTMLFormElement>> {
        self.form_owner.get()
    }

    fn set_form_owner(&self, form: Option<&HTMLFormElement>) {
        self.form_owner.set(form);
    }

    fn to_element<'a>(&'a self) -> &'a Element {
        self.upcast::<Element>()
    }
}
