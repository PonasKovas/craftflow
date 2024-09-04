use crate::{
	datatypes::{Text, VarInt},
	MCPReadable, MCPWritable,
};
use serde::{Deserialize, Serialize};

impl_mcp_traits! {
	C2S: ConfigurationC2S;
	[0] ClientInformation {
		locale: String,
		view_distance: u8,
		chat_mode: ChatMode,
		chat_colors: bool,
		displayed_skin_parts: u8,
		main_hand: MainHand,
		text_filtering: bool,
		allow_server_listing: bool,
	}
	[1] CookieResponse {
		key: String,
		payload: Option<Vec<u8>>,
	}
	[2] PluginMessageC2S {
		channel: String,
		data: Box<[u8]>,
	}
	[3] AcknowledgeFinishConfiguration {}
	[4] KeepAliveC2S {
		id: u64
	}
	[5] Pong {
		payload: i32,
	}
	[6] ResourcePackResponse {
		uuid: u128,
		result: ResPackResult,
	}
	[7] KnownPacksC2S {
		packs: Vec<KnownPack>,
	}
}

impl_mcp_traits! {
	S2C: ConfigurationS2C;
	[0] CookieRequest {
		key: String,
	}
	[1] PluginMessageS2C {
		channel: String,
		data: Box<[u8]>,
	}
	[2] Disconnect {
		reason: Text,
	}
	[3] FinishConfiguration {}
	[4] KeepAliveS2C {
		id: u64,
	}
	[5] Ping {
		id: i32
	}
	[6] ResetChat {}
	[7] RegistryData {
		id: String,
		entries: Vec<RegistryEntry>,
	}
	[8] RemoveResourcePack {
		uuid: Option<u128>,
	}
	[9] AddResourcePack {
		uuid: u128,
		url: String,
		hash: String,
		forced: bool,
		message: Option<Text>,
	}
	[10] StoreCookie {
		key: String,
		payload: Vec<u8>,
	}
	[11] Transfer {
		host: String,
		port: VarInt,
	}
	[12] FeatureFlags {
		flags: Vec<String>,
	}
	[13] UpdateTags {
		registries: Vec<TagRegistry>,
	}
	[14] KnownPacksS2C {
		packs: Vec<KnownPack>,
	}
	[15] CustomReportDetails {
		details: Vec<ReportDetail>
	}
	[16] ServerLinks {
		links: Vec<ServerLink>,
	}
}

varint_enum! {
	ChatMode {
		Enabled = 0,
		CommandsOnly = 1,
		Hidden = 2,
	}
}

varint_enum! {
	MainHand {
		Left = 0,
		Right = 1,
	}
}

varint_enum! {
	ResPackResult {
		SuccessfullyLoaded = 0,
		Declined = 1,
		FailedDownload = 2,
		Accepted = 3,
		Downloaded = 4,
		InvalidUrl = 5,
		FailedToReload = 6,
		Discarded = 7,
	}
}

mcp_struct! {
	KnownPack {
		namespace: String,
		id: String,
		version: String,
	}
}

mcp_struct! {
	RegistryEntry {
		id: String,
		data: Option<Nbt>,
	}
}

mcp_struct! {
	TagRegistry {
		registry: String,
		tags: Vec<Tag>,
	}
}

mcp_struct! {
	Tag {
		name: String,
		entries: Vec<VarInt>,
	}
}

mcp_struct! {
	ReportDetail {
		title: String,
		description: String,
	}
}

mcp_struct! {
	ServerLink {
		label: ServerLinkLabel,
		url: String,
	}
}

#[derive(Debug, Clone)]
pub enum ServerLinkLabel {
	BuiltIn(BuiltInLabel),
	Custom(Text),
}

varint_enum! {
	BuiltInLabel {
		BugReport = 0,
		CommunityGuidelines = 1,
		Support = 2,
		Status = 3,
		Feedback = 4,
		Community = 5,
		Website = 6,
		Forums = 7,
		News = 8,
		Announcements = 9,
	}
}

impl MCPReadable for ServerLinkLabel {
	fn read(source: &mut impl std::io::Read) -> anyhow::Result<Self>
	where
		Self: Sized,
	{
		let builtin = bool::read(source)?;
		if builtin {
			Ok(ServerLinkLabel::BuiltIn(BuiltInLabel::read(source)?))
		} else {
			Ok(ServerLinkLabel::Custom(Text::read(source)?))
		}
	}
}

impl MCPWritable for ServerLinkLabel {
	fn write(&self, to: &mut impl std::io::Write) -> anyhow::Result<usize> {
		match self {
			ServerLinkLabel::BuiltIn(builtin) => {
				true.write(to)?;
				builtin.write(to)
			}
			ServerLinkLabel::Custom(text) => {
				false.write(to)?;
				text.write(to)
			}
		}
	}
}
