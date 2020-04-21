//! Data handling

use crate::{
    core::Library,
    delta::{DeltaBuilder, DeltaType},
    error::Result,
    record::Record,
    utils::{Diff, Id, Path, Subscription, TagSet},
};
use async_std::sync::Arc;

pub struct Data<'a> {
    pub(crate) inner: &'a Library,
    pub(crate) id: Option<Id>,
}

/// A data query type
#[derive(Clone, Debug)]
pub enum Query {
    /// Return a record by exact Id
    Id(Id),
    /// Get a record by it's path
    Path(Path),
    /// Make a query for the tag set
    Tag(SetQuery<TagSet>),
}

/// The result of a query to the database
#[derive(Clone, Debug)]
pub enum QueryResult {
    /// There was a single matching item
    Single(Arc<Record>),
    /// There were many matching items
    Many(Vec<Arc<Record>>),
}

/// A special type of query on a set
///
/// The query can either be run for a partial, or complete match.  The
/// complete match is synonymous with equality (`A = B`), whereas the
/// partial match does not have to be a true subset (`A ⊆ B`)
#[derive(Clone, Debug)]
pub enum SetQuery<T> {
    /// A partial match (subset)
    Partial(T),
    /// An  equality match
    Matching(T),
}

impl<'a> Data<'a> {
    pub fn drop(&'a self) -> &'a Library {
        self.inner
    }

    /// Similar to `insert`, but instead operating on a batch of Diffs
    pub async fn batch<T, D>(&self, path: Path, tags: T, data: Vec<D>) -> Result<Id>
    where
        T: Into<TagSet>,
        D: Into<Diff>,
    {
        let mut db = DeltaBuilder::new(self.id, DeltaType::Insert);
        let tags = tags.into();

        let mut store = self.inner.store.write().await;
        let id = store.batch(
            &mut db,
            self.id,
            &path,
            tags.clone(),
            data.into_iter().map(|d| d.into()).collect(),
        )?;
        drop(store);

        let mut tc = self.inner.tag_cache.write().await;
        tags.iter().fold(Ok(()), |res, t| {
            res.and_then(|_| tc.insert(self.id, path.clone(), t.clone()))
        })?;
        drop(tc);

        self.inner.subs.queue(db.make()).await;
        Ok(id)
    }

    /// Insert a new record into the library and return it's ID
    ///
    /// You need to have a valid and active user session to do so, and
    /// the `path` must be unique.
    pub async fn insert<T, D>(&self, path: Path, tags: T, data: D) -> Result<Id>
    where
        T: Into<TagSet>,
        D: Into<Diff>,
    {
        let mut db = DeltaBuilder::new(self.id, DeltaType::Insert);
        let tags = tags.into();

        let mut store = self.inner.store.write().await;
        let id = store.insert(&mut db, self.id, &path, tags.clone(), data.into())?;
        drop(store);

        let mut tc = self.inner.tag_cache.write().await;
        tags.iter().fold(Ok(()), |res, t| {
            res.and_then(|_| tc.insert(self.id, path.clone(), t.clone()))
        })?;
        drop(tc);

        self.inner.subs.queue(db.make()).await;
        Ok(id)
    }

    pub async fn delete(&self, path: Path) -> Result<()> {
        let mut db = DeltaBuilder::new(self.id, DeltaType::Delete);

        let mut store = self.inner.store.write().await;
        store.destroy(&mut db, self.id, &path)?;
        drop(store);

        let mut tc = self.inner.tag_cache.write().await;
        tc.delete_path(self.id, path)?;
        drop(tc);

        self.inner.subs.queue(db.make()).await;
        Ok(())
    }

    /// Update a record in-place
    pub async fn update<D>(&self, path: Path, diff: D) -> Result<()>
    where
        D: Into<Diff>,
    {
        let mut db = DeltaBuilder::new(self.id, DeltaType::Update);

        let mut store = self.inner.store.write().await;
        store.update(&mut db, self.id, &path, diff.into())?;
        drop(store);

        self.inner.subs.queue(db.make()).await;
        Ok(())
    }

    /// Query the database with a specific query object
    ///
    /// Request data from alexandria via a `Query` object.  A query
    /// can only touch a single parameter, such as the Record Id, the
    /// path or a set query via tags.  The data returned are snapshots
    /// or records that are immutable.  If you want to make changes to
    /// them, use `update()` with a Diff instead.
    ///
    /// Also: future writes will not propagate to the copy of the
    /// Record returned from this function, because alexandria is
    /// Copy-on-Write.  You will need to query the database again in
    /// the future.
    ///
    /// ## Examples
    ///
    /// This code makes a direct query via the path of a record.  This
    /// will only return a single record if successful.
    ///
    /// ```
    /// # use alexandria::{Builder, Library, error::Result, utils::{Tag, TagSet, Path, Query, SetQuery}};
    /// # async fn foo() -> Result<()> {
    /// # let tmp = tempfile::tempdir().unwrap();
    /// # let lib = Builder::new().offset(tmp.path()).build().unwrap();
    /// let path = Path::from("/msg:alice");
    /// lib.data(None).await?.query(Query::Path(path)).await;
    /// # Ok(()) }
    /// ```
    ///
    /// In alexandria you can "tag" records with extra metadata, which
    /// is also encrypted.  These tags are String-keyd, with an
    /// arbitrary payload and can be used to make more precise (and
    /// fast!) search queries into the database.
    ///
    /// The following code makes a query for any record that contains
    /// the provided tags.
    ///
    /// ```
    /// # use alexandria::{Builder, Library, error::Result, utils::{Tag, TagSet, Path, Query, SetQuery}};
    /// # async fn foo() -> Result<()> {
    /// # let tmp = tempfile::tempdir().unwrap();
    /// # let lib = Builder::new().offset(tmp.path()).build().unwrap();
    /// # let tag1 = Tag::new("tag1", vec![1, 3, 1, 2]);
    /// # let tag2 = Tag::new("tag2", vec![13, 12]);
    /// let tags = TagSet::from(vec![tag1, tag2]);
    /// lib.data(None).await?.query(Query::Tag(SetQuery::Partial(tags))).await;
    /// # Ok(()) }
    /// ```
    ///
    /// Lastly, a "matching" query makes sure that *only* the provided
    /// tags are present, no more.
    ///
    /// ```
    /// # use alexandria::{Builder, Library, error::Result, utils::{Tag, TagSet, Path, Query, SetQuery}};
    /// # async fn foo() -> Result<()> {
    /// # let tmp = tempfile::tempdir().unwrap();
    /// # let lib = Builder::new().offset(tmp.path()).build().unwrap();
    /// # let tag1 = Tag::new("tag1", vec![1, 3, 1, 2]);
    /// # let tag2 = Tag::new("tag2", vec![13, 12]);
    /// let tags = TagSet::from(vec![tag1, tag2]);
    /// lib.data(None).await?.query(Query::Tag(SetQuery::Matching(tags))).await;
    /// # Ok(()) }
    /// ```
    pub async fn query(&self, q: Query) -> Result<QueryResult> {
        let store = self.inner.store.read().await;
        match q {
            Query::Path(ref path) => store
                .get_path(self.id, path)
                .map(|rec| QueryResult::Single(rec)),
            Query::Tag(query) => {
                let tc = self.inner.tag_cache.read().await;

                match query {
                    SetQuery::Matching(ref tags) => tc.get_paths_matching(self.id, tags),
                    SetQuery::Partial(ref tags) => tc.get_paths(self.id, tags),
                }
                .map(|paths| {
                    paths
                        .iter()
                        .map(|p| store.get_path(self.id, p))
                        .collect::<Result<Vec<_>>>()
                        .map(|vec| QueryResult::Many(vec))
                })?
            }
            _ => unimplemented!(),
        }
    }

    /// Subscribe to future database updates via a query filter
    ///
    /// When querying repeatedly isn't an option, or would lead to
    /// decreased performance, it's also possible to register a
    /// subscription.  They use the same mechanism as Queries to
    /// filter through tags and paths, but return a type that can be
    /// async-polled for updates.
    ///
    /// This doesn't give immediate access to the data, only the path
    /// that was changed, but can then be used to make a real query
    /// into the database to get an updated set of data.
    ///
    /// ```
    /// # use alexandria::{Builder, Library, error::Result, utils::{Tag, TagSet, Path, Query, SetQuery}};
    /// # async fn foo() -> Result<()> {
    /// # let tmp = tempfile::tempdir().unwrap();
    /// # let lib = Builder::new().offset(tmp.path()).build().unwrap();
    /// # let my_tag = Tag::new("tag1", vec![1, 3, 1, 2]);
    /// let tags = TagSet::from(vec![my_tag]);
    /// let sub = lib.data(None).await?.subscribe(Query::Tag(SetQuery::Partial(tags))).await;
    ///
    /// let path = sub.next().await;
    /// let new_data = lib.data(None).await?.query(Query::Path(path)).await?;
    /// # Ok(()) }
    /// ```
    pub async fn subscribe(&self, q: Query) -> Subscription {
        self.inner.subs.add_sub(q).await
    }
}