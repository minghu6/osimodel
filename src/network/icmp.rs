use std::{fmt::Debug, mem::transmute};

use m6tobytes::*;

use super::{InetCkSum, inet_cksum};


////////////////////////////////////////////////////////////////////////////////
//// Structures



#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[derive_to_bits_into(ICMPType)]
#[derive_to_bits(u8)]
#[non_exhaustive]
#[repr(u8)]
pub enum ICMPTypeKind {
    EchoReply = 0,
    DestinationUnreachable = 3,
    RedirectMessage = 5,
    EchoRequest = 8,
    /// [Router Advertisement](https://en.wikipedia.org/wiki/ICMP_Router_Discovery_Protocol)
    RouterAdvertisement = 9,
    RouterSolicitation = 10,
    TimeExceeded = 11,
    /// Bad IP header
    BadParam = 12,
    Timestamp = 13,
    TimestampReply = 14,
    ExtendedEchoRequest = 42,
    ExtendedEchoReply = 43,
    Oth(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, FromBytes, ToBytes)]
#[derive_to_bits(u8)]
#[repr(transparent)]
pub struct ICMPType(u8);

#[derive(Clone, Copy, PartialEq, Eq, Hash, FromBytes, ToBytes, Default)]
#[derive_to_bits(u8)]
#[repr(transparent)]
pub struct ICMPCode(u8);

pub struct DebugICMPCode {
    ty: ICMPType,
    code: ICMPCode,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[derive_to_bits_into(ICMPCode)]
#[derive_to_bits(u8)]
#[repr(u8)]
pub enum UnreachCode {
    DstNetworkUnreachable = 0,
    DstHostUnreachable = 1,
    DstProtocolUnreachable = 2,
    DstPortUnreachable = 3,
    FragRequiredDFFlagset = 4,
    SrcRouteFailed = 5,
    DstNetworkUnknown = 6,
    DstHostUnknown = 7,
    SrcHostIsolated = 8,
    NetworkAdmiProhibited = 9,
    HostAdmiProhibited = 10,
    NetworkUnreachableforToS = 11,
    HostUnreachableforToS = 12,
    /// 13 Communication Administratively Prohibited
    CommunicationAdmiProhibited = 13,
    HostPrecedenceViolation = 14,
    /// 15 Sent by a router when receiving a datagram whose Precedence value (priority)
    /// is lower than the minimum allowed for the network at that time.
    PrecedenceCutOff = 15,
    Oth(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[derive_to_bits_into(ICMPCode)]
#[derive_to_bits(u8)]
#[repr(u8)]
pub enum RedirectCode {
    ForNetwork = 0,
    ForHost = 1,
    ForToSAndNetwork = 2,
    ForToSAndHost = 3,
    Oth(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[derive_to_bits_into(ICMPCode)]
#[derive_to_bits(u8)]
#[repr(u8)]
pub enum TimeExceededCode {
    TTLExpired = 0,
    FragmentReassemblyTimeExceeded = 1,
    Oth(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[derive_to_bits_into(ICMPCode)]
#[derive_to_bits(u8)]
#[repr(u8)]
pub enum BadIPHeaderCode {
    PtrIndicatesError = 0,
    MissingRequiredOption = 1,
    BadLen = 2,
    Oth(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[derive_to_bits_into(ICMPCode)]
#[derive_to_bits(u8)]
#[repr(u8)]
pub enum ExtendedErrorCode {
    NoError = 0,
    MalformedQuery = 1,
    NoSuchInterface = 2,
    NoSuchTableEntry = 3,
    MultipleInterfacesSatisfyQuery = 4,
    Oth(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ICMP {
    pub ty: ICMPType,
    pub code: ICMPCode,
    pub cksum: InetCkSum,
    pub un: u32,
}

////////////////////////////////////////////////////////////////////////////////
//// Implementations

impl ICMP {
    pub fn checksummed(mut self) -> Self {
        self.cksum = Default::default();
        self.cksum = inet_cksum(self.as_buf()).into();

        self
    }

    pub fn verify_cksum(&self) -> bool {
        inet_cksum(self.as_buf()) == 0
    }

    pub fn as_buf(&self) -> &[u8] {
        as_raw_slice(self)
    }

    pub fn debug_icmp_code(&self) -> DebugICMPCode {
        DebugICMPCode::new(self.ty, self.code)
    }
}

impl Debug for ICMP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ICMP")
            .field("ty", &self.ty)
            .field(
                "code",
                &DebugICMPCode::new(self.ty, self.code),
            )
            .field("cksum", &self.cksum)
            .field("un", &self.un)
            .finish()
    }
}

impl Debug for ICMPType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", ICMPTypeKind::from(*self))
    }
}

impl From<ICMPCode> for UnreachCode {
    fn from(value: ICMPCode) -> Self {
        let v = value.to_bits();

        match v {
            ..=15 => unsafe { transmute(v as u16) },
            _ => Self::Oth(v),
        }
    }
}

impl From<ICMPCode> for RedirectCode {
    fn from(value: ICMPCode) -> Self {
        let v = value.to_bits();

        match v {
            ..=3 => unsafe { transmute(v as u16) },
            _ => Self::Oth(v),
        }
    }
}

impl From<ICMPCode> for TimeExceededCode {
    fn from(value: ICMPCode) -> Self {
        let v = value.to_bits();

        match v {
            0 | 1 => unsafe { transmute(v as u16) },
            _ => Self::Oth(v),
        }
    }
}

impl From<ICMPCode> for BadIPHeaderCode {
    fn from(value: ICMPCode) -> Self {
        let v = value.to_bits();

        match v {
            ..=2 => unsafe { transmute(v as u16) },
            _ => Self::Oth(v),
        }
    }
}

impl From<ICMPCode> for ExtendedErrorCode {
    fn from(value: ICMPCode) -> Self {
        let v = value.to_bits();

        match v {
            ..=4 => unsafe { transmute(v as u16) },
            _ => Self::Oth(v),
        }
    }
}

impl From<ICMPType> for ICMPTypeKind {
    fn from(value: ICMPType) -> Self {
        use ICMPTypeKind::*;

        match value.to_bits() {
            0 => EchoReply,
            3 => DestinationUnreachable,
            5 => RedirectMessage,
            8 => EchoRequest,
            9 => RouterAdvertisement,
            10 => RouterSolicitation,
            11 => TimeExceeded,
            12 => BadParam,
            13 => Timestamp,
            14 => TimestampReply,
            42 => ExtendedEchoRequest,
            43 => ExtendedEchoReply,
            x => Oth(x)
        }
    }
}

impl DebugICMPCode {
    pub fn new(ty: ICMPType, code: ICMPCode) -> Self {
        Self {
            ty,
            code,
        }
    }
}

impl Debug for DebugICMPCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ICMPTypeKind::*;

        let ty = self.ty.into();
        let code = self.code;

        match ty {
            EchoReply => write!(f, "{}", code.0),
            DestinationUnreachable => {
                write!(f, "{:?}", UnreachCode::from(code))
            }
            RedirectMessage => write!(f, "{:?}", RedirectCode::from(code)),
            EchoRequest => write!(f, "{}", code.0),
            RouterAdvertisement => write!(f, "{}", code.0),
            RouterSolicitation => write!(f, "{}", code.0),
            TimeExceeded => write!(f, "{:?}", TimeExceededCode::from(code)),
            BadParam => write!(f, "{:?}", BadIPHeaderCode::from(code)),
            Timestamp => write!(f, "{}", code.0),
            TimestampReply => write!(f, "{}", code.0),
            ExtendedEchoRequest => write!(f, "{}", code.0),
            ExtendedEchoReply => {
                write!(f, "{:?}", ExtendedErrorCode::from(code))
            }
            Oth(..) => write!(f, "{}", code.0),
        }
    }
}
