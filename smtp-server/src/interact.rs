use bytes::{BufMut, Bytes, BytesMut};
use tokio::prelude::*;

use smtp_message::*;

use helpers::*;

pub fn interact<
    'a,
    ReaderError,
    Reader: 'a + Stream<Item = BytesMut, Error = ReaderError>,
    WriterError,
    Writer: Sink<SinkItem = Bytes, SinkError = WriterError>,
    UserProvidedMetadata: 'a,
    HandleReaderError: 'a + FnMut(ReaderError) -> (),
    HandleWriterError: 'a + FnMut(WriterError) -> (),
    State: 'a,
    FilterFrom: 'a + Fn(&Option<Email>, &ConnectionMetadata<UserProvidedMetadata>) -> Decision<State>,
    FilterTo: 'a
        + Fn(&Email, &mut State, &ConnectionMetadata<UserProvidedMetadata>, &MailMetadata)
            -> Decision<()>,
    HandleMailReturn: 'a
        + Future<
            Item = (
                Option<Prependable<stream::MapErr<Reader, HandleReaderError>>>,
                Decision<()>,
            ),
            Error = (),
        >,
    HandleMail: 'a
        + Fn(
            MailMetadata<'static>,
            State,
            &ConnectionMetadata<UserProvidedMetadata>,
            DataStream<stream::MapErr<Reader, HandleReaderError>>,
        ) -> HandleMailReturn,
>(
    incoming: Reader,
    outgoing: &'a mut Writer,
    metadata: UserProvidedMetadata,
    handle_reader_error: HandleReaderError,
    handle_writer_error: HandleWriterError,
    filter_from: &'a FilterFrom,
    filter_to: &'a FilterTo,
    handler: &'a HandleMail,
) -> impl Future<Item = (), Error = ()> + 'a {
    let conn_meta = ConnectionMetadata { user: metadata };
    let writer = outgoing.sink_map_err(handle_writer_error).with(|c: Reply| {
        let mut w = BytesMut::with_capacity(c.byte_len()).writer();
        c.send_to(&mut w).unwrap();
        // By design of BytesMut::writer, this cannot fail so long as the buffer
        // has sufficient capacity. As if this is not respected it is a clear
        // programming error, there's no need to try and handle this cleanly.
        future::ok(w.into_inner().freeze())
    });
    CrlfLines::new(incoming.map_err(handle_reader_error).prependable())
        .fold_with_stream((writer, conn_meta, None), move |acc, line, reader| {
            handle_line(
                reader.into_inner(),
                acc,
                line,
                filter_from,
                filter_to,
                handler,
            ).and_then(|(reader, acc)| {
                future::result(reader.ok_or(()).map(|read| (CrlfLines::new(read), acc)))
            })
        })
        .map(|_acc| ()) // TODO: warn of unfinished commands?
}

fn handle_line<
    'a,
    U: 'a,
    Writer: 'a + Sink<SinkItem = Reply<'a>, SinkError = ()>,
    Reader: 'a + Stream<Item = BytesMut, Error = ()>,
    State: 'a,
    FilterFrom: 'a + Fn(&Option<Email>, &ConnectionMetadata<U>) -> Decision<State>,
    FilterTo: 'a + Fn(&Email, &mut State, &ConnectionMetadata<U>, &MailMetadata) -> Decision<()>,
    HandleMailReturn: 'a + Future<Item = (Option<Prependable<Reader>>, Decision<()>), Error = ()>,
    HandleMail: 'a
        + Fn(MailMetadata<'static>, State, &ConnectionMetadata<U>, DataStream<Reader>)
            -> HandleMailReturn,
>(
    reader: Prependable<Reader>,
    (writer, conn_meta, mail_data): (
        Writer,
        ConnectionMetadata<U>,
        Option<(MailMetadata<'static>, State)>,
    ),
    line: BytesMut,
    filter_from: &FilterFrom,
    filter_to: &FilterTo,
    handler: &HandleMail,
) -> impl Future<
    Item = (
        Option<Prependable<Reader>>,
        (
            Writer,
            ConnectionMetadata<U>,
            Option<(MailMetadata<'static>, State)>,
        ),
    ),
    Error = (),
>
         + 'a {
    // TODO: is this take_ownership actually required?
    let cmd = Command::parse(&line).map(|x| x.take_ownership());
    match cmd {
        Ok(Command::Mail(m)) => {
            if mail_data.is_some() {
                // TODO: make the message configurable
                FutIn11::Fut1(
                    send_reply(
                        writer,
                        ReplyCode::BAD_SEQUENCE,
                        (&b"Bad sequence of commands"[..]).into(),
                    ).and_then(|writer| {
                        future::ok((Some(reader), (writer, conn_meta, mail_data)))
                    }),
                )
            } else {
                match filter_from(m.from(), &conn_meta) {
                    Decision::Accept(state) => {
                        let from = m.into_from();
                        let to = Vec::new();
                        // TODO: make this "Okay" configurable
                        FutIn11::Fut2(
                            send_reply(writer, ReplyCode::OKAY, (&b"Okay"[..]).into()).and_then(
                                |writer| {
                                    future::ok((
                                        Some(reader),
                                        (
                                            writer,
                                            conn_meta,
                                            Some((MailMetadata { from, to }, state)),
                                        ),
                                    ))
                                },
                            ),
                        )
                    }
                    Decision::Reject(r) => {
                        FutIn11::Fut3(send_reply(writer, r.code, r.msg.into()).and_then(|writer| {
                            future::ok((Some(reader), (writer, conn_meta, mail_data)))
                        }))
                    }
                }
            }
        }
        Ok(Command::Rcpt(r)) => {
            if let Some((mail_meta, mut state)) = mail_data {
                match filter_to(r.to(), &mut state, &conn_meta, &mail_meta) {
                    Decision::Accept(()) => {
                        let MailMetadata { from, mut to } = mail_meta;
                        to.push(r.into_to());
                        // TODO: make this "Okay" configurable
                        FutIn11::Fut4(
                            send_reply(writer, ReplyCode::OKAY, (&b"Okay"[..]).into()).and_then(
                                |writer| {
                                    future::ok((
                                        Some(reader),
                                        (
                                            writer,
                                            conn_meta,
                                            Some((MailMetadata { from, to }, state)),
                                        ),
                                    ))
                                },
                            ),
                        )
                    }
                    Decision::Reject(r) => {
                        FutIn11::Fut5(send_reply(writer, r.code, r.msg.into()).and_then(|writer| {
                            future::ok((
                                Some(reader),
                                (writer, conn_meta, Some((mail_meta, state))),
                            ))
                        }))
                    }
                }
            } else {
                // TODO: make the message configurable
                FutIn11::Fut6(
                    send_reply(
                        writer,
                        ReplyCode::BAD_SEQUENCE,
                        (&b"Bad sequence of commands"[..]).into(),
                    ).and_then(|writer| {
                        future::ok((Some(reader), (writer, conn_meta, mail_data)))
                    }),
                )
            }
        }
        Ok(Command::Data(_)) => {
            if let Some((mail_meta, state)) = mail_data {
                if !mail_meta.to.is_empty() {
                    FutIn11::Fut7(
                        handler(mail_meta, state, &conn_meta, DataStream::new(reader)).and_then(
                            |(reader, decision)| match decision {
                                Decision::Accept(()) => future::Either::A(
                                    send_reply(writer, ReplyCode::OKAY, (&b"Okay"[..]).into())
                                        .and_then(|writer| {
                                            future::ok((reader, (writer, conn_meta, None)))
                                        }),
                                ),
                                Decision::Reject(r) => future::Either::B(
                                    send_reply(writer, r.code, r.msg.into()).and_then(|writer| {
                                        // Other mail systems (at least postfix, OpenSMTPD and
                                        // gmail) appear to drop the state on an unsuccessful
                                        // DATA command (eg. too long). Couldn't find the RFC
                                        // reference anywhere, though.
                                        future::ok((reader, (writer, conn_meta, None)))
                                    }),
                                ),
                            },
                        ),
                    )
                } else {
                    FutIn11::Fut8(
                        send_reply(
                            writer,
                            ReplyCode::BAD_SEQUENCE,
                            (&b"Bad sequence of commands"[..]).into(),
                        ).and_then(|writer| {
                            future::ok((
                                Some(reader),
                                (writer, conn_meta, Some((mail_meta, state))),
                            ))
                        }),
                    )
                }
            } else {
                FutIn11::Fut9(
                    send_reply(
                        writer,
                        ReplyCode::BAD_SEQUENCE,
                        (&b"Bad sequence of commands"[..]).into(),
                    ).and_then(|writer| {
                        future::ok((Some(reader), (writer, conn_meta, mail_data)))
                    }),
                )
            }
        }
        // TODO: this case should just no longer be needed
        Ok(_) => FutIn11::Fut10(
            // TODO: make the message configurable
            send_reply(
                writer,
                ReplyCode::COMMAND_UNIMPLEMENTED,
                (&b"Command not implemented"[..]).into(),
            ).and_then(|writer| future::ok((Some(reader), (writer, conn_meta, mail_data)))),
        ),
        Err(_) => FutIn11::Fut11(
            // TODO: make the message configurable
            send_reply(
                writer,
                ReplyCode::COMMAND_UNRECOGNIZED,
                (&b"Command not recognized"[..]).into(),
            ).and_then(|writer| future::ok((Some(reader), (writer, conn_meta, mail_data)))),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{self, cell::Cell};

    use itertools::Itertools;

    fn filter_from(addr: &Option<Email>, _: &ConnectionMetadata<()>) -> Decision<()> {
        if opt_email_repr(addr) == (&b"bad@quux.example.org"[..]).into() {
            Decision::Reject(Refusal {
                code: ReplyCode::POLICY_REASON,
                msg:  "User 'bad' banned".to_owned(),
            })
        } else {
            Decision::Accept(())
        }
    }

    fn filter_to(
        email: &Email,
        _: &mut (),
        _: &ConnectionMetadata<()>,
        _: &MailMetadata,
    ) -> Decision<()> {
        if email.localpart().as_bytes() == b"baz" {
            Decision::Reject(Refusal {
                code: ReplyCode::MAILBOX_UNAVAILABLE,
                msg:  "No user 'baz'".to_owned(),
            })
        } else {
            Decision::Accept(())
        }
    }

    fn handler<'a, R: 'a + Stream<Item = BytesMut, Error = ()>>(
        meta: MailMetadata<'static>,
        (): (),
        _: &ConnectionMetadata<()>,
        reader: DataStream<R>,
        mails: &'a Cell<Vec<(Option<Email<'a>>, Vec<Email<'a>>, BytesMut)>>,
    ) -> impl Future<Item = (Option<Prependable<R>>, Decision<()>), Error = ()> + 'a {
        reader
            .concat_and_recover()
            .map_err(|_| ())
            .and_then(move |(mail_text, reader)| {
                if mail_text.windows(5).position(|x| x == b"World").is_some() {
                    future::ok((
                        Some(reader.into_inner()),
                        Decision::Reject(Refusal {
                            code: ReplyCode::POLICY_REASON,
                            msg:  "Don't you dare say 'World'!".to_owned(),
                        }),
                    ))
                } else {
                    let mut m = mails.take();
                    m.push((meta.from, meta.to, mail_text));
                    mails.set(m);
                    future::ok((Some(reader.into_inner()), Decision::Accept(())))
                }
            })
    }

    #[test]
    fn interacts_ok() {
        let tests: &[(&[&[u8]], &[u8], &[(Option<&[u8]>, &[&[u8]], &[u8])])] = &[
            // TODO: send banner before EHLO
            // TODO: send please go on after DATA
            (
                &[b"MAIL FROM:<>\r\n\
                    RCPT TO:<baz@quux.example.org>\r\n\
                    RCPT TO:<foo2@bar.example.org>\r\n\
                    RCPT TO:<foo3@bar.example.org>\r\n\
                    DATA\r\n\
                    Hello world\r\n\
                    .\r\n\
                    QUIT\r\n"],
                b"250 Okay\r\n\
                  550 No user 'baz'\r\n\
                  250 Okay\r\n\
                  250 Okay\r\n\
                  250 Okay\r\n\
                  502 Command not implemented\r\n",
                &[(
                    None,
                    &[b"foo2@bar.example.org", b"foo3@bar.example.org"],
                    b"Hello world\r\n",
                )],
            ),
            (
                &[
                    b"MAIL FROM:<test@example.org>\r\n",
                    b"RCPT TO:<foo@example.org>\r\n",
                    b"DATA\r\n",
                    b"Hello World\r\n",
                    b".\r\n",
                    b"QUIT\r\n",
                ],
                b"250 Okay\r\n\
                  250 Okay\r\n\
                  550 Don't you dare say 'World'!\r\n\
                  502 Command not implemented\r\n",
                &[],
            ),
            (
                &[b"HELP hello\r\n"],
                b"502 Command not implemented\r\n",
                &[],
            ),
            (
                &[
                    b"MAIL FROM:<bad@quux.example.org>\r\n\
                      MAIL FROM:<foo@bar.example.org>\r\n\
                      MAIL FROM:<baz@quux.example.org>\r\n",
                    b"RCPT TO:<foo2@bar.example.org>\r\n\
                      DATA\r\n\
                      Hello\r\n",
                    b".\r\n\
                      QUIT\r\n",
                ],
                b"550 User 'bad' banned\r\n\
                  250 Okay\r\n\
                  503 Bad sequence of commands\r\n\
                  250 Okay\r\n\
                  250 Okay\r\n\
                  502 Command not implemented\r\n",
                &[(
                    Some(b"foo@bar.example.org"),
                    &[b"foo2@bar.example.org"],
                    b"Hello\r\n",
                )],
            ),
            (
                &[b"MAIL FROM:<foo@test.example.com>\r\n\
                    DATA\r\n\
                    QUIT\r\n"],
                b"250 Okay\r\n\
                  503 Bad sequence of commands\r\n\
                  502 Command not implemented\r\n",
                &[],
            ),
            (
                &[b"MAIL FROM:<foo@test.example.com>\r\n\
                    RCPT TO:<foo@bar.example.org>\r"],
                b"250 Okay\r\n",
                &[],
            ),
        ];
        for &(inp, out, mail) in tests {
            println!(
                "\nSending\n---\n{:?}---",
                inp.iter()
                    .map(|x| std::str::from_utf8(x).unwrap())
                    .collect::<Vec<&str>>()
            );
            let stream = stream::iter_ok(inp.iter().map(|x| BytesMut::from(*x)));
            let mut resp = Vec::new();
            let mut resp_mail = Cell::new(Vec::new());
            let handler_closure = |a, b, c: &_, d| handler(a, b, c, d, &resp_mail);
            interact(
                stream,
                &mut resp,
                (),
                |()| (),
                |()| (),
                &filter_from,
                &filter_to,
                &handler_closure,
            ).wait()
                .unwrap();
            let resp = resp.into_iter().concat();
            println!("Expecting\n---\n{}---", std::str::from_utf8(out).unwrap());
            println!("Got\n---\n{}---", std::str::from_utf8(&resp).unwrap());
            assert_eq!(resp, out);
            println!("Checking mails:");
            let resp_mail = resp_mail.take();
            assert_eq!(resp_mail.len(), mail.len());
            for ((fr, tr, cr), &(fo, to, co)) in resp_mail.into_iter().zip(mail) {
                println!("Mail\n---");
                let fo = fo.map(SmtpString::from);
                let fr = fr.map(|x| x.as_smtp_string());
                println!("From: expected {:?}, got {:?}", fo, fr);
                assert_eq!(fo, fr);
                let to_smtp = to.iter().map(|x| SmtpString::from(*x)).collect::<Vec<_>>();
                let tr_smtp = tr.into_iter()
                    .map(|x| x.into_smtp_string())
                    .collect::<Vec<_>>();
                println!("To: expected {:?}, got {:?}", to_smtp, tr_smtp);
                assert_eq!(to_smtp, tr_smtp);
                println!("Expected text\n--\n{}--", std::str::from_utf8(co).unwrap());
                println!("Got text\n--\n{}--", std::str::from_utf8(&cr).unwrap());
                assert_eq!(co, &cr[..]);
            }
        }
    }

    // Fuzzer-found
    #[test]
    fn interrupted_data() {
        let txt: &[&[u8]] = &[b"MAIL FROM:foo\r\n\
                                RCPT TO:bar\r\n\
                                DATA\r\n\
                                hello"];
        let stream = stream::iter_ok(txt.iter().map(|x| BytesMut::from(*x)));
        let mut resp = Vec::new();
        let resp_mail = Cell::new(Vec::new());
        let handler_closure = |a, b, c: &_, d| handler(a, b, c, d, &resp_mail);
        let res = interact(
            stream,
            &mut resp,
            (),
            |()| (),
            |()| (),
            &|_: &_, _: &_| Decision::Accept(()),
            &|_: &_, _: &mut _, _: &_, _: &_| Decision::Accept(()),
            &handler_closure,
        ).wait();
        assert!(res.is_err());
    }

    // Fuzzer-found
    #[test]
    fn no_stack_overflow() {
        let txt: &[&[u8]] = &[
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
            b"\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r\n\r\n\r\n\r\n\r\n\n\r\n\n\r",
        ];
        let stream = stream::iter_ok(txt.iter().map(|x| BytesMut::from(*x)));
        let mut resp = Vec::new();
        let resp_mail = Cell::new(Vec::new());
        let handler_closure = |a, b, c: &_, d| handler(a, b, c, d, &resp_mail);
        interact(
            stream,
            &mut resp,
            (),
            |()| (),
            |()| (),
            &|_: &_, _: &_| Decision::Accept(()),
            &|_: &_, _: &mut _, _: &_, _: &_| Decision::Accept(()),
            &handler_closure,
        ).wait()
            .unwrap();
    }
}