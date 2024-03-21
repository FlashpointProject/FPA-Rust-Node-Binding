/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface GameSearch {
  filter: GameFilter
  loadRelations: GameSearchRelations
  customIdOrder?: Array<string>
  order: GameSearchOrder
  offset?: GameSearchOffset
  limit: number
  slim: boolean
  withTagFilter?: Array<string>
}
export interface GameSearchOffset {
  value: string
  title: string
  gameId: string
}
export interface GameSearchOrder {
  column: GameSearchSortable
  direction: GameSearchDirection
}
export const enum GameSearchSortable {
  TITLE = 0,
  DEVELOPER = 1,
  PUBLISHER = 2,
  SERIES = 3,
  PLATFORM = 4,
  DATEADDED = 5,
  DATEMODIFIED = 6,
  RELEASEDATE = 7,
  LASTPLAYED = 8,
  PLAYTIME = 9,
  RANDOM = 10,
  CUSTOM = 11
}
export const enum GameSearchDirection {
  ASC = 0,
  DESC = 1
}
export interface GameSearchRelations {
  tags: boolean
  platforms: boolean
  gameData: boolean
  addApps: boolean
}
export interface GameFilter {
  subfilters: Array<GameFilter>
  whitelist: FieldFilter
  blacklist: FieldFilter
  exactWhitelist: FieldFilter
  exactBlacklist: FieldFilter
  lowerThan: SizeFilter
  higherThan: SizeFilter
  equalTo: SizeFilter
  matchAny: boolean
}
export interface FieldFilter {
  id?: Array<string>
  generic?: Array<string>
  library?: Array<string>
  title?: Array<string>
  developer?: Array<string>
  publisher?: Array<string>
  series?: Array<string>
  tags?: Array<string>
  platforms?: Array<string>
  playMode?: Array<string>
  status?: Array<string>
  notes?: Array<string>
  source?: Array<string>
  originalDescription?: Array<string>
  language?: Array<string>
  applicationPath?: Array<string>
  launchCommand?: Array<string>
}
export interface SizeFilter {
  tags?: number
  platforms?: number
  dateAdded?: string
  dateModified?: string
  releaseDate?: string
  gameData?: number
  addApps?: number
  playtime?: number
  playcount?: number
  lastPlayed?: string
}
export interface PageTuple {
  id: string
  orderVal: string
  title: string
}
export interface AdditionalApp {
  id: string
  name: string
  applicationPath: string
  launchCommand: string
  autoRunBefore: boolean
  waitForExit: boolean
  parentGameId: string
}
export interface Game {
  id: string
  library: string
  title: string
  alternateTitles: string
  series: string
  developer: string
  publisher: string
  primaryPlatform: string
  platforms: TagVec
  dateAdded: string
  dateModified: string
  detailedPlatforms?: Array<Tag>
  legacyBroken: boolean
  legacyExtreme: boolean
  playMode: string
  status: string
  notes: string
  tags: TagVec
  detailedTags?: Array<Tag>
  source: string
  legacyApplicationPath: string
  legacyLaunchCommand: string
  releaseDate: string
  version: string
  originalDescription: string
  language: string
  activeDataId?: number
  activeDataOnDisk: boolean
  lastPlayed?: string
  playtime: number
  playCounter: number
  activeGameConfigId?: number
  activeGameConfigOwner?: string
  archiveState: number
  gameData?: Array<GameData>
  addApps?: Array<AdditionalApp>
}
export interface PartialGame {
  id: string
  library?: string
  title?: string
  alternateTitles?: string
  series?: string
  developer?: string
  publisher?: string
  primaryPlatform?: string
  platforms?: TagVec
  dateAdded?: string
  dateModified?: string
  legacyBroken?: boolean
  legacyExtreme?: boolean
  playMode?: string
  status?: string
  notes?: string
  tags?: TagVec
  source?: string
  legacyApplicationPath?: string
  legacyLaunchCommand?: string
  releaseDate?: string
  version?: string
  originalDescription?: string
  language?: string
  activeDataId?: number
  activeDataOnDisk?: boolean
  lastPlayed?: string
  playtime?: number
  activeGameConfigId?: number
  activeGameConfigOwner?: string
  archiveState?: number
  addApps?: Array<AdditionalApp>
}
export interface GameRedirect {
  sourceId: string
  destId: string
}
export interface GameData {
  id: number
  gameId: string
  title: string
  dateAdded: string
  sha256: string
  crc32: number
  presentOnDisk: boolean
  path?: string
  size: number
  parameters?: string
  applicationPath: string
  launchCommand: string
}
export interface PartialGameData {
  id?: number
  gameId: string
  title?: string
  dateAdded?: string
  sha256?: string
  crc32?: number
  presentOnDisk?: boolean
  path?: string
  size?: number
  parameters?: string
  applicationPath?: string
  launchCommand?: string
}
export interface PlatformAppPath {
  appPath: string
  count: number
}
export interface Tag {
  id: number
  name: string
  description: string
  dateModified: string
  aliases: Array<string>
  category?: string
}
export interface PartialTag {
  id: number
  name: string
  description?: string
  dateModified?: string
  aliases?: Array<string>
  category?: string
}
export interface TagSuggestion {
  id: number
  name: string
  matchedFrom: string
  gamesCount: number
  category?: string
}
export interface LooseTagAlias {
  id: number
  value: string
}
export interface TagCategory {
  id: number
  name: string
  color: string
  description?: string
}
export interface PartialTagCategory {
  id: number
  name: string
  color: string
  description?: string
}
export interface RemoteDeletedGamesRes {
  games: Array<RemoteDeletedGame>
}
export interface RemoteDeletedGame {
  id: string
  dateModified: string
  reason: string
}
export interface RemoteGamesRes {
  games: Array<RemoteGame>
  addApps: Array<RemoteAddApp>
  gameData: Array<RemoteGameData>
  tagRelations: Array<Array<string>>
  platformRelations: Array<Array<string>>
}
export interface RemoteGameData {
  gameId: string
  title: string
  dateAdded: string
  sha256: string
  crc32: number
  size: number
  parameters?: string
  applicationPath: string
  launchCommand: string
}
export interface RemoteAddApp {
  name: string
  applicationPath: string
  launchCommand: string
  waitForExit: boolean
  autoRunBefore: boolean
  parentGameId: string
}
export interface RemoteGame {
  id: string
  title: string
  alternateTitles: string
  series: string
  developer: string
  publisher: string
  dateAdded: string
  dateModified: string
  playMode: string
  status: string
  notes: string
  source: string
  applicationPath: string
  launchCommand: string
  releaseDate: string
  version: string
  originalDescription: string
  language: string
  library: string
  platformName: string
  archiveState: number
}
export interface RemoteCategory {
  id: number
  name: string
  color: string
  description: string
}
export interface RemoteTag {
  id: number
  name: string
  description: string
  category: string
  dateModified: string
  aliases: Array<string>
  deleted: boolean
}
export interface RemotePlatform {
  id: number
  name: string
  description: string
  dateModified: string
  aliases: Array<string>
  deleted: boolean
}
export interface ContentTreeNode {
  name: string
  expanded: boolean
  size: number
  nodeType: string
  children: Array<ContentTreeNode>
  count: number
}
export function genContentTree(root: string): Promise<ContentTreeNode>
export function copyFolder(src: string, dest: string): Promise<number>
export function parseUserSearchInput(input: string): GameSearch
export function newSubfilter(): GameFilter
export function enableDebug(): void
export function disableDebug(): void
export function debugEnabled(): boolean
export function loggerSusbcribe(callback: (...args: any[]) => any): void
export type FlashpointNode = FlashpointArchive
export class FlashpointArchive {
  constructor()
  loadDatabase(source: string): void
  searchGames(search: GameSearch): Promise<Array<Game>>
  searchGamesIndex(search: GameSearch, limit?: number | undefined | null): Promise<Array<PageTuple>>
  searchGamesTotal(search: GameSearch): Promise<number>
  searchGamesWithTag(tag: string): Promise<Array<Game>>
  searchGamesRandom(search: GameSearch, count: number): Promise<Array<Game>>
  searchTagSuggestions(partial: string, blacklist: Array<string>): Promise<Array<TagSuggestion>>
  searchPlatformSuggestions(partial: string): Promise<Array<TagSuggestion>>
  findAllGameIds(): Promise<Array<string>>
  findGame(id: string): Promise<Game | null>
  createGame(partialGame: PartialGame): Promise<Game>
  saveGame(partialGame: PartialGame): Promise<Game>
  saveGames(partialGames: Array<PartialGame>): Promise<Array<Game>>
  deleteGame(id: string): Promise<void>
  countGames(): Promise<number>
  findAddAppById(id: string): Promise<AdditionalApp | null>
  createAddApp(addApp: AdditionalApp): Promise<AdditionalApp>
  findAllTags(): Promise<Array<Tag>>
  findTag(name: string): Promise<Tag | null>
  findTagById(id: number): Promise<Tag | null>
  createTag(name: string, category?: string | undefined | null, id?: number | undefined | null): Promise<Tag>
  saveTag(partial: PartialTag): Promise<Tag>
  deleteTag(name: string): Promise<void>
  deleteTagById(id: number): Promise<void>
  countTags(): Promise<number>
  mergeTags(name: string, mergedInto: string): Promise<Tag>
  findAllPlatforms(): Promise<Array<Tag>>
  findPlatform(name: string): Promise<Tag | null>
  findPlatformById(id: number): Promise<Tag | null>
  createPlatform(name: string, id?: number | undefined | null): Promise<Tag>
  savePlatform(partial: PartialTag): Promise<Tag>
  deletePlatform(name: string): Promise<void>
  countPlatforms(): Promise<number>
  findAllTagCategories(): Promise<Array<TagCategory>>
  findTagCategory(name: string): Promise<TagCategory | null>
  findTagCategoryById(id: number): Promise<TagCategory | null>
  createTagCategory(partial: PartialTagCategory): Promise<TagCategory>
  saveTagCategory(partial: PartialTagCategory): Promise<TagCategory>
  findGameDataById(gameDataId: number): Promise<GameData | null>
  findGameData(gameId: string): Promise<Array<GameData>>
  createGameData(gameData: PartialGameData): Promise<GameData>
  saveGameData(gameData: PartialGameData): Promise<GameData>
  deleteGameData(id: number): Promise<void>
  newTagFilterIndex(search: GameSearch): Promise<void>
  findAllGameLibraries(): Promise<Array<string>>
  addGamePlaytime(id: string, seconds: number): Promise<void>
  clearPlaytimeTracking(): Promise<void>
  findAllGameStatuses(): Promise<Array<string>>
  findAllGamePlayModes(): Promise<Array<string>>
  findAllGameApplicationPaths(): Promise<Array<string>>
  findPlatformAppPaths(): Promise<Record<string, Array<PlatformAppPath>>>
  forceGamesActiveDataMostRecent(): Promise<void>
  createGameRedirect(srcId: string, destId: string): Promise<void>
  deleteGameRedirect(srcId: string, destId: string): Promise<void>
  updateApplyCategories(cats: Array<RemoteCategory>): Promise<void>
  updateApplyPlatforms(plats: Array<RemotePlatform>): Promise<void>
  updateApplyTags(tags: Array<RemoteTag>): Promise<void>
  updateApplyGames(games: RemoteGamesRes): Promise<void>
  updateDeleteGames(games: RemoteDeletedGamesRes): Promise<void>
  updateApplyRedirects(redirects: Array<GameRedirect>): Promise<void>
  optimizeDatabase(): Promise<void>
  newCustomIdOrder(customIdOrder: Array<string>): Promise<void>
}

export type TagVec = string[];
