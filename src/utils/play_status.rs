pub enum PlayStatus {
    /// Sent after Login has been successfully decoded and the player has logged in
    LoginSuccess = 0,

    /// Displays "Could not connect: Outdated client!"
    FailedClient = 1,

    /// Displays "Could not connect: Outdated server!"
    FailedSpawn = 2,

    /// Sent after world data to spawn the player
    PlayerSpawn = 3,

    /// Displays "Unable to connect to world. Your school does not have access to this server."
    FailedInvalidTenant = 4,

    /// Displays "The server is not running Minecraft: Education Edition. Failed to connect."
    FailedVanillaEdu = 5,

    /// Displays "The server is running an incompatible edition of Minecraft. Failed to connect."
    FailedEduVanilla = 6,

    /// Displays "Wow this server is popular! Check back later to see if space opens up. Server Full"
    FailedServerFull = 7,

    /// Cannot join a vanilla game on editor
    FailedEditorVanillaMismatch = 8,

    /// Cannot join an editor game on vanilla
    FailedVanillaEditorMismatch = 9,
}
