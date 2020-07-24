initSidebarItems({"fn":[["block_on","Blocks the current thread on a future."],["join","Joins two futures, waiting for both to complete."],["pending","Creates a future that is always pending."],["poll_fn","Creates a future from a function returning [`Poll`]."],["poll_once","Polls a future just once and returns an [`Option`] with the result."],["race","Returns the result of the future that completes first, with no preference if both are ready."],["ready","Creates a future that resolves to the provided value."],["try_join","Joins two fallible futures, waiting for both to complete or one of them to error."],["yield_now","Wakes the current task and returns [`Poll::Pending`] once."]],"struct":[["Join","Future for the [`join()`] function."],["Or","Future for the [`or()`][`FutureExt::or()`] method."],["Pending","Future for the [`pending()`] function."],["PollFn","Future for the [`poll_fn()`] function."],["PollOnce","Future for the [`poll_once()`] function."],["Race","Future for the [`race()`] function."],["Ready","Future for the [`ready()`] function."],["TryJoin","Future for the [`try_join()`] function."],["YieldNow","Future for the [`yield_now()`] function."]],"trait":[["Future","A future represents an asynchronous computation."],["FutureExt","Extension trait for [`Future`]."]],"type":[["Boxed","Type alias for `Pin<Box<dyn Future<Output = T> + Send>>`."],["BoxedLocal","Type alias for `Pin<Box<dyn Future<Output = T>>>`."]]});