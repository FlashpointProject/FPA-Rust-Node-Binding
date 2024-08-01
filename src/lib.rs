use std::{collections::HashMap, thread};

use napi::{threadsafe_function::{ErrorStrategy::CalleeHandled, ThreadSafeCallContext, ThreadsafeFunction}, Error, JsFunction, Result, Status};
use napi_derive::napi;
use flashpoint_archive::{game::{search::{GameFilter, GameSearch, ParsedInput, PageTuple}, GameRedirect, AdditionalApp, Game, PartialGame}, game_data::{GameData, PartialGameData}, platform::PlatformAppPath, tag::{PartialTag, Tag, TagSuggestion}, tag_category::{PartialTagCategory, TagCategory}, update::{RemoteCategory, RemoteDeletedGamesRes, RemoteGamesRes, RemotePlatform, RemoteTag}, util::ContentTreeNode, FlashpointArchive};

#[napi(js_name = "FlashpointArchive")]
pub struct FlashpointNode {
    flashpoint: FlashpointArchive
}

#[napi]
impl FlashpointNode {
    #[napi(constructor)]
    pub fn new() -> Self {
        FlashpointNode {
            flashpoint: FlashpointArchive::new()
        }
    }

    #[napi]
    pub fn load_database(&mut self, source: String) -> Result<()> {
        self.flashpoint.load_database(source.as_str()).map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn search_games(&self, search: GameSearch) -> Result<Vec<Game>> {
        self.flashpoint.search_games(&search).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn search_games_index(&self, mut search: GameSearch, limit: Option<i64>) -> Result<Vec<PageTuple>> {
        self.flashpoint.search_games_index(&mut search, limit).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn search_games_total(&self, search: GameSearch) -> Result<i64> {
        self.flashpoint.search_games_total(&search).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn search_games_with_tag(&self, tag: String) -> Result<Vec<Game>> {
        self.flashpoint.search_games_with_tag(&tag).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn search_games_random(&self, search: GameSearch, count: i64) -> Result<Vec<Game>> {
        self.flashpoint.search_games_random(&search, count).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn search_tag_suggestions(&self, partial: String, blacklist: Vec<String>) -> Result<Vec<TagSuggestion>> {
        self.flashpoint.search_tag_suggestions(&partial, blacklist).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn search_platform_suggestions(&self, partial: String) -> Result<Vec<TagSuggestion>> {
        self.flashpoint.search_platform_suggestions(&partial).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_all_game_ids(&self) -> Result<Vec<String>> {
        self.flashpoint.find_all_game_ids().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_game(&self, id: String) -> Result<Option<Game>> {
        self.flashpoint.find_game(&id).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }
    
    #[napi]
    pub async fn create_game(&self, partial_game: PartialGame) -> Result<Game> {
        self.flashpoint.create_game(&partial_game).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn save_game(&self, mut partial_game: PartialGame) -> Result<Game> {
        self.flashpoint.save_game(&mut partial_game).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn save_games(&self, partial_games: Vec<PartialGame>) -> Result<Vec<Game>> {
        let mut saved_games = vec![];
        for mut game in partial_games {
            saved_games.push(self.flashpoint.save_game(&mut game).await.map_err(|e| {
                Error::new(Status::GenericFailure, e)
            })?);
        }
        Ok(saved_games)
    }

    #[napi]
    pub async fn delete_game(&self, id: String) -> Result<()> {
        self.flashpoint.delete_game(&id).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn count_games(&self) -> Result<i64> {
        self.flashpoint.count_games().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_add_app_by_id(&self, id: String) -> Result<Option<AdditionalApp>> {
        self.flashpoint.find_add_app_by_id(&id).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn create_add_app(&self, mut add_app: AdditionalApp) -> Result<AdditionalApp> {
        self.flashpoint.create_add_app(&mut add_app).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })?;
        Ok(add_app)
    }

    #[napi]
    pub async fn find_all_tags(&self) -> Result<Vec<Tag>> {
        self.flashpoint.find_all_tags().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_tag(&self, name: String) -> Result<Option<Tag>> {
        self.flashpoint.find_tag(&name).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_tag_by_id(&self, id: i64) -> Result<Option<Tag>> {
        self.flashpoint.find_tag_by_id(id).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn create_tag(&self, name: String, category: Option<String>, id: Option<i64>) -> Result<Tag> {
        self.flashpoint.create_tag(&name, category, id).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn save_tag(&self, mut partial: PartialTag) -> Result<Tag> {
        self.flashpoint.save_tag(&mut partial).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn delete_tag(&self, name: String) -> Result<()> {
        self.flashpoint.delete_tag(&name).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn delete_tag_by_id(&self, id: i64) -> Result<()> {
        self.flashpoint.delete_tag_by_id(id).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn count_tags(&self) -> Result<i64> {
        self.flashpoint.count_tags().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn merge_tags(&self, name: String, merged_into: String) -> Result<Tag> {
        self.flashpoint.merge_tags(&name, &merged_into).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_all_platforms(&self) -> Result<Vec<Tag>> {
        self.flashpoint.find_all_platforms().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_platform(&self, name: String) -> Result<Option<Tag>> {
        self.flashpoint.find_platform(&name).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_platform_by_id(&self, id: i64) -> Result<Option<Tag>> {
        self.flashpoint.find_platform_by_id(id).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn create_platform(&self, name: String, id: Option<i64>) -> Result<Tag> {
        self.flashpoint.create_platform(&name, id).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn save_platform(&self, mut partial: PartialTag) -> Result<Tag> {
        self.flashpoint.save_platform(&mut partial).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn delete_platform(&self, name: String) -> Result<()> {
        self.flashpoint.delete_platform(&name).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn count_platforms(&self) -> Result<i64> {
        self.flashpoint.count_platforms().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_all_tag_categories(&self) -> Result<Vec<TagCategory>> {
        self.flashpoint.find_all_tag_categories().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_tag_category(&self, name: String) -> Result<Option<TagCategory>> {
        self.flashpoint.find_tag_category(&name).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_tag_category_by_id(&self, id: i64) -> Result<Option<TagCategory>> {
        self.flashpoint.find_tag_category_by_id(id).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn create_tag_category(&self, partial: PartialTagCategory) -> Result<TagCategory> {
        self.flashpoint.create_tag_category(&partial).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn save_tag_category(&self, partial: PartialTagCategory) -> Result<TagCategory> {
        self.flashpoint.save_tag_category(&partial).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_game_data_by_id(&self, game_data_id: i64) -> Result<Option<GameData>> {
        self.flashpoint.find_game_data_by_id(game_data_id).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_game_data(&self, game_id: String) -> Result<Vec<GameData>> {
        self.flashpoint.find_game_data(&game_id).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn create_game_data(&self, game_data: PartialGameData) -> Result<GameData> {
        self.flashpoint.create_game_data(&game_data).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn save_game_data(&self, game_data: PartialGameData) -> Result<GameData> {
        self.flashpoint.save_game_data(&game_data).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn delete_game_data(&self, id: i64) -> Result<()> {
        self.flashpoint.delete_game_data(id).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn new_tag_filter_index(&self, mut search: GameSearch) -> Result<()> {
        self.flashpoint.new_tag_filter_index(&mut search).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_all_game_developers(&self) -> Result<Vec<String>> {
        self.flashpoint.find_all_game_developers().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_all_game_publishers(&self) -> Result<Vec<String>> {
        self.flashpoint.find_all_game_publishers().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_all_game_series(&self) -> Result<Vec<String>> {
        self.flashpoint.find_all_game_series().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_all_game_libraries(&self) -> Result<Vec<String>> {
        self.flashpoint.find_all_game_libraries().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn add_game_playtime(&self, id: String, seconds: i64) -> Result<()> {
        self.flashpoint.add_game_playtime(&id, seconds).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn clear_playtime_tracking_by_id(&self, game_id: String) -> Result<()> {
        self.flashpoint.clear_playtime_tracking_by_id(&game_id).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn clear_playtime_tracking(&self) -> Result<()> {
        self.flashpoint.clear_playtime_tracking().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_all_game_statuses(&self) -> Result<Vec<String>> {
        self.flashpoint.find_all_game_statuses().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_all_game_play_modes(&self) -> Result<Vec<String>> {
        self.flashpoint.find_all_game_play_modes().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn find_all_game_application_paths(&self) -> Result<Vec<String>> {
        self.flashpoint.find_all_game_application_paths().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }
    
    #[napi]
    pub async fn find_platform_app_paths(&self) -> Result<HashMap<String, Vec<PlatformAppPath>>> {
        self.flashpoint.find_platform_app_paths().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn force_games_active_data_most_recent(&self) -> Result<()> {
        self.flashpoint.force_games_active_data_most_recent().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn create_game_redirect(&self, src_id: String, dest_id: String) -> Result<()> {
        self.flashpoint.create_game_redirect(&src_id, &dest_id).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn delete_game_redirect(&self, src_id: String, dest_id: String) -> Result<()> {
        self.flashpoint.delete_game_redirect(&src_id, &dest_id).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn update_apply_categories(&self, cats: Vec<RemoteCategory>) -> Result<()> {
        self.flashpoint.update_apply_categories(cats).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn update_apply_platforms(&self, plats: Vec<RemotePlatform>) -> Result<()> {
        self.flashpoint.update_apply_platforms(plats).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn update_apply_tags(&self, tags: Vec<RemoteTag>) -> Result<()> {
        self.flashpoint.update_apply_tags(tags).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn update_apply_games(&self, games: RemoteGamesRes) -> Result<()> {
        self.flashpoint.update_apply_games(&games).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn update_delete_games(&self, games: RemoteDeletedGamesRes) -> Result<()> {
        self.flashpoint.update_delete_games(&games).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn update_apply_redirects(&self, redirects: Vec<GameRedirect>) -> Result<()> {
        self.flashpoint.update_apply_redirects(redirects).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }
    
    #[napi]
    pub async fn optimize_database(&self) -> Result<()> {
        self.flashpoint.optimize_database().await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }

    #[napi]
    pub async fn new_custom_id_order(&self, custom_id_order: Vec<String>) -> Result<()> {
        self.flashpoint.new_custom_id_order(custom_id_order).await.map_err(|e| {
            Error::new(Status::GenericFailure, e)
        })
    }
}

#[napi]
pub async fn gen_content_tree(root: String) -> Result<ContentTreeNode> {
    flashpoint_archive::generate_content_tree(&root).map_err(|e| {
        Error::new(Status::GenericFailure, e)
    })
}

#[napi]
pub async fn copy_folder(src: String, dest: String) -> Result<i64> {
    flashpoint_archive::copy_folder(&src, &dest)
    .map(|v| {
        v as i64
    })
    .map_err(|e| {
        Error::new(Status::GenericFailure, e)
    })
}

#[napi]
pub fn merge_game_filters(a: GameFilter, b: GameFilter) -> GameFilter {
    flashpoint_archive::merge_game_filters(&a, &b)
}

#[napi]
pub fn parse_user_search_input(input: String) -> ParsedInput {
    flashpoint_archive::game::search::parse_user_input(&input)
}

#[napi]
pub fn new_subfilter() -> GameFilter {
    GameFilter::default()
}

#[napi]
pub fn enable_debug() {
    flashpoint_archive::enable_debug();
}

#[napi]
pub fn disable_debug() {
    flashpoint_archive::disable_debug();
}

#[napi]
pub fn debug_enabled() -> bool {
    flashpoint_archive::debug_enabled()
}

#[napi]
pub fn logger_susbcribe(callback: JsFunction) -> Result<()> {
    // Convert the JsFunction to a ThreadsafeFunction for calling from any thread
    let tsfn: ThreadsafeFunction<String, CalleeHandled> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<String>| {
        ctx.env.create_string(&ctx.value).map(|v| vec![v])
    })?;

    // Assuming `rx` is your mpsc receiver for log messages
    let (_, rx) = flashpoint_archive::logger_subscribe();

    // Spawn a new thread or task to listen for messages and call the callback
    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(message) => {
                    let _ = tsfn.call(Ok(message), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
                },
                _ => ()
            }
        }
    });

    Ok(())
}