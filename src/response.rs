use super::twain_h::*;
use super::twain_h_ext::*;

use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ReturnCode {
	Success,
	Failure,
	CheckStatus,
	Cancel,
	DSEvent,
	NotDSEvent,
	XferDone,
	EndOfList,
	InfoNotSupported,
	DataNotAvailable,
	Busy,
	ScannerLocked,
	Unknown(TW_UINT16),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ConditionCode {
	NoConditionCode(ReturnCode),
	Success,
	Bummer,
	LowMemory,
	NoDS,
	MaxConnections,
	OperationError,
	BadCap,
	BadProtocol,
	BadValue,
	SeqError,
	BadDest,
	CapUnsupported,
	CapBadOperation,
	CapSeqError,
	Denied,
	FileExists,
	FileNotFound,
	NotEmpty,
	PaperJam,
	PaperDoubleFeed,
	FileWriteError,
	CheckDeviceOnline,
	Interlock,
	DamagedCorner,
	FocusError,
	DocTooLight,
	DocTooDark,
	NoMedia,
	Unknown(TW_UINT16),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Response {
	pub return_code: ReturnCode,
	pub condition_code: ConditionCode,
}

impl ReturnCode {
	pub fn from_rc(rc: TW_UINT16) -> Self {
		match rc as TwainUConst {
			TWRC_SUCCESS          => Self::Success,
			TWRC_FAILURE          => Self::Failure,
			TWRC_CHECKSTATUS      => Self::CheckStatus,
			TWRC_CANCEL           => Self::Cancel,
			TWRC_DSEVENT          => Self::DSEvent,
			TWRC_NOTDSEVENT       => Self::NotDSEvent,
			TWRC_XFERDONE         => Self::XferDone,
			TWRC_ENDOFLIST        => Self::EndOfList,
			TWRC_INFONOTSUPPORTED => Self::InfoNotSupported,
			TWRC_DATANOTAVAILABLE => Self::DataNotAvailable,
			TWRC_BUSY             => Self::Busy,
			TWRC_SCANNERLOCKED    => Self::ScannerLocked,
			_                     => Self::Unknown(rc)
		}
	}
}

impl fmt::Display for ReturnCode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		match self {
			Self::Success          => write!(f, "Success"),
			Self::Failure          => write!(f, "Failure"),
			Self::CheckStatus      => write!(f, "CheckStatus"),
			Self::Cancel           => write!(f, "Cancel"),
			Self::DSEvent          => write!(f, "DSEvent"),
			Self::NotDSEvent       => write!(f, "NotDSEvent"),
			Self::XferDone         => write!(f, "XferDone"),
			Self::EndOfList        => write!(f, "EndOfList"),
			Self::InfoNotSupported => write!(f, "InfoNotSupported"),
			Self::DataNotAvailable => write!(f, "DataNotAvailable"),
			Self::Busy             => write!(f, "Busy"),
			Self::ScannerLocked    => write!(f, "ScannerLocked"),
			Self::Unknown(n)       => write!(f, "Unknown({})", n),
		}
	}
}

impl ConditionCode {
	pub fn from_cc(cc: TW_UINT16) -> Self {
		match cc as TwainUConst {
			TWCC_SUCCESS           => Self::Success,
			TWCC_BUMMER            => Self::Bummer,
			TWCC_LOWMEMORY         => Self::LowMemory,
			TWCC_NODS              => Self::NoDS,
			TWCC_MAXCONNECTIONS    => Self::MaxConnections,
			TWCC_OPERATIONERROR    => Self::OperationError,
			TWCC_BADCAP            => Self::BadCap,
			TWCC_BADPROTOCOL       => Self::BadProtocol,
			TWCC_BADVALUE          => Self::BadValue,
			TWCC_SEQERROR          => Self::SeqError,
			TWCC_BADDEST           => Self::BadDest,
			TWCC_CAPUNSUPPORTED    => Self::CapUnsupported,
			TWCC_CAPBADOPERATION   => Self::CapBadOperation,
			TWCC_CAPSEQERROR       => Self::CapSeqError,
			TWCC_DENIED            => Self::Denied,
			TWCC_FILEEXISTS        => Self::FileExists,
			TWCC_FILENOTFOUND      => Self::FileNotFound,
			TWCC_NOTEMPTY          => Self::NotEmpty,
			TWCC_PAPERJAM          => Self::PaperJam,
			TWCC_PAPERDOUBLEFEED   => Self::PaperDoubleFeed,
			TWCC_FILEWRITEERROR    => Self::FileWriteError,
			TWCC_CHECKDEVICEONLINE => Self::CheckDeviceOnline,
			TWCC_INTERLOCK         => Self::Interlock,
			TWCC_DAMAGEDCORNER     => Self::DamagedCorner,
			TWCC_FOCUSERROR        => Self::FocusError,
			TWCC_DOCTOOLIGHT       => Self::DocTooLight,
			TWCC_DOCTOODARK        => Self::DocTooDark,
			TWCC_NOMEDIA           => Self::NoMedia,
			_                      => Self::Unknown(cc)
		}
	}
}

impl fmt::Display for ConditionCode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		match self {
			Self::NoConditionCode(rc) => write!(f, "No CC (RC={})", rc),
			Self::Success             => write!(f, "Success"),
			Self::Bummer              => write!(f, "Bummer"),
			Self::LowMemory           => write!(f, "LowMemory"),
			Self::NoDS                => write!(f, "NoDS"),
			Self::MaxConnections      => write!(f, "MaxConnections"),
			Self::OperationError      => write!(f, "OperationError"),
			Self::BadCap              => write!(f, "BadCap"),
			Self::BadProtocol         => write!(f, "BadProtocol"),
			Self::BadValue            => write!(f, "BadValue"),
			Self::SeqError            => write!(f, "SeqError"),
			Self::BadDest             => write!(f, "BadDest"),
			Self::CapUnsupported      => write!(f, "CapUnsupported"),
			Self::CapBadOperation     => write!(f, "CapBadOperation"),
			Self::CapSeqError         => write!(f, "CapSeqError"),
			Self::Denied              => write!(f, "Denied"),
			Self::FileExists          => write!(f, "FileExists"),
			Self::FileNotFound        => write!(f, "FileNotFound"),
			Self::NotEmpty            => write!(f, "NotEmpty"),
			Self::PaperJam            => write!(f, "PaperJam"),
			Self::PaperDoubleFeed     => write!(f, "PaperDoubleFeed"),
			Self::FileWriteError      => write!(f, "FileWriteError"),
			Self::CheckDeviceOnline   => write!(f, "CheckDeviceOnline"),
			Self::Interlock           => write!(f, "Interlock"),
			Self::DamagedCorner       => write!(f, "DamagedCorner"),
			Self::FocusError          => write!(f, "FocusError"),
			Self::DocTooLight         => write!(f, "DocTooLight"),
			Self::DocTooDark          => write!(f, "DocTooDark"),
			Self::NoMedia             => write!(f, "NoMedia"),
			Self::Unknown(n)          => write!(f, "Unknown({})", n),
		}
	}
}

impl fmt::Display for Response {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		write!(f, "RC={}, CC={}", self.return_code, self.condition_code)
	}
}
