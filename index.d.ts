/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export function parseUserSearchInput(input: string): GameSearch
export type FlashpointNode = FlashpointArchive
export class FlashpointArchive {
  constructor()
  loadDatabase(source: string): void
  searchGames(search: GameSearch): Promise<Array<Game>>
  searchGamesIndex(search: GameSearch): Promise<Array<PageTuple>>
  searchGamesTotal(search: GameSearch): Promise<number>
  searchGamesWithTag(tag: string): Promise<Array<Game>>
  searchGamesRandom(search: GameSearch, count: number): Promise<Array<Game>>
  findGame(id: string): Promise<Game | null>
  createGame(partialGame: PartialGame): Promise<Game>
  saveGame(partialGame: PartialGame): Promise<Game>
  saveGames(partialGames: Array<PartialGame>): Promise<Array<Game>>
  deleteGame(id: string): Promise<void>
  countGames(): Promise<number>
  findAddAppById(id: string): Promise<AdditionalApp | null>
  findAllTags(): Promise<Array<Tag>>
  findTag(name: string): Promise<Tag | null>
  findTagById(id: number): Promise<Tag | null>
  createTag(name: string, category?: string | undefined | null): Promise<Tag>
  saveTag(partial: PartialTag): Promise<Tag>
  deleteTag(name: string): Promise<void>
  countTags(): Promise<number>
  mergeTags(name: string, mergedInto: string): Promise<Tag>
  findAllPlatforms(): Promise<Array<Tag>>
  findPlatform(name: string): Promise<Tag | null>
  findPlatformById(id: number): Promise<Tag | null>
  createPlatform(name: string): Promise<Tag>
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
  optimizeDatabase(): Promise<void>
}

export type TagVec = string[];