(function() {var implementors = {};
implementors["async_executor"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"async_executor/struct.Task.html\" title=\"struct async_executor::Task\">Task</a>&lt;T&gt;","synthetic":false,"types":["async_executor::Task"]}];
implementors["async_io"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"async_io/struct.Timer.html\" title=\"struct async_io::Timer\">Timer</a>","synthetic":false,"types":["async_io::Timer"]}];
implementors["async_task"] = [{"text":"impl&lt;R, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"async_task/struct.JoinHandle.html\" title=\"struct async_task::JoinHandle\">JoinHandle</a>&lt;R, T&gt;","synthetic":false,"types":["async_task::join_handle::JoinHandle"]}];
implementors["event_listener"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"event_listener/struct.EventListener.html\" title=\"struct event_listener::EventListener\">EventListener</a>","synthetic":false,"types":["event_listener::EventListener"]}];
implementors["futures_channel"] = [{"text":"impl&lt;'_, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_channel/oneshot/struct.Cancellation.html\" title=\"struct futures_channel::oneshot::Cancellation\">Cancellation</a>&lt;'_, T&gt;","synthetic":false,"types":["futures_channel::oneshot::Cancellation"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_channel/oneshot/struct.Receiver.html\" title=\"struct futures_channel::oneshot::Receiver\">Receiver</a>&lt;T&gt;","synthetic":false,"types":["futures_channel::oneshot::Receiver"]}];
implementors["futures_lite"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/future/struct.Pending.html\" title=\"struct futures_lite::future::Pending\">Pending</a>&lt;T&gt;","synthetic":false,"types":["futures_lite::future::Pending"]},{"text":"impl&lt;T, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/future/struct.PollOnce.html\" title=\"struct futures_lite::future::PollOnce\">PollOnce</a>&lt;F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = T&gt;,&nbsp;</span>","synthetic":false,"types":["futures_lite::future::PollOnce"]},{"text":"impl&lt;T, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/future/struct.PollFn.html\" title=\"struct futures_lite::future::PollFn\">PollFn</a>&lt;F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html\" title=\"struct core::task::wake::Context\">Context</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html\" title=\"enum core::task::poll::Poll\">Poll</a>&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":["futures_lite::future::PollFn"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/future/struct.Ready.html\" title=\"struct futures_lite::future::Ready\">Ready</a>&lt;T&gt;","synthetic":false,"types":["futures_lite::future::Ready"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/future/struct.YieldNow.html\" title=\"struct futures_lite::future::YieldNow\">YieldNow</a>","synthetic":false,"types":["futures_lite::future::YieldNow"]},{"text":"impl&lt;Fut1, Fut2&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/future/struct.Join.html\" title=\"struct futures_lite::future::Join\">Join</a>&lt;Fut1, Fut2&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Fut1: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Fut2: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>,&nbsp;</span>","synthetic":false,"types":["futures_lite::future::Join"]},{"text":"impl&lt;T1, T2, E, Fut1, Fut2&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/future/struct.TryJoin.html\" title=\"struct futures_lite::future::TryJoin\">TryJoin</a>&lt;Fut1, Fut2&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Fut1: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;T1, E&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Fut2: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;T2, E&gt;&gt;,&nbsp;</span>","synthetic":false,"types":["futures_lite::future::TryJoin"]},{"text":"impl&lt;T, A, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/future/struct.Race.html\" title=\"struct futures_lite::future::Race\">Race</a>&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = T&gt;,&nbsp;</span>","synthetic":false,"types":["futures_lite::future::Race"]},{"text":"impl&lt;T, A, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/future/struct.Or.html\" title=\"struct futures_lite::future::Or\">Or</a>&lt;A, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = T&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = T&gt;,&nbsp;</span>","synthetic":false,"types":["futures_lite::future::Or"]},{"text":"impl&lt;'_, T:&nbsp;<a class=\"trait\" href=\"futures_io/if_std/trait.AsyncBufRead.html\" title=\"trait futures_io::if_std::AsyncBufRead\">AsyncBufRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/io/struct.ReadUntilFuture.html\" title=\"struct futures_lite::io::ReadUntilFuture\">ReadUntilFuture</a>&lt;'_, T&gt;","synthetic":false,"types":["futures_lite::io::ReadUntilFuture"]},{"text":"impl&lt;'_, T:&nbsp;<a class=\"trait\" href=\"futures_io/if_std/trait.AsyncBufRead.html\" title=\"trait futures_io::if_std::AsyncBufRead\">AsyncBufRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/io/struct.ReadLineFuture.html\" title=\"struct futures_lite::io::ReadLineFuture\">ReadLineFuture</a>&lt;'_, T&gt;","synthetic":false,"types":["futures_lite::io::ReadLineFuture"]},{"text":"impl&lt;'_, T:&nbsp;<a class=\"trait\" href=\"futures_io/if_std/trait.AsyncRead.html\" title=\"trait futures_io::if_std::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/io/struct.ReadFuture.html\" title=\"struct futures_lite::io::ReadFuture\">ReadFuture</a>&lt;'_, T&gt;","synthetic":false,"types":["futures_lite::io::ReadFuture"]},{"text":"impl&lt;'_, T:&nbsp;<a class=\"trait\" href=\"futures_io/if_std/trait.AsyncRead.html\" title=\"trait futures_io::if_std::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/io/struct.ReadVectoredFuture.html\" title=\"struct futures_lite::io::ReadVectoredFuture\">ReadVectoredFuture</a>&lt;'_, T&gt;","synthetic":false,"types":["futures_lite::io::ReadVectoredFuture"]},{"text":"impl&lt;'_, T:&nbsp;<a class=\"trait\" href=\"futures_io/if_std/trait.AsyncRead.html\" title=\"trait futures_io::if_std::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/io/struct.ReadToEndFuture.html\" title=\"struct futures_lite::io::ReadToEndFuture\">ReadToEndFuture</a>&lt;'_, T&gt;","synthetic":false,"types":["futures_lite::io::ReadToEndFuture"]},{"text":"impl&lt;'_, T:&nbsp;<a class=\"trait\" href=\"futures_io/if_std/trait.AsyncRead.html\" title=\"trait futures_io::if_std::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/io/struct.ReadToStringFuture.html\" title=\"struct futures_lite::io::ReadToStringFuture\">ReadToStringFuture</a>&lt;'_, T&gt;","synthetic":false,"types":["futures_lite::io::ReadToStringFuture"]},{"text":"impl&lt;'_, T:&nbsp;<a class=\"trait\" href=\"futures_io/if_std/trait.AsyncRead.html\" title=\"trait futures_io::if_std::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/io/struct.ReadExactFuture.html\" title=\"struct futures_lite::io::ReadExactFuture\">ReadExactFuture</a>&lt;'_, T&gt;","synthetic":false,"types":["futures_lite::io::ReadExactFuture"]},{"text":"impl&lt;'_, T:&nbsp;<a class=\"trait\" href=\"futures_io/if_std/trait.AsyncSeek.html\" title=\"trait futures_io::if_std::AsyncSeek\">AsyncSeek</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/io/struct.SeekFuture.html\" title=\"struct futures_lite::io::SeekFuture\">SeekFuture</a>&lt;'_, T&gt;","synthetic":false,"types":["futures_lite::io::SeekFuture"]},{"text":"impl&lt;'_, T:&nbsp;<a class=\"trait\" href=\"futures_io/if_std/trait.AsyncWrite.html\" title=\"trait futures_io::if_std::AsyncWrite\">AsyncWrite</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/io/struct.WriteFuture.html\" title=\"struct futures_lite::io::WriteFuture\">WriteFuture</a>&lt;'_, T&gt;","synthetic":false,"types":["futures_lite::io::WriteFuture"]},{"text":"impl&lt;'_, T:&nbsp;<a class=\"trait\" href=\"futures_io/if_std/trait.AsyncWrite.html\" title=\"trait futures_io::if_std::AsyncWrite\">AsyncWrite</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/io/struct.WriteVectoredFuture.html\" title=\"struct futures_lite::io::WriteVectoredFuture\">WriteVectoredFuture</a>&lt;'_, T&gt;","synthetic":false,"types":["futures_lite::io::WriteVectoredFuture"]},{"text":"impl&lt;'_, T:&nbsp;<a class=\"trait\" href=\"futures_io/if_std/trait.AsyncWrite.html\" title=\"trait futures_io::if_std::AsyncWrite\">AsyncWrite</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/io/struct.WriteAllFuture.html\" title=\"struct futures_lite::io::WriteAllFuture\">WriteAllFuture</a>&lt;'_, T&gt;","synthetic":false,"types":["futures_lite::io::WriteAllFuture"]},{"text":"impl&lt;'_, T:&nbsp;<a class=\"trait\" href=\"futures_io/if_std/trait.AsyncWrite.html\" title=\"trait futures_io::if_std::AsyncWrite\">AsyncWrite</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/io/struct.FlushFuture.html\" title=\"struct futures_lite::io::FlushFuture\">FlushFuture</a>&lt;'_, T&gt;","synthetic":false,"types":["futures_lite::io::FlushFuture"]},{"text":"impl&lt;'_, T:&nbsp;<a class=\"trait\" href=\"futures_io/if_std/trait.AsyncWrite.html\" title=\"trait futures_io::if_std::AsyncWrite\">AsyncWrite</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/io/struct.CloseFuture.html\" title=\"struct futures_lite::io::CloseFuture\">CloseFuture</a>&lt;'_, T&gt;","synthetic":false,"types":["futures_lite::io::CloseFuture"]},{"text":"impl&lt;'_, T:&nbsp;<a class=\"trait\" href=\"futures_core/stream/trait.Stream.html\" title=\"trait futures_core::stream::Stream\">Stream</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/stream/struct.NextFuture.html\" title=\"struct futures_lite::stream::NextFuture\">NextFuture</a>&lt;'_, T&gt;","synthetic":false,"types":["futures_lite::stream::NextFuture"]},{"text":"impl&lt;St, C&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/stream/struct.CollectFuture.html\" title=\"struct futures_lite::stream::CollectFuture\">CollectFuture</a>&lt;St, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;St: <a class=\"trait\" href=\"futures_core/stream/trait.Stream.html\" title=\"trait futures_core::stream::Stream\">Stream</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;St::<a class=\"type\" href=\"futures_core/stream/trait.Stream.html#associatedtype.Item\" title=\"type futures_core::stream::Stream::Item\">Item</a>&gt;,&nbsp;</span>","synthetic":false,"types":["futures_lite::stream::CollectFuture"]},{"text":"impl&lt;T, E, St, C&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/stream/struct.TryCollectFuture.html\" title=\"struct futures_lite::stream::TryCollectFuture\">TryCollectFuture</a>&lt;St, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;St: <a class=\"trait\" href=\"futures_core/stream/trait.Stream.html\" title=\"trait futures_core::stream::Stream\">Stream</a>&lt;Item = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;T, E&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":["futures_lite::stream::TryCollectFuture"]},{"text":"impl&lt;S, F, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/stream/struct.FoldFuture.html\" title=\"struct futures_lite::stream::FoldFuture\">FoldFuture</a>&lt;S, F, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"futures_core/stream/trait.Stream.html\" title=\"trait futures_core::stream::Stream\">Stream</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(B, S::<a class=\"type\" href=\"futures_core/stream/trait.Stream.html#associatedtype.Item\" title=\"type futures_core::stream::Stream::Item\">Item</a>) -&gt; B,&nbsp;</span>","synthetic":false,"types":["futures_lite::stream::FoldFuture"]},{"text":"impl&lt;'a, T, E, S, F, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_lite/stream/struct.TryFoldFuture.html\" title=\"struct futures_lite::stream::TryFoldFuture\">TryFoldFuture</a>&lt;'a, S, F, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"futures_core/stream/trait.Stream.html\" title=\"trait futures_core::stream::Stream\">Stream</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S::<a class=\"type\" href=\"futures_core/stream/trait.Stream.html#associatedtype.Item\" title=\"type futures_core::stream::Stream::Item\">Item</a>: Result&lt;Ok = T, Err = E&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(B, T) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;B, E&gt;,&nbsp;</span>","synthetic":false,"types":["futures_lite::stream::TryFoldFuture"]}];
implementors["futures_task"] = [{"text":"impl&lt;'_, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_task/struct.LocalFutureObj.html\" title=\"struct futures_task::LocalFutureObj\">LocalFutureObj</a>&lt;'_, T&gt;","synthetic":false,"types":["futures_task::future_obj::LocalFutureObj"]},{"text":"impl&lt;'_, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_task/struct.FutureObj.html\" title=\"struct futures_task::FutureObj\">FutureObj</a>&lt;'_, T&gt;","synthetic":false,"types":["futures_task::future_obj::FutureObj"]}];
implementors["futures_test"] = [{"text":"impl&lt;Fut:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_test/future/struct.AssertUnmoved.html\" title=\"struct futures_test::future::AssertUnmoved\">AssertUnmoved</a>&lt;Fut&gt;","synthetic":false,"types":["futures_test::future::assert_unmoved::AssertUnmoved"]},{"text":"impl&lt;Fut:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_test/future/struct.PendingOnce.html\" title=\"struct futures_test::future::PendingOnce\">PendingOnce</a>&lt;Fut&gt;","synthetic":false,"types":["futures_test::future::pending_once::PendingOnce"]},{"text":"impl&lt;Fut:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"futures_test/future/struct.InterleavePending.html\" title=\"struct futures_test::future::InterleavePending\">InterleavePending</a>&lt;Fut&gt;","synthetic":false,"types":["futures_test::interleave_pending::InterleavePending"]}];
implementors["multitask"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"multitask/struct.Task.html\" title=\"struct multitask::Task\">Task</a>&lt;T&gt;","synthetic":false,"types":["multitask::Task"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()