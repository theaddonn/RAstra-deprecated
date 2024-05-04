use std::cell::Cell;
use std::io::Write;
use crate::gamepacket::GamePacket;
use rak_rs::connection::Connection as RakConnection;
use byteorder::{WriteBytesExt, ReadBytesExt};
use varint_rs::VarintWriter;
use serialize::proto::ser::MCProtoSerialize;
use crate::compression;
use crate::compression::{CompressionMethod};
use crate::info::RAKNET_GAME_PACKET_ID;

pub struct Connection {
    rak_connection: RakConnection,
    compression: Option<Box<dyn CompressionMethod>>,
    encryption: Option<Box<()>>
}

impl Connection {
    pub fn handle_login() { unimplemented!() }

    pub async fn send_gamepackets(&self, game_packets: Vec<GamePacket>) {
        let mut buf_pks = vec![];

        // Batch all gamepackets together
        for game_packet in game_packets {
            // Write gamepacket id as varint
            buf_pks.write_u64_varint(game_packet.id()).unwrap();

            // Serialize each packet
            let res = match game_packet {
                GamePacket::Login(pk) => { pk.proto_serialize(&mut buf_pks) }
                GamePacket::PlayStatus() => { unimplemented!() }
                GamePacket::ServerToClientHandshake(pk) => { pk.proto_serialize(&mut buf_pks) }
                GamePacket::ClientToServerHandshake() => { unimplemented!() }
                GamePacket::Disconnect() => { unimplemented!() }
                GamePacket::ResourcePacksInfo() => { unimplemented!() }
                GamePacket::ResourcePackStack() => { unimplemented!() }
                GamePacket::ResourcePackClientResponse() => { unimplemented!() }
                GamePacket::Text() => { unimplemented!() }
                GamePacket::SetTime() => { unimplemented!() }
                GamePacket::StartGame() => { unimplemented!() }
                GamePacket::AddPlayer() => { unimplemented!() }
                GamePacket::AddEntity() => { unimplemented!() }
                GamePacket::RemoveEntity() => { unimplemented!() }
                GamePacket::AddItemEntity() => { unimplemented!() }
                GamePacket::TakeItemEntity() => { unimplemented!() }
                GamePacket::MoveEntity() => { unimplemented!() }
                GamePacket::MovePlayer() => { unimplemented!() }
                GamePacket::RiderJump() => { unimplemented!() }
                GamePacket::UpdateBlock() => { unimplemented!() }
                GamePacket::AddPainting() => { unimplemented!() }
                GamePacket::TickSync() => { unimplemented!() }
                GamePacket::LevelSoundEventOld() => { unimplemented!() }
                GamePacket::LevelEvent() => { unimplemented!() }
                GamePacket::BlockEvent() => { unimplemented!() }
                GamePacket::EntityEvent() => { unimplemented!() }
                GamePacket::MobEffect() => { unimplemented!() }
                GamePacket::UpdateAttributes() => { unimplemented!() }
                GamePacket::InventoryTransaction() => { unimplemented!() }
                GamePacket::MobEquipment() => { unimplemented!() }
                GamePacket::MobArmorEquipment() => { unimplemented!() }
                GamePacket::Interact() => { unimplemented!() }
                GamePacket::BlockPickRequest() => { unimplemented!() }
                GamePacket::EntityPickRequest() => { unimplemented!() }
                GamePacket::PlayerAction() => { unimplemented!() }
                GamePacket::HurtArmor() => { unimplemented!() }
                GamePacket::SetEntityData() => { unimplemented!() }
                GamePacket::SetEntityMotion() => { unimplemented!() }
                GamePacket::SetEntityLink() => { unimplemented!() }
                GamePacket::SetHealth() => { unimplemented!() }
                GamePacket::SetSpawnPosition() => { unimplemented!() }
                GamePacket::Animate() => { unimplemented!() }
                GamePacket::Respawn() => { unimplemented!() }
                GamePacket::ContainerOpen() => { unimplemented!() }
                GamePacket::ContainerClose() => { unimplemented!() }
                GamePacket::PlayerHotbar() => { unimplemented!() }
                GamePacket::InventoryContent() => { unimplemented!() }
                GamePacket::InventorySlot() => { unimplemented!() }
                GamePacket::ContainerSetData() => { unimplemented!() }
                GamePacket::CraftingData() => { unimplemented!() }
                GamePacket::CraftingEvent() => { unimplemented!() }
                GamePacket::GuiDataPickItem() => { unimplemented!() }
                GamePacket::AdventureSettings() => { unimplemented!() }
                GamePacket::BlockEntityData() => { unimplemented!() }
                GamePacket::PlayerInput() => { unimplemented!() }
                GamePacket::LevelChunk() => { unimplemented!() }
                GamePacket::SetCommandsEnabled() => { unimplemented!() }
                GamePacket::SetDifficulty() => { unimplemented!() }
                GamePacket::ChangeDimension() => { unimplemented!() }
                GamePacket::SetPlayerGameType() => { unimplemented!() }
                GamePacket::PlayerList() => { unimplemented!() }
                GamePacket::SimpleEvent() => { unimplemented!() }
                GamePacket::TelemetryEvent() => { unimplemented!() }
                GamePacket::SpawnExperienceOrb() => { unimplemented!() }
                GamePacket::ClientboundMapItemData() => { unimplemented!() }
                GamePacket::MapInfoRequest() => { unimplemented!() }
                GamePacket::RequestChunkRadius() => { unimplemented!() }
                GamePacket::ChunkRadiusUpdate() => { unimplemented!() }
                GamePacket::ItemFrameDropItem() => { unimplemented!() }
                GamePacket::GameRulesChanged() => { unimplemented!() }
                GamePacket::Camera() => { unimplemented!() }
                GamePacket::BossEvent() => { unimplemented!() }
                GamePacket::ShowCredits() => { unimplemented!() }
                GamePacket::AvailableCommands() => { unimplemented!() }
                GamePacket::CommandRequest() => { unimplemented!() }
                GamePacket::CommandBlockUpdate() => { unimplemented!() }
                GamePacket::CommandOutput() => { unimplemented!() }
                GamePacket::UpdateTrade() => { unimplemented!() }
                GamePacket::UpdateEquipment() => { unimplemented!() }
                GamePacket::ResourcePackDataInfo() => { unimplemented!() }
                GamePacket::ResourcePackChunkData() => { unimplemented!() }
                GamePacket::ResourcePackChunkRequest() => { unimplemented!() }
                GamePacket::Transfer() => { unimplemented!() }
                GamePacket::PlaySound() => { unimplemented!() }
                GamePacket::StopSound() => { unimplemented!() }
                GamePacket::SetTitle() => { unimplemented!() }
                GamePacket::AddBehaviorTree() => { unimplemented!() }
                GamePacket::StructureBlockUpdate() => { unimplemented!() }
                GamePacket::ShowStoreOffer() => { unimplemented!() }
                GamePacket::PurchaseReceipt() => { unimplemented!() }
                GamePacket::PlayerSkin() => { unimplemented!() }
                GamePacket::SubClientLogin() => { unimplemented!() }
                GamePacket::InitiateWebSocketConnection() => { unimplemented!() }
                GamePacket::SetLastHurtBy() => { unimplemented!() }
                GamePacket::BookEdit() => { unimplemented!() }
                GamePacket::NpcRequest() => { unimplemented!() }
                GamePacket::PhotoTransfer() => { unimplemented!() }
                GamePacket::ModalFormRequest() => { unimplemented!() }
                GamePacket::ModalFormResponse() => { unimplemented!() }
                GamePacket::ServerSettingsRequest() => { unimplemented!() }
                GamePacket::ServerSettingsResponse() => { unimplemented!() }
                GamePacket::ShowProfile() => { unimplemented!() }
                GamePacket::SetDefaultGameType() => { unimplemented!() }
                GamePacket::RemoveObjective() => { unimplemented!() }
                GamePacket::SetDisplayObjective() => { unimplemented!() }
                GamePacket::SetScore() => { unimplemented!() }
                GamePacket::LabTable() => { unimplemented!() }
                GamePacket::UpdateBlockSynced() => { unimplemented!() }
                GamePacket::MoveEntityDelta() => { unimplemented!() }
                GamePacket::SetScoreboardIdentity() => { unimplemented!() }
                GamePacket::SetLocalPlayerAsInitialized() => { unimplemented!() }
                GamePacket::UpdateSoftEnum() => { unimplemented!() }
                GamePacket::NetworkStackLatency() => { unimplemented!() }
                GamePacket::ScriptCustomEvent() => { unimplemented!() }
                GamePacket::SpawnParticleEffect() => { unimplemented!() }
                GamePacket::AvailableEntityIdentifiers() => { unimplemented!() }
                GamePacket::LevelSoundEventV2() => { unimplemented!() }
                GamePacket::NetworkChunkPublisherUpdate() => { unimplemented!() }
                GamePacket::BiomeDefinitionList() => { unimplemented!() }
                GamePacket::LevelSoundEvent() => { unimplemented!() }
                GamePacket::LevelEventGeneric() => { unimplemented!() }
                GamePacket::LecternUpdate() => { unimplemented!() }
                GamePacket::VideoStreamConnect() => { unimplemented!() }
                GamePacket::ClientCacheStatus() => { unimplemented!() }
                GamePacket::OnScreenTextureAnimation() => { unimplemented!() }
                GamePacket::MapCreateLockedCopy() => { unimplemented!() }
                GamePacket::StructureTemplateDataExportRequest() => { unimplemented!() }
                GamePacket::StructureTemplateDataExportResponse() => { unimplemented!() }
                GamePacket::UpdateBlockProperties() => { unimplemented!() }
                GamePacket::ClientCacheBlobStatus() => { unimplemented!() }
                GamePacket::ClientCacheMissResponse() => { unimplemented!() }
                GamePacket::NetworkSettings(pk) => { pk.proto_serialize(&mut buf_pks) }
                GamePacket::PlayerAuthInput() => { unimplemented!() }
                GamePacket::CreativeContent() => { unimplemented!() }
                GamePacket::PlayerEnchantOptions() => { unimplemented!() }
                GamePacket::ItemStackRequest() => { unimplemented!() }
                GamePacket::ItemStackResponse() => { unimplemented!() }
                GamePacket::UpdatePlayerGameType() => { unimplemented!() }
                GamePacket::PacketViolationWarning() => { unimplemented!() }
                GamePacket::ItemComponent() => { unimplemented!() }
                GamePacket::FilterTextPacket() => { unimplemented!() }
                GamePacket::UpdateSubChunkBlocksPacket() => { unimplemented!() }
                GamePacket::SubChunkPacket() => { unimplemented!() }
                GamePacket::SubChunkRequestPacket() => { unimplemented!() }
                GamePacket::DimensionData() => { unimplemented!() }
                GamePacket::RequestNetworkSettings(pk) => { pk.proto_serialize(&mut buf_pks) }
                GamePacket::AlexEntityAnimation() => { unimplemented!() }
            };
        }

        // TODO: Encrypt the data

        let mut buf = vec![];

        buf.write_u8(RAKNET_GAME_PACKET_ID).unwrap();

        // Compress the data depending on compression method
        if let Some(compression) = &self.compression {
            // Write compression id
            buf.write_u8(compression.get_IDu8()).unwrap();

            buf_pks = compression.compress(&buf_pks).unwrap();
        }

        // Write final data into buf
        buf.write_all(&*buf_pks).unwrap();

        self.rak_connection.send(&*buf, true).await.unwrap()
    }
    pub fn recv_ganepackets(&self) { unimplemented!() }
}
