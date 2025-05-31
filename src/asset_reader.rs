use std::path::Path;

use bevy::{
    asset::io::{
        AssetReader, AssetReaderError, AssetSource, AssetSourceId, ErasedAssetReader, PathStream,
        Reader,
    },
    prelude::*,
};

pub fn plugin(app: &mut App) {
    app.register_asset_source(
        AssetSourceId::Default,
        AssetSource::build().with_reader(|| {
            Box::new(CustomAssetReader(
                // This is the default reader for the current platform
                AssetSource::get_default_reader("assets".to_string())(),
            ))
        }),
    );
}

/// A custom asset reader implementation that wraps a given asset reader implementation
struct CustomAssetReader(Box<dyn ErasedAssetReader>);

impl AssetReader for CustomAssetReader {
    async fn read<'a>(&'a self, path: &'a Path) -> Result<impl Reader + 'a, AssetReaderError> {
        info!("Reading {}", path.display());
        self.0.read(path).await
    }
    async fn read_meta<'a>(&'a self, path: &'a Path) -> Result<impl Reader + 'a, AssetReaderError> {
        info!("Reading meta {}", path.display());
        self.0.read_meta(path).await
    }

    async fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> Result<Box<PathStream>, AssetReaderError> {
        info!("Reading directory {}", path.display());
        self.0.read_directory(path).await
    }

    async fn is_directory<'a>(&'a self, path: &'a Path) -> Result<bool, AssetReaderError> {
        let result = self.0.is_directory(path).await;
        info!(
            "wondering if {} directory. Answer: {:?}",
            path.display(),
            result
        );
        result
    }
}
