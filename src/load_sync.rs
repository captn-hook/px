use std::{
    ops::Drop,
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        Arc,
    },
};

use bevy::prelude::*;
use event_listener::Event;
use futures_lite::Future;

/// [`States`] of asset loading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, States, Default)]
pub enum LoadingState {
    /// Is loading.
    #[default]
    Loading,
    /// Loading completed.
    Loaded,
}

/// This is required to support both sync and async.
///
/// For sync only the easiest implementation is
/// [`Arc<()>`] and use [`Arc::strong_count`] for completion.
/// [`Arc<Atomic>`] is a more robust alternative.
#[derive(Debug, Resource, Deref)]
pub struct AssetBarrier(Arc<AssetBarrierInner>);

/// This guard is to be acquired by [`AssetServer::load_acquire`]
/// and dropped once finished.
#[derive(Debug, Deref)]
pub struct AssetBarrierGuard(Arc<AssetBarrierInner>);

/// Tracks how many guards are remaining.
#[derive(Debug, Resource)]
pub struct AssetBarrierInner {
    count: AtomicU32,
    /// This can be omitted if async is not needed.
    notify: Event,
}

/// State of loading asynchronously.
#[derive(Debug, Resource)]
pub struct AsyncLoadingState(Arc<AtomicBool>);

/// Entities that are to be removed once loading finished
#[derive(Debug, Component)]
pub struct Loading;

/// Marker for the "Loading..." Text component.
#[derive(Debug, Component)]
pub struct LoadingText;

impl AssetBarrier {
    /// Create an [`AssetBarrier`] with a [`AssetBarrierGuard`].
    pub fn new() -> (AssetBarrier, AssetBarrierGuard) {
        let inner = Arc::new(AssetBarrierInner {
            count: AtomicU32::new(1),
            notify: Event::new(),
        });
        (AssetBarrier(inner.clone()), AssetBarrierGuard(inner))
    }

    /// Returns true if all [`AssetBarrierGuard`] is dropped.
    pub fn is_ready(&self) -> bool {
        self.count.load(Ordering::Acquire) == 0
    }

    /// Wait for all [`AssetBarrierGuard`]s to be dropped asynchronously.
    pub fn wait_async(&self) -> impl Future<Output = ()> + 'static {
        let shared = self.0.clone();
        async move {
            loop {
                // Acquire an event listener.
                let listener = shared.notify.listen();
                // If all barrier guards are dropped, return
                if shared.count.load(Ordering::Acquire) == 0 {
                    return;
                }
                // Wait for the last barrier guard to notify us
                listener.await;
            }
        }
    }
}

// Increment count on clone.
impl Clone for AssetBarrierGuard {
    fn clone(&self) -> Self {
        self.count.fetch_add(1, Ordering::AcqRel);
        AssetBarrierGuard(self.0.clone())
    }
}

// Decrement count on drop.
impl Drop for AssetBarrierGuard {
    fn drop(&mut self) {
        let prev = self.count.fetch_sub(1, Ordering::AcqRel);
        if prev == 1 {
            // Notify all listeners if count reaches 0.
            self.notify.notify(usize::MAX);
        }
    }
}