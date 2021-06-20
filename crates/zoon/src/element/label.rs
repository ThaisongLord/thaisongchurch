use crate::*;
use std::marker::PhantomData;

// ------ ------
//    Element
// ------ ------

make_flags!(Label, ForInput);

pub struct Label<LabelFlag, ForInputFlag> {
    raw_el: RawHtmlEl,
    flags: PhantomData<(LabelFlag, ForInputFlag)>,
}

impl Label<LabelFlagNotSet, ForInputFlagNotSet> {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("label").attr("class", "label"),
            flags: PhantomData,
        }
    }
}

impl<ForInputFlag> Element for Label<LabelFlagSet, ForInputFlag> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<LabelFlag, ForInputFlag> UpdateRawEl<RawHtmlEl> for Label<LabelFlag, ForInputFlag> {
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

impl<LabelFlag, ForInputFlag> Styleable<RawHtmlEl> for Label<LabelFlag, ForInputFlag> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, LabelFlag, ForInputFlag> Label<LabelFlag, ForInputFlag> {
    pub fn label(mut self, label: impl IntoElement<'a> + 'a) -> Label<LabelFlagSet, ForInputFlag>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child(label);
        self.into_type()
    }

    pub fn label_signal(
        mut self,
        label: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Label<LabelFlagSet, ForInputFlag>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child_signal(label);
        self.into_type()
    }

    pub fn for_input(
        mut self,
        id: impl IntoCowStr<'a>,
    ) -> Label<LabelFlag, ForInputFlagSet>
    where
        ForInputFlag: FlagNotSet,
    {
        self.raw_el = self
            .raw_el
            .attr("for", &id.into_cow_str());
        self.into_type()
    }

    fn into_type<NewLabelFlag, NewForInputFlag>(self) -> Label<NewLabelFlag, NewForInputFlag> {
        Label {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}