extern HSteamListenSocket cppISteamNetworkingSockets_SteamNetworkingSockets009_CreateListenSocketIP(void *, const SteamNetworkingIPAddr *, int, const SteamNetworkingConfigValue_t *);
extern HSteamNetConnection cppISteamNetworkingSockets_SteamNetworkingSockets009_ConnectByIPAddress(void *, const SteamNetworkingIPAddr *, int, const SteamNetworkingConfigValue_t *);
extern HSteamListenSocket cppISteamNetworkingSockets_SteamNetworkingSockets009_CreateListenSocketP2P(void *, int, int, const SteamNetworkingConfigValue_t *);
extern HSteamNetConnection cppISteamNetworkingSockets_SteamNetworkingSockets009_ConnectP2P(void *, const SteamNetworkingIdentity *, int, int, const SteamNetworkingConfigValue_t *);
extern EResult cppISteamNetworkingSockets_SteamNetworkingSockets009_AcceptConnection(void *, HSteamNetConnection);
extern bool cppISteamNetworkingSockets_SteamNetworkingSockets009_CloseConnection(void *, HSteamNetConnection, int, const char *, bool);
extern bool cppISteamNetworkingSockets_SteamNetworkingSockets009_CloseListenSocket(void *, HSteamListenSocket);
extern bool cppISteamNetworkingSockets_SteamNetworkingSockets009_SetConnectionUserData(void *, HSteamNetConnection, int64);
extern int64 cppISteamNetworkingSockets_SteamNetworkingSockets009_GetConnectionUserData(void *, HSteamNetConnection);
extern void cppISteamNetworkingSockets_SteamNetworkingSockets009_SetConnectionName(void *, HSteamNetConnection, const char *);
extern bool cppISteamNetworkingSockets_SteamNetworkingSockets009_GetConnectionName(void *, HSteamNetConnection, char *, int);
extern EResult cppISteamNetworkingSockets_SteamNetworkingSockets009_SendMessageToConnection(void *, HSteamNetConnection, const void *, uint32, int, int64 *);
extern void cppISteamNetworkingSockets_SteamNetworkingSockets009_SendMessages(void *, int, winSteamNetworkingMessage_t_152 *const *, int64 *);
extern EResult cppISteamNetworkingSockets_SteamNetworkingSockets009_FlushMessagesOnConnection(void *, HSteamNetConnection);
extern int cppISteamNetworkingSockets_SteamNetworkingSockets009_ReceiveMessagesOnConnection(void *, HSteamNetConnection, winSteamNetworkingMessage_t_152 **, int);
extern bool cppISteamNetworkingSockets_SteamNetworkingSockets009_GetConnectionInfo(void *, HSteamNetConnection, SteamNetConnectionInfo_t *);
extern bool cppISteamNetworkingSockets_SteamNetworkingSockets009_GetQuickConnectionStatus(void *, HSteamNetConnection, SteamNetworkingQuickConnectionStatus *);
extern int cppISteamNetworkingSockets_SteamNetworkingSockets009_GetDetailedConnectionStatus(void *, HSteamNetConnection, char *, int);
extern bool cppISteamNetworkingSockets_SteamNetworkingSockets009_GetListenSocketAddress(void *, HSteamListenSocket, SteamNetworkingIPAddr *);
extern bool cppISteamNetworkingSockets_SteamNetworkingSockets009_CreateSocketPair(void *, HSteamNetConnection *, HSteamNetConnection *, bool, const SteamNetworkingIdentity *, const SteamNetworkingIdentity *);
extern bool cppISteamNetworkingSockets_SteamNetworkingSockets009_GetIdentity(void *, SteamNetworkingIdentity *);
extern ESteamNetworkingAvailability cppISteamNetworkingSockets_SteamNetworkingSockets009_InitAuthentication(void *);
extern ESteamNetworkingAvailability cppISteamNetworkingSockets_SteamNetworkingSockets009_GetAuthenticationStatus(void *, SteamNetAuthenticationStatus_t *);
extern HSteamNetPollGroup cppISteamNetworkingSockets_SteamNetworkingSockets009_CreatePollGroup(void *);
extern bool cppISteamNetworkingSockets_SteamNetworkingSockets009_DestroyPollGroup(void *, HSteamNetPollGroup);
extern bool cppISteamNetworkingSockets_SteamNetworkingSockets009_SetConnectionPollGroup(void *, HSteamNetConnection, HSteamNetPollGroup);
extern int cppISteamNetworkingSockets_SteamNetworkingSockets009_ReceiveMessagesOnPollGroup(void *, HSteamNetPollGroup, winSteamNetworkingMessage_t_152 **, int);
extern bool cppISteamNetworkingSockets_SteamNetworkingSockets009_ReceivedRelayAuthTicket(void *, const void *, int, SteamDatagramRelayAuthTicket *);
extern int cppISteamNetworkingSockets_SteamNetworkingSockets009_FindRelayAuthTicketForServer(void *, const SteamNetworkingIdentity *, int, SteamDatagramRelayAuthTicket *);
extern HSteamNetConnection cppISteamNetworkingSockets_SteamNetworkingSockets009_ConnectToHostedDedicatedServer(void *, const SteamNetworkingIdentity *, int, int, const SteamNetworkingConfigValue_t *);
extern uint16 cppISteamNetworkingSockets_SteamNetworkingSockets009_GetHostedDedicatedServerPort(void *);
extern SteamNetworkingPOPID cppISteamNetworkingSockets_SteamNetworkingSockets009_GetHostedDedicatedServerPOPID(void *);
extern EResult cppISteamNetworkingSockets_SteamNetworkingSockets009_GetHostedDedicatedServerAddress(void *, SteamDatagramHostedAddress *);
extern HSteamListenSocket cppISteamNetworkingSockets_SteamNetworkingSockets009_CreateHostedDedicatedServerListenSocket(void *, int, int, const SteamNetworkingConfigValue_t *);
extern EResult cppISteamNetworkingSockets_SteamNetworkingSockets009_GetGameCoordinatorServerLogin(void *, SteamDatagramGameCoordinatorServerLogin *, int *, void *);
extern HSteamNetConnection cppISteamNetworkingSockets_SteamNetworkingSockets009_ConnectP2PCustomSignaling(void *, ISteamNetworkingConnectionSignaling *, const SteamNetworkingIdentity *, int, int, const SteamNetworkingConfigValue_t *);
extern bool cppISteamNetworkingSockets_SteamNetworkingSockets009_ReceivedP2PCustomSignal(void *, const void *, int, ISteamNetworkingSignalingRecvContext *);
extern bool cppISteamNetworkingSockets_SteamNetworkingSockets009_GetCertificateRequest(void *, int *, void *, SteamNetworkingErrMsg *);
extern bool cppISteamNetworkingSockets_SteamNetworkingSockets009_SetCertificate(void *, const void *, int, SteamNetworkingErrMsg *);
extern void cppISteamNetworkingSockets_SteamNetworkingSockets009_RunCallbacks(void *);
