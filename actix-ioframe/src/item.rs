use std::cell::{Ref, RefMut};
use std::fmt;
use std::ops::{Deref, DerefMut};

use actix_codec::{Decoder, Encoder};

use crate::sink::Sink;
use crate::state::State;

pub struct Item<St, Codec: Encoder + Decoder> {
    state: State<St>,
    sink: Sink<<Codec as Encoder>::Item>,
    item: <Codec as Decoder>::Item,
}

impl<St, Codec> Item<St, Codec>
where
    Codec: Encoder + Decoder,
{
    pub(crate) fn new(
        state: State<St>,
        sink: Sink<<Codec as Encoder>::Item>,
        item: <Codec as Decoder>::Item,
    ) -> Self {
        Item { state, sink, item }
    }

    #[inline]
    pub fn state(&self) -> Ref<St> {
        self.state.get_ref()
    }

    #[inline]
    pub fn state_mut(&mut self) -> RefMut<St> {
        self.state.get_mut()
    }

    #[inline]
    pub fn sink(&self) -> &Sink<<Codec as Encoder>::Item> {
        &self.sink
    }

    #[inline]
    pub fn into_inner(self) -> <Codec as Decoder>::Item {
        self.item
    }

    #[inline]
    pub fn into_parts(
        self,
    ) -> (
        State<St>,
        Sink<<Codec as Encoder>::Item>,
        <Codec as Decoder>::Item,
    ) {
        (self.state, self.sink, self.item)
    }
}

impl<St, Codec> Deref for Item<St, Codec>
where
    Codec: Encoder + Decoder,
{
    type Target = <Codec as Decoder>::Item;

    #[inline]
    fn deref(&self) -> &<Codec as Decoder>::Item {
        &self.item
    }
}

impl<St, Codec> DerefMut for Item<St, Codec>
where
    Codec: Encoder + Decoder,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut <Codec as Decoder>::Item {
        &mut self.item
    }
}

impl<St, Codec> fmt::Debug for Item<St, Codec>
where
    Codec: Encoder + Decoder,
    <Codec as Decoder>::Item: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("FramedItem").field(&self.item).finish()
    }
}
