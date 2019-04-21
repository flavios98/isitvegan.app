use is_it_vegan::{
    constant::*,
    search_engine::{ElasticSearch, SearchEngine},
    item_loader::{ItemLoader, TomlItemLoader}
};

fn main() {
    let search_engine = ElasticSearch::try_new(ELASTICSEARCH_ADDRESS, ELASTICSEARCH_PORT).unwrap();
    let item_loader = TomlItemLoader::new(ITEMS_TOML.to_string());

    let items = item_loader.load_items().unwrap();

    search_engine.wipe_storage().unwrap();
    search_engine.import_items(&items.items).unwrap();
}

