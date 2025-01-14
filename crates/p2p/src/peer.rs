use std::{net::SocketAddr, sync::Arc};

use crate::{Manager, ManagerStreamAction, Metadata, PeerId};

/// Represents a discovered peer.
/// This is held by [Manager] to keep track of discovered peers
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DiscoveredPeer<TMetadata: Metadata> {
	#[cfg_attr(any(feature = "serde", feature = "specta"), serde(skip))]
	pub(crate) manager: Arc<Manager<TMetadata>>,
	/// get the peer id of the discovered peer
	pub peer_id: PeerId,
	/// get the metadata of the discovered peer
	pub metadata: TMetadata,
	/// get the addresses of the discovered peer
	pub addresses: Vec<SocketAddr>,
}

impl<TMetadata: Metadata> DiscoveredPeer<TMetadata> {
	/// dial will queue an event to start a connection with the peer
	pub async fn dial(self) {
		self.manager
			.emit(ManagerStreamAction::Dial {
				peer_id: self.peer_id,
				addresses: self.addresses,
			})
			.await;
	}
}

/// Represents a connected peer.
/// This is held by [Manager] to keep track of connected peers
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ConnectedPeer {
	/// get the peer id of the discovered peer
	pub peer_id: PeerId,
	/// Did I open the connection?
	pub establisher: bool,
}
