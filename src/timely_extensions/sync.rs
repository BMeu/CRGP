//! An extension to timely dataflow ``Scope``s allowing to wait for the computation to finish the current batch of data.

use timely::Data;
use timely::dataflow::operators::input::Handle as InputHandle;
use timely::dataflow::operators::probe::Handle as ProbeHandle;
use timely::progress::nested::product::Product;
use timely::progress::timestamp::RootTimestamp;
use timely::dataflow::scopes::root::Root;
use timely_communication::allocator::Allocate;

/// An extension to timely dataflow ``Scope``s allowing to wait for the computation to finish the current batch of data.
pub trait Sync<D1: Data, D2: Data> {
    /// Wait for the computation to finish the current batch of data.
    ///
    /// Both ``input``s' times will be advanced. The computation ``self`` will step until the time of ``probe`` has
    /// reached the time of ``input1``.
    fn sync(&mut self, probe: &ProbeHandle<Product<RootTimestamp, u64>>, input1: &mut InputHandle<u64, D1>,
            input2: &mut InputHandle<u64, D2>);
}

impl<A: Allocate, D1: Data, D2: Data> Sync<D1, D2> for Root<A> {
    #[inline]
    fn sync(&mut self, probe: &ProbeHandle<Product<RootTimestamp, u64>>, input1: &mut InputHandle<u64, D1>,
            input2: &mut InputHandle<u64, D2>) {
        let input1_next = input1.epoch() + 1;
        let input2_next = input2.epoch() + 1;

        input1.advance_to(input1_next);
        input2.advance_to(input2_next);

        while probe.lt(input1.time()) {
            self.step();
        }
    }
}
