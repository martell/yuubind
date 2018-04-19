use nom::IResult;
use std::{fmt,
          io,
          str::{self, FromStr}};

use helpers::*;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct ReplyCode {
    code: u16,
}

#[cfg_attr(test, allow(dead_code))]
impl ReplyCode {
    pub const SYSTEM_STATUS: ReplyCode = ReplyCode { code: 211 };
    pub const HELP_MESSAGE: ReplyCode = ReplyCode { code: 214 };
    pub const SERVICE_READY: ReplyCode = ReplyCode { code: 220 };
    pub const CLOSING_CHANNEL: ReplyCode = ReplyCode { code: 221 };
    pub const OKAY: ReplyCode = ReplyCode { code: 250 };
    pub const USER_NOT_LOCAL_WILL_FORWARD: ReplyCode = ReplyCode { code: 251 };
    pub const CANNOT_VRFY_BUT_PLEASE_TRY: ReplyCode = ReplyCode { code: 252 };
    pub const START_MAIL_INPUT: ReplyCode = ReplyCode { code: 354 };
    pub const SERVICE_NOT_AVAILABLE: ReplyCode = ReplyCode { code: 421 };
    pub const MAILBOX_TEMPORARILY_UNAVAILABLE: ReplyCode = ReplyCode { code: 450 };
    pub const LOCAL_ERROR: ReplyCode = ReplyCode { code: 451 };
    pub const INSUFFICIENT_STORAGE: ReplyCode = ReplyCode { code: 452 };
    pub const UNABLE_TO_ACCEPT_PARAMETERS: ReplyCode = ReplyCode { code: 455 };
    pub const COMMAND_UNRECOGNIZED: ReplyCode = ReplyCode { code: 500 };
    pub const SYNTAX_ERROR: ReplyCode = ReplyCode { code: 501 };
    pub const COMMAND_UNIMPLEMENTED: ReplyCode = ReplyCode { code: 502 };
    pub const BAD_SEQUENCE: ReplyCode = ReplyCode { code: 503 };
    pub const PARAMETER_UNIMPLEMENTED: ReplyCode = ReplyCode { code: 504 };
    pub const MAILBOX_UNAVAILABLE: ReplyCode = ReplyCode { code: 550 };
    pub const POLICY_REASON: ReplyCode = ReplyCode { code: 550 };
    pub const USER_NOT_LOCAL: ReplyCode = ReplyCode { code: 551 };
    pub const EXCEEDED_STORAGE: ReplyCode = ReplyCode { code: 552 };
    pub const MAILBOX_NAME_INCORRECT: ReplyCode = ReplyCode { code: 553 };
    pub const TRANSACTION_FAILED: ReplyCode = ReplyCode { code: 554 };
    pub const MAIL_OR_RCPT_PARAMETER_UNIMPLEMENTED: ReplyCode = ReplyCode { code: 555 };

    pub fn custom(code: u16) -> ReplyCode {
        assert!(code < 1000);
        ReplyCode { code }
    }

    pub fn code(&self) -> u16 {
        self.code
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IsLastLine {
    Yes,
    No,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Clone)]
pub struct Reply<'a> {
    code:    ReplyCode,
    is_last: IsLastLine,
    line:    &'a [u8],
}

impl<'a> Reply<'a> {
    pub fn build<'b>(
        code: ReplyCode,
        is_last: IsLastLine,
        line: &'b [u8],
    ) -> Result<Reply<'b>, BuildError> {
        if line.len() > 506 {
            // 506 is 512 - strlen(code) - strlen(is_last) - strlen("\r\n")
            Err(BuildError::LineTooLong {
                length: line.len(),
                limit:  506,
            })
        } else if let Some(p) = line.iter()
            .position(|&x| !(x == 9 || (x >= 32 && x <= 126)))
        {
            Err(BuildError::DisallowedByte {
                b:   line[p],
                pos: p,
            })
        } else {
            Ok(Reply {
                code,
                is_last,
                line,
            })
        }
    }

    // Parse one line of SMTP reply
    pub fn parse(arg: &[u8]) -> Result<(Reply, &[u8]), ParseError> {
        match reply(arg) {
            IResult::Done(rem, res) => Ok((res, rem)),
            IResult::Error(e) => Err(ParseError::ParseError(e)),
            IResult::Incomplete(n) => Err(ParseError::IncompleteString(n)),
        }
    }

    pub fn send_to(&self, w: &mut io::Write) -> io::Result<()> {
        let code = &[
            ((self.code.code() % 1000) / 100) as u8 + b'0',
            ((self.code.code() % 100) / 10) as u8 + b'0',
            (self.code.code() % 10) as u8 + b'0',
        ];
        w.write_all(code)?;
        w.write_all(if self.is_last == IsLastLine::Yes {
            b" "
        } else {
            b"-"
        })?;
        w.write_all(self.line)?;
        w.write_all(b"\r\n")
    }
}

impl<'a> fmt::Debug for Reply<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "Reply {{ code: {:?}, is_last: {:?}, line: {:?} }}",
            self.code,
            self.is_last,
            bytes_to_dbg(self.line)
        )
    }
}

named!(pub reply(&[u8]) -> Reply, do_parse!(
    code: map!(
        verify!(
            map_res!(
                map_res!(take!(3), |bytes| str::from_utf8(bytes)),
                |utf8| u16::from_str(utf8)
            ),
            |x: u16| x < 1000
        ),
        ReplyCode::custom
    ) >>
    is_last: map!(alt!(tag!("-") | tag!(" ")), |b| if b == b" " { IsLastLine::Yes } else { IsLastLine::No }) >>
    line: take_until_and_consume!("\r\n") >>
    (Reply { code, is_last, line })
));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reply_not_last() {
        let r = Reply::build(ReplyCode::SERVICE_READY, IsLastLine::No, b"hello world!").unwrap();
        assert_eq!(
            r,
            Reply {
                code:    ReplyCode { code: 220 },
                is_last: IsLastLine::No,
                line:    b"hello world!",
            }
        );

        let mut res = Vec::new();
        r.send_to(&mut res).unwrap();
        assert_eq!(res, b"220-hello world!\r\n");
    }

    #[test]
    fn reply_last() {
        let r = Reply::build(ReplyCode::COMMAND_UNIMPLEMENTED, IsLastLine::Yes, b"test").unwrap();
        assert_eq!(
            r,
            Reply {
                code:    ReplyCode { code: 502 },
                is_last: IsLastLine::Yes,
                line:    b"test",
            }
        );

        let mut res = Vec::new();
        r.send_to(&mut res).unwrap();
        assert_eq!(res, b"502 test\r\n");
    }

    #[test]
    fn refuse_build() {
        assert!(
            Reply::build(
                ReplyCode::EXCEEDED_STORAGE,
                IsLastLine::Yes,
                &vec![b'a'; 1000],
            ).is_err()
        );
        assert!(Reply::build(ReplyCode::EXCEEDED_STORAGE, IsLastLine::No, b"\r").is_err());
    }

    #[test]
    fn parse_ok() {
        let tests: &[(&[u8], Reply)] = &[
            (
                b"250 All is well\r\n",
                Reply {
                    code:    ReplyCode { code: 250 },
                    is_last: IsLastLine::Yes,
                    line:    b"All is well",
                },
            ),
            (
                b"450-Temporary\r\n",
                Reply {
                    code:    ReplyCode { code: 450 },
                    is_last: IsLastLine::No,
                    line:    b"Temporary",
                },
            ),
            (
                b"354-Please do start input now\r\n",
                Reply {
                    code:    ReplyCode { code: 354 },
                    is_last: IsLastLine::No,
                    line:    b"Please do start input now",
                },
            ),
            (
                b"550 Something is really very wrong!\r\n",
                Reply {
                    code:    ReplyCode { code: 550 },
                    is_last: IsLastLine::Yes,
                    line:    b"Something is really very wrong!",
                },
            ),
        ];
        for test in tests {
            assert_eq!(reply(test.0), IResult::Done(&b""[..], test.1.clone()));
        }
    }
}
