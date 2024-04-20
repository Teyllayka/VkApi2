<h1 style="text-align: center;">VkApi2</h1>

[![Crates.io](https://img.shields.io/crates/v/vkapi2)](https://crates.io/crates/vkapi2)
[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg)](https://www.paypal.com/donate/?hosted_button_id=HPUSR7EB559TU)
![Crates.io](https://img.shields.io/crates/d/vkapi2)

# Async Rust wrapper of [vk](https://dev.vk.com/ru/method)

## Please fell free to contribute

### create api:

```rust
let api = VkApi::new(Some("service_key"), Some("group_key"), Some("flow_key"), Some("version, default is 5.131"))
```

### call function:

```rust
let response = vkapi2::account::methods::get_info(
  &api,
  Some(GetInfoOptions {
      lang: true,
      country: true,
      ..Default::default()
  }),
)
//or
let v = vkapi2::account::methods::get_info(&api, None).await?;
```

### And get your results

```rust
 println!("{:?}", v.country);

```

- **Account**

  - [x] account.ban
  - [ ] account.changePassword
  - [ ] account.getActiveOffers
  - [ ] account.getAppPermissions
  - [x] account.getBanned
  - [ ] account.getCounters
  - [x] account.getInfo
  - [ ] account.getProfileInfo
  - [ ] account.getPushSettings
  - [ ] account.lookupContacts
  - [ ] account.registerDevice
  - [ ] account.saveProfileInfo
  - [ ] account.setInfo
  - [ ] account.setNameInMenu
  - [x] account.setOffline
  - [x] account.setOnline
  - [ ] account.setPushSettings
  - [ ] account.setSilenceMode
  - [x] account.unban
  - [ ] account.unregisterDevice

- **Ads**

  - [ ] ads.addOfficeUsers
  - [ ] ads.checkLink
  - [ ] ads.createAds
  - [ ] ads.createCampaigns
  - [ ] ads.createClients
  - [ ] ads.createLookalikeRequest
  - [ ] ads.createTargetGroup
  - [ ] ads.createTargetPixel
  - [ ] ads.deleteAds
  - [ ] ads.deleteCampaigns
  - [ ] ads.deleteClients
  - [ ] ads.deleteTargetGroup
  - [ ] ads.deleteTargetPixel
  - [ ] ads.getAccounts
  - [ ] ads.getAds
  - [ ] ads.getAdsLayout
  - [ ] ads.getAdsPostsReach
  - [ ] ads.getAdsTargeting
  - [ ] ads.getBudget
  - [ ] ads.getCampaigns
  - [ ] ads.getCategories
  - [ ] ads.getClients
  - [ ] ads.getDemographics
  - [ ] ads.getFloodStats
  - [ ] ads.getLookalikeRequests
  - [ ] ads.getMusicians
  - [ ] ads.getMusiciansByIds
  - [ ] ads.getOfficeUsers
  - [ ] ads.getPostsReach
  - [ ] ads.getRejectionReason
  - [ ] ads.getStatistics
  - [ ] ads.getSuggestions
  - [ ] ads.getTargetGroups
  - [ ] ads.getTargetPixels
  - [ ] ads.getTargetingStats
  - [ ] ads.getUploadURL
  - [ ] ads.getVideoUploadURL
  - [ ] ads.importTargetContacts
  - [ ] ads.removeOfficeUsers
  - [ ] ads.removeTargetContacts
  - [ ] ads.saveLookalikeRequestResult
  - [ ] ads.shareTargetGroup
  - [ ] ads.updateAds
  - [ ] ads.updateCampaigns
  - [ ] ads.updateClients
  - [ ] ads.updateOfficeUsers
  - [ ] ads.updateTargetGroup
  - [ ] ads.updateTargetPixel

- **AppWidgets**

  - [ ] appWidgets.getAppImageUploadServer
  - [ ] appWidgets.getAppImages
  - [ ] appWidgets.getGroupImageUploadServer
  - [ ] appWidgets.getGroupImages
  - [ ] appWidgets.getImagesById
  - [ ] appWidgets.saveAppImage
  - [ ] appWidgets.saveGroupImage
  - [ ] appWidgets.update

- **Apps**

  - [ ] apps.addUsersToTestingGroup
  - [x] apps.deleteAppRequests
  - [ ] apps.get
  - [ ] apps.getCatalog
  - [ ] apps.getFriendsList
  - [ ] apps.getLastUploadedVersion
  - [ ] apps.getLeaderboard
  - [ ] apps.getMiniAppPolicies
  - [ ] apps.getScopes
  - [ ] apps.getScore
  - [ ] apps.getTestingGroups
  - [ ] apps.isNotificationsAllowed
  - [ ] apps.promoHasActiveGift
  - [ ] apps.promoUseGift
  - [ ] apps.removeTestingGroup
  - [ ] apps.removeUsersFromTestingGroups
  - [ ] apps.sendRequest
  - [ ] apps.updateMetaForTestingGroup

- **Asr**

  - [ ] asr.checkStatus
  - [ ] asr.getUploadUrl
  - [ ] asr.process

- **Auth**

  - [ ] auth.checkPhone
  - [ ] auth.restore

- **Board**

  - [ ] board.addTopic
  - [ ] board.closeTopic
  - [ ] board.createComment
  - [ ] board.deleteComment
  - [ ] board.deleteTopic
  - [ ] board.editComment
  - [ ] board.editTopic
  - [ ] board.fixTopic
  - [ ] board.getComments
  - [ ] board.getTopics
  - [ ] board.openTopic
  - [ ] board.restoreComment
  - [x] board.unfixTopic

- **Bugtracker**

  - [ ] bugtracker.addCompanyGroupsMembers
  - [ ] bugtracker.addCompanyMembers
  - [ ] bugtracker.changeBugreportStatus
  - [ ] bugtracker.createComment
  - [ ] bugtracker.getBugreportById
  - [ ] bugtracker.getCompanyGroupMembers
  - [ ] bugtracker.getCompanyMembers
  - [ ] bugtracker.getDownloadVersionUrl
  - [ ] bugtracker.getProductBuildUploadServer
  - [ ] bugtracker.removeCompanyGroupMember
  - [ ] bugtracker.removeCompanyMember
  - [ ] bugtracker.saveProductVersion
  - [ ] bugtracker.setCompanyMemberRole
  - [ ] bugtracker.setProductIsOver

- **Calls**

  - [ ] calls.forceFinish
  - [ ] calls.start

- **Database**

  - [ ] database.getChairs
  - [ ] database.getCities
  - [ ] database.getCitiesById
  - [ ] database.getCountries
  - [ ] database.getCountriesById
  - [ ] database.getFaculties
  - [ ] database.getMetroStations
  - [ ] database.getMetroStationsById
  - [ ] database.getRegions
  - [ ] database.getSchoolClasses
  - [ ] database.getSchools
  - [ ] database.getUniversities

- **Docs**

  - [x] docs.add
  - [x] docs.delete
  - [ ] docs.edit
  - [ ] docs.get
  - [ ] docs.getById
  - [ ] docs.getMessagesUploadServer
  - [ ] docs.getTypes
  - [ ] docs.getUploadServer
  - [ ] docs.getWallUploadServer
  - [ ] docs.save
  - [ ] docs.search

- **Donut**

  - [ ] donut.getFriends
  - [ ] donut.getSubscription
  - [ ] donut.getSubscriptions
  - [x] donut.isDon

- **DownloadedGames**

  - [ ] downloadedGames.getPaidStatus

- **Fave**

  - [x] fave.addArticle
  - [x] fave.addLink
  - [x] fave.addPage
  - [x] fave.addPost
  - [x] fave.addProduct
  - [ ] fave.addTag
  - [x] fave.addVideo
  - [x] fave.editTag
  - [ ] fave.get
  - [ ] fave.getPages
  - [ ] fave.getTags
  - [ ] fave.markSeen
  - [ ] fave.removeArticle
  - [ ] fave.removeLink
  - [ ] fave.removePage
  - [ ] fave.removePost
  - [ ] fave.removeProduct
  - [ ] fave.removeTag
  - [ ] fave.removeVideo
  - [ ] fave.reorderTags
  - [ ] fave.setPageTags
  - [ ] fave.setTags
  - [ ] fave.trackPageInteraction

- **Friends**

  - [x] friends.add
  - [ ] friends.addList
  - [ ] friends.areFriends
  - [ ] friends.delete
  - [ ] friends.deleteAllRequests
  - [ ] friends.deleteList
  - [ ] friends.edit
  - [ ] friends.editList
  - [ ] friends.get
  - [ ] friends.getAppUsers
  - [ ] friends.getAvailableForCall
  - [ ] friends.getByPhones
  - [ ] friends.getLists
  - [ ] friends.getMutual
  - [ ] friends.getOnline
  - [ ] friends.getRecent
  - [ ] friends.getRequests
  - [ ] friends.getSuggestions
  - [ ] friends.search

- **Gifts**

  - [x] gifts.get

- **Groups**

  - [x] groups.addAddress
  - [x] groups.addCallbackServer
  - [x] groups.addLink
  - [x] groups.approveRequest
  - [x] groups.ban
  - [ ] groups.create
  - [x] groups.deleteAddress
  - [ ] groups.deleteCallbackServer
  - [x] groups.deleteLink
  - [x] groups.disableOnline
  - [ ] groups.edit
  - [ ] groups.editAddress
  - [ ] groups.editCallbackServer
  - [x] groups.editLink
  - [ ] groups.editManager
  - [ ] groups.editPlace
  - [x] groups.enableOnline
  - [ ] groups.get
  - [ ] groups.getAddresses
  - [ ] groups.getBanned
  - [ ] groups.getById
  - [ ] groups.getCallbackConfirmationCode
  - [ ] groups.getCallbackServerSettings
  - [ ] groups.getCallbackServers
  - [ ] groups.getCallbackSettings
  - [ ] groups.getCatalog
  - [ ] groups.getCatalogInfo
  - [ ] groups.getInvitedUsers
  - [ ] groups.getInvites
  - [ ] groups.getLongPollServer
  - [ ] groups.getLongPollSettings
  - [ ] groups.getMembers
  - [x] groups.getOnlineStatus
  - [ ] groups.getRequests
  - [ ] groups.getSettings
  - [ ] groups.getTagList
  - [ ] groups.getTokenPermissions
  - [x] groups.invite
  - [ ] groups.isMember
  - [x] groups.join
  - [x] groups.leave
  - [x] groups.removeUser
  - [ ] groups.reorderLink
  - [ ] groups.search
  - [ ] groups.setCallbackSettings
  - [ ] groups.setLongPollSettings
  - [ ] groups.setSettings
  - [ ] groups.setUserNote
  - [ ] groups.tagAdd
  - [ ] groups.tagBind
  - [ ] groups.tagDelete
  - [ ] groups.tagUpdate
  - [ ] groups.toggleMarket
  - [x] groups.unban

- **LeadForms**

  - [ ] leadForms.create
  - [ ] leadForms.delete
  - [ ] leadForms.get
  - [ ] leadForms.getLeads
  - [ ] leadForms.getUploadURL
  - [ ] leadForms.list
  - [ ] leadForms.update

- **Likes**

  - [x] likes.add
  - [x] likes.delete
  - [x] likes.getList
  - [x] likes.isLiked

- **Market**

  - [ ] market.add
  - [ ] market.addAlbum
  - [ ] market.addProperty
  - [ ] market.addPropertyVariant
  - [ ] market.addToAlbum
  - [ ] market.createComment
  - [x] market.delete
  - [ ] market.deleteAlbum
  - [ ] market.deleteComment
  - [x] market.deleteProperty
  - [ ] market.deletePropertyVariant
  - [ ] market.edit
  - [ ] market.editAlbum
  - [ ] market.editComment
  - [ ] market.editOrder
  - [ ] market.editProperty
  - [ ] market.editPropertyVariant
  - [ ] market.filterCategories
  - [ ] market.get
  - [ ] market.getAlbumById
  - [ ] market.getAlbums
  - [ ] market.getById
  - [ ] market.getCategories
  - [ ] market.getComments
  - [ ] market.getGroupOrders
  - [ ] market.getOrderById
  - [ ] market.getOrderItems
  - [ ] market.getOrders
  - [ ] market.groupItems
  - [ ] market.removeFromAlbum
  - [ ] market.reorderAlbums
  - [ ] market.reorderItems
  - [ ] market.report
  - [ ] market.reportComment
  - [ ] market.restore
  - [ ] market.restoreComment
  - [ ] market.search
  - [ ] market.searchItems
  - [ ] market.searchItemsBasic
  - [x] market.ungroupItems

- **Messages**

  - [x] messages.addChatUser
  - [ ] messages.allowMessagesFromGroup
  - [ ] messages.createChat
  - [ ] messages.delete
  - [ ] messages.deleteChatPhoto
  - [ ] messages.deleteConversation
  - [ ] messages.deleteReaction
  - [ ] messages.denyMessagesFromGroup
  - [ ] messages.edit
  - [ ] messages.editChat
  - [ ] messages.forceCallFinish
  - [ ] messages.get
  - [ ] messages.getByConversationMessageId
  - [ ] messages.getById
  - [ ] messages.getChat
  - [ ] messages.getChatPreview
  - [ ] messages.getChatUsers
  - [ ] messages.getConversationMembers
  - [ ] messages.getConversations
  - [ ] messages.getConversationsById
  - [ ] messages.getDialogs
  - [ ] messages.getHistory
  - [ ] messages.getHistoryAttachments
  - [ ] messages.getImportantMessages
  - [ ] messages.getIntentUsers
  - [ ] messages.getInviteLink
  - [ ] messages.getLastActivity
  - [ ] messages.getLongPollHistory
  - [ ] messages.getLongPollServer
  - [ ] messages.getMessagesReactions
  - [ ] messages.getReactedPeers
  - [ ] messages.getReactionsAssets
  - [ ] messages.isMessagesFromGroupAllowed
  - [ ] messages.joinChat

- **Status**

  - [x] status.set
  - [x] status.get

- **Users**

  - [x] users.report
  - [x] users.get
  - [x] users.getFollowers

- **Podcasts**

  - [x] Podcasts.searchPodcast

- **Notes**

  - [x] notes.delete
  - [x] notes.edit
  - [x] notes.deleteComment
  - [x] notes.restoreComment

- **Photos**

  - [x] photos.confirmTag
  - [x] photos.move
  - [x] photos.removeTag
  - [x] photos.reorderAlbums
  - [x] photos.reorderPhotos

- **Utils**

  - [x] utils.checkLink
  - [x] utils.deleteFromLastShortened
  - [x] utils.getLastShortenedLinks
  - [x] utils.getServerTime
  - [x] utils.getShortLink
  - [x] utils.getLinkStats
  - [x] getStatisticsUrl - not a vkapi method, used for get shortened url statistics url
  - [x] utils.resolveScreenName


- **Wall**

  - [x] Wall.checkCopyrightLink
  - [x] Wall.closeComments
  - [x] Wall.openComments
  - [x] Wall.restore
  - [x] Wall.restoreComment
  - [x] Wall.unpin
  - [x] Wall.delete
  - [x] Wall.deleteComment
  - 
- **Orders**

  - [x] Orders.updateSubscription

- **Streaming**

  - [x] Streaming.getStem
  - [x] Streaming.getSettings
  - [x] Streaming.setSettings

- **Stories**

  - [x] Stories.hideReply

